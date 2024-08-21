use alloy::{providers::ProviderBuilder, sol, transports::http::reqwest::Url};
use anyhow::Result;
use clap::Parser;
use host_utils::{fetcher::SP1KonaDataFetcher, get_proof_stdin, ProgramType};
use kona_host::start_server_and_native_client;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::{fs, path::PathBuf};

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
    /// The chain ID. If not provided, requires the rpc_url argument to be provided.
    #[clap(long)]
    chain_id: Option<u64>,
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

struct SpanBatchRange {
    start: u64,
    end: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::setup_logger();

    let args = HostArgs::parse();

    let data_fetcher = SP1KonaDataFetcher::new();

    // Fetch the span batch ranges according to args.start and args.end
    let span_batch_ranges = get_span_batch_ranges(args.start, args.end)?;

    let prover = ProverClient::new();
    let (pk, _) = prover.setup(MULTI_BLOCK_ELF);

    let mut reports = Vec::new();

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
