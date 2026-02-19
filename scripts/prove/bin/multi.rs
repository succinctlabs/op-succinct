use anyhow::{bail, Context, Result};
use clap::Parser;
use op_succinct_host_utils::{
    block_range::get_validated_block_range,
    fetcher::{BlockInfo, OPSuccinctDataFetcher},
    host::OPSuccinctHost,
    stats::ExecutionStats,
    witness_cache::{load_stdin_from_cache, save_stdin_to_cache},
    witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::{get_range_elf_embedded, initialize_host};
use op_succinct_prove::execute_multi;
use op_succinct_scripts::HostExecutorArgs;
use sp1_sdk::{utils, ProverClient};
use std::{
    fs,
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{debug, info, warn};

/// Execute the OP Succinct program for multiple blocks.
#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file)
        .context(format!("Environment file not found: {}", args.env_file.display()))?;
    utils::setup_logger();

    // Cache-only fast path: when --cache is set with explicit --start and --end,
    // skip all heavy RPC calls (rollup config, block range validation, host initialization)
    // and run SP1 execution purely from cached witness data.
    if args.cache && args.start.is_some() && args.end.is_some() {
        let l2_start_block = args.start.unwrap();
        let l2_end_block = args.end.unwrap();

        // Use basic constructor — avoids optimism_rollupConfig RPC call.
        let data_fetcher = OPSuccinctDataFetcher::new();

        // eth_chainId is universally supported, even on restricted endpoints.
        let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

        // Load cached witness data.
        let sp1_stdin = match load_stdin_from_cache(l2_chain_id, l2_start_block, l2_end_block) {
            Ok(Some(stdin)) => {
                info!("Loaded stdin from cache");
                stdin
            }
            Ok(None) => {
                bail!(
                    "Cache file not found for blocks {}-{}. \
                     Run without --cache first to generate the witness, or remove --cache to use full RPC flow.",
                    l2_start_block,
                    l2_end_block
                );
            }
            Err(e) => {
                bail!(
                    "Failed to load cache for blocks {}-{}: {e}. \
                     Run without --cache first to regenerate the witness.",
                    l2_start_block,
                    l2_end_block
                );
            }
        };

        if args.prove {
            let prover = ProverClient::from_env();
            let (pk, _) = prover.setup(get_range_elf_embedded());
            let proof = prover.prove(&pk, &sp1_stdin).compressed().run().unwrap();

            let proof_dir = format!("data/{}/proofs", l2_chain_id);
            if !std::path::Path::new(&proof_dir).exists() {
                fs::create_dir_all(&proof_dir).unwrap();
            }
            proof
                .save(format!("{proof_dir}/{l2_start_block}-{l2_end_block}.bin"))
                .expect("saving proof failed");
        } else {
            // Inline SP1 execution (same logic as execute_multi, without the block data RPC).
            let start_time = Instant::now();
            let prover = ProverClient::builder().mock().build();
            let (_, report) = prover
                .execute(get_range_elf_embedded(), &sp1_stdin)
                .calculate_gas(true)
                .deferred_proof_verification(false)
                .run()?;
            let execution_duration = start_time.elapsed();

            // Try to fetch block data for stats; fall back to synthetic entries if RPC fails.
            let block_data = match data_fetcher
                .get_l2_block_data_range(l2_start_block, l2_end_block)
                .await
            {
                Ok(data) => data,
                Err(e) => {
                    warn!(
                        "Failed to fetch block data for stats (RPC may be restricted): {e}. \
                         Using zeroed per-block stats; cycle/gas counts remain accurate."
                    );
                    (l2_start_block..l2_end_block)
                        .map(|block_number| BlockInfo {
                            block_number,
                            transaction_count: 0,
                            gas_used: 0,
                            total_l1_fees: 0,
                            total_tx_fees: 0,
                        })
                        .collect()
                }
            };

            let stats = ExecutionStats::new(
                0,
                &block_data,
                &report,
                0, // witness generation was cached
                execution_duration.as_secs(),
            );

            println!("Execution Stats: \n{stats:?}");

            let report_dir = format!("execution-reports/multi/{l2_chain_id}");
            if !std::path::Path::new(&report_dir).exists() {
                fs::create_dir_all(&report_dir)?;
            }

            let report_path = format!(
                "execution-reports/multi/{l2_chain_id}/{l2_start_block}-{l2_end_block}.csv"
            );

            let mut csv_writer = csv::Writer::from_path(report_path)?;
            csv_writer.serialize(&stats)?;
            csv_writer.flush()?;
        }

        return Ok(());
    }

    // Standard flow: full RPC initialization and witness generation.
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

    let prover = ProverClient::from_env();

    if args.prove {
        // If the prove flag is set, generate a proof.
        let (pk, _) = prover.setup(get_range_elf_embedded());
        // Generate proofs in compressed mode for aggregation verification.
        let proof = prover.prove(&pk, &sp1_stdin).compressed().run().unwrap();

        // Create a proof directory for the chain ID if it doesn't exist.
        let proof_dir = format!("data/{}/proofs", l2_chain_id);
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).unwrap();
        }
        // Save the proof to the proof directory corresponding to the chain ID.
        proof
            .save(format!("{proof_dir}/{l2_start_block}-{l2_end_block}.bin"))
            .expect("saving proof failed");
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
