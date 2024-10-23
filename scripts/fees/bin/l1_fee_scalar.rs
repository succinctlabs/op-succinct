use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    start: u64,
    #[clap(long)]
    end: u64,
    #[clap(long, default_value = ".env")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    dotenv::from_filename(args.env_file).ok();
    let fetcher = OPSuccinctDataFetcher::default();

    let fee_data = fetcher.get_l2_fee_data_range(args.start, args.end).await?;

    for data in fee_data {
        println!(
            "Block: {}, Tx Index: {}, Tx Hash: {}, L1 Gas Cost: {}",
            data.block_number, data.tx_index, data.tx_hash, data.l1_gas_cost
        );
    }

    Ok(())
}
