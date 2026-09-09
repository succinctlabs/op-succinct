use std::sync::Arc;

use crate::witness_generator::ETHDAWitnessGenerator;
use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use kona_host::single::SingleChainHost;
use op_succinct_ethereum_client_utils::executor::ETHDAWitnessExecutor;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

#[derive(Clone)]
pub struct SingleChainOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    witness_generator: Arc<ETHDAWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for SingleChainOPSuccinctHost {
    type Args = SingleChainHost;
    type WitnessGenerator = ETHDAWitnessGenerator;

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
        // Calculate L1 head hash using simple logic if not provided.
        let l1_head_hash = match l1_head_hash {
            Some(hash) => hash,
            None => self.fetcher.calculate_safe_l1_head(l2_end_block, safe_db_fallback).await?,
        };

        let host = self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head_hash).await?;
        Ok(host)
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.l1_head)
    }
}

impl SingleChainOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self {
            fetcher,
            witness_generator: Arc::new(ETHDAWitnessGenerator {
                executor: ETHDAWitnessExecutor::new(),
            }),
        }
    }
}
