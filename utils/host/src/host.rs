use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use hokulea_host_bin::cfg::SingleChainHostWithEigenDA;
use kona_host::single::{SingleChainHost, SingleChainHostError};
use kona_preimage::{BidirectionalChannel, Channel};
use tokio::task::JoinHandle;

use crate::witness_generation::WitnessGenerator;

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
}
