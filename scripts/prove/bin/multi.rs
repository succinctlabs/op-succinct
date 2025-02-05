use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::{
    block_range::get_validated_block_range,
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_proof_stdin, start_server_and_native_client,
    stats::ExecutionStats,
    ProgramType,
};
use op_succinct_prove::{
    execute_multi, DEFAULT_RANGE, RANGE_ELF, RANGE_ELF_BUMP, RANGE_ELF_EMBEDDED,
};
use op_succinct_scripts::HostExecutorArgs;
use sp1_sdk::{utils, ProverClient};
use std::{fs, time::Instant};

/// Execute the OP Succinct program for multiple blocks.
#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file)?;
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;

    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };

    // If the end block is provided, check that it is less than the latest finalized block. If the end block is not provided, use the latest finalized block.
    let (l2_start_block, l2_end_block) =
        get_validated_block_range(&data_fetcher, args.start, args.end, args.default_range).await?;

    let host_cli = data_fetcher
        .get_host_cli_args(l2_start_block, l2_end_block, ProgramType::Multi, cache_mode)
        .await?;

    let start_time = Instant::now();
    let oracle = start_server_and_native_client(&host_cli).await?;
    let witness_generation_duration = start_time.elapsed();

    // Get the stdin for the block.
    let sp1_stdin = get_proof_stdin(oracle)?;

    let prover = ProverClient::from_env();

    if args.prove {
        // If the prove flag is set, generate a proof.
        let (pk, _) = prover.setup(RANGE_ELF);

        // Generate proofs in compressed mode for aggregation verification.
        let proof = prover.prove(&pk, &sp1_stdin).compressed().run().unwrap();

        // Create a proof directory for the chain ID if it doesn't exist.
        let proof_dir = format!(
            "data/{}/proofs",
            data_fetcher.get_l2_chain_id().await.unwrap()
        );
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).unwrap();
        }
        // Save the proof to the proof directory corresponding to the chain ID.
        proof
            .save(format!(
                "{}/{}-{}.bin",
                proof_dir, l2_start_block, l2_end_block
            ))
            .expect("saving proof failed");
    } else {
        let prover = ProverClient::builder().mock().build();

        // Execute the embedded ELF and the bump ELF in parallel. Show the difference between the execution report for both.
        let (embedded_result, bump_result) = rayon::join(
            || prover.execute(RANGE_ELF_EMBEDDED, &sp1_stdin).run(),
            || prover.execute(RANGE_ELF_BUMP, &sp1_stdin).run(),
        );

        let ((_, report_embedded), (_, report_bump)) =
            (embedded_result.unwrap(), bump_result.unwrap());

        println!(
            "Embedded ELF Cycle Tracker: \n{:?}. Total Cycles: {}",
            report_embedded.cycle_tracker,
            report_embedded.total_instruction_count()
        );
        println!(
            "Bump ELF Cycle Tracker: \n{:?}. Total Cycles: {}",
            report_bump.cycle_tracker,
            report_bump.total_instruction_count()
        );
    }

    Ok(())
}
