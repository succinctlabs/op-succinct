use std::sync::Arc;

use alloy_consensus::Transaction;
use alloy_eips::BlockId;
use alloy_primitives::B256;
use alloy_provider::Provider;
use anyhow::Result;
use async_trait::async_trait;
use hana_blobstream::blobstream::{blostream_address, SP1Blobstream};
use hana_host::celestia::{CelestiaCfg, CelestiaChainHost};
use kona_rpc::SafeHeadResponse;
use op_succinct_celestia_client_utils::executor::CelestiaDAWitnessExecutor;
use op_succinct_host_utils::{
    fetcher::{OPSuccinctDataFetcher, RPCMode},
    host::OPSuccinctHost,
};

use crate::blobstream_utils::{calculate_celestia_safe_l1_head, extract_celestia_height};

use crate::witness_generator::CelestiaDAWitnessGenerator;

#[derive(Clone)]
pub struct CelestiaOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    pub witness_generator: Arc<CelestiaDAWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for CelestiaOPSuccinctHost {
    type Args = CelestiaChainHost;
    type WitnessGenerator = CelestiaDAWitnessGenerator;

    fn witness_generator(&self) -> &Self::WitnessGenerator {
        &self.witness_generator
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<CelestiaChainHost> {
        // Calculate L1 head hash using blobstream logic if not provided.
        let l1_head_hash = match l1_head_hash {
            Some(hash) => hash,
            None => {
                self.calculate_safe_l1_head(&self.fetcher, l2_end_block, safe_db_fallback).await?
            }
        };

        let host = self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head_hash).await?;

        // Create `CelestiaCfg` directly from environment variables.
        let celestia_args = CelestiaCfg {
            celestia_connection: std::env::var("CELESTIA_CONNECTION").ok(),
            auth_token: std::env::var("AUTH_TOKEN").ok(),
            namespace: std::env::var("NAMESPACE").ok(),
        };

        Ok(CelestiaChainHost { single_host: host, celestia_args })
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.single_host.l1_head)
    }

    /// Converts the latest Celestia block height in Blobstream to the highest L2 block that can be
    /// included in a range proof.
    ///
    /// 1. Get the latest Celestia block included in a Blobstream commitment.
    /// 2. Loop over the `BatchInbox` from the l1 origin of the latest proposed block number to the
    ///    finalized L1 block number.
    /// 3. For each `BatchInbox` transaction, check if it contains a Celestia block number greater
    ///    than the latest Celestia block.
    /// 4. If it does, return the L2 block number.
    /// 5. If it doesn't, return None.
    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        latest_proposed_block_number: u64,
    ) -> Result<Option<u64>> {
        let batch_inbox_address = fetcher.rollup_config.as_ref().unwrap().batch_inbox_address;

        let blobstream_contract = SP1Blobstream::new(
            blostream_address(fetcher.rollup_config.as_ref().unwrap().l1_chain_id)
                .expect("Failed to fetch blobstream contract address"),
            fetcher.l1_provider.clone(),
        );
        // Get the latest Celestia block included in a Blobstream commitment.
        let latest_celestia_block = blobstream_contract.latestBlock().call().await?;

        let mut low = fetcher.get_safe_l1_block_for_l2_block(latest_proposed_block_number).await?.1;
        let mut high = fetcher.get_l1_header(BlockId::finalized()).await?.number;

        let mut l2_block_number = None;

        // Binary search between the latest proposed block number and the finalized L1 block number
        // for the batch transaction that has the highest Celestia height less than the
        // latest Celestia height in the latest Blobstream commitment.
        //
        // At each block in the binary search, get the current safe head (this returns the L1 block
        // where the batch was posted). Then, get the block at the safe head and check if it
        // contains a batch transaction with a Celestia height greater than the latest Celestia
        // height in the latest Blobstream commitment.
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
            let l2_safe_head_number = result.safe_head.number;
            let block = fetcher
                .l1_provider
                .get_block_by_number(alloy_eips::BlockNumberOrTag::Number(
                    safe_head_l1_block_number,
                ))
                .full()
                .await?
                .unwrap();

            let mut found_valid_tx = false;
            for tx in block.transactions.txns() {
                if let Some(to_addr) = tx.to() {
                    if to_addr == batch_inbox_address {
                        match extract_celestia_height(tx)? {
                            None => {
                                // ETH DA transaction.
                                found_valid_tx = true;
                                l2_block_number =
                                    Some(fetcher.get_l2_header(BlockId::finalized()).await?.number);
                            }
                            Some(celestia_height) => {
                                if celestia_height < latest_celestia_block {
                                    found_valid_tx = true;
                                    l2_block_number = Some(l2_safe_head_number);
                                }
                            }
                        }
                    }
                }
            }

            // If a batch b with a lower Celestia height, h1, than the latest Blobstream Celestia
            // height, h, in the latest Blobstream commitment was found, we should try
            // to find a batch b' with a Celestia height, h2, that is h1 < h2 <= h.
            if found_valid_tx {
                low = mid + 1; // Look in higher blocks for a batch with a higher Celestia height
                               // that's less than the latest Blobstream Celestia height.
            } else {
                high = mid - 1; // The Celestia height in the latest committed batch is greater than
                                // the latest Blobstream Celestia height, so look in earlier blocks.
            }
        }

        Ok(l2_block_number)
    }

    async fn calculate_safe_l1_head(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        l2_end_block: u64,
        _safe_db_fallback: bool,
    ) -> Result<B256> {
        calculate_celestia_safe_l1_head(fetcher, l2_end_block).await
    }
}

impl CelestiaOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self {
            fetcher,
            witness_generator: Arc::new(CelestiaDAWitnessGenerator {
                executor: CelestiaDAWitnessExecutor::new(),
            }),
        }
    }
}
