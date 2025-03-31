use tonic::{Request, Response, Status};
use tracing::info;

use crate::proposer::Proposer;
use alloy_provider::{Network, Provider};
use grpc::proofs::proofs_server::Proofs;
use grpc::proofs::{AggProofRequest, AggProofResponse};
use op_succinct_host_utils::hosts::OPSuccinctHost;

use std::sync::Arc;

pub struct Service<P, N, H>
where
    P: Provider<N> + 'static + Clone,
    N: Network,
    H: OPSuccinctHost,
{
    proposer: Arc<Proposer<P, N, H>>,
}

impl<P, N, H> Service<P, N, H>
where
    P: Provider<N> + 'static + Clone,
    N: Network,
    H: OPSuccinctHost,
{
    pub fn new(proposer: Arc<Proposer<P, N, H>>) -> Self {
        Self { proposer }
    }
}

#[tonic::async_trait]
impl<P, N, H> Proofs for Service<P, N, H>
// Update trait implementation
where
    P: Provider<N> + 'static + Clone,
    N: Network,
    H: OPSuccinctHost,
{
    async fn request_agg_proof(
        // Update method name
        &self,
        request: Request<AggProofRequest>, // Update request type
    ) -> Result<Response<AggProofResponse>, Status> {
        // Update response type
        info!("Received AggProofRequest: {:?}", request);

        let _req = request.into_inner();

        // TODO: Implement the logic to handle the proof request using the inner proposer.
        // This is a placeholder implementation.  You'll need to adapt it to your specific needs.
        let request_id = 12345; // Replace with actual request ID generation logic

        let reply = AggProofResponse {
            success: true,
            error: "".into(),
            request_id: request_id,
        };

        Ok(Response::new(reply))
    }
}
