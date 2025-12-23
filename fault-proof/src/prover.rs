//! Proof Provider Abstraction
//!
//! This module provides a unified abstraction over proof generation, enabling proposer
//! to be agnostic about how proofs are generated (network, local mock, etc.).

use std::{sync::Arc, time::Duration};

use alloy_primitives::{Address, B256};
use anyhow::{bail, Context, Result};
use sp1_sdk::{
    network::{proto::types::FulfillmentStatus, FulfillmentStrategy, NetworkMode},
    NetworkProver, SP1ProofMode, SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin,
    SP1VerifyingKey, SP1_CIRCUIT_VERSION,
};

use op_succinct_proof_utils::get_range_elf_embedded;

// ============================================================================
// Constants and Types
// ============================================================================

/// Polling interval (in seconds) for checking proof status.
/// Matches the SP1 SDK's internal polling interval.
pub const PROOF_STATUS_POLL_INTERVAL: u64 = 2;

/// Unique identifier for a proof request.
pub type ProofId = B256;

/// Configuration for proof provider operations.
#[derive(Debug, Clone)]
pub struct ProofProviderConfig {
    /// Overall proving timeout (seconds).
    pub timeout: u64,
    /// Timeout for individual network calls (seconds).
    pub network_calls_timeout: u64,
    /// Auction timeout (seconds) - cancel if no prover picks up.
    pub auction_timeout: u64,
    /// Fulfillment strategy for range proofs.
    pub range_proof_strategy: FulfillmentStrategy,
    /// Fulfillment strategy for aggregation proofs.
    pub agg_proof_strategy: FulfillmentStrategy,
    /// Proof mode for aggregation proofs.
    pub agg_proof_mode: SP1ProofMode,
    /// Cycle limit for range proofs.
    pub range_cycle_limit: u64,
    /// Gas limit for range proofs.
    pub range_gas_limit: u64,
    /// Cycle limit for aggregation proofs.
    pub agg_cycle_limit: u64,
    /// Gas limit for aggregation proofs.
    pub agg_gas_limit: u64,
    /// Maximum price per PGU.
    pub max_price_per_pgu: u64,
    /// Minimum auction period.
    pub min_auction_period: u64,
    /// Whitelist of allowed prover addresses.
    pub whitelist: Option<Vec<Address>>,
}

/// Container for proving and verifying keys.
#[derive(Clone)]
pub struct ProofKeys {
    pub range_pk: Arc<SP1ProvingKey>,
    pub range_vk: Arc<SP1VerifyingKey>,
    pub agg_pk: Arc<SP1ProvingKey>,
    pub agg_vk: Arc<SP1VerifyingKey>,
}

/// Proof provider abstraction for generating range and aggregation proofs.
///
/// This enum wraps the concrete provider implementations, allowing the proposer
/// to be agnostic about how proofs are generated (network vs mock).
#[derive(Clone)]
pub enum ProofProvider {
    /// Network-based proving via SP1 prover network.
    Network(NetworkProofProvider),
    /// Local mock execution (creates mock proofs, no real proving).
    Mock(MockProofProvider),
}

impl ProofProvider {
    /// Generate a range proof.
    ///
    /// In mock mode: executes locally and returns execution stats.
    /// In network mode: submits to network, waits for completion, returns (proof, 0, 0).
    ///
    /// Returns: (proof, instruction_cycles, sp1_gas)
    pub async fn generate_range_proof(
        &self,
        stdin: &SP1Stdin,
    ) -> Result<(SP1ProofWithPublicValues, u64, u64)> {
        match self {
            ProofProvider::Network(p) => p.generate_range_proof(stdin).await,
            ProofProvider::Mock(p) => p.generate_range_proof(stdin).await,
        }
    }

