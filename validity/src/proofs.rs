use alloy_primitives::hex::FromHex;
use alloy_primitives::FixedBytes;
use bincode::Options;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::proof_requester::OPSuccinctProofRequester;
use crate::OPSuccinctRequest;
use crate::ProgramConfig;
use crate::RequestMode;
use crate::RequesterConfig;
use crate::ValidityGauge;
use grpc::proofs::proofs_server::Proofs;
use grpc::proofs::{AggProofRequest, AggProofResponse};
use op_succinct_host_utils::hosts::OPSuccinctHost;
use op_succinct_host_utils::metrics::MetricsGauge;
use std::{sync::Arc, time::Instant};

pub struct Service<H>
where
    H: OPSuccinctHost,
{
    proof_requester: Arc<OPSuccinctProofRequester<H>>,
    program_config: ProgramConfig,
    requester_config: RequesterConfig,
}

impl<H> Service<H>
where
    H: OPSuccinctHost,
{
    pub fn new(
        proof_requester: Arc<OPSuccinctProofRequester<H>>,
        program_config: ProgramConfig,
        requester_config: RequesterConfig,
    ) -> Self {
        Self {
            proof_requester,
            program_config,
            requester_config,
        }
    }

    // Limit the L1 block number to the safe head if it is greater than the requested end block
    async fn limit_l1_block_number(
        &self,
        requested_end_block: u64,
        l1_block_number: u64,
    ) -> Result<u64, Status> {
        let safe_head = self
            .proof_requester
            .fetcher
            .get_l2_safe_head_from_l1_block_number(l1_block_number - 20)
            .await
            .expect("Failed to get safe head");

        if safe_head < requested_end_block {
            return Ok(safe_head);
        }

        Ok(requested_end_block)
    }
}

#[tonic::async_trait]
impl<H> Proofs for Service<H>
// Update trait implementation
where
    H: OPSuccinctHost,
{
    #[tracing::instrument(name = "proofs.request_agg_proof", skip(self, request))]
    async fn request_agg_proof(
        // Update method name
        &self,
        request: Request<AggProofRequest>, // Update request type
    ) -> Result<Response<AggProofResponse>, Status> {
        // Update response type
        info!("Received AggProofRequest: {:?}", request);

        let req = request.into_inner();

        let l1_limited_end_block = self
            .limit_l1_block_number(req.requested_end_block, req.l1_block_number)
            .await
            .map_err(|e| {
                ValidityGauge::WitnessgenErrorCount.increment(1.0);
                Status::internal(format!("Failed to limit L1 block number: {}", e))
            })?;

        // Check if the requested end block is less than the requested start block
        if l1_limited_end_block <= req.last_proven_block {
            return Err(Status::invalid_argument(
                "Requested end block must be greater than the last proven block",
            ));
        }

        // Limit according to the existing span proofs range
        // Fetch consecutive range proofs from the database.
        let range_proofs = self
            .proof_requester
            .db_client
            .get_consecutive_complete_range_proofs(
                req.last_proven_block as i64,
                l1_limited_end_block as i64,
                &self.program_config.commitments,
                self.requester_config.l1_chain_id,
                self.requester_config.l2_chain_id,
            )
            .await
            .unwrap();

        // Error in case there's no range proofs
        if range_proofs.len() == 0 {
            return Err(Status::internal("No consecutive span proof range found"));
        }

        // Set the requested_end_block to the last block from the range proofs
        let end_block = range_proofs.last().unwrap().end_block;

        // Prepare the request and query the proof requester
        let op_request = OPSuccinctRequest::new_agg_request(
            if self.requester_config.mock {
                RequestMode::Mock
            } else {
                RequestMode::Real
            },
            req.last_proven_block as i64,
            end_block,
            self.program_config.commitments.range_vkey_commitment,
            self.program_config.commitments.agg_vkey_hash,
            self.program_config.commitments.rollup_config_hash,
            self.requester_config.l1_chain_id,
            self.requester_config.l2_chain_id,
            req.l1_block_number as i64,
            FixedBytes::<32>::from_hex(req.l1_block_hash)
                .expect("Invalid hex string for block hash"),
            self.requester_config.prover_address,
        );

        info!(
            request_type = ?op_request.req_type,
            start_block = op_request.start_block,
            end_block = op_request.end_block,
            "Starting witness generation"
        );

        let witnessgen_duration = Instant::now();
        // Generate the stdin needed for the proof. If this fails, retry the request.
        let stdin = match self.proof_requester.generate_proof_stdin(&op_request).await {
            Ok(stdin) => stdin,
            Err(e) => {
                ValidityGauge::WitnessgenErrorCount.increment(1.0);
                return Err(Status::internal(format!(
                    "Failed to generate proof stdin: {}",
                    e
                )));
            }
        };
        let duration = witnessgen_duration.elapsed();

        info!(
            start_block = op_request.start_block,
            end_block = op_request.end_block,
            request_type = ?op_request.req_type,
            duration_s = duration.as_secs(),
            "Completed witness generation"
        );

        let reply: AggProofResponse;
        if self.proof_requester.mock {
            let proof = self
                .proof_requester
                .generate_mock_agg_proof(&op_request, stdin)
                .await
                .expect("Failed to generate mock proof");

            // If it's a compressed proof, we need to serialize the entire struct with bincode.
            let proof_bytes = bincode::DefaultOptions::new()
                .with_big_endian()
                .with_fixint_encoding()
                .serialize(&proof)
                .unwrap();

            reply = AggProofResponse {
                success: true,
                error: "".into(),
                last_proven_block: req.last_proven_block,
                end_block: end_block as u64,
                proof_request_id: proof_bytes.into(),
            };
        } else {
            let proof_id = self
                .proof_requester
                .request_agg_proof(stdin)
                .await
                .expect("Failed to request proof");

            reply = AggProofResponse {
                success: true,
                error: "".into(),
                last_proven_block: req.last_proven_block,
                end_block: end_block as u64,
                proof_request_id: alloy_primitives::Bytes::from(proof_id).into(),
            };
        }

        Ok(Response::new(reply))
    }
}
