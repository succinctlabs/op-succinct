use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::Http,
};
use anyhow::Result;
use clap::Parser;
use futures::{stream, StreamExt};
use log::info;
use op_alloy_network::Optimism;
use op_succinct_host_utils::fetcher::BlockInfo;
use reqwest::{Client, Url};
use sp1_sdk::utils;
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
};

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

#[derive(Debug, Clone, Default, serde::Serialize)]
struct AggregatedBlockData {
    transaction_count: u64,
    gas_used: u64,
    total_l1_fees: u128,
    total_tx_fees: u128,
    nb_blocks: u64,
    avg_txns_per_block: f64,
    avg_gas_per_block: f64,
    avg_l1_fees_per_block: f64,
    avg_tx_fees_per_block: f64,
}

impl AggregatedBlockData {
    fn new(block_data: &[BlockInfo]) -> Self {
        let total_txns: u64 = block_data.iter().map(|b| b.transaction_count).sum();
        let total_gas: u64 = block_data.iter().map(|b| b.gas_used).sum();
        let total_l1_fees: u128 = block_data.iter().map(|b| b.total_l1_fees).sum();
        let total_tx_fees: u128 = block_data.iter().map(|b| b.total_tx_fees).sum();
        let num_blocks = block_data.len() as u64;

        Self {
            transaction_count: total_txns,
            gas_used: total_gas,
            total_l1_fees,
            total_tx_fees,
            nb_blocks: num_blocks,
            avg_txns_per_block: ((total_txns - num_blocks) as f64 / num_blocks as f64),
            avg_gas_per_block: total_gas as f64 / num_blocks as f64,
            avg_l1_fees_per_block: total_l1_fees as f64 / num_blocks as f64,
            avg_tx_fees_per_block: total_tx_fees as f64 / num_blocks as f64,
        }
    }
}

impl std::fmt::Display for AggregatedBlockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nAggregate Block Data:")?;
        writeln!(f, "Total Blocks: {}", self.nb_blocks)?;
        writeln!(
            f,
            "Total Transactions (excluding system txns): {}",
            self.transaction_count - self.nb_blocks
        )?;
        writeln!(f, "Total Gas Used: {}", self.gas_used)?;
        writeln!(
            f,
            "Total L1 Fees: {:.6} ETH",
            self.total_l1_fees as f64 / 1e18
        )?;
        writeln!(
            f,
            "Total TX Fees: {:.6} ETH",
            self.total_tx_fees as f64 / 1e18
        )?;
        writeln!(
            f,
            "Avg Txns/Block (excluding system txns): {:.5}",
            self.avg_txns_per_block
        )?;
        writeln!(f, "Avg Gas/Block: {:.2}", self.avg_gas_per_block)?;
        writeln!(
            f,
            "Avg L1 Fees/Block: {:.6} ETH",
            self.avg_l1_fees_per_block / 1e18
        )?;
        writeln!(
            f,
            "Avg TX Fees/Block: {:.6} ETH",
            self.avg_tx_fees_per_block / 1e18
        )
    }
}

/// Write the block data to a CSV file. Returns the block data.
async fn write_block_data_to_csv(
    args: &BlockDataArgs,
    l2_provider: Arc<RootProvider<Http<Client>, Optimism>>,
    report_path: PathBuf,
) -> Result<Vec<BlockInfo>> {
    // Create CSV writer with headers
    let mut csv_writer = csv::Writer::from_path(&report_path)?;

    // Process blocks in chunks and write incrementally.
    const CHUNK_SIZE: u64 = 1000;
    let mut current_start = args.start;

    let mut all_data: Vec<BlockInfo> = Vec::new();
    while current_start <= args.end {
        let chunk_end = (current_start + CHUNK_SIZE - 1).min(args.end);

        let chunk_data = stream::iter(current_start..=chunk_end)
            .map(|block_number| {
                let l2_provider = l2_provider.clone();
                async move {
                    let block = match l2_provider
                        .get_block_by_number(block_number.into(), false)
                        .await?
                    {
                        Some(block) => block,
                        None => return Err(anyhow::anyhow!("Block {} not found", block_number)),
                    };

                    let receipts = match l2_provider.get_block_receipts(block_number.into()).await?
                    {
                        Some(receipts) => receipts,
                        None => {
                            return Err(anyhow::anyhow!(
                                "Receipts for block {} not found",
                                block_number
                            ))
                        }
                    };
                    let total_l1_fees: u128 = receipts
                        .iter()
                        .map(|tx| tx.l1_block_info.l1_fee.unwrap_or(0))
                        .sum();
                    let total_tx_fees: u128 = receipts
                        .iter()
                        .map(|tx| {
                            tx.inner.effective_gas_price * tx.inner.gas_used
                                + tx.l1_block_info.l1_fee.unwrap_or(0)
                        })
                        .sum();

                    Ok(BlockInfo {
                        block_number,
                        transaction_count: block.transactions.len() as u64,
                        gas_used: block.header.gas_used,
                        total_l1_fees,
                        total_tx_fees,
                    })
                }
            })
            .buffered(100)
            .collect::<Vec<Result<BlockInfo>>>()
            .await;

        // Write the data for each block in the chunk to the CSV file.
        for block_result in chunk_data {
            if let Ok(block) = block_result {
                csv_writer.serialize(&block)?;
                all_data.push(block);
            }
        }
        csv_writer.flush()?;

        info!("Processed blocks {} to {}", current_start, chunk_end);
        current_start = chunk_end + 1;
    }

    Ok(all_data)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = BlockDataArgs::parse();

    dotenv::from_path(&args.env_file).ok();
    utils::setup_logger();

    let l2_provider: Arc<RootProvider<Http<Client>, Optimism>> = Arc::new(
        ProviderBuilder::default()
            .on_http(Url::from_str(&std::env::var("L2_RPC").unwrap()).unwrap()),
    );
    let l2_chain_id = l2_provider.get_chain_id().await?;

    // Confirm that the start and end blocks are valid.
    let start_block = l2_provider
        .get_block_by_number(args.start.into(), false)
        .await?;
    let end_block = l2_provider
        .get_block_by_number(args.end.into(), false)
        .await?;
    if start_block.is_none() || end_block.is_none() {
        return Err(anyhow::anyhow!("Invalid start or end block"));
    }

    // Create the file at the report path.
    let report_path = PathBuf::from(format!(
        "block-data/{}/{}-{}-block-data.csv",
        l2_chain_id, args.start, args.end
    ));
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let l2_block_data = write_block_data_to_csv(&args, l2_provider, report_path).await?;

    // Calculate aggregate statistics.
    let aggregated_data = AggregatedBlockData::new(&l2_block_data);
    println!("{}", aggregated_data);
    Ok(())
}
