use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use kona_host::HostCli;
use log::info;
use op_succinct_host_utils::{
    block_range::{
        get_rolling_block_range, get_validated_block_range, split_range_based_on_safe_heads,
        split_range_basic, SpanBatchRange,
    },
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_proof_stdin,
    stats::ExecutionStats,
    witnessgen::run_native_data_generation,
    ProgramType,
};
use op_succinct_scripts::HostExecutorArgs;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use sp1_sdk::{utils, ProverClient};
use std::{
    cmp::{max, min},
    fs::{self, OpenOptions},
    io::Seek,
    path::PathBuf,
    time::{Duration, Instant},
};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../../elf/range-elf");

const TWELVE_HOURS: Duration = Duration::from_secs(60 * 60 * 12);

/// Run the zkVM execution process for each split range in parallel. Writes the execution stats for
/// each block range to a CSV file after each execution completes (not guaranteed to be in order).
async fn execute_blocks_and_write_stats_csv(
    host_clis: &[HostCli],
    ranges: Vec<SpanBatchRange>,
    prover: &ProverClient,
    l2_chain_id: u64,
    start: u64,
    end: u64,
) {
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev)
        .await
        .unwrap();

    // Fetch all of the execution stats block ranges in parallel.
    let block_data = futures::stream::iter(ranges.clone())
        .map(|range| async {
            let block_data = data_fetcher
                .get_l2_block_data_range(range.start, range.end)
                .await
                .expect("Failed to fetch block data range.");
            (range, block_data)
        })
        .buffered(15)
        .collect::<Vec<_>>()
        .await;

    let cargo_metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let root_dir = PathBuf::from(cargo_metadata.workspace_root);
    let report_path = root_dir.join(format!(
        "execution-reports/{}/{}-{}-report.csv",
        l2_chain_id, start, end
    ));
    // Create the parent directory if it doesn't exist
    if let Some(parent) = report_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }
    }

    // Create an empty file since canonicalize requires the path to exist
    fs::File::create(&report_path).unwrap();
    let report_path = report_path.canonicalize().unwrap();

    // Run the zkVM execution process for each split range in parallel and fill in the execution stats.
    host_clis
        .par_iter()
        .zip(block_data.par_iter())
        .for_each(|(host_cli, (range, block_data))| {
            let sp1_stdin = get_proof_stdin(host_cli).unwrap();

            // FIXME: Implement retries with a smaller block range if this fails.
            let result = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run();

            // If the execution fails, skip this block range and log the error.
            if let Some(err) = result.as_ref().err() {
                log::warn!(
                    "Failed to execute blocks {:?} - {:?} because of {:?}. Reduce your `batch-size` if you're running into OOM issues on SP1.",
                    range.start,
                    range.end,
                    err
                );
                return;
            }

            let (_, report) = result.unwrap();

            // Get the existing execution stats and modify it in place.
            let execution_stats = ExecutionStats::new(block_data, &report, 0, 0);

            let mut file = OpenOptions::new()
                .read(true)
                .append(true)
                .open(&report_path)
                .unwrap();

            // Writes the headers only if the file is empty.
            let needs_header = file.seek(std::io::SeekFrom::End(0)).unwrap() == 0;

            let mut csv_writer = csv::WriterBuilder::new()
                .has_headers(needs_header)
                .from_writer(file);

            csv_writer
                .serialize(execution_stats.clone())
                .expect("Failed to write execution stats to CSV.");
            csv_writer.flush().expect("Failed to flush CSV writer.");
        });

    info!("Execution is complete.");
}
/// Aggregate the execution statistics for an array of execution stats objects.
fn aggregate_execution_stats(
    execution_stats: &[ExecutionStats],
    total_execution_time_sec: u64,
    witness_generation_time_sec: u64,
) -> ExecutionStats {
    let mut aggregate_stats = ExecutionStats::default();
    let mut batch_start = u64::MAX;
    let mut batch_end = u64::MIN;
    for stats in execution_stats {
        batch_start = min(batch_start, stats.batch_start);
        batch_end = max(batch_end, stats.batch_end);

        // Accumulate most statistics across all blocks.
        aggregate_stats.total_instruction_count += stats.total_instruction_count;
        aggregate_stats.oracle_verify_instruction_count += stats.oracle_verify_instruction_count;
        aggregate_stats.derivation_instruction_count += stats.derivation_instruction_count;
        aggregate_stats.block_execution_instruction_count +=
            stats.block_execution_instruction_count;
        aggregate_stats.blob_verification_instruction_count +=
            stats.blob_verification_instruction_count;
        aggregate_stats.total_sp1_gas += stats.total_sp1_gas;
        aggregate_stats.nb_blocks += stats.nb_blocks;
        aggregate_stats.nb_transactions += stats.nb_transactions;
        aggregate_stats.eth_gas_used += stats.eth_gas_used;
        aggregate_stats.bn_pair_cycles += stats.bn_pair_cycles;
        aggregate_stats.bn_add_cycles += stats.bn_add_cycles;
        aggregate_stats.bn_mul_cycles += stats.bn_mul_cycles;
        aggregate_stats.kzg_eval_cycles += stats.kzg_eval_cycles;
        aggregate_stats.ec_recover_cycles += stats.ec_recover_cycles;
    }

    // For statistics that are per-block or per-transaction, we take the average over the entire
    // range.
    aggregate_stats.cycles_per_block =
        aggregate_stats.total_instruction_count / aggregate_stats.nb_blocks;
    aggregate_stats.cycles_per_transaction =
        aggregate_stats.total_instruction_count / aggregate_stats.nb_transactions;
    aggregate_stats.transactions_per_block =
        aggregate_stats.nb_transactions / aggregate_stats.nb_blocks;
    aggregate_stats.gas_used_per_block = aggregate_stats.eth_gas_used / aggregate_stats.nb_blocks;
    aggregate_stats.gas_used_per_transaction =
        aggregate_stats.eth_gas_used / aggregate_stats.nb_transactions;

    // Use the earliest start and latest end across all blocks.
    aggregate_stats.batch_start = batch_start;
    aggregate_stats.batch_end = batch_end;

    // Set the total execution time to the total execution time of the entire range.
    aggregate_stats.total_execution_time_sec = total_execution_time_sec;
    aggregate_stats.witness_generation_time_sec = witness_generation_time_sec;

    aggregate_stats
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file).ok();
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;
    let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

    let (l2_start_block, l2_end_block) = if args.rolling {
        get_rolling_block_range(&data_fetcher, TWELVE_HOURS, args.default_range).await?
    } else {
        get_validated_block_range(&data_fetcher, args.start, args.end, args.default_range).await?
    };

    // Check if the safeDB is activated on the L2 node. If it is, we use the safeHead based range
    // splitting algorithm. Otherwise, we use the simple range splitting algorithm.
    let safe_db_activated = data_fetcher.is_safe_db_activated().await?;

    let split_ranges = if safe_db_activated {
        split_range_based_on_safe_heads(l2_start_block, l2_end_block, args.batch_size).await?
    } else {
        split_range_basic(l2_start_block, l2_end_block, args.batch_size)
    };

    info!(
        "The span batch ranges which will be executed: {:?}",
        split_ranges
    );

    let prover = ProverClient::new();

    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };

    // Get the host CLIs in order, in parallel.
    let host_clis = futures::stream::iter(split_ranges.iter())
        .map(|range| async {
            data_fetcher
                .get_host_cli_args(range.start, range.end, ProgramType::Multi, cache_mode)
                .await
                .expect("Failed to get host CLI args")
        })
        .buffered(15)
        .collect::<Vec<_>>()
        .await;

    let start_time = Instant::now();
    if !args.use_cache {
        // Get the host CLI args
        run_native_data_generation(&host_clis).await;
    }
    let total_witness_generation_time_sec = start_time.elapsed().as_secs();

    let start_time = Instant::now();
    execute_blocks_and_write_stats_csv(
        &host_clis,
        split_ranges,
        &prover,
        l2_chain_id,
        l2_start_block,
        l2_end_block,
    )
    .await;
    let total_execution_time_sec = start_time.elapsed().as_secs();

    // Get the path to the execution report CSV file.
    let cargo_metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let root_dir = PathBuf::from(cargo_metadata.workspace_root);
    let report_path = root_dir.join(format!(
        "execution-reports/{}/{}-{}-report.csv",
        l2_chain_id, l2_start_block, l2_end_block
    ));

    // Read the execution stats from the CSV file and aggregate them to output to the user.
    let mut final_execution_stats = Vec::new();
    let mut csv_reader = csv::Reader::from_path(&report_path)?;
    for result in csv_reader.deserialize() {
        let stats: ExecutionStats = result?;
        final_execution_stats.push(stats);
    }

    println!("Wrote execution stats to {}", report_path.display());

    // Aggregate the execution stats and print them to the user.
    println!(
        "Aggregate Execution Stats for Chain {}: \n {}",
        l2_chain_id,
        aggregate_execution_stats(
            &final_execution_stats,
            total_execution_time_sec,
            total_witness_generation_time_sec
        )
    );

    Ok(())
}
