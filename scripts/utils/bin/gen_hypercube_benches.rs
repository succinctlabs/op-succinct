use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use log::info;
use op_succinct_host_utils::{
    block_range::SpanBatchRange, fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost,
    witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::initialize_host;
use sp1_sdk::utils;
use std::{fs, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// The environment file to use.
    #[arg(long, default_value = ".env")]
    pub env_file: PathBuf,
    /// Output directory for the generated stdin files.
    #[arg(long, default_value = "./hypercube-benches")]
    pub output_dir: PathBuf,
}

/// Block range sizes for hypercube benchmarks.
const BLOCK_RANGE_SIZES: [u64; 4] = [5, 10, 20, 50];

/// Buffer from finalized block to ensure stability.
const FINALIZED_BUFFER: u64 = 100;

/// Gap between consecutive ranges to ensure non-overlapping.
const RANGE_GAP: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    dotenv::from_path(&args.env_file).ok();
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
    let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

    info!("L2 chain ID: {}", l2_chain_id);

    let host = initialize_host(Arc::new(data_fetcher.clone()));

    // Get the finalized L2 block number.
    let finalized_l2_block = host
        .get_finalized_l2_block_number(&data_fetcher, 0)
        .await?
        .expect("Failed to get finalized L2 block number");

    info!("Finalized L2 block: {}", finalized_l2_block);

    // Calculate block ranges relative to the finalized block.
    // Start from (finalized - buffer) and work backwards to create non-overlapping ranges.
    let mut ranges: Vec<SpanBatchRange> = Vec::new();
    let mut current_end = finalized_l2_block - FINALIZED_BUFFER;

    for &size in BLOCK_RANGE_SIZES.iter().rev() {
        let start = current_end - size;
        ranges.push(SpanBatchRange { start, end: current_end });
        current_end = start - RANGE_GAP;
    }

    // Reverse to get ranges in order of size (5, 10, 20, 50).
    ranges.reverse();

    info!("Block ranges to generate:");
    for range in &ranges {
        info!("  {} - {} ({} blocks)", range.start, range.end, range.end - range.start);
    }

    // Create output directory.
    let output_dir = args.output_dir;
    fs::create_dir_all(&output_dir)?;
    info!("Output directory: {:?}", output_dir);

    // Fetch host args for all ranges in parallel.
    let host_args = futures::stream::iter(ranges.iter())
        .map(|range| async {
            host.fetch(range.start, range.end, None, false).await.expect("Failed to get host args")
        })
        .buffered(4)
        .collect::<Vec<_>>()
        .await;

    // Generate SP1Stdin for each range and save to file.
    for (range, host_args) in ranges.iter().zip(host_args.iter()) {
        info!("Generating SP1Stdin for range {} - {}...", range.start, range.end);

        let witness_data = host.run(host_args).await?;
        let sp1_stdin = host.witness_generator().get_sp1_stdin(witness_data)?;

        let filename = format!("{}-{}.bin", range.start, range.end);
        let filepath = output_dir.join(&filename);

        let serialized = bincode::serialize(&sp1_stdin)?;
        fs::write(&filepath, &serialized)?;

        info!("Saved {} ({} bytes)", filename, serialized.len());
    }

    // Print summary for upload.
    println!();
    println!("Generated {} SP1Stdin files in {:?}", ranges.len(), output_dir);
    println!("Files:");
    for range in &ranges {
        println!("  {}-{}.bin", range.start, range.end);
    }

    Ok(())
}
