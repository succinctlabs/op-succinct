//! gRPC server that lets an external coordinator drive the aggregation lifecycle.
//!
//! The proposer's own loop creates and submits aggregation proofs on a schedule. When this
//! server is enabled the coordinator decides the timing instead: it picks the range, asks for
//! the aggregation proof here, and consumes it downstream. The proposer no longer queues
//! aggregation proofs or submits them to L1. Range proof production is unchanged either way.
//!
//! See `proto/proofs.proto` for the wire contract.

use std::sync::Arc;

use alloy_primitives::{hex::FromHex, B256};
use alloy_provider::Provider;
use op_succinct_host_utils::host::OPSuccinctHost;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::{ExternalAggregationError, ExternalAggregationRequest, Proposer};

pub mod proofs {
    tonic::include_proto!("proofs");
}

use proofs::{
    proofs_server::Proofs, AggProofRequest, AggProofResponse, GetMockProofRequest,
    GetMockProofResponse,
};

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
}

fn grpc_status(error: ExternalAggregationError) -> Status {
    match error {
        ExternalAggregationError::InvalidArgument(message) => Status::invalid_argument(message),
        ExternalAggregationError::NotFound(message) => Status::not_found(message),
        ExternalAggregationError::Internal(message) => Status::internal(message),
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

        let result = self
            .proposer
            .request_external_aggregation(ExternalAggregationRequest {
                last_proven_block: req.last_proven_block,
                requested_end_block: req.requested_end_block,
                l1_block_number: req.l1_block_number,
                l1_block_hash: B256::from_hex(&req.l1_block_hash).map_err(|error| {
                    Status::invalid_argument(format!(
                        "Invalid hex string for l1_block_hash: {error}"
                    ))
                })?,
            })
            .await
            .map_err(grpc_status)?;

        Ok(Response::new(AggProofResponse {
            last_proven_block: result.last_proven_block,
            end_block: result.end_block,
            proof_request_id: result.proof_request_id.to_vec(),
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
            .get_external_mock_proof(req.proof_id)
            .await
            .map_err(|e| Status::not_found(format!("Mock proof not found: {e}")))?;

        Ok(Response::new(GetMockProofResponse { proof }))
    }
}

#[cfg(test)]
mod tests {
    use tonic::Code;

    use super::{grpc_status, ExternalAggregationError};

    #[test]
    fn maps_domain_errors_to_grpc_statuses() {
        assert_eq!(
            grpc_status(ExternalAggregationError::InvalidArgument("invalid".into())).code(),
            Code::InvalidArgument
        );
        assert_eq!(
            grpc_status(ExternalAggregationError::NotFound("missing".into())).code(),
            Code::NotFound
        );
        assert_eq!(
            grpc_status(ExternalAggregationError::Internal("failed".into())).code(),
            Code::Internal
        );
    }
}
