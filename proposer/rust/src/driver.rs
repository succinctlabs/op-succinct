use crate::config::Config;
use crate::metrics::Metrics;
use crate::rpc::RpcClient;
use crate::types::{ProofRequest, ProofStatus, ProofType, Span};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, U64},
};
use eyre::Result;
use std::sync::Arc;
use tokio::time::Duration;
use tracing::{error, info, warn};

pub struct Driver {
    config: Config,
    l1_provider: Provider<Http>,
    l2_provider: Provider<Http>,
    metrics: Arc<Metrics>,
    rpc_client: RpcClient,
}

impl Driver {
    pub async fn new(
        config: Config,
        l1_provider: Provider<Http>,
        l2_provider: Provider<Http>,
        metrics: Arc<Metrics>,
    ) -> Result<Self> {
        let rpc_client = RpcClient::new(config.op_succinct_server_url.clone());

        Ok(Self {
            config,
            l1_provider,
            l2_provider,
            metrics,
            rpc_client,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting driver");

        // Main loop
        loop {
            tokio::time::sleep(Duration::from_secs(self.config.poll_interval)).await;

            if let Err(e) = self.process_next_batch().await {
                error!("Error processing batch: {}", e);
                continue;
            }
        }
    }

    async fn process_next_batch(&self) -> Result<()> {
        // Get the latest finalized L2 block
        let finalized_block = self.get_finalized_l2_block().await?;
        self.metrics.record_l2_finalized_block(finalized_block);

        // Get the latest proven block from our local state
        let latest_proven_block = self.get_latest_proven_block().await?;

        // If we've already proven up to the finalized block, nothing to do
        if latest_proven_block >= finalized_block {
            return Ok(());
        }

        // Split the range into spans based on max_block_range_per_span_proof
        let spans = self.split_range(latest_proven_block + 1, finalized_block);

        // Request proofs for each span
        for span in spans {
            if let Err(e) = self.request_span_proof(span).await {
                error!("Failed to request span proof: {}", e);
                continue;
            }
        }

        Ok(())
    }

    async fn get_finalized_l2_block(&self) -> Result<u64> {
        let block = self
            .l2_provider
            .get_block(BlockNumber::Finalized)
            .await?
            .ok_or_else(|| eyre::eyre!("No finalized block found"))?;

        Ok(block.number.unwrap_or(U64::zero()).as_u64())
    }

    async fn get_latest_proven_block(&self) -> Result<u64> {
        // TODO: Implement getting the latest proven block from L2OutputOracle contract
        // For now, return 0 to start from the beginning
        Ok(0)
    }

    fn split_range(&self, start: u64, end: u64) -> Vec<Span> {
        let mut spans = Vec::new();
        let mut current = start;

        while current + self.config.max_block_range_per_span_proof <= end {
            spans.push(Span {
                start: current,
                end: current + self.config.max_block_range_per_span_proof,
            });
            current += self.config.max_block_range_per_span_proof;
        }

        // Add final partial span if there are remaining blocks
        if current < end {
            spans.push(Span {
                start: current,
                end,
            });
        }

        spans
    }

    async fn request_span_proof(&self, span: Span) -> Result<()> {
        info!(
            "Requesting span proof for blocks {} to {}",
            span.start, span.end
        );

        let proof_id = self
            .rpc_client
            .request_span_proof(span.start, span.end)
            .await?;

        // TODO: Store proof request in database
        let request = ProofRequest {
            id: 0, // This should be generated when storing in DB
            start_block: span.start,
            end_block: span.end,
            proof_type: ProofType::Span,
            status: ProofStatus::Pending,
            proof: None,
        };

        info!(
            "Requested span proof with ID {:?} for blocks {} to {}",
            proof_id, span.start, span.end
        );

        Ok(())
    }
}
