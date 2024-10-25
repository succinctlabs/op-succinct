use std::{env, fmt};

use alloy_primitives::U256;
use anyhow::Result;
use op_succinct_host_utils::fetcher::{FeeData, OPSuccinctDataFetcher, RPCConfig};

pub struct AggregateFeeData {
    pub start: u64,
    pub end: u64,
    pub num_transactions: u64,
    pub total_l1_fee: U256,
}

impl fmt::Display for AggregateFeeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eth = self.total_l1_fee / U256::from(10).pow(U256::from(18));
        let gwei = (self.total_l1_fee / U256::from(10).pow(U256::from(9)))
            % U256::from(10).pow(U256::from(9));
        write!(
            f,
            "Start: {}, End: {}, Aggregate: {} transactions, {}.{:09} ETH L1 fee",
            self.start, self.end, self.num_transactions, eth, gwei
        )
    }
}

pub fn aggregate_fee_data(fee_data: Vec<FeeData>) -> Result<AggregateFeeData> {
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

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_l1_fee_scalar() -> Result<()> {
    env::set_var("L1_RPC", "https://ethereum-rpc.publicnode.com");
    env::set_var("L2_RPC", "https://mainnet.optimism.io");

    let fetcher = OPSuccinctDataFetcher::default();

    let (fee_data, modified_fee_data) = tokio::join!(
        fetcher.get_l2_fee_data_range(17423924, 17423928),
        fetcher.get_l2_fee_data_with_modified_l1_fee_scalar(17423924, 17423928, None)
    );

    let fee_data = fee_data?;
    let modified_fee_data = modified_fee_data?;

    let total_aggregate_fee_data = aggregate_fee_data(fee_data)?;
    let modified_total_aggregate_fee_data = aggregate_fee_data(modified_fee_data)?;

    assert_eq!(
        total_aggregate_fee_data.total_l1_fee,
        modified_total_aggregate_fee_data.total_l1_fee
    );

    println!("Success!");

    Ok(())
}
