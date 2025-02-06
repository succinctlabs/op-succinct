use anyhow::Result;
use clap::Parser;
use op_succinct_client_utils::InMemoryOracle;
use op_succinct_host_utils::{
    block_range::get_validated_block_range,
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_proof_stdin, start_server_and_native_client, ProgramType,
};
use op_succinct_prove::{RANGE_ELF_BUMP, RANGE_ELF_EMBEDDED_LLSF, RANGE_ELF_EMBEDDED_TLSF};
use op_succinct_scripts::HostExecutorArgs;
use rayon::prelude::*;
use sp1_sdk::{utils, ExecutionReport, ProverClient};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::Semaphore;

/// Main
#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();
    let (data_fetcher, cache_mode) = init_env(&args).await?;

    // let sizes = [5, 100, 300, 1000];
    // let sizes = [5, 100, 300];
    let sizes = [5];
    let oracles = fetch_oracles(&data_fetcher, None, None, &sizes, cache_mode).await?;

    let elfs = [
        (RANGE_ELF_BUMP, "bump".to_string()),
        (RANGE_ELF_EMBEDDED_LLSF, "embedded-llsf".to_string()),
        (RANGE_ELF_EMBEDDED_TLSF, "embedded-tlsf".to_string()),
    ];

    let reports = run_prover_tests(&oracles, &sizes, &elfs);
    print_comparison_results(&reports, &sizes);

    Ok(())
}

/// Set up environment, logger and data fetcher.
async fn init_env(args: &HostExecutorArgs) -> Result<(OPSuccinctDataFetcher, CacheMode)> {
    dotenv::from_path(&args.env_file)?;
    utils::setup_logger();
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;
    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };
    Ok((data_fetcher, cache_mode))
}

/// Fetch oracles corresponding to each block size.
///
/// Replace `YourOracleType` with the actual type that
/// `start_server_and_native_client` returns.
async fn fetch_oracles(
    data_fetcher: &OPSuccinctDataFetcher,
    start_block: Option<u64>,
    end_block: Option<u64>,
    sizes: &[u64],
    cache_mode: CacheMode,
) -> Result<HashMap<u64, InMemoryOracle>> {
    let semaphore = Arc::new(Semaphore::new(sizes.len()));
    let mut handles = Vec::new();

    for &size in sizes {
        let (start, end) =
            get_validated_block_range(data_fetcher, start_block, end_block, size).await?;
        let host_cli = data_fetcher
            .get_host_cli_args(start, end, ProgramType::Multi, cache_mode)
            .await?;
        let semaphore = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await?;
            start_server_and_native_client(&host_cli).await
        });
        handles.push((size, handle));
    }

    let mut oracles = HashMap::new();
    for (size, handle) in handles {
        let oracle = handle.await??;
        oracles.insert(size, oracle);
    }
    Ok(oracles)
}

/// Run the prover tests concurrently over all sizes and both ELFs.
/// This function uses Rayon for parallelism.
///
/// Each spawned task clones its oracle, runs `prover.execute` and sends back a report.
/// The returned vector items are tuples of (block size, ELF name, report).
fn run_prover_tests(
    oracles: &HashMap<u64, InMemoryOracle>,
    sizes: &[u64],
    elfs: &[(&[u8], String); 3],
) -> Vec<(u64, String, ExecutionReport)> {
    let combinations: Vec<_> = elfs
        .iter()
        .flat_map(|(elf, name)| sizes.iter().map(move |size| (size, elf, name.clone())))
        .collect();

    combinations
        .par_iter()
        .filter_map(|(size, elf, name)| {
            let oracle = oracles.get(size)?.clone();
            let prover = ProverClient::builder().mock().build();
            let start = Instant::now();
            let stdin = get_proof_stdin(oracle).ok()?;
            if let Ok((_, report)) = prover.execute(elf, &stdin).run() {
                let elapsed = start.elapsed();
                println!(
                    "{name} {size} blocks: elapsed: {:?}, cycle_tracker: {:?}, instructions: {}",
                    elapsed,
                    report.cycle_tracker,
                    report.total_instruction_count()
                );
                Some((**size, name.clone(), report))
            } else {
                None
            }
        })
        .collect()
}

/// Group reports by block size and print a comparison table.
/// Assumes each block size returns reports for both "bump" and "embedded".
fn print_comparison_results(reports: &[(u64, String, ExecutionReport)], sizes: &[u64]) {
    let mut grouped_reports: HashMap<u64, Vec<(String, ExecutionReport)>> = HashMap::new();
    for (size, name, report) in reports {
        grouped_reports
            .entry(size.clone())
            .or_default()
            .push((name.clone(), report.clone()));
    }

    for &size in sizes {
        if let Some(reps) = grouped_reports.get(&size) {
            println!("\nResults for {} blocks:", size);
            println!("| Metric | bump | llsf | tlsf | % diff (llsf/bump) | % diff (tlsf/bump) |");
            println!("|--------|------|------|------|--------------|--------------|");

            if reps.len() == 3 {
                // Determine which report corresponds to which ELF.
                let (bump_report, llsf_report, tlsf_report) = if reps[0].0 == "bump" {
                    (&reps[0].1, &reps[1].1, &reps[2].1)
                } else if reps[1].0 == "bump" {
                    (&reps[1].1, &reps[0].1, &reps[2].1)
                } else if reps[2].0 == "bump" {
                    (&reps[2].1, &reps[0].1, &reps[1].1)
                } else {
                    panic!("Invalid reports");
                };

                // Compare cycle tracker metrics.
                for (metric, bump_val) in bump_report.cycle_tracker.iter() {
                    if let Some(llsf_val) = llsf_report.cycle_tracker.get(metric) {
                        if let Some(tlsf_val) = tlsf_report.cycle_tracker.get(metric) {
                            let llsf_diff_pct =
                                ((*llsf_val as f64 - *bump_val as f64) / *bump_val as f64) * 100.0;
                            let tlsf_diff_pct =
                                ((*tlsf_val as f64 - *bump_val as f64) / *bump_val as f64) * 100.0;
                            println!(
                                "| {} | {} | {} | {} | {:.2}% | {:.2}% |",
                                metric, bump_val, llsf_val, tlsf_val, llsf_diff_pct, tlsf_diff_pct
                            );
                        }
                    }
                }
                // Compare total instruction counts.
                let bump_instr = bump_report.total_instruction_count();
                let llsf_instr = llsf_report.total_instruction_count();
                let tlsf_instr = tlsf_report.total_instruction_count();
                let llsf_instr_diff_pct =
                    ((llsf_instr as f64 - bump_instr as f64) / bump_instr as f64) * 100.0;
                let tlsf_instr_diff_pct =
                    ((tlsf_instr as f64 - bump_instr as f64) / bump_instr as f64) * 100.0;
                println!(
                    "| Total Instructions | {} | {} | {} | {:.2}% | {:.2}% |",
                    bump_instr, llsf_instr, tlsf_instr, llsf_instr_diff_pct, tlsf_instr_diff_pct
                );
            }
        }
    }
}
