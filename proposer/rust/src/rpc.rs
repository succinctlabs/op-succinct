use eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SpanProofRequest {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Serialize)]
pub struct AggProofRequest {
    pub subproofs: Vec<Vec<u8>>,
    pub head: String,
}

#[derive(Debug, Deserialize)]
pub struct ProofResponse {
    pub proof_id: Vec<u8>,
}

pub struct RpcClient {
    client: Client,
    base_url: String,
}

impl RpcClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn request_span_proof(&self, start: u64, end: u64) -> Result<Vec<u8>> {
        let request = SpanProofRequest { start, end };
        let response = self
            .client
            .post(format!("{}/request_span_proof", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<ProofResponse>()
            .await?;

        Ok(response.proof_id)
    }
}
