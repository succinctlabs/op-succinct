use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::fetcher::{BlockInfo, OPSuccinctDataFetcher};
use sp1_sdk::utils;
use std::{
    fs::{self},
    path::PathBuf,
};

/// Write the block data to a CSV file.
fn write_block_data_to_csv(
    block_data: &[BlockInfo],
    l2_chain_id: u64,
    args: &BlockDataArgs,
) -> Result<()> {
    let report_path = PathBuf::from(format!(
        "block-data/{}/{}-{}-block-data.csv",
        l2_chain_id, args.start, args.end
    ));
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut csv_writer = csv::Writer::from_path(report_path)?;

    for block in block_data {
        csv_writer
            .serialize(block)
            .expect("Failed to write execution stats to CSV.");
    }
    csv_writer.flush().expect("Failed to flush CSV writer.");

    Ok(())
}

/// The arguments for the host executable.
#[derive(Debug, Clone, Parser)]
struct BlockDataArgs {
    /// The start block of the range to execute.
    #[clap(long)]
    start: u64,
    /// The end block of the range to execute.
    #[clap(long)]
    end: u64,
    /// The environment file to use.
    #[clap(long, default_value = ".env")]
    env_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = BlockDataArgs::parse();

    dotenv::from_path(&args.env_file).ok();
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::default();

    let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

    let l2_block_data = data_fetcher
        .get_l2_block_data_range(args.start, args.end)
        .await?;
    write_block_data_to_csv(&l2_block_data, l2_chain_id, &args)?;
    Ok(())
}
