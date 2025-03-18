use tonic::{Request, Response, Status};
use tracing::info;

use crate::proofs_server::proofs_server::Proofs;
use crate::proposer::Proposer;
use alloy_provider::{Network, Provider};
use op_succinct_host_utils::hosts::OPSuccinctHost;

// Include the generated protobuf code
pub mod proofs_server {
    tonic::include_proto!("proofs"); // Replace "agglayer" with the actual package name in your proto file
}

use proofs_server::{AggProofRequest, AggProofResponse}; // Update imports

pub struct ProofsService<'a, P, N, H>
where
    P: Provider<N> + 'static + Clone,
    N: Network,
    H: OPSuccinctHost,
{
    proposer: &'a Proposer<P, N, H>,
}

impl<'a, P, N, H> ProofsService<'a, P, N, H>
where
    P: Provider<N> + 'static + Clone,
    N: Network,
    H: OPSuccinctHost,
{
    pub fn new(proposer: &'a Proposer<P, N, H>) -> Self {
        Self { proposer }
    }
}

#[tonic::async_trait]
impl<'a, P, N, H> Proofs for ProofsService<'a, P, N, H>
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

        let req = request.into_inner();

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
