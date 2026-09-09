//! [`OPSuccinctHost`] implementation for AltDA-backed OP Stack chains.
//!
//! Uses the shared [`OPSuccinctDataFetcher`] to select the L1 head.

use std::sync::Arc;

use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use op_succinct_altda_client_utils::executor::AltDAWitnessExecutor;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

use crate::{cfg::AltDAChainHost, witness_generator::AltDAWitnessGenerator};

#[derive(Clone)]
pub struct AltDAOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    witness_generator: Arc<AltDAWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for AltDAOPSuccinctHost {
    type Args = AltDAChainHost;
    type WitnessGenerator = AltDAWitnessGenerator;

    fn witness_generator(&self) -> &Self::WitnessGenerator {
        &self.witness_generator
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<AltDAChainHost> {
        let l1_head_hash = match l1_head_hash {
            Some(hash) => hash,
            None => self.fetcher.calculate_safe_l1_head(l2_end_block, safe_db_fallback).await?,
        };

        // Get the standard kona SingleChainHost args.
        let single_host =
            self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head_hash).await?;

        // Read the DA server URL from the environment. This is set by the operator when
        // running the host binary with the `--altda-server-url` flag or `ALTDA_SERVER_URL` env.
        let altda_server_url = std::env::var("ALTDA_SERVER_URL").ok();

        Ok(AltDAChainHost { single_host, altda_server_url })
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.single_host.l1_head)
    }
}

impl AltDAOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self {
            fetcher,
            witness_generator: Arc::new(AltDAWitnessGenerator {
                executor: AltDAWitnessExecutor::new(),
            }),
        }
    }
}
