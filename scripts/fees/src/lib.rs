use std::fmt;

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

#[tokio::test]
async fn test_l1_fee_scalar() -> Result<()> {
    let fetcher = OPSuccinctDataFetcher {
        rpc_config: RPCConfig {
            l2_rpc: "https://mainnet.optimism.io".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let fee_data = fetcher.get_l2_fee_data_range(17423924, 17423925).await?;

    let modified_fee_data = fetcher
        .get_l2_fee_data_with_modified_l1_fee_scalar(17423924, 17423925, U256::from(1000000000))
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
