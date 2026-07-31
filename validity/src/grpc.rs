//! gRPC server that lets an external coordinator drive the aggregation lifecycle.
//!
//! The proposer's own loop creates and submits aggregation proofs on a schedule. When this
//! server is enabled the coordinator decides the timing instead: it picks the range, asks for
//! the aggregation proof here, and consumes it downstream. The proposer no longer queues
//! aggregation proofs or submits them to L1. Range proof production is unchanged either way.
//!
//! See `proto/proofs.proto` for the wire contract.

use std::{sync::Arc, time::Instant};

use alloy_primitives::{hex::FromHex, FixedBytes};
use alloy_provider::Provider;
use bincode::Options;
use op_succinct_host_utils::{host::OPSuccinctHost, metrics::MetricsGauge};
use tonic::{Code, Request, Response, Status};
use tracing::{debug, info, warn};

use crate::{db::RequestStatus, OPSuccinctRequest, Proposer, RequestMode, ValidityGauge};

pub mod proofs {
    tonic::include_proto!("proofs");
}

use proofs::{
    proofs_server::Proofs, AggProofRequest, AggProofResponse, GetMockProofRequest,
    GetMockProofResponse,
};

/// L1 blocks to look back when resolving the L2 safe head.
///
/// The safe head derived from the L1 tip can still move, so the lookup is offset by a margin
/// to get a value that is stable for the requested aggregation range.
const SAFE_HEAD_L1_LOOKBACK: u64 = 20;

pub struct ProofsService<P, H>
where
    P: Provider + 'static,
    H: OPSuccinctHost,
{
    proposer: Arc<Proposer<P, H>>,
}

