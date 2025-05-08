use std::{fmt::Write as _, fs::File, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
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

fn init_tracing() {
    // swallow the error if it's already been initialized
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}

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

    let diff_percentage = |base_val: u64, current_val: u64| -> f64 {
        if base_val == 0 {
            if current_val == 0 {
                0.0
            } else {
                100.0
            }
        } else {
            ((current_val as f64 - base_val as f64) / base_val as f64) * 100.0
        }
    };

    let write_metric = |report: &mut String, name: &str, base_val: u64, current_val: u64| {
        let diff = diff_percentage(base_val, current_val);
        writeln!(
            report,
            "| {:<30} | {:<25} | {:<25} | {:>+9.2}% |",
            name,
            base_val.to_string(),
            current_val.to_string(),
            diff
        )
        .unwrap();
    };

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
    init_tracing();

    dotenv::dotenv()?;

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let host = initialize_host(Arc::new(data_fetcher.clone()));

    let is_new_branch_run =
        std::env::var("NEW_BRANCH").expect("NEW_BRANCH must be set").parse::<bool>()?;

    let (l2_start_block, l2_end_block) = if is_new_branch_run {
        get_rolling_block_range(&data_fetcher, ONE_HOUR, DEFAULT_RANGE).await?
    } else {
        let new_stats_path_from_env = std::env::var("NEW_STATS_PATH_FOR_OLD_RUN")
            .map_err(|e| anyhow!("NEW_STATS_PATH_FOR_OLD_RUN env var not set: {}", e))?;

        eprintln!("Reading new branch stats for block range from: {new_stats_path_from_env}");
        eprintln!("Current CWD for test: {:?}", std::env::current_dir().unwrap_or_default());

        let file = File::open(&new_stats_path_from_env).map_err(|e| {
            anyhow!("Failed to open new branch stats file {}: {}", new_stats_path_from_env, e)
        })?;
        let base_stats = serde_json::from_reader::<_, ExecutionStats>(file)
            .map_err(|e| anyhow!("Failed to parse JSON from {}: {}", new_stats_path_from_env, e))?;
        (base_stats.batch_start, base_stats.batch_end)
    };

    let host_args = host.fetch(l2_start_block, l2_end_block, None, Some(false)).await?;
    let oracle = host.run(&host_args).await?;
    let sp1_stdin = host.witness_generator().get_sp1_stdin(oracle).unwrap();
    let (block_data, report, execution_duration) =
        execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

    let new_stats = ExecutionStats::new(0, &block_data, &report, 0, execution_duration.as_secs());

    println!("Execution Stats:\n{}", MarkdownExecutionStats::new(new_stats.clone()));

    let output_filename_stem = std::env::var("OUTPUT_FILENAME")
        .expect("OUTPUT_FILENAME environment variable must be set.");

    // Write directly to the repository root
    let path_to_write = std::env::current_dir()?
        .ancestors()
        .nth(2) // Go up two levels: scripts/prove/tests -> scripts -> repo_root
        .ok_or_else(|| anyhow!("Could not find repo root"))?
        .to_path_buf()
        .join(&output_filename_stem);

    // Log the path for easier debugging in CI
    eprintln!("Attempting to write stats to: {path_to_write:?}");

    if let Some(parent_dir) = path_to_write.parent() {
        if !parent_dir.as_os_str().is_empty() {
            std::fs::create_dir_all(parent_dir).map_err(|e| {
                anyhow!("Failed to create parent directories for {:?}: {}", path_to_write, e)
            })?;
        }
    }

    let mut file = File::create(&path_to_write)
        .map_err(|e| anyhow!("Failed to create output file {:?}: {}", path_to_write, e))?;
    serde_json::to_writer_pretty(&mut file, &new_stats)
        .map_err(|e| anyhow!("Failed to write JSON to {:?}: {}", path_to_write, e))?;

    eprintln!("Successfully wrote stats to: {path_to_write:?}");

    Ok(())
}

#[tokio::test]
async fn test_post_to_github() -> Result<()> {
    init_tracing();

    let old_stats_path = std::env::var("OLD_STATS_FILE")
        .map_err(|e| anyhow!("OLD_STATS_FILE env var not set: {}", e))?;
    let new_stats_path = std::env::var("NEW_STATS_FILE")
        .map_err(|e| anyhow!("NEW_STATS_FILE env var not set: {}", e))?;

    eprintln!("Reading old stats from: {old_stats_path}");
    eprintln!("Reading new stats from: {new_stats_path}");
    eprintln!("Current working directory: {:?}", std::env::current_dir()?);

    // Check if files exist
    eprintln!("Old stats file exists: {}", std::path::Path::new(&old_stats_path).exists());
    eprintln!("New stats file exists: {}", std::path::Path::new(&new_stats_path).exists());

    if !std::path::Path::new(&old_stats_path).exists() {
        eprintln!("Old stats file does not exist. Looking for similar files:");
        let parent =
            std::path::Path::new(&old_stats_path).parent().unwrap_or(std::path::Path::new("."));
        for entry in
            std::fs::read_dir(parent).unwrap_or_else(|_| std::fs::read_dir(".").unwrap()).flatten()
        {
            eprintln!("  Found: {:?}", entry.path());
        }
    }

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
