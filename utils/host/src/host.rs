use alloy_primitives::B256;
use anyhow::{bail, Result};
use async_trait::async_trait;
use hokulea_host_bin::cfg::SingleChainHostWithEigenDA;
use kona_host::single::{SingleChainHost, SingleChainHostError};
use kona_preimage::{BidirectionalChannel, Channel};
use tokio::task::JoinHandle;

use crate::{
    fetcher::OPSuccinctDataFetcher, l1_selection::L1BlockSelectionConfig,
    witness_generation::WitnessGenerator,
};

#[async_trait]
pub trait PreimageServerStarter {
    async fn start_server<C>(
        &self,
        hint: C,
        preimage: C,
    ) -> Result<JoinHandle<Result<(), SingleChainHostError>>, SingleChainHostError>
    where
        C: Channel + Send + Sync + 'static;
}

#[async_trait]
impl PreimageServerStarter for SingleChainHost {
    async fn start_server<C>(
        &self,
        hint: C,
        preimage: C,
    ) -> Result<JoinHandle<Result<(), SingleChainHostError>>, SingleChainHostError>
    where
        C: Channel + Send + Sync + 'static,
    {
        self.start_server(hint, preimage).await
    }
}

#[async_trait]
impl PreimageServerStarter for SingleChainHostWithEigenDA {
    async fn start_server<C>(
        &self,
        hint: C,
        preimage: C,
    ) -> Result<JoinHandle<Result<(), SingleChainHostError>>, SingleChainHostError>
    where
        C: Channel + Send + Sync + 'static,
    {
        self.start_server(hint, preimage).await
    }
}

#[async_trait]
pub trait OPSuccinctHost: Send + Sync + 'static {
    type Args: Send + Sync + 'static + Clone + PreimageServerStarter;
    type WitnessGenerator: WitnessGenerator + Send + Sync;

    fn witness_generator(&self) -> &Self::WitnessGenerator;

    /// Fetch the host arguments.
    ///
    /// Parameters:
    /// - `l2_start_block`: The starting L2 block number.
    /// - `l2_end_block`: The ending L2 block number.
    /// - `l1_head_hash`: Optionally supplied L1 head block hash used as the L1 origin.
    /// - `safe_db_fallback`: Flag to indicate whether to fallback to timestamp-based L1 head
    ///   estimation when SafeDB is not available.
    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: bool,
    ) -> Result<Self::Args>;

    /// Run the host and client program.
    ///
    /// Returns the witness which can be supplied to the zkVM.
    async fn run(
        &self,
        args: &Self::Args,
    ) -> Result<<Self::WitnessGenerator as WitnessGenerator>::WitnessData> {
        let preimage = BidirectionalChannel::new()?;
        let hint = BidirectionalChannel::new()?;

        let server_task = args.start_server(hint.host, preimage.host).await?;

        let witness = self.witness_generator().run(preimage.client, hint.client).await?;
        // Unlike the upstream, manually abort the server task, as it will hang if you wait for both
        // tasks to complete.
        server_task.abort();

        Ok(witness)
    }

    /// Get the L1 head hash from the host args.
    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256>;

    /// Get the maximum L2 block number that this host can currently prove against — i.e. the
    /// highest L2 block that may safely appear as the end of a range proof.
    ///
    /// What this resolves to is DA-specific *and* L1-selection-specific:
    /// - ETH DA, default selection: the finalized L2 block number.
    /// - ETH DA, non-default selection (`L1_BLOCK_TAG`/`L1_CONFIRMATIONS`): the L2 safe head at the
    ///   configured L1 anchor (resolved via `optimism_safeHeadAtL1Block`). This is *not* the
    ///   literal L2 finalized block.
    /// - EigenDA: same shape as ETH DA.
    /// - AltDA: same shape as ETH DA.
    ///
    /// The latest proposed block number is assumed to be the highest block number that has been
    /// successfully processed by the host, and is used as a search-start hint.
    async fn get_max_provable_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        latest_proposed_block_number: u64,
    ) -> Result<Option<u64>>;

    /// Calculate a safe L1 head hash for the given L2 end block.
    ///
    /// Each retained DA backend resolves the L1 head from its batch posting block.
    ///
    /// Parameters:
    /// - `fetcher`: The data fetcher for accessing blockchain data.
    /// - `l2_end_block`: The ending L2 block number for the range.
    /// - `safe_db_fallback`: Whether to fallback to timestamp-based estimation when SafeDB is
    ///   unavailable.
    async fn calculate_safe_l1_head(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        l2_end_block: u64,
        safe_db_fallback: bool,
    ) -> Result<B256>;
}

/// Require SafeDB at startup when the configured L1 selection is non-default.
/// The retained DA backends resolve their provable L2 head through SafeDB for this selection.
pub async fn enforce_l1_selection_supported(
    fetcher: &OPSuccinctDataFetcher,
    l1_selection: L1BlockSelectionConfig,
) -> Result<()> {
    if l1_selection.is_default() {
        return Ok(());
    }

    if !fetcher.is_safe_db_activated().await? {
        bail!(
            "L1_BLOCK_TAG={:?} with L1_CONFIRMATIONS={} requires SafeDB to be activated on \
             op-node. Either enable SafeDB on the L2 node, or unset L1_BLOCK_TAG and \
             L1_CONFIRMATIONS to use the default (finalized).",
            l1_selection.tag,
            l1_selection.confirmations
        );
    }

    Ok(())
}
