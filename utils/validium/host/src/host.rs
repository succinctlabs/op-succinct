//! Validium OP Succinct Host
//!
//! Implements OPSuccinctHost for validium DA.
//! Uses SingleChainHost for L1 preimage serving (same as Ethereum DA).
//! Additionally fetches batch data from the op-alt-da server.

use std::sync::Arc;

use alloy_eips::BlockId;
use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use kona_host::single::SingleChainHost;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

use crate::witness_generator::ValidiumWitnessGenerator;

/// Validium OP Succinct Host.
///
/// Uses the same L1 preimage server as Ethereum DA (SingleChainHost).
/// The op-alt-da server URL is stored for batch data fetching.
#[derive(Clone)]
pub struct ValidiumOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    pub witness_generator: Arc<ValidiumWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for ValidiumOPSuccinctHost {
    // Uses SingleChainHost for L1 preimage serving (same as Ethereum DA).
    type Args = SingleChainHost;
    type WitnessGenerator = ValidiumWitnessGenerator;

    fn witness_generator(&self) -> &Self::WitnessGenerator {
        &self.witness_generator
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<SingleChainHost> {
        // Same as Ethereum DA - fetch L1 head and create host args.
        let l1_head_hash = match l1_head_hash {
            Some(hash) => hash,
            None => {
                self.calculate_safe_l1_head(&self.fetcher, l2_end_block, safe_db_fallback).await?
            }
        };

        let host = self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head_hash).await?;
        Ok(host)
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.l1_head)
    }

    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        _: u64,
    ) -> Result<Option<u64>> {
        let finalized_l2_block_number = fetcher.get_l2_header(BlockId::finalized()).await?;
        Ok(Some(finalized_l2_block_number.number))
    }

    async fn calculate_safe_l1_head(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        l2_end_block: u64,
        safe_db_fallback: bool,
    ) -> Result<B256> {
        // Same logic as Ethereum DA.
        let (_, l1_head_number) = fetcher.get_l1_head(l2_end_block, safe_db_fallback).await?;
        // FIXME(fakedev9999): Investigate requirement for L1 head offset beyond batch posting block
        // with safe head > L2 end block.
        let l1_head_number = l1_head_number + 20;
        let finalized_l1_header = fetcher.get_l1_header(BlockId::finalized()).await?;
        let safe_l1_head_number = std::cmp::min(l1_head_number, finalized_l1_header.number);
        Ok(fetcher.get_l1_header(safe_l1_head_number.into()).await?.hash_slow())
    }
}

impl ValidiumOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self {
            fetcher,
            witness_generator: Arc::new(ValidiumWitnessGenerator::new()),
        }
    }
}
