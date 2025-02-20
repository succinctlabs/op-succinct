use alloy_eips::BlockId;
use alloy_primitives::{Address, B256, U256};
use alloy_provider::{network::ReceiptResponse, Network, Provider};
use anyhow::Result;
use chrono::Local;
use op_succinct_client_utils::{
    boot::{hash_rollup_config, BootInfoStruct},
    types::u32_to_u8,
};
use op_succinct_host_utils::{
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_agg_proof_stdin, get_proof_stdin, start_server_and_native_client, ProgramType,
};
use sp1_sdk::{
    network::{
        proto::network::{ExecutionStatus, FulfillmentStatus},
        FulfillmentStrategy,
    },
    HashableKey, NetworkProver, Prover, ProverClient, SP1Proof, SP1ProofMode,
    SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin, SP1VerifyingKey, SP1_CIRCUIT_VERSION,
};
use std::{
    env,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{
    db::{DriverDBClient, OPSuccinctRequest, RequestMode, RequestStatus, RequestType},
    get_latest_proposed_block_number, RequestExecutionStatistics, AGG_ELF, RANGE_ELF,
};

use op_succinct_host_utils::OPSuccinctL2OutputOracle::OPSuccinctL2OutputOracleInstance as OPSuccinctL2OOContract;

pub struct ContractConfig<P, N>
where
    P: Provider<N> + 'static,
    N: Network,
{
    pub l2oo_address: Address,
    pub dgf_address: Address,
    pub l2oo_contract: OPSuccinctL2OOContract<(), P, N>,
}

#[derive(Debug, Clone)]
pub struct CommitmentConfig {
    pub range_vkey_commitment: B256,
    pub agg_vkey_hash: B256,
    pub rollup_config_hash: B256,
}

pub struct ProgramConfig {
    pub range_vk: SP1VerifyingKey,
    pub range_pk: SP1ProvingKey,
    pub agg_vk: SP1VerifyingKey,
    pub agg_pk: SP1ProvingKey,
    pub commitments: CommitmentConfig,
}

pub struct RequesterConfig {
    pub l2oo_address: Address,
    pub dgf_address: Address,
    pub range_proof_interval: u64,
    pub max_concurrent_witness_gen: u64,
    pub max_concurrent_proof_requests: u64,
}

pub struct DriverConfig {
    // // TODO: Add prover key, etc.
    // // ****
    // // Proposer configuration
    // // ****
    pub network_prover: Arc<NetworkProver>,
    pub fetcher: Arc<OPSuccinctDataFetcher>,
    pub range_proof_strategy: FulfillmentStrategy,
    pub agg_proof_strategy: FulfillmentStrategy,
    pub agg_proof_mode: SP1ProofMode,
    pub range_proof_size: u64,
    pub submission_interval: u64,
    pub proof_timeout: u64,
    pub mock: bool,
    // // TODO: Make sure this works in both local and hosted mode.
    // // Note: There's no cached DB.
    pub driver_db_client: Arc<DriverDBClient>,
    /// Limits on the maximum number of concurrent proof requests and witness generation requests.
    pub max_concurrent_proof_requests: u64,
    pub max_concurrent_witness_gen: u64,
}

pub struct Proposer<P, N>
where
    P: Provider<N> + 'static,
    N: Network,
{
    driver_config: DriverConfig,
    contract_config: ContractConfig<P, N>,
    program_config: ProgramConfig,
    requester_config: RequesterConfig,
}

// 5 confirmations (1 minute)
const NUM_CONFIRMATIONS: u64 = 5;
// 2 minute timeout.
const TIMEOUT: u64 = 120;

impl<P, N> Proposer<P, N>
where
    P: Provider<N> + 'static,
    N: Network,
{
    pub async fn new(provider: P, db_client: Arc<DriverDBClient>) -> Result<Self> {
        let network_prover = Arc::new(ProverClient::builder().network().build());
        let (range_pk, range_vk) = network_prover.setup(RANGE_ELF);
        let (agg_pk, agg_vk) = network_prover.setup(AGG_ELF);
        let multi_block_vkey_u8 = u32_to_u8(range_vk.vk.hash_u32());
        let range_vkey_commitment = B256::from(multi_block_vkey_u8);
        let agg_vkey_hash = B256::from_str(&agg_vk.bytes32()).unwrap();

        // TODO: Fix this so we don't need to run in Docker.
        // TODO: It's really weird that we have a Docker/Dev mode.
        let fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;
        // Note: The rollup config hash never changes for a given chain, so we can just hash it once at
        // server start-up. The only time a rollup config changes is typically when a new version of the
        // [`RollupConfig`] is released from `op-alloy`.
        let rollup_config_hash = hash_rollup_config(fetcher.rollup_config.as_ref().unwrap());

        // Set the proof strategies based on environment variables. Default to reserved to keep existing behavior.
        let range_proof_strategy = match env::var("RANGE_PROOF_STRATEGY") {
            Ok(strategy) if strategy.to_lowercase() == "hosted" => FulfillmentStrategy::Hosted,
            _ => FulfillmentStrategy::Reserved,
        };
        let agg_proof_strategy = match env::var("AGG_PROOF_STRATEGY") {
            Ok(strategy) if strategy.to_lowercase() == "hosted" => FulfillmentStrategy::Hosted,
            _ => FulfillmentStrategy::Reserved,
        };

        // Set the aggregation proof type based on environment variable. Default to groth16.
        let agg_proof_mode = match env::var("AGG_PROOF_MODE") {
            Ok(proof_type) if proof_type.to_lowercase() == "plonk" => SP1ProofMode::Plonk,
            _ => SP1ProofMode::Groth16,
        };

        let l2oo_address: Address = env::var("L2OO_ADDRESS")
            .expect("L2OO_ADDRESS not set")
            .parse::<Address>()
            .expect("Invalid L2OO_ADDRESS");
        let dgf_address: Address = env::var("DISPUTE_GAME_FACTORY_ADDRESS")
            .expect("DISPUTE_GAME_FACTORY_ADDRESS not set")
            .parse::<Address>()
            .expect("Invalid DISPUTE_GAME_FACTORY_ADDRESS");
        let range_proof_interval = env::var("RANGE_PROOF_INTERVAL")
            .expect("RANGE_PROOF_INTERVAL not set")
            .parse::<u64>()
            .expect("Invalid RANGE_PROOF_INTERVAL");
        let max_concurrent_witness_gen = env::var("MAX_CONCURRENT_WITNESS_GEN")
            .expect("MAX_CONCURRENT_WITNESS_GEN not set")
            .parse::<u64>()
            .expect("Invalid MAX_CONCURRENT_WITNESS_GEN");
        let max_concurrent_proof_requests = env::var("MAX_CONCURRENT_PROOF_REQUESTS")
            .expect("MAX_CONCURRENT_PROOF_REQUESTS not set")
            .parse::<u64>()
            .expect("Invalid MAX_CONCURRENT_PROOF_REQUESTS");
        let mock = env::var("OP_SUCCINCT_MOCK")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);
        const PROOF_TIMEOUT: u64 = 60 * 60;

        let fetcher = Arc::new(fetcher);

        let l2oo_contract = OPSuccinctL2OOContract::new(l2oo_address, provider);

        let proposer = Proposer {
            driver_config: DriverConfig {
                network_prover,
                fetcher,
                range_proof_strategy,
                agg_proof_strategy,
                agg_proof_mode,
                range_proof_size: 0,
                submission_interval: 0,
                proof_timeout: PROOF_TIMEOUT,
                mock,
                driver_db_client: db_client,
                max_concurrent_proof_requests,
                max_concurrent_witness_gen,
            },
            contract_config: ContractConfig {
                l2oo_address,
                dgf_address,
                l2oo_contract,
            },
            program_config: ProgramConfig {
                range_vk,
                range_pk,
                agg_vk,
                agg_pk,
                commitments: CommitmentConfig {
                    range_vkey_commitment,
                    agg_vkey_hash,
                    rollup_config_hash,
                },
            },
            requester_config: RequesterConfig {
                l2oo_address,
                dgf_address,
                range_proof_interval,
                max_concurrent_witness_gen,
                max_concurrent_proof_requests,
            },
        };
        Ok(proposer)
    }

    /// Use the in-memory index of the highest block number to add new ranges to the database.
    pub async fn add_new_ranges(&self) -> Result<()> {
        let latest_finalized_header = self
            .driver_config
            .fetcher
            .get_l2_header(BlockId::finalized())
            .await?;

        // Get the highest block number of any of the requests in the database that are not FAILED or CANCELLED with the same commitment.
        let highest_request = self
            .driver_config
            .driver_db_client
            .fetch_highest_request_with_statuses_and_commitment(
                &[
                    RequestStatus::Unrequested,
                    RequestStatus::WitnessGeneration,
                    RequestStatus::Execution,
                    RequestStatus::Relayed,
                    RequestStatus::Complete,
                    RequestStatus::Prove,
                ],
                &self.program_config.commitments,
            )
            .await?;

        // If there are no requests in the database, the current processed block number is the latest finalized block number on the contract. Otherwise, it's the highest block number
        // of any of the requests in the database that are not FAILED or CANCELLED.
        let mut current_processed_block = match highest_request {
            Some(request) => request.end_block as u64,
            None => {
                log::info!(
                    "No requests in the database, using latest proposed block number on contract."
                );
                get_latest_proposed_block_number(
                    self.contract_config.l2oo_address,
                    self.driver_config.fetcher.as_ref(),
                )
                .await?
            }
        };

        let mut requests = Vec::new();

        // Only add new ranges if the current block is less than the latest finalized block minus the
        // range proof interval. This ensures that only ranges that cover range_proof_interval blocks
        // are added to the database.
        while current_processed_block
            < latest_finalized_header.number - self.requester_config.range_proof_interval
        {
            let end_block = std::cmp::min(
                current_processed_block + self.requester_config.range_proof_interval,
                latest_finalized_header.number,
            );

            let request = OPSuccinctRequest {
                status: RequestStatus::Unrequested,
                req_type: RequestType::Range,
                mode: if self.driver_config.mock {
                    RequestMode::Mock
                } else {
                    RequestMode::Real
                },
                start_block: current_processed_block as i64,
                end_block: end_block as i64,
                range_vkey_commitment: self.program_config.commitments.range_vkey_commitment.into(),
                rollup_config_hash: self.program_config.commitments.rollup_config_hash.into(),
                ..Default::default()
            };

            requests.push(request);
            current_processed_block = end_block;
        }

        if !requests.is_empty() {
            self.driver_config
                .driver_db_client
                .insert_requests(&requests)
                .await?;
        }

        Ok(())
    }

    /// Handle all proof requests in the PROVE state.
    pub async fn handle_proving_requests(&self) -> Result<()> {
        // Get all requests from the database.
        let prove_requests = self
            .driver_config
            .driver_db_client
            .fetch_requests_by_status(RequestStatus::Prove, &self.program_config.commitments)
            .await?;

        // Get the proof status of all of the requests in parallel.
        // TODO: Do this in parallel.
        for request in prove_requests {
            self.process_proof_request_status(request).await?;
        }

        Ok(())
    }

    /// Process a single OP Succinct request's proof status.
    pub async fn process_proof_request_status(&self, request: OPSuccinctRequest) -> Result<()> {
        if let Some(proof_request_id) = request.proof_request_id {
            let (status, proof) = self
                .driver_config
                .network_prover
                .get_proof_status(B256::from_slice(&proof_request_id))
                .await?;

            let execution_status = ExecutionStatus::try_from(status.execution_status).unwrap();
            let fulfillment_status =
                FulfillmentStatus::try_from(status.fulfillment_status).unwrap();

            // If the proof request has been fulfilled, update the request to status COMPLETE and add the proof bytes to the database.
            if fulfillment_status == FulfillmentStatus::Fulfilled {
                let proof: SP1ProofWithPublicValues = proof.unwrap();

                let proof_bytes = match proof.proof {
                    // If it's a compressed proof, serialize with bincode.
                    SP1Proof::Compressed(_) => bincode::serialize(&proof).unwrap(),
                    // If it's Groth16 or PLONK, get the on-chain proof bytes.
                    _ => proof.bytes(),
                };

                // Add the completed proof to the database.
                self.driver_config
                    .driver_db_client
                    .update_proof_to_complete(request.id, &proof_bytes)
                    .await?;
            } else if status.fulfillment_status == FulfillmentStatus::Unfulfillable as i32 {
                self.handle_unfulfillable_request(request, execution_status)
                    .await?;
            }
        } else {
            // TODO: If there is no proof request id, this should be a hard error. There should
            // never be a proof request in PROVE status without a proof request id.
            log::warn!("Request has no proof request id: {:?}", request);
        }

        Ok(())
    }

    /// Handle an unfulfillable proof request.
    pub async fn handle_unfulfillable_request(
        &self,
        request: OPSuccinctRequest,
        execution_status: ExecutionStatus,
    ) -> Result<()> {
        // If the proof request is unfulfillable, check if the request is a range proof and if the execution status is unexecutable.
        // If so, split the request into two requests.
        // Otherwise, retry the same request.
        log::info!("Request is unfulfillable: {:?}", request);

        // Set the existing request to status Failed.
        self.driver_config
            .driver_db_client
            .update_request_status(request.id, RequestStatus::Failed)
            .await?;

        if request.end_block - request.start_block > 1 && request.req_type == RequestType::Range {
            let failed_requests = self
                .driver_config
                .driver_db_client
                .fetch_failed_requests_by_block_range(
                    request.start_block,
                    request.end_block,
                    &self.program_config.commitments,
                )
                .await?;

            if failed_requests.len() > 1 || execution_status == ExecutionStatus::Unexecutable {
                log::info!("Splitting request into two: {:?}", request);
                // Add the two new requests to the database.
                let new_requests = vec![
                    OPSuccinctRequest {
                        status: RequestStatus::Unrequested,
                        req_type: RequestType::Range,
                        mode: request.mode,
                        start_block: request.start_block,
                        end_block: (request.start_block + request.end_block) / 2,
                        range_vkey_commitment: self
                            .program_config
                            .commitments
                            .range_vkey_commitment
                            .into(),
                        rollup_config_hash: self
                            .program_config
                            .commitments
                            .rollup_config_hash
                            .into(),
                        ..Default::default()
                    },
                    OPSuccinctRequest {
                        status: RequestStatus::Unrequested,
                        req_type: RequestType::Range,
                        mode: request.mode,
                        start_block: (request.start_block + request.end_block) / 2,
                        end_block: request.end_block,
                        range_vkey_commitment: self
                            .program_config
                            .commitments
                            .range_vkey_commitment
                            .into(),
                        rollup_config_hash: self
                            .program_config
                            .commitments
                            .rollup_config_hash
                            .into(),
                        ..Default::default()
                    },
                ];

                self.driver_config
                    .driver_db_client
                    .insert_requests(&new_requests)
                    .await?;

                // Return early, as the request has been split into two new requests.
                return Ok(());
            }
        }

        // If the logic to split into two requests has not been triggered, retry the same request.
        self.driver_config
            .driver_db_client
            .insert_request(&OPSuccinctRequest {
                status: RequestStatus::Unrequested,
                req_type: request.req_type,
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local(),
                mode: request.mode,
                start_block: request.start_block,
                end_block: request.end_block,
                range_vkey_commitment: request.range_vkey_commitment,
                aggregation_vkey_hash: request.aggregation_vkey_hash,
                checkpointed_l1_block_hash: request.checkpointed_l1_block_hash,
                checkpointed_l1_block_number: request.checkpointed_l1_block_number,
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    /// Create aggregation proofs based on the completed range proofs. The range proofs must be contiguous and have
    /// the same range vkey commitment. Assumes that the range proof retry logic guarantees that there is not
    /// two potential contiguous chains of range proofs.
    ///
    /// Only creates an AGG proof if there's not an AGG proof in progress with the same start block.
    pub async fn create_aggregation_proofs(&self) -> Result<()> {
        // Check if there's an AGG proof with the same start block AND range verification key commitment AND aggregation vkey.
        // If so, return.
        let latest_proposed_block_number = get_latest_proposed_block_number(
            self.contract_config.l2oo_address,
            self.driver_config.fetcher.as_ref(),
        )
        .await?;

        // Get all active AGG proofs with the same start block, range vkey commitment, and aggregation vkey.
        let agg_proofs = self
            .driver_config
            .driver_db_client
            .fetch_active_agg_proofs(
                latest_proposed_block_number as i64,
                &self.program_config.commitments,
            )
            .await?;

        if agg_proofs.len() > 0 {
            log::info!("There is already an AGG proof queued with the same start block, range vkey commitment, and aggregation vkey.");
            return Ok(());
        }

        // Get the latest proposed block number on the contract.
        // TODO: This could also be the latest relayed block number.
        let latest_proposed_block_number = get_latest_proposed_block_number(
            self.contract_config.l2oo_address,
            self.driver_config.fetcher.as_ref(),
        )
        .await?;

        // Get all completed range proofs from the database.
        let completed_range_proofs = self
            .driver_config
            .driver_db_client
            .fetch_completed_range_proofs(
                &self.program_config.commitments,
                latest_proposed_block_number as i64,
            )
            .await?;

        // Get the largest contiguous range of completed range proofs.
        let largest_contiguous_range = self.get_largest_contiguous_range(completed_range_proofs)?;

        if let Some(last_request) = largest_contiguous_range.last() {
            if (last_request.end_block - last_request.start_block) as u64
                >= self.requester_config.range_proof_interval
            {
                // Checkpoint an L1 block hash that will be used to create the aggregation proof.
                let latest_header = self
                    .driver_config
                    .fetcher
                    .get_l1_header(BlockId::latest())
                    .await?;

                // Checkpoint the L1 block hash.
                let receipt = self
                    .contract_config
                    .l2oo_contract
                    .checkpointBlockHash(U256::from(latest_header.number))
                    .send()
                    .await?
                    .with_required_confirmations(NUM_CONFIRMATIONS)
                    .with_timeout(Some(Duration::from_secs(TIMEOUT)))
                    .get_receipt()
                    .await?;

                // If transaction reverted, log the error.
                if !receipt.status() {
                    log::error!("Transaction reverted: {:?}", receipt);
                }

                // Create an aggregation proof request to cover the range with the checkpointed L1 block hash.
                self.driver_config
                    .driver_db_client
                    .insert_request(&OPSuccinctRequest {
                        status: RequestStatus::Unrequested,
                        req_type: RequestType::Aggregation,
                        created_at: Local::now().naive_local(),
                        updated_at: Local::now().naive_local(),
                        mode: last_request.mode,
                        start_block: latest_proposed_block_number as i64,
                        end_block: last_request.end_block,
                        range_vkey_commitment: self
                            .program_config
                            .commitments
                            .range_vkey_commitment
                            .into(),
                        rollup_config_hash: self
                            .program_config
                            .commitments
                            .rollup_config_hash
                            .into(),
                        aggregation_vkey_hash: Some(
                            self.program_config.commitments.agg_vkey_hash.into(),
                        ),
                        checkpointed_l1_block_hash: Some(latest_header.hash_slow().into()),
                        checkpointed_l1_block_number: Some(latest_header.number as i64),
                        ..Default::default()
                    })
                    .await?;
            }
        }

        Ok(())
    }

    /// Request all unrequested proofs up to MAX_CONCURRENT_PROOF_REQUESTS. Also, be aware of MAX_CONCURRENT_WITNESS_GEN.
    ///
    /// TODO: Submit up to MAX_CONCURRENT_PROOF_REQUESTS at a time. Don't do one per loop.
    ///
    /// TODO: Limit the maximum number of WITNESSGEN threads to MAX_CONCURRENT_WITNESS_GEN.
    async fn request_queued_proofs(&self) -> Result<()> {
        let requests = self
            .driver_config
            .driver_db_client
            .fetch_requests_by_statuses(
                &[
                    RequestStatus::WitnessGeneration,
                    RequestStatus::Execution,
                    RequestStatus::Prove,
                ],
                &self.program_config.commitments,
            )
            .await?;

        // If there are already MAX_CONCURRENT_PROOF_REQUESTS proofs in WITNESSGEN, EXECUTE, and PROVE status, return.
        if requests.len() >= self.driver_config.max_concurrent_proof_requests as usize {
            return Ok(());
        }

        // Get the next proof to request.
        let next_request = self.get_next_unrequested_proof().await?;

        if let Some(request) = next_request {
            // For mock and range proofs, set the status to WITNESSGEN.
            self.driver_config
                .driver_db_client
                .update_request_status(request.id, RequestStatus::WitnessGeneration)
                .await?;

            // Get the stdin for the proof.
            // TODO: Turn this into a separate stage. NOTE: Never store the stdin in the DB, as it's quite large.
            let stdin = match request.req_type {
                RequestType::Range => self.range_proof_witnessgen(&request).await?,
                RequestType::Aggregation => {
                    // Query the database for the consecutive range proofs.
                    let range_proofs = self
                        .driver_config
                        .driver_db_client
                        .get_consecutive_range_proofs(
                            request.start_block,
                            request.end_block,
                            &self.program_config.commitments,
                        )
                        .await?;

                    // Parse the range proofs into SP1ProofWithPublicValues.
                    let range_proofs: Vec<SP1ProofWithPublicValues> = range_proofs
                        .iter()
                        .map(|proof| bincode::deserialize(proof.proof.as_ref().unwrap()).unwrap())
                        .collect();

                    // Generate the witness for the aggregation proof.
                    self.agg_proof_witnessgen(&request, range_proofs).await?
                }
            };

            // If mock mode, set status to EXECUTE. For real mode, only set to PROVE after the request is sent.
            if self.driver_config.mock {
                // Set the status to EXECUTE.
                self.driver_config
                    .driver_db_client
                    .update_request_status(request.id, RequestStatus::Execution)
                    .await?;
            }

            if request.req_type == RequestType::Range {
                if self.driver_config.mock {
                    let proof = self.request_mock_range_proof(&request, stdin).await?;

                    // For mock range proofs, the proof bytes are the bincode serialized proof.
                    let proof_bytes = bincode::serialize(&proof).unwrap();
                    self.driver_config
                        .driver_db_client
                        .update_proof_to_complete(request.id, &proof_bytes)
                        .await?;
                } else {
                    // Request the range proof.
                    let proof_id = self.request_range_proof(stdin).await?;

                    // Update the request with the proof ID.
                    self.driver_config
                        .driver_db_client
                        .update_request_to_prove(request.id, proof_id.into())
                        .await?;
                }
            } else if request.req_type == RequestType::Aggregation {
                if self.driver_config.mock {
                    // Generate the mock aggregation proof.
                    let proof = self.request_mock_agg_proof(stdin).await?;

                    // For mock aggregation proofs, the proof bytes are on-chain encoded proof bytes.
                    self.driver_config
                        .driver_db_client
                        .update_proof_to_complete(request.id, &proof.bytes())
                        .await?;
                } else {
                    // Request the aggregation proof.
                    let proof_id = self.request_agg_proof(stdin).await?;

                    // Update the request with the proof ID.
                    self.driver_config
                        .driver_db_client
                        .update_request_to_prove(request.id, proof_id.into())
                        .await?;
                }
            }
        }
        Ok(())
    }

    /// Get the next unrequested proof from the database.
    ///
    /// If there is an AGG proof with the same start block, range vkey commitment, and aggregation vkey, return that.
    /// Otherwise, return a range proof with the lowest start block.
    async fn get_next_unrequested_proof(&self) -> Result<Option<OPSuccinctRequest>> {
        let latest_proposed_block_number = get_latest_proposed_block_number(
            self.contract_config.l2oo_address,
            self.driver_config.fetcher.as_ref(),
        )
        .await?;

        let unreq_agg_request = self
            .driver_config
            .driver_db_client
            .fetch_unrequested_agg_proof(
                latest_proposed_block_number as i64,
                &self.program_config.commitments,
            )
            .await?;

        if let Some(unreq_agg_request) = unreq_agg_request {
            return Ok(Some(unreq_agg_request));
        }

        let unreq_range_request = self
            .driver_config
            .driver_db_client
            .fetch_unrequested_range_proofs(
                latest_proposed_block_number as i64,
                &self.program_config.commitments,
            )
            .await?;

        if let Some(unreq_range_request) = unreq_range_request {
            return Ok(Some(unreq_range_request));
        }

        Ok(None)
    }

    /// Request a range proof.
    ///
    /// Returns the proof ID.
    async fn request_range_proof(&self, stdin: SP1Stdin) -> Result<B256> {
        self.driver_config
            .network_prover
            .prove(&self.program_config.range_pk, &stdin)
            .compressed()
            .strategy(self.driver_config.range_proof_strategy)
            .skip_simulation(true)
            .cycle_limit(1_000_000_000_000)
            .request_async()
            .await
    }

    /// Request an aggregation proof.
    ///
    /// Returns the proof ID.
    async fn request_agg_proof(&self, stdin: SP1Stdin) -> Result<B256> {
        self.driver_config
            .network_prover
            .prove(&self.program_config.agg_pk, &stdin)
            .mode(self.driver_config.agg_proof_mode)
            .strategy(self.driver_config.agg_proof_strategy)
            .request_async()
            .await
    }

    /// Generate a mock range proof.
    ///
    /// Writes the execution statistics to the database.
    async fn request_mock_range_proof(
        &self,
        request: &OPSuccinctRequest,
        stdin: SP1Stdin,
    ) -> Result<SP1ProofWithPublicValues> {
        let start_time = Instant::now();
        let (pv, report) = self
            .driver_config
            .network_prover
            .execute(RANGE_ELF, &stdin)
            .run()?;
        let execution_duration = start_time.elapsed().as_secs();

        let execution_statistics = RequestExecutionStatistics::new(report);

        // Write the execution data to the database.
        self.driver_config
            .driver_db_client
            .insert_execution_statistics(
                request.id,
                serde_json::to_value(execution_statistics)?,
                execution_duration as i64,
            )
            .await?;

        Ok(SP1ProofWithPublicValues::create_mock_proof(
            &self.program_config.range_pk,
            pv.clone(),
            SP1ProofMode::Compressed,
            SP1_CIRCUIT_VERSION,
        ))
    }

    /// Generate a mock aggregation proof.
    ///
    /// TODO: Add execution statistics.
    async fn request_mock_agg_proof(&self, stdin: SP1Stdin) -> Result<SP1ProofWithPublicValues> {
        // Note(ratan): In a future version of the server which only supports mock proofs, Arc<MockProver> should be used to reduce memory usage.
        let prover = ProverClient::builder().mock().build();

        // TODO: In the future, add statistics for aggregation proof execution.
        prover
            .prove(&self.program_config.range_pk, &stdin)
            .mode(self.driver_config.agg_proof_mode)
            .deferred_proof_verification(false)
            .run()
    }

    /// Generate the witness for a range proof.
    async fn range_proof_witnessgen(&self, request: &OPSuccinctRequest) -> Result<SP1Stdin> {
        let host_args = match self
            .driver_config
            .fetcher
            .get_host_args(
                request.start_block as u64,
                request.end_block as u64,
                ProgramType::Multi,
                CacheMode::DeleteCache,
            )
            .await
        {
            Ok(cli) => cli,
            Err(e) => {
                log::error!("Failed to get host CLI args: {}", e);
                return Err(anyhow::anyhow!("Failed to get host CLI args: {}", e));
            }
        };

        let mem_kv_store = start_server_and_native_client(host_args).await?;

        let sp1_stdin = match get_proof_stdin(mem_kv_store) {
            Ok(stdin) => stdin,
            Err(e) => {
                log::error!("Failed to get proof stdin: {}", e);
                return Err(anyhow::anyhow!("Failed to get proof stdin: {}", e));
            }
        };

        Ok(sp1_stdin)
    }

    /// Generate the witness for an aggregation proof.
    async fn agg_proof_witnessgen(
        &self,
        request: &OPSuccinctRequest,
        mut range_proofs: Vec<SP1ProofWithPublicValues>,
    ) -> Result<SP1Stdin> {
        let boot_infos: Vec<BootInfoStruct> = range_proofs
            .iter_mut()
            .map(|proof| proof.public_values.read())
            .collect();

        let proofs: Vec<SP1Proof> = range_proofs
            .iter_mut()
            .map(|proof| proof.proof.clone())
            .collect();

        let l1_head = request
            .checkpointed_l1_block_hash
            .expect("Aggregation proof has no checkpointed block.");

        let headers = self
            .driver_config
            .fetcher
            .get_header_preimages(&boot_infos, l1_head.into())
            .await?;

        let stdin = get_agg_proof_stdin(
            proofs,
            boot_infos,
            headers,
            &self.program_config.range_vk,
            l1_head.into(),
        )?;

        Ok(stdin)
    }

    /// Get the largest contiguous range of completed range proofs.
    fn get_largest_contiguous_range(
        &self,
        completed_range_proofs: Vec<OPSuccinctRequest>,
    ) -> Result<Vec<OPSuccinctRequest>> {
        let mut largest_contiguous_range: Vec<OPSuccinctRequest> = Vec::new();

        for proof in completed_range_proofs {
            if largest_contiguous_range.is_empty() {
                largest_contiguous_range.push(proof);
            } else if proof.start_block == largest_contiguous_range.last().unwrap().end_block {
                largest_contiguous_range.push(proof);
            } else {
                break;
            }
        }

        Ok(largest_contiguous_range)
    }

    /// Submit all completed aggregation proofs to the prover network.
    async fn submit_agg_proofs(&self) -> Result<()> {
        let latest_proposed_block_number = get_latest_proposed_block_number(
            self.contract_config.l2oo_address,
            self.driver_config.fetcher.as_ref(),
        )
        .await?;

        // See if there is an aggregation proof that is complete for this start block. NOTE: There should only be one "pending" aggregation proof at a time for a specific start block.
        let completed_agg_proof = self
            .driver_config
            .driver_db_client
            .fetch_completed_aggregation_proofs(
                latest_proposed_block_number as i64,
                &self.program_config.commitments,
            )
            .await?;

        // If there are no completed aggregation proofs, do nothing.
        let completed_agg_proof = match completed_agg_proof {
            Some(proof) => proof,
            None => return Ok(()),
        };

        // Get the output at the end block of the last completed aggregation proof.
        let output = self
            .driver_config
            .fetcher
            .get_l2_output_at_block(completed_agg_proof.end_block as u64)
            .await?;

        // Propose the L2 output.
        let receipt = self
            .contract_config
            .l2oo_contract
            .proposeL2Output(
                output.output_root,
                U256::from(completed_agg_proof.end_block),
                U256::from(completed_agg_proof.checkpointed_l1_block_number.unwrap()),
                completed_agg_proof.proof.unwrap().into(),
            )
            .send()
            .await?
            .with_required_confirmations(NUM_CONFIRMATIONS)
            .with_timeout(Some(Duration::from_secs(TIMEOUT)))
            .get_receipt()
            .await?;

        // If the transaction reverted, log the error.
        if !receipt.status() {
            log::error!("Transaction reverted: {:?}", receipt);
        }

        // Update the request to status RELAYED.
        self.driver_config
            .driver_db_client
            .update_request_to_relayed(completed_agg_proof.id, receipt.transaction_hash().into())
            .await?;

        Ok(())
    }

    /// Update the DB state if the proposer is being re-started.
    ///
    /// 1. If any of range vkey, agg vkey or rollup config hash has changed, set all proofs that are not RELAYED or COMPLETED to CANCELLED.
    /// 2. If all the same, keep all proofs in UNREQUESTED and PROVE proofs. Retry all WITNESSGEN and EXECUTION proofs.
    async fn initialize_proposer(&self) -> Result<()> {
        // Get highest request with one of the given statuses.
        let request = self
            .driver_config
            .driver_db_client
            .fetch_highest_request_with_statuses(&[
                RequestStatus::Unrequested,
                RequestStatus::Prove,
                RequestStatus::Execution,
                RequestStatus::WitnessGeneration,
            ])
            .await?;

        // Cancel all ongoing requests if the rollup config hash or range vkey commitment has changed.
        // Only one set of pending requests with a given (rollup_config_hash, range_vkey_commitment, agg_vkey_hash)
        // can exist at a time.
        if let Some(request) = request {
            let config_changed = request.rollup_config_hash
                != self.program_config.commitments.rollup_config_hash
                || request.range_vkey_commitment
                    != self.program_config.commitments.range_vkey_commitment;

            let agg_key_changed = request
                .aggregation_vkey_hash
                .map(|hash| hash != self.program_config.commitments.agg_vkey_hash)
                .unwrap_or(false);

            if config_changed || agg_key_changed {
                // Cancel all old requests.
                self.driver_config
                    .driver_db_client
                    .cancel_all_requests_with_statuses(&[
                        RequestStatus::Unrequested,
                        RequestStatus::Prove,
                        RequestStatus::Execution,
                        RequestStatus::WitnessGeneration,
                    ])
                    .await?;

                // The range proof creation will view the latest state of the DB and the highest proposed block number to retrieve the correct latest block number.
                // This is safe to do because the proposer state is only updated when the proposer is re-started.
                return Ok(());
            }
        }

        // Fetch all requests in status Execution and WitnessGeneration for re-trying.
        let requests = self
            .driver_config
            .driver_db_client
            .fetch_requests_by_statuses(
                &[RequestStatus::Execution, RequestStatus::WitnessGeneration],
                &self.program_config.commitments,
            )
            .await?;

        // Retry all requests with status Execution and WitnessGeneration.
        for request in requests {
            // Retry the request by using the unfulfillable request handler.
            // NOTE: There is only special logic with Unexecutable requests, so any other status will be handled by retrying the same range.
            self.handle_unfulfillable_request(request, ExecutionStatus::UnspecifiedExecutionStatus)
                .await?;
        }

        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        // Handle the case where the proposer is being re-started and the proposer state needs to be updated.
        self.initialize_proposer().await?;

        // Main proposer loop.
        const PROPOSER_LOOP_INTERVAL: u64 = 60;
        loop {
            // Add new ranges to the database.
            self.add_new_ranges().await?;

            // Get all proof statuses of all requests in the proving state.
            self.handle_proving_requests().await?;

            // Get all proof statuses of all requests in the witness generation state.
            // TODO: Decide whether to implement this. Currently, all it does in go proposer is a timeout check.

            // Create aggregation proofs based on the completed range proofs. Checkpoints the block hash associated with the aggregation proof
            // in advance.
            self.create_aggregation_proofs().await?;

            // Request all unrequested proofs from the prover network.
            self.request_queued_proofs().await?;

            // Determine if any aggregation proofs that are complete need to be checkpointed.
            self.submit_agg_proofs().await?;

            // Sleep for the proposer loop interval.
            tokio::time::sleep(Duration::from_secs(PROPOSER_LOOP_INTERVAL)).await;
        }
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
