use anyhow::Result;
use metrics::gauge;
use std::time::Duration;
use tracing::{error, info};

use crate::proposer::Proposer;
use alloy_provider::{Network, Provider};

/// ProposerAgglayer wraps the standard Proposer but modifies the run loop
/// to skip the submit_agg_proofs step, as that will be handled by Agglayer.
pub struct ProposerAgglayer<'a, P, N>
where
    P: Provider<N> + 'static,
    N: Network,
{
    inner: &'a Proposer<P, N>,
}

impl<'a, P, N> ProposerAgglayer<'a, P, N>
where
    P: Provider<N> + 'static + Clone,
    N: Network,
{
    /// Create a new ProposerAgglayer that wraps an existing Proposer
    pub fn new(proposer: &'a Proposer<P, N>) -> Self {
        Self { inner: proposer }
    }

    /// Run the proposer in Agglayer mode, which skips the submit_agg_proofs step
    pub async fn run(&self, _grpc_addr: &str) -> Result<()> {
        // Spawn the task completion handler from the inner proposer
        self.inner.spawn_task_completion_handler().await?;

        // Initialize the inner proposer
        self.inner.initialize_proposer().await?;

        // Reset error count
        gauge!("succinct_error_count").set(0.0);

        info!("Starting ProposerAgglayer run loop");

        // Loop interval in seconds
        loop {
            // Wrap the entire loop body in a match to handle errors
            match self.run_loop_iteration().await {
                Ok(_) => {
                    // Normal sleep between iterations
                    tokio::time::sleep(Duration::from_secs(
                        self.inner.driver_config.loop_interval_seconds,
                    ))
                    .await;
                }
                Err(e) => {
                    // Log the error
                    error!("Error in Agglayer proposer loop: {}", e);
                    // Update the error gauge
                    let error_gauge = gauge!("succinct_error_count");
                    error_gauge.increment(1.0);
                    // Pause for 10 seconds before restarting
                    info!("Pausing for 10 seconds before restarting the process");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            }
        }
    }

    /// Run a single loop iteration of the Agglayer proposer.
    /// This is based on the standard Proposer's run_loop_iteration but omits the submit_agg_proofs step.
    async fn run_loop_iteration(&self) -> Result<()> {
        // Validate the requester config matches the contract
        self.inner.validate_contract_config().await?;

        // Log the proposer metrics
        self.inner.log_proposer_metrics().await?;

        // Add new range requests to the database
        self.inner.add_new_ranges().await?;

        // Get all proof statuses of all requests in the proving state
        self.inner.handle_proving_requests().await?;

        // Request all unrequested proofs from the prover network
        self.inner.request_queued_proofs().await?;

        Ok(())
    }
}
