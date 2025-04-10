use std::sync::Arc;

use alloy_primitives::B256;
use async_trait::async_trait;
use kona_host::single::SingleChainHost;
use kona_preimage::BidirectionalChannel;
use op_succinct_client_utils::InMemoryOracle;

use crate::fetcher::OPSuccinctDataFetcher;
use crate::hosts::OPSuccinctHost;
use anyhow::Result;

#[derive(Clone)]
pub struct SingleChainOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,
}

#[async_trait]
impl OPSuccinctHost for SingleChainOPSuccinctHost {
    async fn fetch_and_run(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: Option<bool>,
    ) -> Result<InMemoryOracle> {
        let args = self
            .fetch(l2_start_block, l2_end_block, l1_head_hash, safe_db_fallback)
            .await?;
        self.run(&args).await
    }
}

impl SingleChainOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self { fetcher }
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: Option<bool>,
    ) -> Result<SingleChainHost> {
        self.fetcher
            .get_host_args(
                l2_start_block,
                l2_end_block,
                l1_head_hash,
                safe_db_fallback.expect("`safe_db_fallback` must be set"),
            )
            .await
    }

    async fn run(&self, args: &SingleChainHost) -> Result<InMemoryOracle> {
        let hint = BidirectionalChannel::new()?;
        let preimage = BidirectionalChannel::new()?;

        let server_task = args.start_server(hint.host, preimage.host).await?;

        let in_memory_oracle = Self::run_witnessgen_client(preimage.client, hint.client).await?;
        // Unlike the upstream, manually abort the server task, as it will hang if you wait for both tasks to complete.
        server_task.abort();

        Ok(in_memory_oracle)
    }
}
