use alloy_consensus::Transaction;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::B256;
use alloy_provider::Provider;
use alloy_rpc_types::eth::Transaction as EthTransaction;
use anyhow::{anyhow, Result};
use hana_blobstream::blobstream::{blostream_address, SP1Blobstream};
use kona_rpc::SafeHeadResponse;
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCMode};

/// Extract the Celestia height from batcher transaction based on the version byte.
///
/// Returns:
/// - Some(height) if the transaction is a valid Celestia batcher transaction.
/// - None if the transaction is an ETH DA transaction (EIP4844 transaction or non-EIP4844
///   transaction with version byte 0x00).
/// - Err if the version byte is invalid or the da layer byte is incorrect for non-EIP4844
///   transactions.
pub fn extract_celestia_height(tx: &EthTransaction) -> Result<Option<u64>> {
    // Skip calldata parsing for EIP4844 transactions since there is no calldata.
    if tx.inner.is_eip4844() {
        Ok(None)
    } else {
        let calldata = tx.input();
        // Check version byte to determine if it is ETH DA or Alt DA.
        // https://specs.optimism.io/protocol/derivation.html#batcher-transaction-format.
        match calldata[0] {
            0x00 => Ok(None), // ETH DA transaction.
            0x01 => {
                // Check that the DA layer byte prefix is correct.
                // https://github.com/ethereum-optimism/specs/discussions/135.
                if calldata[2] != 0x0c {
                    return Err(anyhow!("Invalid prefix for Celestia batcher transaction"));
                }

                // The encoding of the commitment is the Celestia block height followed
                // by the Celestia commitment.
                let height_bytes = &calldata[3..11];
                let celestia_height = u64::from_le_bytes(height_bytes.try_into().unwrap());

                Ok(Some(celestia_height))
            }
            _ => Err(anyhow!("Invalid version byte for batcher transaction")),
        }
    }
}

/// Get the latest Celestia block height that has been committed to Ethereum via Blobstream.
pub async fn get_latest_blobstream_celestia_block(fetcher: &OPSuccinctDataFetcher) -> Result<u64> {
    let blobstream_contract = SP1Blobstream::new(
        blostream_address(fetcher.rollup_config.as_ref().unwrap().l1_chain_id)
            .expect("Failed to fetch blobstream contract address"),
        fetcher.l1_provider.clone(),
    );

    let latest_celestia_block = blobstream_contract.latestBlock().call().await?;
    Ok(latest_celestia_block)
}

/// Find the L1 block that posted a batch with Celestia height less than or equal to the given
/// target height. This uses binary search to efficiently find the appropriate L1 block.
pub async fn find_l1_block_for_celestia_height(
    fetcher: &OPSuccinctDataFetcher,
    target_celestia_height: u64,
    search_start_l1_block: u64,
    search_end_l1_block: u64,
) -> Result<Option<u64>> {
    let batch_inbox_address = fetcher.rollup_config.as_ref().unwrap().batch_inbox_address;

    let mut low = search_start_l1_block;
    let mut high = search_end_l1_block;
    let mut result_l1_block = None;

    while low <= high {
        let mid = (high + low) / 2;
        let l1_block_hex = format!("0x{mid:x}");

        // Get the safe head for the chain at the midpoint. This will return the latest
        // transaction with a batch.
        let result: SafeHeadResponse = fetcher
            .fetch_rpc_data_with_mode(
                RPCMode::L2Node,
                "optimism_safeHeadAtL1Block",
                vec![l1_block_hex.into()],
            )
            .await?;
        let safe_head_l1_block_number = result.l1_block.number;

        let block = fetcher
            .l1_provider
            .get_block_by_number(BlockNumberOrTag::Number(safe_head_l1_block_number))
            .full()
            .await?
            .unwrap();

        let mut found_valid_batch = false;
        for tx in block.transactions.txns() {
            if let Some(to_addr) = tx.to() {
                if to_addr == batch_inbox_address {
                    match extract_celestia_height(tx)? {
                        None => {
                            // ETH DA transaction - always valid
                            found_valid_batch = true;
                            result_l1_block = Some(mid);
                        }
                        Some(celestia_height) => {
                            if celestia_height <= target_celestia_height {
                                found_valid_batch = true;
                                result_l1_block = Some(mid);
                            }
                        }
                    }
                }
            }
        }

        if found_valid_batch {
            low = mid + 1; // Look for a more recent batch
        } else {
            high = mid - 1; // The batch at this block is too new, look earlier
        }
    }

    Ok(result_l1_block)
}

/// Calculate a safe L1 head for Celestia DA by finding the L1 block where the last safe
/// Celestia batch was posted, considering blobstream commitments.
pub async fn calculate_celestia_safe_l1_head(
    fetcher: &OPSuccinctDataFetcher,
    l2_end_block: u64,
    safe_db_fallback: bool,
) -> Result<B256> {
    // Get the latest Celestia block committed via Blobstream
    let latest_committed_celestia_block = get_latest_blobstream_celestia_block(fetcher).await?;

    // Get the L1 block range to search using the existing method from the fetcher
    let (_, start_l1_block) = match fetcher.get_safe_l1_block_for_l2_block(l2_end_block).await {
        Ok((_, block_num)) => (B256::ZERO, block_num), // We only need the block number
        Err(_) => {
            // Fallback to timestamp-based estimation if SafeDB is not available
            if safe_db_fallback {
                let l2_block_timestamp =
                    fetcher.get_l2_header(l2_end_block.into()).await?.timestamp;
                let finalized_l1_timestamp =
                    fetcher.get_l1_header(alloy_eips::BlockId::finalized()).await?.timestamp;
                let max_batch_post_delay_minutes = 40;
                let target_timestamp = std::cmp::min(
                    l2_block_timestamp + (max_batch_post_delay_minutes * 60),
                    finalized_l1_timestamp,
                );
                fetcher.find_l1_block_by_timestamp(target_timestamp).await?
            } else {
                return Err(anyhow!("SafeDB is not available and safe_db_fallback is disabled"));
            }
        }
    };

    let finalized_l1_header = fetcher.get_l1_header(alloy_eips::BlockId::finalized()).await?;

    // Find the L1 block that posted a batch with Celestia height <= latest committed height
    if let Some(safe_l1_block) = find_l1_block_for_celestia_height(
        fetcher,
        latest_committed_celestia_block,
        start_l1_block,
        finalized_l1_header.number,
    )
    .await?
    {
        // Add a small buffer to ensure data availability
        let l1_head_number = std::cmp::min(safe_l1_block + 10, finalized_l1_header.number);
        Ok(fetcher.get_l1_header(l1_head_number.into()).await?.hash_slow())
    } else {
        // Fallback: use a conservative offset
        let l1_head_number = std::cmp::min(start_l1_block + 50, finalized_l1_header.number);
        Ok(fetcher.get_l1_header(l1_head_number.into()).await?.hash_slow())
    }
}
