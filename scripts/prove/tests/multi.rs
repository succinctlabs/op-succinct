use std::sync::Arc;

use anyhow::Result;
use common::DEFAULT_RANGE;
use op_succinct_host_utils::{
    block_range::get_rolling_block_range,
    fetcher::OPSuccinctDataFetcher,
    host::OPSuccinctHost,
    stats::{ExecutionStats, MarkdownExecutionStats},
    witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::initialize_host;
use op_succinct_prove::execute_multi;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn execute_batch() -> Result<()> {
    if let Err(error) = dotenv::dotenv() {
        if !error.not_found() {
            return Err(error.into());
        }
    }

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let host = initialize_host(Arc::new(data_fetcher.clone()));

    // Take the latest blocks
    let (l2_start_block, l2_end_block) =
        get_rolling_block_range(&data_fetcher, DEFAULT_RANGE).await?;

    let host_args = host.fetch(l2_start_block, l2_end_block, None, false).await?;

    let witness_data = host.run(&host_args).await?;

    // Get the stdin for the block.
    let sp1_stdin = host.witness_generator().get_sp1_stdin(witness_data)?;

    let (block_data, report, execution_duration) =
        execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

    let stats = ExecutionStats::new(0, &block_data, &report, 0, execution_duration.as_secs());

    println!("Execution Stats: \n{:?}", stats.to_string());

    std::fs::write("execution_report.md", MarkdownExecutionStats::new(stats).to_string())?;

    Ok(())
}

/// A guest panic must fail the same execution path used by execute_batch.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn execute_batch_rejects_invalid_witness() -> Result<()> {
    let data_fetcher = OPSuccinctDataFetcher::new();
    let mut stdin = sp1_sdk::SP1Stdin::new();
    stdin.write_slice(&[]);
    let result = execute_multi(&data_fetcher, stdin, 1, 6).await;
    assert!(result.is_err(), "A panicking range program was accepted");
    assert!(
        format!("{:#}", result.err().unwrap()).contains("Range program exited with code 1"),
        "Failure must come from the guest exit check"
    );
    Ok(())
}
