use alloy_primitives::B256;
use anyhow::Result;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{error, info};

use crate::ProposerAgglayer;

// Include the generated protobuf code
pub mod agglayer {
    tonic::include_proto!("agglayer");
}

use agglayer::{
    proofs_server::{Proofs, ProofsServer},
    AggProofRequest, AggProofResponse,
};

/// Implementation of the Proofs service
pub struct ProofsService<'a, P, N>
where
    P: alloy_provider::Provider<N> + 'static + Clone + Send + Sync,
    N: alloy_provider::Network + Send + Sync + 'static,
{
    proposer: Arc<ProposerAgglayer<'a, P, N>>,
}

impl<'a, P, N> ProofsService<'a, P, N>
where
    P: alloy_provider::Provider<N> + 'static + Clone + Send + Sync,
    N: alloy_provider::Network + Send + Sync + 'static,
{
    pub fn new(proposer: Arc<ProposerAgglayer<'a, P, N>>) -> Self {
        Self { proposer }
    }
}

#[async_trait]
impl<'a, P, N> Proofs for ProofsService<'a, P, N>
where
    P: alloy_provider::Provider<N> + 'static + Clone + Send + Sync,
    N: alloy_provider::Network + Send + Sync + 'static,
{
    async fn request_agg_proof(
        &self,
        request: Request<AggProofRequest>,
    ) -> std::result::Result<Response<AggProofResponse>, Status> {
        let agg_request = request.into_inner();

        // Validate input parameters
        if agg_request.l1_block_hash.len() != 32 {
            return Err(Status::invalid_argument("Invalid L1 block hash length"));
        }

        let start_block = agg_request.start;
        let end_block = agg_request.end;
        let l1_block_number = agg_request.l1_block_number;
        let l1_block_hash = B256::from_slice(&agg_request.l1_block_hash);

        info!(
            start_block = start_block,
            end_block = end_block,
            l1_block_number = l1_block_number,
            l1_block_hash = ?l1_block_hash,
            "Received RequestAggProof gRPC request"
        );

        // Create aggregation request
        match self.proposer.create_aggregation_proofs().await {
            Ok(request_id) => {
                // info!(request_id = request_id, "Created aggregation proof request");
                Ok(Response::new(AggProofResponse {
                    success: true,
                    error: "".to_string(),
                    request_id,
                }))
            }
            Err(err) => {
                let error_msg = err.to_string();
                error!(error = %error_msg, "Failed to create aggregation proof request");
                Ok(Response::new(AggProofResponse {
                    success: false,
                    error: error_msg,
                    request_id: -1,
                }))
            }
        }
    }
}

/// Start the gRPC server
pub async fn start_grpc_server<'a, P, N>(
    proposer: Arc<ProposerAgglayer<'a, P, N>>,
    grpc_addr: &str,
) -> Result<()>
where
    P: alloy_provider::Provider<N> + 'static + Clone + Send + Sync,
    N: alloy_provider::Network + Send + Sync + 'static,
{
    let addr = grpc_addr.parse()?;
    let proofs_service = ProofsService::new(proposer);

    info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(ProofsServer::new(proofs_service))
        .serve(addr)
        .await?;

    Ok(())
}