    /// Generate an aggregation proof.
    ///
    /// In mock mode: executes locally and creates mock proof.
    /// In network mode: submits to network, waits for completion.
    pub async fn generate_agg_proof(&self, stdin: &SP1Stdin) -> Result<SP1ProofWithPublicValues> {
        match self {
            ProofProvider::Network(p) => p.generate_agg_proof(stdin).await,
            ProofProvider::Mock(p) => p.generate_agg_proof(stdin).await,
        }
    }

    /// Access to proving keys.
    pub fn keys(&self) -> &ProofKeys {
        match self {
            ProofProvider::Network(p) => &p.keys,
            ProofProvider::Mock(p) => &p.keys,
        }
    }

    /// Access to configuration.
    pub fn config(&self) -> &ProofProviderConfig {
        match self {
            ProofProvider::Network(p) => &p.config,
            ProofProvider::Mock(p) => &p.config,
        }
    }
}

// =============================================================================
// Implementation: NetworkProofProvider
// =============================================================================

/// Network-based proof provider using SP1 prover network.
#[derive(Clone)]
pub struct NetworkProofProvider {
    prover: Arc<NetworkProver>,
    keys: ProofKeys,
    config: ProofProviderConfig,
    network_mode: NetworkMode,
}

impl NetworkProofProvider {
    pub fn new(
        prover: Arc<NetworkProver>,
        keys: ProofKeys,
        config: ProofProviderConfig,
        network_mode: NetworkMode,
    ) -> Self {
        Self { prover, keys, config, network_mode }
    }

    /// Get a reference to the underlying network prover.
    pub fn inner(&self) -> &NetworkProver {
        &self.prover
    }

    /// Generate a range proof via network.
    pub async fn generate_range_proof(
        &self,
        stdin: &SP1Stdin,
    ) -> Result<(SP1ProofWithPublicValues, u64, u64)> {
        tracing::info!("Generating range proof via network");
        let proof_id = self.request_range_proof(stdin).await?;
        let proof = self.wait_for_proof(proof_id).await?;
        Ok((proof, 0, 0))
    }

    /// Generate an aggregation proof via network.
    pub async fn generate_agg_proof(&self, stdin: &SP1Stdin) -> Result<SP1ProofWithPublicValues> {
        tracing::info!("Generating aggregation proof via network");
        let proof_id = self.request_agg_proof(stdin).await?;
        self.wait_for_proof(proof_id).await
    }

    /// Submit a range proof request to the network.
    async fn request_range_proof(&self, stdin: &SP1Stdin) -> Result<ProofId> {
        let proof_id = self
            .prover
            .prove(&self.keys.range_pk, stdin)
            .compressed()
            .skip_simulation(true)
            .strategy(self.config.range_proof_strategy)
            .timeout(Duration::from_secs(self.config.timeout))
            .min_auction_period(self.config.min_auction_period)
            .max_price_per_pgu(self.config.max_price_per_pgu)
            .cycle_limit(self.config.range_cycle_limit)
            .gas_limit(self.config.range_gas_limit)
            .whitelist(self.config.whitelist.clone())
            .request_async()
            .await?;

        tracing::info!(proof_id = %proof_id, "Range proof request submitted");
        Ok(proof_id)
    }

    /// Submit an aggregation proof request to the network.
    async fn request_agg_proof(&self, stdin: &SP1Stdin) -> Result<ProofId> {
        let proof_id = self
            .prover
            .prove(&self.keys.agg_pk, stdin)
            .mode(self.config.agg_proof_mode)
            .strategy(self.config.agg_proof_strategy)
            .timeout(Duration::from_secs(self.config.timeout))
            .min_auction_period(self.config.min_auction_period)
            .max_price_per_pgu(self.config.max_price_per_pgu)
            .cycle_limit(self.config.agg_cycle_limit)
            .gas_limit(self.config.agg_gas_limit)
            .whitelist(self.config.whitelist.clone())
            .request_async()
            .await?;

        tracing::info!(proof_id = %proof_id, "Aggregation proof request submitted");
        Ok(proof_id)
    }

