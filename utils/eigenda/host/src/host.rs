use std::{sync::Arc, time::Duration};

use alloy_eips::BlockId;
use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use hydro_host::eigenda::{EigenDACfg, EigenDAChainHost};
use op_succinct_eigenda_client_utils::executor::EigendaDAWitnessExecutor;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, host::OPSuccinctHost};

use crate::witness_generator::EigendaDAWitnessGenerator;

#[derive(Clone)]
pub struct EigendaOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    pub witness_generator: Arc<EigendaDAWitnessGenerator>,
}

#[async_trait]
impl OPSuccinctHost for EigendaOPSuccinctHost {
    type Args = EigenDAChainHost;
    type WitnessGenerator = EigendaDAWitnessGenerator;

    fn witness_generator(&self) -> &Self::WitnessGenerator {
        &self.witness_generator
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<EigenDAChainHost> {
        // Calculate L1 head hash using blobstream logic if not provided.
        let l1_head_hash = match l1_head_hash {
            Some(hash) => hash,
            None => {
                self.calculate_safe_l1_head(&self.fetcher, l2_end_block, safe_db_fallback).await?
            }
        };

        let host = self.fetcher.get_host_args(l2_start_block, l2_end_block, l1_head_hash).await?;

        // Create `EigenDACfg` directly from environment variables.
        let eigen_da_args = EigenDACfg {
            proxy_url: std::env::var("EIGEN_DA_PROXY_URL").ok(),
            retrieve_timeout: Duration::from_secs(
                std::env::var("EIGEN_DA_RETRIEVE_TIMEOUT")
                    .unwrap_or_else(|_| "120".into())
                    .parse()
                    .unwrap(),
            ),
        };

        Ok(EigenDAChainHost { single_host: host, eigen_da_args })
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.single_host.l1_head)
    }

    /// Get the highest L2 block that can be safely proven given EigenDA's commitments.
    /// Returns the maximum L2 block number where all referenced EigenDA data has been committed
    /// to Ethereum and is verifiable in proofs.
    /// [TODO] CHECK
    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        _latest_proposed_block_number: u64,
    ) -> Result<Option<u64>> {
        let finalized_l2_block_number = fetcher.get_l2_header(BlockId::finalized()).await?;
        Ok(Some(finalized_l2_block_number.number))
    }

    /// Calculate the safe L1 head hash for EigenDA considering EigenDA commitments.
    /// Finds the latest L1 block containing batches with EigenDA data committed via EigenDA.
    /// [TODO] CHECK
    async fn calculate_safe_l1_head(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        l2_end_block: u64,
        safe_db_fallback: bool,
    ) -> Result<B256> {
        // For Ethereum DA, use a simple approach with minimal offset.
        let (_, l1_head_number) = fetcher.get_l1_head(l2_end_block, safe_db_fallback).await?;

        // FIXME(fakedev9999): Investigate requirement for L1 head offset beyond batch posting block
        // with safe head > L2 end block.
        // Add a small buffer for Ethereum DA.
        let l1_head_number = l1_head_number + 20;

        // Ensure we don't exceed the finalized L1 header.
        let finalized_l1_header = fetcher.get_l1_header(BlockId::finalized()).await?;
        let safe_l1_head_number = std::cmp::min(l1_head_number, finalized_l1_header.number);

        Ok(fetcher.get_l1_header(safe_l1_head_number.into()).await?.hash_slow())
    }
}

impl EigendaOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self {
            fetcher,
            witness_generator: Arc::new(EigendaDAWitnessGenerator {
                executor: EigendaDAWitnessExecutor::new(),
            }),
        }
    }
}
