use anyhow::Result;
use op_succinct_host_utils::{
    block_range::get_rolling_block_range,
    fetcher::OPSuccinctDataFetcher,
    get_proof_stdin,
    hosts::{default::SingleChainOPSuccinctHost, OPSuccinctHost},
    stats::ExecutionStats,
};
use op_succinct_prove::{execute_multi, DEFAULT_RANGE, ONE_HOUR};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::Write, sync::Arc};
use thousands::Separable;

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    pub total_instruction_count: u64,
    pub oracle_verify_instruction_count: u64,
    pub derivation_instruction_count: u64,
    pub block_execution_instruction_count: u64,
    pub blob_verification_instruction_count: u64,
    pub total_sp1_gas: u64,
    pub cycles_per_block: u64,
    pub cycles_per_transaction: u64,
    pub bn_pair_cycles: u64,
    pub bn_add_cycles: u64,
    pub bn_mul_cycles: u64,
    pub kzg_eval_cycles: u64,
    pub ec_recover_cycles: u64,
    pub p256_verify_cycles: u64,
}

impl From<&ExecutionStats> for Stats {
    fn from(stats: &ExecutionStats) -> Self {
        Self {
            total_instruction_count: stats.total_instruction_count,
            oracle_verify_instruction_count: stats.oracle_verify_instruction_count,
            derivation_instruction_count: stats.derivation_instruction_count,
            block_execution_instruction_count: stats.block_execution_instruction_count,
            blob_verification_instruction_count: stats.blob_verification_instruction_count,
            total_sp1_gas: stats.total_sp1_gas,
            cycles_per_block: stats.cycles_per_block,
            cycles_per_transaction: stats.cycles_per_transaction,
            bn_pair_cycles: stats.bn_pair_cycles,
            bn_add_cycles: stats.bn_add_cycles,
            bn_mul_cycles: stats.bn_mul_cycles,
            kzg_eval_cycles: stats.kzg_eval_cycles,
            ec_recover_cycles: stats.ec_recover_cycles,
            p256_verify_cycles: stats.p256_verify_cycles,
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cycle_count() -> Result<()> {
    dotenv::dotenv()?;

    let is_base_branch = env::var("BASE_BRANCH").is_ok();
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    // Take the latest blocks
    let (l2_start_block, l2_end_block) =
        get_rolling_block_range(&data_fetcher, ONE_HOUR, DEFAULT_RANGE).await?;
    println!("Block range: {} to {}", l2_start_block, l2_end_block);

    let host = SingleChainOPSuccinctHost {
        fetcher: Arc::new(data_fetcher.clone()),
    };

    let host_args = host
        .fetch(l2_start_block, l2_end_block, None, Some(false))
        .await?;

    let oracle = host.run(&host_args).await?;
    let sp1_stdin = get_proof_stdin(oracle)?;
    let (block_data, report, execution_duration) =
        execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

    let stats = ExecutionStats::new(0, &block_data, &report, 0, execution_duration.as_secs());
    let stats_for_comparison = Stats::from(&stats);

    if !is_base_branch {
        // Save current PR stats
        serde_json::to_writer(File::create("cycle_stats.json")?, &stats_for_comparison)?;
    } else {
        // Compare with PR stats and generate report
        let path = env::var("GITHUB_OUTPUT")?;
        let current_stats = serde_json::from_reader::<_, Stats>(File::open("cycle_stats.json")?)?;
        let mut output_file = File::options().create(true).append(true).open(path)?;

        let mut report = String::new();
        report.push_str("## Cycle Count Comparison\n\n");
        report.push_str("| Metric | Base Branch | Current PR | Diff (%) |\n");
        report.push_str("|--------|-------------|------------|----------|\n");

        let diff_percentage = |base: u64, current: u64| -> f64 {
            if base == 0 {
                return 0.0;
            }
            ((current as f64 - base as f64) / base as f64) * 100.0
        };

        let metrics = [
            (
                "Total Instructions",
                stats_for_comparison.total_instruction_count,
                current_stats.total_instruction_count,
            ),
            (
                "Oracle Verify",
                stats_for_comparison.oracle_verify_instruction_count,
                current_stats.oracle_verify_instruction_count,
            ),
            (
                "Derivation",
                stats_for_comparison.derivation_instruction_count,
                current_stats.derivation_instruction_count,
            ),
            (
                "Block Execution",
                stats_for_comparison.block_execution_instruction_count,
                current_stats.block_execution_instruction_count,
            ),
            (
                "Blob Verification",
                stats_for_comparison.blob_verification_instruction_count,
                current_stats.blob_verification_instruction_count,
            ),
            (
                "Total SP1 Gas",
                stats_for_comparison.total_sp1_gas,
                current_stats.total_sp1_gas,
            ),
            (
                "Cycles per Block",
                stats_for_comparison.cycles_per_block,
                current_stats.cycles_per_block,
            ),
            (
                "Cycles per Tx",
                stats_for_comparison.cycles_per_transaction,
                current_stats.cycles_per_transaction,
            ),
        ];

        for (name, base, current) in metrics {
            let diff = diff_percentage(base, current);
            report.push_str(&format!(
                "| {} | {} | {} | {:.2}% |\n",
                name,
                base.separate_with_commas(),
                current.separate_with_commas(),
                diff
            ));
        }

        // Add warning for significant degradation
        let has_degradation = metrics
            .iter()
            .any(|(_, base, current)| diff_percentage(*base, *current) > 5.0);

        if has_degradation {
            report.push_str(
                "\n⚠️ **Warning:** Performance has degraded by more than 5% in some metrics.\n",
            );
        }

        writeln!(output_file, "EXECUTION_REPORT<<EOF")?;
        writeln!(output_file, "{}", report)?;
        writeln!(output_file, "EOF")?;
    }

    Ok(())
}
