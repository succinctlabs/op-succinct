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
use std::{collections::HashMap, fs, sync::Arc, time::Instant};
use tokio::sync::Semaphore;

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

    let sizes = [5, 100, 300, 1000];

    // Get block ranges and execute ELFs in parallel
    let mut handles = Vec::new();
    let semaphore = Arc::new(Semaphore::new(sizes.len()));

    for size in sizes {
        let (start, end) = get_validated_block_range(&data_fetcher, None, None, size).await?;
        let host_cli = data_fetcher
            .get_host_cli_args(start, end, ProgramType::Multi, cache_mode)
            .await?;

        let semaphore = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await?;
            start_server_and_native_client(&host_cli).await
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    let (oracle_5, oracle_100, oracle_300, oracle_1000) = (
        results[0].as_ref().unwrap().as_ref().unwrap(),
        results[1].as_ref().unwrap().as_ref().unwrap(),
        results[2].as_ref().unwrap().as_ref().unwrap(),
        results[3].as_ref().unwrap().as_ref().unwrap(),
    );

    let elfs = [(RANGE_ELF_BUMP, "bump"), (RANGE_ELF_EMBEDDED, "embedded")];

    let reports = {
        let mut reports = Vec::new();
        let (tx, rx) = std::sync::mpsc::channel();

        rayon::scope(|s| {
            for (oracle, size) in [
                (oracle_5, 5),
                (oracle_100, 100),
                (oracle_300, 300),
                (oracle_1000, 1000),
            ] {
                for (elf, name) in elfs {
                    let tx = tx.clone();
                    let oracle = oracle.clone();
                    s.spawn(move |_| {
                        let prover = ProverClient::builder().mock().build();
                        let start = Instant::now();
                        let stdin = get_proof_stdin(oracle).unwrap();
                        let (_, report) = prover.execute(elf, &stdin).run().unwrap();

                        let elapsed = start.elapsed();
                        println!(
                            "{name} {size} blocks: {:?}, cycles: {:?}, instructions: {}",
                            elapsed,
                            report.cycle_tracker,
                            report.total_instruction_count()
                        );
                        tx.send((size, name, report)).unwrap();
                    });
                }
            }
        });

        drop(tx);
        for result in rx {
            reports.push(result);
        }
        reports
    };

    // Group reports by block size
    let mut grouped_reports: HashMap<u64, Vec<(&str, _)>> = HashMap::new();
    for (size, name, report) in reports {
        grouped_reports
            .entry(size)
            .or_default()
            .push((name, report));
    }

    // Print results for each block size
    for size in [5, 100, 300, 1000] {
        if let Some(reports) = grouped_reports.get(&size) {
            println!("\nResults for {} blocks:", size);
            println!("| Metric | bump | embedded | % diff |");
            println!("|--------|------|----------|--------|");

            let (bump_report, embedded_report) = if reports[0].0 == "bump" {
                (&reports[0].1, &reports[1].1)
            } else {
                (&reports[1].1, &reports[0].1)
            };

            // Print cycle tracker metrics
            for (metric, bump_val) in bump_report.cycle_tracker.iter() {
                if let Some(embedded_val) = embedded_report.cycle_tracker.get(metric) {
                    let diff_pct =
                        ((*embedded_val as f64 - *bump_val as f64) / *bump_val as f64) * 100.0;
                    println!(
                        "| {} | {} | {} | {:.2}% |",
                        metric, bump_val, embedded_val, diff_pct
                    );
                }
            }

            // Print total instruction count
            let bump_instr = bump_report.total_instruction_count();
            let embedded_instr = embedded_report.total_instruction_count();
            let instr_diff_pct =
                ((embedded_instr as f64 - bump_instr as f64) / bump_instr as f64) * 100.0;
            println!(
                "| Total Instructions | {} | {} | {:.2}% |",
                bump_instr, embedded_instr, instr_diff_pct
            );
        }
    }

    Ok(())
}