impl<P, H> ProofsService<P, H>
where
    P: Provider + 'static,
    H: OPSuccinctHost,
{
    pub fn new(proposer: Arc<Proposer<P, H>>) -> Self {
        Self { proposer }
    }

    /// Clamps `requested_end_block` to the L2 safe head, so an aggregation is never requested
    /// over blocks that are not yet safe.
    async fn clamp_to_safe_head(
        &self,
        requested_end_block: u64,
        l1_block_number: u64,
    ) -> Result<u64, Status> {
        let safe_head = self
            .proposer
            .proof_requester
            .fetcher
            .get_l2_safe_head_from_l1_block_number(
                l1_block_number.saturating_sub(SAFE_HEAD_L1_LOOKBACK),
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to get L2 safe head: {e}")))?;

        Ok(requested_end_block.min(safe_head))
    }
}

#[tonic::async_trait]
impl<P, H> Proofs for ProofsService<P, H>
where
    P: Provider + 'static + Clone,
    H: OPSuccinctHost,
{
    #[tracing::instrument(name = "proofs.request_agg_proof", skip(self, request))]
    async fn request_agg_proof(
        &self,
        request: Request<AggProofRequest>,
    ) -> Result<Response<AggProofResponse>, Status> {
        let req = request.into_inner();
        info!(
            last_proven_block = req.last_proven_block,
            requested_end_block = req.requested_end_block,
            l1_block_number = req.l1_block_number,
            "Received aggregation proof request"
        );

        let end_block =
            self.clamp_to_safe_head(req.requested_end_block, req.l1_block_number).await?;

        if end_block <= req.last_proven_block {
            return Err(Status::new(
                Code::InvalidArgument,
                format!(
                    "Requested end block ({end_block} after clamping to the safe head) must be \
                     greater than the last proven block ({})",
                    req.last_proven_block
                ),
            ));
        }

        // Aggregate over whatever contiguous run of completed range proofs is already available
        // within the requested window.
        let range_proofs = self
            .proposer
            .proof_requester
            .db_client
            .get_consecutive_complete_range_proofs(
                req.last_proven_block as i64,
                end_block as i64,
                &self.proposer.program_config.commitments,
                self.proposer.requester_config.l1_chain_id,
                self.proposer.requester_config.l2_chain_id,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch range proofs: {e}")))?;

        let Some(last_range_proof) = range_proofs.last() else {
            return Err(Status::new(
                Code::NotFound,
                "No completed range proofs found for the requested range",
            ));
        };

        // The available range proofs may stop short of `end_block`.
        let end_block = last_range_proof.end_block;

        let op_request = OPSuccinctRequest::new_agg_request(
            if self.proposer.requester_config.mock { RequestMode::Mock } else { RequestMode::Real },
            req.last_proven_block as i64,
            end_block,
            self.proposer.program_config.commitments.range_vkey_commitment,
            self.proposer.program_config.commitments.agg_vkey_hash,
            self.proposer.program_config.commitments.rollup_config_hash,
            self.proposer.requester_config.l1_chain_id,
            self.proposer.requester_config.l2_chain_id,
            req.l1_block_number as i64,
            FixedBytes::<32>::from_hex(&req.l1_block_hash).map_err(|e| {
                Status::invalid_argument(format!("Invalid hex string for l1_block_hash: {e}"))
            })?,
            self.proposer.driver_config.signer.address(),
        );

        if !self.proposer.validate_aggregation_request(&range_proofs, &op_request).await {
            warn!(
                last_proven_block = req.last_proven_block,
                end_block, "Aggregation request validation failed"
            );
            return Err(Status::new(Code::InvalidArgument, "Aggregation request validation failed"));
        }
        debug!(
            start_block = op_request.start_block,
            end_block = op_request.end_block,
            "Aggregation request validated"
        );

        info!(
            start_block = op_request.start_block,
            end_block = op_request.end_block,
            l1_block_number = ?op_request.checkpointed_l1_block_number,
            "Starting witness generation"
        );
        let witnessgen_start = Instant::now();
        let stdin =
            self.proposer.proof_requester.generate_proof_stdin(&op_request).await.map_err(|e| {
                ValidityGauge::WitnessgenErrorCount.increment(1.0);
                Status::internal(format!("Failed to generate proof stdin: {e}"))
            })?;
        info!(
            start_block = op_request.start_block,
            end_block = op_request.end_block,
            duration_s = witnessgen_start.elapsed().as_secs(),
            "Completed witness generation"
        );

        let proof_request_id = if self.proposer.proof_requester.mock {
            let proof = self
                .proposer
                .proof_requester
                .generate_mock_agg_proof(&op_request, stdin)
                .await
                .map_err(|e| Status::internal(format!("Failed to generate mock proof: {e}")))?;

            let proof_bytes = bincode::DefaultOptions::new()
                .with_big_endian()
                .with_fixint_encoding()
                .serialize(&proof)
                .map_err(|e| Status::internal(format!("Failed to serialize mock proof: {e}")))?;

            // The mock proof is not relayed by the proposer, so it is stored as a completed
            // request that the caller can fetch back through `GetMockProof`.
            let stored = OPSuccinctRequest {
                proof: Some(proof_bytes),
                status: RequestStatus::Complete,
                ..op_request
            };
            let row_id = self
                .proposer
                .proof_requester
                .db_client
                .insert_request(&stored)
                .await
                .map_err(|e| Status::internal(format!("Failed to save request to DB: {e}")))?;

            FixedBytes::<32>::left_padding_from(&row_id.to_be_bytes())
        } else {
            self.proposer
                .proof_requester
                .request_agg_proof(stdin)
                .await
                .map_err(|e| Status::internal(format!("Failed to request proof: {e}")))?
        };

        Ok(Response::new(AggProofResponse {
            last_proven_block: req.last_proven_block,
            end_block: end_block as u64,
            proof_request_id: proof_request_id.to_vec(),
        }))
    }

    #[tracing::instrument(name = "proofs.get_mock_proof", skip(self, request))]
    async fn get_mock_proof(
        &self,
        request: Request<GetMockProofRequest>,
    ) -> Result<Response<GetMockProofResponse>, Status> {
        let req = request.into_inner();

        let proof = self
            .proposer
            .proof_requester
            .db_client
            .get_proof_by_request_id(req.proof_id)
            .await
            .map_err(|e| Status::not_found(format!("Mock proof not found: {e}")))?;

        Ok(Response::new(GetMockProofResponse { proof }))
    }
}
