use anyhow::{Context, Result};
use clap::Parser;
use op_succinct_host_utils::{
    block_range::get_validated_block_range,
    fetcher::OPSuccinctDataFetcher,
    host::OPSuccinctHost,
    network::{get_network_signer, parse_fulfillment_strategy},
    stats::ExecutionStats,
    witness_cache::{load_stdin_from_cache, save_stdin_to_cache},
    witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::{get_range_elf_embedded, initialize_host};
use op_succinct_prove::execute_multi;
use op_succinct_scripts::HostExecutorArgs;
use sp1_sdk::{
    network::proto::types::FulfillmentStatus, utils, Elf, NetworkProver, ProveRequest, Prover,
    ProverClient, SP1ProofWithPublicValues,
};
use std::{
    env,
    fs,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// Helper function to wait for a proof to complete on the network.
async fn wait_for_proof(
    prover: &NetworkProver,
    proof_id: alloy_primitives::B256,
) -> Result<SP1ProofWithPublicValues> {
    const POLL_INTERVAL: Duration = Duration::from_secs(2);

    loop {
        let (status, proof) = prover.get_proof_status(proof_id).await?;
        let fulfillment_status =
            FulfillmentStatus::try_from(status.fulfillment_status()).unwrap_or_default();

        match fulfillment_status {
            FulfillmentStatus::Fulfilled => {
                return proof.ok_or_else(|| {
                    anyhow::anyhow!("Proof status is Fulfilled but proof data is missing")
                });
            }
            FulfillmentStatus::UnspecifiedFulfillmentStatus
            | FulfillmentStatus::Assigned
            | FulfillmentStatus::Requested => {
                info!("Proof {} in progress, waiting...", proof_id);
                sleep(POLL_INTERVAL).await;
            }
            FulfillmentStatus::Unfulfillable => {
                return Err(anyhow::anyhow!("Proof {} is unfulfillable", proof_id));
            }
        }
    }
}

/// Execute the OP Succinct program for multiple blocks.
#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file)
        .context(format!("Environment file not found: {}", args.env_file.display()))?;
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let host = initialize_host(Arc::new(data_fetcher.clone()));

    // If the end block is provided, check that it is less than the latest finalized block. If the
    // end block is not provided, use the latest finalized block.
    let (l2_start_block, l2_end_block) = get_validated_block_range(
        host.as_ref(),
        &data_fetcher,
        args.start,
        args.end,
        args.default_range,
    )
    .await?;

    let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

    // Helper closure to generate stdin (runs witness generation and converts to SP1Stdin)
    let generate_stdin = || async {
        let host_args =
            host.fetch(l2_start_block, l2_end_block, None, args.safe_db_fallback).await?;
        debug!("Host args: {:?}", host_args);

        let start_time = Instant::now();
        let witness = host.run(&host_args).await?;
        let duration = start_time.elapsed();

        // Convert witness to SP1Stdin
        let stdin = host.witness_generator().get_sp1_stdin(witness)?;

        // Save to cache if enabled
        if args.cache {
            let cache_path =
                save_stdin_to_cache(l2_chain_id, l2_start_block, l2_end_block, &stdin)?;
            info!("Saved stdin to cache: {}", cache_path.display());
        }

        Ok::<_, anyhow::Error>((stdin, duration))
    };

    // Check cache first if enabled (with graceful fallback)
    let (sp1_stdin, witness_generation_duration) = if args.cache {
        match load_stdin_from_cache(l2_chain_id, l2_start_block, l2_end_block) {
            Ok(Some(stdin)) => {
                info!("Loaded stdin from cache");
                (stdin, Duration::ZERO)
            }
            Ok(None) => generate_stdin().await?,
            Err(e) => {
                warn!("Failed to load cache: {e}, regenerating...");
                generate_stdin().await?
            }
        }
    } else {
        generate_stdin().await?
    };

    if args.prove {
        // When proving, use the network prover with strategy from environment.
        let use_kms_requester = env::var("USE_KMS_REQUESTER")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let network_signer = get_network_signer(use_kms_requester).await?;
        let range_proof_strategy =
            parse_fulfillment_strategy(env::var("RANGE_PROOF_STRATEGY").unwrap_or_default());

        let prover = ProverClient::builder().network().signer(network_signer).build().await;

        let pk = prover.setup(Elf::Static(get_range_elf_embedded())).await?;

        // Generate proofs in compressed mode for aggregation verification.
        let proof_id = prover
            .prove(&pk, sp1_stdin.clone())
            .compressed()
            .strategy(range_proof_strategy)
            .request()
            .await
            .context("Failed to submit proof request")?;
        info!("Proof request submitted: {}", proof_id);
        let proof = wait_for_proof(&prover, proof_id)
            .await
            .context("Failed to generate proof")?;

        // Create a proof directory for the chain ID if it doesn't exist.
        let proof_dir = format!("data/{}/proofs", l2_chain_id);
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).unwrap();
        }
        // Save the proof to the proof directory corresponding to the chain ID.
        proof
            .save(format!("{proof_dir}/{l2_start_block}-{l2_end_block}.bin"))
            .expect("saving proof failed");

        info!(
            "Proof saved to {}/proofs/{}-{}.bin",
            l2_chain_id, l2_start_block, l2_end_block
        );
    } else {
        let (block_data, report, execution_duration) =
            execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

        let stats = ExecutionStats::new(
            0,
            &block_data,
            &report,
            witness_generation_duration.as_secs(),
            execution_duration.as_secs(),
        );

        println!("Execution Stats: \n{stats:?}");

        // Create the report directory if it doesn't exist.
        let report_dir = format!("execution-reports/multi/{l2_chain_id}");
        if !std::path::Path::new(&report_dir).exists() {
            fs::create_dir_all(&report_dir)?;
        }

        let report_path =
            format!("execution-reports/multi/{l2_chain_id}/{l2_start_block}-{l2_end_block}.csv");

        // Write to CSV.
        let mut csv_writer = csv::Writer::from_path(report_path)?;
        csv_writer.serialize(&stats)?;
        csv_writer.flush()?;
    }

    Ok(())
}
