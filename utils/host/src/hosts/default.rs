use std::sync::Arc;

use alloy_primitives::B256;
use async_trait::async_trait;
use kona_host::single::SingleChainHost;
use op_succinct_client_utils::InMemoryOracle;

use crate::fetcher::OPSuccinctDataFetcher;
use crate::hosts::OPSuccinctHost;
use anyhow::Result;
use kona_preimage::BidirectionalChannel;

#[derive(Clone)]
pub struct SingleChainOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
}

#[async_trait]
impl OPSuccinctHost for SingleChainOPSuccinctHost {
    type Args = SingleChainHost;

    /// Run the host and client program.
    ///
    /// Returns the in-memory oracle which can be supplied to the zkVM.
    async fn run(&self, args: &Self::Args) -> Result<InMemoryOracle> {
        let hint = BidirectionalChannel::new()?;
        let preimage = BidirectionalChannel::new()?;

        let server_task = args.start_server(hint.host, preimage.host).await?;

        let in_memory_oracle = Self::run_witnessgen_client(preimage.client, hint.client).await?;
        // Unlike the upstream, manually abort the server task, as it will hang if you wait for both tasks to complete.
        server_task.abort();

        Ok(in_memory_oracle)
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
    ) -> Result<Self::Args> {
        self.fetcher
            .get_host_args(l2_start_block, l2_end_block, l1_head_hash)
            .await
    }
}

impl SingleChainOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self { fetcher }
    }
}
