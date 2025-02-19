use alloy_eips::BlockId;
use alloy_primitives::{Address, B256};
use anyhow::Result;
use op_succinct_client_utils::{boot::hash_rollup_config, types::u32_to_u8};
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RunContext};
use sp1_sdk::{
    network::{
        proto::network::{ExecutionStatus, FulfillmentStatus},
        FulfillmentStrategy,
    },
    HashableKey, NetworkProver, Prover, ProverClient, SP1Proof, SP1ProofMode,
    SP1ProofWithPublicValues, SP1ProvingKey, SP1VerifyingKey,
};
use std::{env, str::FromStr, sync::Arc};

use crate::{
    db::{DriverDBClient, OPSuccinctRequest, RequestMode, RequestStatus, RequestType},
    get_latest_proposed_block_number, AGG_ELF, RANGE_ELF,
};

pub struct ContractConfig {
    pub l2oo_address: Address,
    pub dgf_address: Address,
}

pub struct ProgramConfig {
    pub range_vk: SP1VerifyingKey,
    pub range_pk: SP1ProvingKey,
    pub agg_vk: SP1VerifyingKey,
    pub agg_pk: SP1ProvingKey,
    pub agg_vkey_hash: B256,
    pub range_vkey_commitment: B256,
    pub rollup_config_hash: B256,
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
    /// The current in-memory index of the highest block number in the database.
    pub current_processed_block_number: u64,
    /// Limits on the maximum number of concurrent proof requests and witness generation requests.
    pub max_concurrent_proof_requests: u64,
    pub max_concurrent_witness_gen: u64,
}

pub struct Proposer {
    driver_config: DriverConfig,
    contract_config: ContractConfig,
    program_config: ProgramConfig,
    requester_config: RequesterConfig,
}

impl Proposer {
    pub async fn new() -> Result<Self> {
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
        let mock = env::var("MOCK")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);
        let db_url = env::var("DB_URL").expect("DB_URL not set");
        const PROOF_TIMEOUT: u64 = 60 * 60;

        let driver_db_client = Arc::new(DriverDBClient::new(&db_url).await?);

        // The latest proposed block number is the highest block number that has been proposed.
        let latest_block_number = get_latest_proposed_block_number(l2oo_address, fetcher).await?;

        let proposer = Proposer {
            driver_config: DriverConfig {
                network_prover,
                range_proof_strategy,
                agg_proof_strategy,
                agg_proof_mode,
                range_proof_size: 0,
                submission_interval: 0,
                proof_timeout: PROOF_TIMEOUT,
                mock,
                driver_db_client,
                current_processed_block_number: latest_block_number,
                max_concurrent_proof_requests,
                max_concurrent_witness_gen,
            },
            contract_config: ContractConfig {
                l2oo_address,
                dgf_address,
            },
            program_config: ProgramConfig {
                range_vk,
                range_pk,
                agg_vk,
                agg_pk,
                agg_vkey_hash,
                range_vkey_commitment,
                rollup_config_hash,
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
        let fetcher = OPSuccinctDataFetcher::default();
        let latest_finalized_header = fetcher.get_l2_header(BlockId::finalized()).await?;

        let mut current_block = self.driver_config.current_processed_block_number;
        let mut requests = Vec::new();

        // Only add new ranges if the current block is less than the latest finalized block minus the
        // range proof interval. This ensures that only ranges that cover range_proof_interval blocks
        // are added to the database.
        while current_block
            < latest_finalized_header.number - self.requester_config.range_proof_interval
        {
            let end_block = std::cmp::min(
                current_block + self.requester_config.range_proof_interval,
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
                start_block: current_block as i64,
                end_block: end_block as i64,
                range_vkey_commitment: self.program_config.range_vkey_commitment.into(),
                ..Default::default()
            };

            requests.push(request);
            current_block = end_block;
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
            .fetch_requests_by_status(RequestStatus::Prove)
            .await?;

        // Get the proof status of all of the requests in parallel.
        // TODO: Make this in parallel.
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
                    .update_completed_proof(request.id, &proof_bytes)
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

        // Set the existing request to status FAILED_RETRYABLE.
        self.driver_config
            .driver_db_client
            .update_request_status(
                request.start_block,
                request.end_block,
                RequestStatus::FailedRetryable,
            )
            .await?;

        if request.end_block - request.start_block > 1 && request.req_type == RequestType::Range {
            let failed_requests = self
                .driver_config
                .driver_db_client
                .fetch_failed_requests_by_block_range(
                    request.start_block,
                    request.end_block,
                    request.range_vkey_commitment,
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
                        range_vkey_commitment: request.range_vkey_commitment,
                        ..Default::default()
                    },
                    OPSuccinctRequest {
                        status: RequestStatus::Unrequested,
                        req_type: RequestType::Range,
                        mode: request.mode,
                        start_block: (request.start_block + request.end_block) / 2,
                        end_block: request.end_block,
                        range_vkey_commitment: request.range_vkey_commitment,
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
                req_type: RequestType::Range,
                mode: request.mode,
                start_block: request.start_block,
                end_block: request.end_block,
                range_vkey_commitment: request.range_vkey_commitment,
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    /// Create aggregation proofs based on the completed range proofs. The range proofs must be contiguous and have
    /// the same range vkey commitment. Assumes that the range proof retry logic guarantees that there is not
    /// two potential contiguous chains of range proofs.
    pub async fn create_aggregation_proofs(&self) -> Result<()> {
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        // Add new ranges to the database.
        self.add_new_ranges().await?;

        // Get all proof statuses of all requests in the proving state.
        self.handle_proving_requests().await?;

        // Get all proof statuses of all requests in the witness generation state.
        // TODO: Decide whether to implement this. Currently, all it does is a timeout check.

        // Create aggregation proofs based on the completed range proofs.

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
