use std::sync::Arc;

use alloy_primitives::B256;
use anyhow::{bail, Result};
use async_trait::async_trait;
use hana_host::celestia::{CelestiaCfg, CelestiaChainHost};
use op_succinct_celestia_client_utils::executor::CelestiaDAWitnessExecutor;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

use crate::{
    blobstream_utils::{get_celestia_safe_head_info, get_highest_finalized_l2_block},
    witness_generator::CelestiaDAWitnessGenerator,
};

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

    /// Get the highest L2 block that can be safely proven given Celestia's Blobstream commitments.
    /// Returns the maximum L2 block number where all referenced Celestia data has been committed
    /// to Ethereum and is verifiable in proofs.
    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        latest_proposed_block_number: u64,
    ) -> Result<Option<u64>> {
        get_highest_finalized_l2_block(fetcher, latest_proposed_block_number).await
    }

    /// Calculate the safe L1 head hash for Celestia DA considering Blobstream commitments.
    /// Finds the latest L1 block containing batches with Celestia data committed via Blobstream.
    async fn calculate_safe_l1_head(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        l2_end_block: u64,
        _safe_db_fallback: bool,
    ) -> Result<B256> {
        match get_celestia_safe_head_info(fetcher, l2_end_block).await? {
            Some(safe_head) => safe_head.get_l1_hash(fetcher).await,
            None => bail!("Failed to find a safe L1 block for the given L2 block."),
        }
    }

    /// Celestia's max provable L2 block is driven by Blobstream commitments, not
    /// `optimism_safeHeadAtL1Block`, so SafeDB activation is irrelevant.
    ///
    /// Only reachable when [`Self::supports_non_default_l1_selection`] returns `true`; kept
    /// here to document intent in case that ever flips.
    fn requires_safe_db_for_non_default_l1_selection(&self) -> bool {
        false
    }

    /// Celestia's proving path ignores `L1_BLOCK_TAG` and `L1_CONFIRMATIONS`:
    /// `calculate_safe_l1_head` is Blobstream-driven (via the op-celestia-indexer) and
    /// `get_finalized_l2_block_number` searches between `latest_proposed_block_number` and
    /// the L2 finalized header without consulting `fetcher.l1_selection`. Accepting the
    /// knob as a silent no-op would mislead operators into believing they had tightened
    /// or relaxed proof latency, so non-default selections are rejected at startup by the
    /// shared `enforce_l1_selection_supported` helper used in proposer binaries and the
    /// covered operator-facing utility scripts.
    fn supports_non_default_l1_selection(&self) -> bool {
        false
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