    /// Execute a network call with timeout.
    async fn network_call_with_timeout<F, T>(
        &self,
        future: F,
        operation: &str,
        proof_id: ProofId,
    ) -> Result<T>
    where
        F: std::future::Future<Output = Result<T, anyhow::Error>>,
    {
        let timeout_secs = self.config.network_calls_timeout;
        match tokio::time::timeout(Duration::from_secs(timeout_secs), future).await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(e)) => {
                tracing::warn!(proof_id = %proof_id, operation, error = %e, "Network error");
                Err(e)
            }
            Err(_) => {
                tracing::warn!(proof_id = %proof_id, operation, timeout_secs, "Network call timed out");
                bail!("Timeout after {}s for {} (proof_id={})", timeout_secs, operation, proof_id)
            }
        }
    }

    /// Wait for a proof to be fulfilled by polling the network.
    ///
    /// Timeout behavior:
    /// - **Proving timeout** (`config.timeout`): Overall maximum wait time from start.
    /// - **Network call timeout** (`config.network_calls_timeout`): Per-call timeout; retries on
    ///   failure.
    /// - **Auction timeout** (`config.auction_timeout`): Cancels if no prover picks up the request.
    /// - **Server deadline** (`status.deadline()`): Server-side proving deadline.
    async fn wait_for_proof(&self, proof_id: ProofId) -> Result<SP1ProofWithPublicValues> {
        let start_time = std::time::Instant::now();
        let proving_timeout = Duration::from_secs(self.config.timeout);

        loop {
            // Proving timeout - ensures we don't wait forever if network calls keep failing.
            if start_time.elapsed() > proving_timeout {
                tracing::warn!(
                    proof_id = %proof_id,
                    elapsed_secs = start_time.elapsed().as_secs(),
                    timeout_secs = self.config.timeout,
                    "proving timeout exceeded"
                );
                bail!(
                    "Proof request {} client timeout after {}s",
                    proof_id,
                    start_time.elapsed().as_secs()
                );
            }

            // Get proof status - retry on transient failures.
            let (status, proof) = match self
                .network_call_with_timeout(
                    async { self.prover.get_proof_status(proof_id).await },
                    "get_proof_status",
                    proof_id,
                )
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    tracing::warn!(proof_id = %proof_id, error = %e, "get_proof_status failed, retrying...");
                    tokio::time::sleep(Duration::from_secs(PROOF_STATUS_POLL_INTERVAL)).await;
                    continue;
                }
            };

            // Get proof request details for auction timeout check - retry on transient failures.
            let request_details = match self
                .network_call_with_timeout(
                    async { self.prover.get_proof_request(proof_id).await },
                    "get_proof_request",
                    proof_id,
                )
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    tracing::warn!(proof_id = %proof_id, error = %e, "get_proof_request failed, retrying...");
                    tokio::time::sleep(Duration::from_secs(PROOF_STATUS_POLL_INTERVAL)).await;
                    continue;
                }
            };

            let current_time =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();

            // Check auction timeout: if request is still in "Requested" state past the deadline.
            // Only cancel on mainnet where auction dynamics are meaningful.
            if let Some(details) = &request_details {
                let auction_deadline = details.created_at + self.config.auction_timeout;
                if self.network_mode == NetworkMode::Mainnet &&
                    details.fulfillment_status == FulfillmentStatus::Requested as i32 &&
                    current_time > auction_deadline
                {
                    tracing::warn!(
                        proof_id = %proof_id,
                        created_at = details.created_at,
                        auction_deadline,
                        current_time,
                        "Auction timeout exceeded, cancelling request"
                    );
                    if let Err(e) = self
                        .network_call_with_timeout(
                            async { self.prover.cancel_request(proof_id).await },
                            "cancel_request",
                            proof_id,
                        )
                        .await
                    {
                        tracing::error!(proof_id = %proof_id, error = %e, "Failed to cancel proof request");
                    }
                    bail!(
                        "Proof request {} auction timeout (no prover picked up, created_at={}, deadline={})",
                        proof_id,
                        details.created_at,
                        auction_deadline
                    );
                }
            }

            // Check if the proof deadline has passed.
            if current_time > status.deadline() {
                tracing::warn!(
                    proof_id = %proof_id,
                    deadline = status.deadline(),
                    current_time,
                    "Proof request deadline exceeded"
                );
                bail!(
                    "Proof request {} deadline exceeded (deadline={}, current={})",
                    proof_id,
                    status.deadline(),
                    current_time
                );
            }

            // Check fulfillment status.
            match FulfillmentStatus::try_from(status.fulfillment_status()) {
                Ok(FulfillmentStatus::Fulfilled) => {
                    tracing::info!(proof_id = %proof_id, "Proof fulfilled");
                    return proof.ok_or_else(|| {
                        anyhow::anyhow!("Proof status is fulfilled but proof is None")
                    });
                }
                Ok(FulfillmentStatus::Unfulfillable) => {
                    bail!(
                        "Proof request {} is unfulfillable (execution_status: {:?})",
                        proof_id,
                        status.execution_status()
                    );
                }
                Ok(FulfillmentStatus::Assigned) => {
                    tracing::debug!(proof_id = %proof_id, "Proof assigned, proving...");
                }
                _ => {
                    tracing::debug!(proof_id = %proof_id, "Proof pending...");
                }
            }

            tokio::time::sleep(Duration::from_secs(PROOF_STATUS_POLL_INTERVAL)).await;
        }
    }
}

