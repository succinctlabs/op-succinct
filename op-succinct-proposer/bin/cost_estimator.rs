use alloy::{providers::ProviderBuilder, sol, transports::http::reqwest::Url};
use anyhow::Result;
use clap::Parser;
use host_utils::{
    fetcher::{ChainMode, SP1KonaDataFetcher},
    get_proof_stdin, ProgramType,
};
use kona_host::start_server_and_native_client;
use kona_primitives::RollupConfig;
use log::info;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::{env, fs, path::PathBuf};

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

/// TODO: Modify this to invoke the Docker server with the span batch ranges.
/// Note: If we're double-paying for span batches, this isn't that much more expensive. Ex. If you pay
/// 250M cycles for additional verification and every span batch is 12B cycles (6 2B proofs), then you're
/// paying 2% more.
fn get_span_batch_ranges(start: u64, end: u64) -> Result<Vec<SpanBatchRange>> {
    let mut ranges = Vec::new();
    for i in (start..=end).step_by(20) {
        let end = std::cmp::min(i + 19, end);
        ranges.push(SpanBatchRange { start: i, end });
    }
    Ok(ranges)
}

use reqwest::Client;
use serde::{Deserialize, Serialize};

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

    // TODO: This should be a POST request to a Docker container. The URL should be supplied via an env var.
    let response: SpanBatchResponse = client
        .post("http://localhost:8080/span-batch-ranges")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.ranges)
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
    let span_batch_ranges = get_span_batch_ranges_from_server(
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

    let prover = ProverClient::new();

    let mut reports = Vec::new();

    info!("Span batch ranges: {:?}", span_batch_ranges);

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
        start_server_and_native_client(host_cli.clone()).await?;

        let sp1_stdin = get_proof_stdin(&host_cli)?;

        let (_, report) = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run().unwrap();
        reports.push(report);
    }

    Ok(())
}
