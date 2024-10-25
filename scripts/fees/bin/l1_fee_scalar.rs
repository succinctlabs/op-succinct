use alloy_primitives::U256;
use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::fetcher::{FeeData, OPSuccinctDataFetcher};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    start: u64,
    #[clap(long)]
    end: u64,
    #[clap(long, default_value = ".env")]
    env_file: String,
}

struct AggregateFeeData {
    start: u64,
    end: u64,
    num_transactions: u64,
    total_l1_fee: U256,
}

fn aggregate_fee_data(fee_data: Vec<FeeData>) -> Result<AggregateFeeData> {
    let mut aggregate_fee_data = AggregateFeeData {
        start: fee_data[0].block_number,
        end: fee_data[fee_data.len() - 1].block_number,
        num_transactions: 0,
        total_l1_fee: U256::ZERO,
    };

    for data in fee_data {
        aggregate_fee_data.num_transactions += 1;
        aggregate_fee_data.total_l1_fee += data.l1_gas_cost;
    }

    Ok(aggregate_fee_data)
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

    // for data in fee_data {
    //     println!(
    //         "Block: {}, Tx Index: {}, Tx Hash: {}, L1 Gas Cost: {}, L1 Fee: {}, L1 Base Fee Scalar: {}, L1 Blob Base Fee: {}, L1 Blob Base Fee Scalar: {}",
    //         data.block_number,
    //         data.tx_index,
    //         data.tx_hash,
    //         data.l1_gas_cost,
    //         data.l1_block_info.l1_fee.unwrap_or(0),
    //         data.l1_block_info.l1_base_fee_scalar.unwrap_or(0),
    //         data.l1_block_info.l1_blob_base_fee.unwrap_or(0),
    //         data.l1_block_info.l1_blob_base_fee_scalar.unwrap_or(0),
    //     );
    // }

    // let aggregate_fee_data = aggregate_fee_data(fee_data)?;
    // println!(
    //     "Start: {}, End: {}, Aggregate: {} transactions, {:.18} GWei L1 fee",
    //     aggregate_fee_data.start,
    //     aggregate_fee_data.end,
    //     aggregate_fee_data.num_transactions,
    //     (aggregate_fee_data.total_l1_fee) / U256::from(10).pow(U256::from(9))
    // );

    let modified_aggregate_fee_data = aggregate_fee_data(modified_fee_data)?;
    println!(
        "Start: {}, End: {}, Aggregate: {} transactions, {:.18} GWei L1 fee",
        modified_aggregate_fee_data.start,
        modified_aggregate_fee_data.end,
        modified_aggregate_fee_data.num_transactions,
        (modified_aggregate_fee_data.total_l1_fee) / U256::from(10).pow(U256::from(9))
    );

    Ok(())
}
