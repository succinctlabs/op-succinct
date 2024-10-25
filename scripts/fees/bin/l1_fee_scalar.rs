use alloy_primitives::U256;
use anyhow::Result;
use clap::Parser;
use op_succinct_fees::aggregate_fee_data;
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

    let modified_fee_data = fetcher
        .get_l2_fee_data_with_modified_l1_fee_scalar(args.start, args.end, U256::from(1000000000))
        .await?;

    let total_aggregate_fee_data = aggregate_fee_data(fee_data)?;
    // println!(
    //     "Start: {}, End: {}, Aggregate: {} transactions, {:.18} GWei L1 fee",
    //     aggregate_fee_data.start,
    //     aggregate_fee_data.end,
    //     aggregate_fee_data.num_transactions,
    //     (aggregate_fee_data.total_l1_fee) / U256::from(10).pow(U256::from(9))
    // );

    let modified_total_aggregate_fee_data = aggregate_fee_data(modified_fee_data)?;

    println!("{modified_total_aggregate_fee_data}");
    println!("{total_aggregate_fee_data}");

    assert_eq!(
        total_aggregate_fee_data.total_l1_fee,
        modified_total_aggregate_fee_data.total_l1_fee
    );

    println!("Success!");

    Ok(())
}
