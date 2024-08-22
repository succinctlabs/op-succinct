use anyhow::Result;
use clap::Parser;
use host_utils::{
    fetcher::{ChainMode, SP1KonaDataFetcher},
    get_proof_stdin,
    stats::{get_execution_stats, ExecutionStats, SpanBatchStats},
    ProgramType,
};
use itertools::Itertools;
use kona_primitives::RollupConfig;
use log::info;
use op_succinct_proposer::run_native_host;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sp1_sdk::{utils, ProverClient};
use std::{env, fs, path::PathBuf, time::Duration};
use tokio::process::Child;

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

async fn start_span_batch_server() -> Result<Child> {
    // Spin up the span_batch_server Docker container
    let docker_command = "docker run -d -p 8080:8080 span_batch_server";

    // Execute the Docker command
    let child = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(docker_command)
        .spawn()?;

    info!("Span batch server container started successfully");

    // Wait for the server to be ready
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(child)
}

async fn stop_span_batch_server(child: &mut Child) -> Result<()> {
    // Stop the Docker container
    child.kill().await?;
    child.wait().await?;
    info!("Span batch server container stopped successfully");
    Ok(())
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

    let mut reports = Vec::new();

    info!(
        "The span batch ranges which will be executed: {:?}",
        span_batch_ranges
    );

    let prover = ProverClient::new();

    // TODO: These should be executed in parallel.
    for range in span_batch_ranges {
        let host_cli = data_fetcher
            .get_host_cli_args(range.start, range.end, ProgramType::Multi)
            .await?;

        let data_dir = host_cli
            .data_dir
            .clone()
            .expect("Data directory is not set.");

        // Overwrite existing data directory.
        fs::create_dir_all(&data_dir).unwrap();

        // Start the server and native client.
        run_native_host(&host_cli, Duration::from_secs(40)).await?;

        let sp1_stdin = get_proof_stdin(&host_cli)?;

        let (_, report) = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run().unwrap();
        reports.push(report);
    }

    // Aggregate the total cycles across all of the reports.
    let total_instruction_count: u64 = reports
        .iter()
        .map(|report| report.total_instruction_count())
        .sum();
    let total_block_execution_instruction_count: u64 = reports
        .iter()
        .map(|report| *report.cycle_tracker.get("block-execution").unwrap())
        .sum();
    let total_bn_pair_cycles: u64 = reports
        .iter()
        .map(|report| *report.cycle_tracker.get("precompile-bn-pair").unwrap())
        .sum();
    let total_bn_add_cycles: u64 = reports
        .iter()
        .map(|report| *report.cycle_tracker.get("precompile-bn-add").unwrap())
        .sum();
    let total_bn_mul_cycles: u64 = reports
        .iter()
        .map(|report| *report.cycle_tracker.get("precompile-bn-mul").unwrap())
        .sum();

    // Fetch the number of transactions in the blocks from the L2 RPC.
    let block_data_range = data_fetcher
        .get_block_data_range(ChainMode::L2, args.start, args.end)
        .await
        .expect("Failed to fetch block data range.");
    let total_nb_blocks = args.end - args.start + 1;

    let total_nb_transactions = block_data_range.iter().map(|b| b.transaction_count).sum();
    let total_gas_used = block_data_range.iter().map(|b| b.gas_used).sum();

    let span_stats = SpanBatchStats {
        span_start: args.start,
        span_end: args.end,
        total_blocks: total_nb_blocks,
        total_transactions: total_nb_transactions,
        total_gas_used,
        total_cycles: total_instruction_count,
        total_sp1_gas: total_gas_used,
        cycles_per_block: total_instruction_count / total_nb_blocks,
        cycles_per_transaction: total_instruction_count / total_nb_transactions,
        gas_used_per_block: total_gas_used / total_nb_blocks,
        gas_used_per_transaction: total_gas_used / total_nb_transactions,
        total_derivation_cycles: 0,
        total_execution_cycles: total_block_execution_instruction_count,
        total_blob_verification_cycles: 0,
        bn_add_cycles: total_bn_add_cycles,
        bn_mul_cycles: total_bn_mul_cycles,
        bn_pair_cycles: total_bn_pair_cycles,
        kzg_eval_cycles: 0,
        ec_recover_cycles: 0,
    };

    println!("Span Batch Stats: {:?}", span_stats);

    Ok(())
}