// =============================================================================
// Implementation: MockProofProvider
// =============================================================================

/// Mock proof provider for local execution without network.
#[derive(Clone)]
pub struct MockProofProvider {
    prover: Arc<NetworkProver>,
    keys: ProofKeys,
    config: ProofProviderConfig,
    agg_elf: &'static [u8],
}

impl MockProofProvider {
    pub fn new(
        prover: Arc<NetworkProver>,
        keys: ProofKeys,
        config: ProofProviderConfig,
        agg_elf: &'static [u8],
    ) -> Self {
        Self { prover, keys, config, agg_elf }
    }

    /// Generate a range proof in mock mode.
    pub async fn generate_range_proof(
        &self,
        stdin: &SP1Stdin,
    ) -> Result<(SP1ProofWithPublicValues, u64, u64)> {
        tracing::info!("Generating range proof in mock mode");

        let (public_values, report) = self
            .prover
            .execute(get_range_elf_embedded(), stdin)
            .calculate_gas(true)
            .deferred_proof_verification(false)
            .run()
            .context("Mock range proof execution failed")?;

        let total_instruction_cycles = report.total_instruction_count();
        let total_sp1_gas = report.gas.unwrap_or(0);

        tracing::info!(
            total_instruction_cycles = total_instruction_cycles,
            total_sp1_gas = total_sp1_gas,
            "Captured execution stats for range proof"
        );

        let proof = SP1ProofWithPublicValues::create_mock_proof(
            &self.keys.range_pk,
            public_values,
            SP1ProofMode::Compressed,
            SP1_CIRCUIT_VERSION,
        );

        Ok((proof, total_instruction_cycles, total_sp1_gas))
    }

    /// Generate an aggregation proof in mock mode.
    pub async fn generate_agg_proof(&self, stdin: &SP1Stdin) -> Result<SP1ProofWithPublicValues> {
        tracing::info!("Generating aggregation proof in mock mode");

        let (public_values, _) = self
            .prover
            .execute(self.agg_elf, stdin)
            .deferred_proof_verification(false)
            .run()
            .context("Mock aggregation proof execution failed")?;

        Ok(SP1ProofWithPublicValues::create_mock_proof(
            &self.keys.agg_pk,
            public_values,
            self.config.agg_proof_mode,
            SP1_CIRCUIT_VERSION,
        ))
    }
}
