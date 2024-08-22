use anyhow::Result;
use clap::Parser;
use host_utils::{
    fetcher::{ChainMode, SP1KonaDataFetcher},
    get_proof_stdin,
    stats::{get_execution_stats, ExecutionStats},
    ProgramType,
};
use kona_host::HostCli;
use kona_primitives::RollupConfig;
use log::info;
use op_succinct_proposer::run_native_host;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sp1_sdk::{utils, ExecutionReport, ProverClient};
use std::{env, fs, path::PathBuf, time::Duration};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/range-elf");

/// The arguments for the host executable.
#[derive(Debug, Clone, Parser)]
struct HostArgs {
    /// The start block of the range to execute.
    #[clap(long)]
    start: u64,
    /// The end block of the range to execute.
    #[clap(long)]
    end: u64,
    /// Whether to generate a proof or just execute the block.
    #[clap(long)]
    prove: bool,
    /// The path to the CSV file containing the execution data.
    #[clap(long, default_value = "report.csv")]
    report_path: PathBuf,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct SpanBatchRequest {
    startBlock: u64,
    endBlock: u64,
    l2ChainId: u64,
    l2Node: String,
    l1Rpc: String,
    l1Beacon: String,
    batchSender: String,
}

#[derive(Deserialize, Debug, Clone)]
struct SpanBatchResponse {
    ranges: Vec<SpanBatchRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpanBatchRange {
    start: u64,
    end: u64,
}

async fn get_span_batch_ranges_from_server(
    data_fetcher: &SP1KonaDataFetcher,
    start: u64,
    end: u64,
    l2_chain_id: u64,
    batch_sender: &str,
) -> Result<Vec<SpanBatchRange>> {
    let client = Client::new();
    let request = SpanBatchRequest {
        startBlock: start,
        endBlock: end,
        l2ChainId: l2_chain_id,
        l2Node: data_fetcher.l2_node_rpc.clone(),
        l1Rpc: data_fetcher.l1_rpc.clone(),
        l1Beacon: data_fetcher.l1_beacon_rpc.clone(),
        batchSender: batch_sender.to_string(),
    };

    let span_batch_server_url =
        env::var("SPAN_BATCH_SERVER_URL").unwrap_or("http://localhost:8080".to_string());

    let query_url = format!("{}/span-batch-ranges", span_batch_server_url);

    let response: SpanBatchResponse = client
        .post(&query_url)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.ranges)
}

async fn write_stats_to_csv(
    execution_stats: &[ExecutionStats],
    report_path: &PathBuf,
) -> Result<()> {
    // Create directory if it doesn't exist.
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut csv_writer = csv::Writer::from_path(report_path)?;
    for stats in execution_stats {
        csv_writer.serialize(stats)?;
    }
    csv_writer.flush()?;

    Ok(())
}

struct BatchHostCli {
    host_cli: HostCli,
    start: u64,
    end: u64,
}

struct BatchExecutionData {
    start: u64,
    end: u64,
    execution_stats: ExecutionReport,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::setup_logger();

    let args = HostArgs::parse();

    let data_fetcher = SP1KonaDataFetcher::new();

    let l2_chain_id = data_fetcher.get_chain_id(ChainMode::L2).await?;
    let rollup_config = RollupConfig::from_l2_chain_id(l2_chain_id).unwrap();

    // Fetch the span batch ranges according to args.start and args.end
    // TODO: If the ranges are greater than 20 blocks, we will have to split them in a custom way.
    let span_batch_ranges: Vec<SpanBatchRange> = get_span_batch_ranges_from_server(
        &data_fetcher,
        args.start,
        args.end,
        l2_chain_id,
        rollup_config
            .genesis
            .system_config
            .unwrap()
            .batcher_address
            .to_string()
            .as_str(),
    )
    .await?;

    info!(
        "The span batch ranges which will be executed: {:?}",
        span_batch_ranges
    );

    let prover = ProverClient::new();

    // TODO: These should be executed in parallel.
    const BATCH_SIZE: usize = 5;
    let futures = span_batch_ranges.chunks(BATCH_SIZE).map(|chunk| {
        futures::future::join_all(chunk.iter().map(|range| async {
            let host_cli = data_fetcher
                .get_host_cli_args(range.start, range.end, ProgramType::Multi)
                .await
                .unwrap();

            let data_dir = host_cli
                .data_dir
                .clone()
                .expect("Data directory is not set.");

            // Overwrite existing data directory.
            fs::create_dir_all(&data_dir).unwrap();

            // Start the server and native client.
            // TODO: This is not resilient to errors, and does not gracefully shut down when we exit.
            // TODO: Add retries if the server fails to start.
            run_native_host(&host_cli, Duration::from_secs(60))
                .await
                .unwrap();

            BatchHostCli {
                host_cli,
                start: range.start,
                end: range.end,
            }
        }))
    });

    let host_cli_futures = futures::future::join_all(futures);

    let host_clis: Vec<BatchHostCli> = host_cli_futures.await.into_iter().flatten().collect();

    // Execute the blocks in parallel with par_iter.
    let reports: Vec<BatchExecutionData> = host_clis
        .par_iter()
        .map(|r| {
            let sp1_stdin = get_proof_stdin(&r.host_cli).unwrap();

            let (_, report) = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run().unwrap();
            BatchExecutionData {
                start: r.start,
                end: r.end,
                execution_stats: report,
            }
        })
        .collect();

    // Get all of the execution stats.
    let mut execution_stats: Vec<ExecutionStats> = futures::future::join_all(
        reports
            .iter()
            .map(|r| get_execution_stats(&data_fetcher, r.start, r.end, &r.execution_stats)),
    )
    .await;

    // Sort the execution stats by the start block of the range.
    execution_stats.sort_by_key(|s| s.batch_start);

    // Write the stats to a CSV file.
    let report = format!(
        "execution-reports/{}/{}-{}-report.csv",
        l2_chain_id, args.start, args.end
    );
    write_stats_to_csv(&execution_stats, &PathBuf::from(report)).await?;

    Ok(())
}
