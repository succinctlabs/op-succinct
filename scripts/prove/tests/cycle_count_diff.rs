use std::{fmt::Write as _, fs::File, sync::Arc}; // Path 트레잇을 사용하기 위해 추가

use anyhow::{anyhow, Result}; // anyhow 오류 처리를 위해 Result::map_err 사용 가능
use common::post_to_github_pr;
use op_succinct_host_utils::{
    block_range::get_rolling_block_range,
    fetcher::OPSuccinctDataFetcher,
    host::OPSuccinctHost,
    stats::{ExecutionStats, MarkdownExecutionStats},
    witness_generation::client::WitnessGenerator,
};
use op_succinct_proof_utils::initialize_host;
use op_succinct_prove::{execute_multi, DEFAULT_RANGE, ONE_HOUR};

mod common;

fn create_diff_report(base: &ExecutionStats, current: &ExecutionStats) -> String {
    let mut report = String::new();
    writeln!(report, "## Performance Comparison\n").unwrap();
    writeln!(
        report,
        "Comparing L2 blocks {}~{} (Base) vs {}~{} (Current)\n",
        base.batch_start, base.batch_end, current.batch_start, current.batch_end
    )
    .unwrap();
    writeln!(
        report,
        "| {:<30} | {:<25} | {:<25} | {:<10} |",
        "Metric", "Base Branch", "Current PR", "Diff (%)"
    )
    .unwrap();
    writeln!(report, "|--------------------------------|---------------------------|---------------------------|------------|").unwrap();

    let diff_percentage = |base: u64, current: u64| -> f64 {
        if base == 0 {
            // Handle division by zero gracefully
            if current == 0 {
                0.0
            } else {
                // If base is 0 and current is non-zero, diff is effectively infinite,
                // but 100% difference (relative increase) is a reasonable representation.
                100.0
            }
        } else {
            ((current as f64 - base as f64) / base as f64) * 100.0
        }
    };

    let write_metric = |report: &mut String, name: &str, base_val: u64, current_val: u64| {
        let diff = diff_percentage(base_val, current_val);
        writeln!(
            report,
            "| {:<30} | {:<25} | {:<25} | {:>+9.2}% |", // %+9.2f로 부호 포함 출력
            name,
            base_val.to_string(),
            current_val.to_string(),
            diff
        )
        .unwrap();
    };

    // Add key metrics with their comparisons
    write_metric(
        &mut report,
        "Total Instructions",
        base.total_instruction_count,
        current.total_instruction_count,
    );
    write_metric(
        &mut report,
        "Oracle Verify Cycles",
        base.oracle_verify_instruction_count,
        current.oracle_verify_instruction_count,
    );
    write_metric(
        &mut report,
        "Derivation Cycles",
        base.derivation_instruction_count,
        current.derivation_instruction_count,
    );
    write_metric(
        &mut report,
        "Block Execution Cycles",
        base.block_execution_instruction_count,
        current.block_execution_instruction_count,
    );
    write_metric(
        &mut report,
        "Blob Verification Cycles",
        base.blob_verification_instruction_count,
        current.blob_verification_instruction_count,
    );
    write_metric(&mut report, "Total SP1 Gas", base.total_sp1_gas, current.total_sp1_gas);
    write_metric(&mut report, "Cycles per Block", base.cycles_per_block, current.cycles_per_block);
    write_metric(
        &mut report,
        "Cycles per Transaction",
        base.cycles_per_transaction,
        current.cycles_per_transaction,
    );
    write_metric(&mut report, "BN Pair Cycles", base.bn_pair_cycles, current.bn_pair_cycles);
    write_metric(&mut report, "BN Add Cycles", base.bn_add_cycles, current.bn_add_cycles);
    write_metric(&mut report, "BN Mul Cycles", base.bn_mul_cycles, current.bn_mul_cycles);
    write_metric(&mut report, "KZG Eval Cycles", base.kzg_eval_cycles, current.kzg_eval_cycles);
    write_metric(
        &mut report,
        "EC Recover Cycles",
        base.ec_recover_cycles,
        current.ec_recover_cycles,
    );
    write_metric(
        &mut report,
        "P256 Verify Cycles",
        base.p256_verify_cycles,
        current.p256_verify_cycles,
    );

    report
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cycle_count_diff() -> Result<()> {
    dotenv::dotenv()?;

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let host = initialize_host(Arc::new(data_fetcher.clone()));

    let is_new_branch_run =
        std::env::var("NEW_BRANCH").expect("NEW_BRANCH must be set").parse::<bool>()?;

    let (l2_start_block, l2_end_block) = if is_new_branch_run {
        get_rolling_block_range(&data_fetcher, ONE_HOUR, DEFAULT_RANGE).await?
    } else {
        let new_stats_path = std::env::var("NEW_STATS_PATH_FOR_OLD_RUN")
            .map_err(|e| anyhow!("NEW_STATS_PATH_FOR_OLD_RUN env var not set: {}", e))?;

        eprintln!("Reading new stats from: {new_stats_path}");

        let file = File::open(&new_stats_path)
            .map_err(|e| anyhow!("Failed to open {}: {}", new_stats_path, e))?;
        let base_stats = serde_json::from_reader::<_, ExecutionStats>(file)
            .map_err(|e| anyhow!("Failed to parse JSON from {}: {}", new_stats_path, e))?;
        (base_stats.batch_start, base_stats.batch_end)
    };

    let host_args = host.fetch(l2_start_block, l2_end_block, None, Some(false)).await?;

    let oracle = host.run(&host_args).await?;
    let sp1_stdin = host.witness_generator().get_sp1_stdin(oracle).unwrap();
    let (block_data, report, execution_duration) =
        execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

    let new_stats = ExecutionStats::new(0, &block_data, &report, 0, execution_duration.as_secs());

    println!("Execution Stats:\n{}", MarkdownExecutionStats::new(new_stats.clone()));

    let output_filename =
        if is_new_branch_run { "new_cycle_stats.json" } else { "old_cycle_stats.json" };

    let mut file = File::create(output_filename)
        .map_err(|e| anyhow!("Failed to create output file {}: {}", output_filename, e))?;
    serde_json::to_writer_pretty(&mut file, &new_stats)
        .map_err(|e| anyhow!("Failed to write JSON to {}: {}", output_filename, e))?;

    Ok(())
}

#[tokio::test]
async fn test_post_to_github() -> Result<()> {
    let old_stats_path = std::env::var("OLD_STATS_FILE")
        .map_err(|e| anyhow!("OLD_STATS_FILE env var not set: {}", e))?;
    let new_stats_path = std::env::var("NEW_STATS_FILE")
        .map_err(|e| anyhow!("NEW_STATS_FILE env var not set: {}", e))?;

    eprintln!("Reading old stats from: {old_stats_path}");
    eprintln!("Reading new stats from: {new_stats_path}");

    let old_stats_file = File::open(&old_stats_path)
        .map_err(|e| anyhow!("Failed to open {}: {}", old_stats_path, e))?;
    let new_stats_file = File::open(&new_stats_path)
        .map_err(|e| anyhow!("Failed to open {}: {}", new_stats_path, e))?;

    let old_stats = serde_json::from_reader::<_, ExecutionStats>(old_stats_file)
        .map_err(|e| anyhow!("Failed to parse JSON from {}: {}", old_stats_path, e))?;
    let new_stats = serde_json::from_reader::<_, ExecutionStats>(new_stats_file)
        .map_err(|e| anyhow!("Failed to parse JSON from {}: {}", new_stats_path, e))?;

    // Sanity check for block range consistency.
    if old_stats.batch_start != new_stats.batch_start || old_stats.batch_end != new_stats.batch_end
    {
        eprintln!(
            "Warning: Comparing different block ranges! Base: {}~{}, Current: {}~{}",
            old_stats.batch_start, old_stats.batch_end, new_stats.batch_start, new_stats.batch_end
        );
    }

    let report = create_diff_report(&old_stats, &new_stats);
    println!("{report}");

    if std::env::var("POST_TO_GITHUB").ok().and_then(|v| v.parse::<bool>().ok()).unwrap_or_default()
    {
        if let (Ok(owner), Ok(repo), Ok(pr_number), Ok(token)) = (
            std::env::var("REPO_OWNER"),
            std::env::var("REPO_NAME"),
            std::env::var("PR_NUMBER"),
            std::env::var("GITHUB_TOKEN"),
        ) {
            let pr_number = pr_number
                .parse::<u64>()
                .map_err(|e| anyhow!("Failed to parse PR_NUMBER '{}': {}", pr_number, e))?;
            post_to_github_pr(&owner, &repo, &pr_number.to_string(), &token, &report)
                .await
                .unwrap();
        } else {
            eprintln!("Missing one or more GitHub environment variables for posting.");
        }
    }

    Ok(())
}
