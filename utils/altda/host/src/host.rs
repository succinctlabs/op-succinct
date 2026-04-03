//! [`OPSuccinctHost`] implementation for AltDA-backed OP Stack chains.
//!
//! Follows the same pattern as [`SingleChainOPSuccinctHost`] (Ethereum DA). AltDA does not
//! require special L1 head calculation (unlike Celestia's Blobstream), so the safe L1 head
//! logic is identical to Ethereum DA: simple offset from the batch posting block.

use std::sync::Arc;

use alloy_eips::BlockId;
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
            None => {
                self.calculate_safe_l1_head(&self.fetcher, l2_end_block, safe_db_fallback).await?
            }
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
        // AltDA uses the same simple offset logic as Ethereum DA.
        // No special commitment tracking (unlike Celestia's Blobstream).
        let (_, l1_head_number) = fetcher.get_l1_head(l2_end_block, safe_db_fallback).await?;

        // Add a buffer to ensure all relevant L1 data is available.
        let l1_head_number = l1_head_number + 20;

        // Ensure we don't exceed the finalized L1 header.
        let finalized_l1_header = fetcher.get_l1_header(BlockId::finalized()).await?;
        let safe_l1_head_number = std::cmp::min(l1_head_number, finalized_l1_header.number);

        Ok(fetcher.get_l1_header(safe_l1_head_number.into()).await?.hash_slow())
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
