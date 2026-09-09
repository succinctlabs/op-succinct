use std::sync::Arc;

use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use hokulea_host_bin::cfg::SingleChainHostWithEigenDA;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

use crate::witness_generator::EigenDAWitnessGenerator;

#[derive(Clone)]
pub struct EigenDAOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    pub witness_generator: Arc<EigenDAWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for EigenDAOPSuccinctHost {
    type Args = SingleChainHostWithEigenDA;
    type WitnessGenerator = EigenDAWitnessGenerator;

    fn witness_generator(&self) -> &Self::WitnessGenerator {
        &self.witness_generator
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<SingleChainHostWithEigenDA> {
        // If l1_head_hash is not provided, calculate a safe L1 head
        let l1_head = match l1_head_hash {
            Some(hash) => hash,
            None => self.fetcher.calculate_safe_l1_head(l2_end_block, safe_db_fallback).await?,
        };

        let host = self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head).await?;

        let eigenda_proxy_address = std::env::var("EIGENDA_PROXY_ADDRESS").ok();
        Ok(SingleChainHostWithEigenDA { kona_cfg: host, eigenda_proxy_address, verbose: 1 })
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.kona_cfg.l1_head)
    }
}

impl EigenDAOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self { fetcher, witness_generator: Arc::new(EigenDAWitnessGenerator {}) }
    }
}
