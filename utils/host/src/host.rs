use alloy_primitives::B256;
use anyhow::{bail, Result};
use async_trait::async_trait;
use hana_host::celestia::CelestiaChainHost;
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
impl PreimageServerStarter for CelestiaChainHost {
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

    /// Get the finalized L2 block number. This is used to determine the highest block that can be
    /// included in a range proof.
    ///
    /// For ETH DA, this is the finalized L2 block number.
    /// For Celestia, this is the highest L2 block included in the latest Blobstream commitment.
    ///
    /// The latest proposed block number is assumed to be the highest block number that has been
    /// successfully processed by the host.
    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        latest_proposed_block_number: u64,
    ) -> Result<Option<u64>>;

    /// Calculate a safe L1 head hash for the given L2 end block.
    ///
    /// This method is DA-specific:
    /// - For ETH DA: Uses simple offset logic.
    /// - For Celestia DA: Uses blobstream commitment logic to ensure data availability.
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

    /// Whether this host's non-default L1 selection path depends on op-node's SafeDB.
    ///
    /// Ethereum and EigenDA resolve the max provable L2 block via
    /// `optimism_safeHeadAtL1Block(resolved_l1_number)` under non-default selections, which
    /// requires SafeDB and has no fallback. Celestia derives the max provable L2 block from
    /// Blobstream commitments and does not touch SafeDB.
    ///
    /// Proposer entry points use this to decide whether to hard-fail at startup when
    /// SafeDB is inactive under a non-default selection. Only consulted after
    /// [`OPSuccinctHost::supports_non_default_l1_selection`] returns `true`.
    fn requires_safe_db_for_non_default_l1_selection(&self) -> bool {
        true
    }

    /// Whether this host actually honors non-default `L1_BLOCK_TAG` / `L1_CONFIRMATIONS`
    /// values in its proving path.
    ///
    /// Ethereum and EigenDA hosts thread the configured selection through
    /// `calculate_safe_l1_head` / `get_finalized_l2_block_number`, so non-default selections
    /// meaningfully change behavior.
    ///
    /// Celestia's proving path is driven by Blobstream commitments and the op-celestia-indexer
    /// rather than by an L1 block tag; non-default selections have no effect on the L1 head
    /// or L2 finality resolution the proposer uses. To avoid silently accepting a knob that
    /// would not actually apply, Celestia returns `false` and proposer / relevant utility
    /// entrypoints reject non-default selections at startup.
    fn supports_non_default_l1_selection(&self) -> bool {
        true
    }
}

/// Enforce, at startup, that the configured L1 selection is compatible with the active DA
/// backend. Used by both production proposer binaries and user-facing utility CLIs that
/// initialize a host so the policy is consistent across the workspace.
///
/// Two invariants:
///
/// 1. If the host does not honor non-default selections in its proving path
///    (`host.supports_non_default_l1_selection() == false`), any non-default value is rejected
///    so operators are not misled into believing they tightened or relaxed proof latency.
/// 2. If the host honors non-default selections but requires SafeDB for the L1 -> L2
///    resolution path (Ethereum / EigenDA), a non-default value combined with an inactive
///    SafeDB is rejected.
///
/// The default selection (`finalized`, `0`) is always allowed and bypasses both checks.
pub async fn enforce_l1_selection_supported<H: OPSuccinctHost>(
    host: &H,
    fetcher: &OPSuccinctDataFetcher,
    l1_selection: L1BlockSelectionConfig,
) -> Result<()> {
    if l1_selection.is_default() {
        return Ok(());
    }

    if !host.supports_non_default_l1_selection() {
        bail!(
            "L1_BLOCK_TAG={:?} with L1_CONFIRMATIONS={} is not supported by this DA backend. \
             The current backend's proving path does not honor these values. Unset \
             L1_BLOCK_TAG and L1_CONFIRMATIONS to use the default (finalized).",
            l1_selection.tag,
            l1_selection.confirmations
        );
    }

    if host.requires_safe_db_for_non_default_l1_selection() && !fetcher.is_safe_db_activated().await?
    {
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
