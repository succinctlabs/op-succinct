//! op-alt-da client
//!
//! Fetches batch data from the op-alt-da server given keccak256 commitments.
//! The op-alt-da server stores the actual batch data and serves it by commitment.

use alloy_primitives::{hex, keccak256, B256};
use anyhow::{anyhow, Result};
use reqwest::Client;

/// Client for the op-alt-da server.
///
/// The op-alt-da server exposes a REST API:
///   GET /get/<commitment_hex> -> raw batch data
///   PUT /put                  -> store batch data, returns commitment
#[derive(Clone, Debug)]
pub struct AltDAClient {
    /// Base URL of the op-alt-da server (e.g. "http://localhost:3100").
    pub base_url: String,
    pub client: Client,
}

impl AltDAClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url, client: Client::new() }
    }

    /// Creates from environment variable `ALT_DA_SERVER`.
    pub fn from_env() -> Result<Self> {
        let base_url =
            std::env::var("ALT_DA_SERVER").map_err(|_| anyhow!("ALT_DA_SERVER not set"))?;
        Ok(Self::new(base_url))
    }

    /// Fetches batch data by keccak256 commitment from the op-alt-da server.
    /// Verifies: keccak256(data) == commitment.
    pub async fn get_input(&self, commitment: &B256) -> Result<Vec<u8>> {
        let url = format!("{}/get/0x{}", self.base_url, hex::encode(commitment));
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let data = resp.bytes().await?.to_vec();

        // Verify the commitment.
        let computed = keccak256(&data);
        if computed != *commitment {
            return Err(anyhow!(
                "AltDA commitment mismatch: expected {}, got {}",
                commitment,
                computed
            ));
        }

        Ok(data)
    }

    /// Fetches multiple batch data entries by their commitments.
    pub async fn get_inputs(&self, commitments: &[B256]) -> Result<Vec<Vec<u8>>> {
        let mut results = Vec::with_capacity(commitments.len());
        for commitment in commitments {
            results.push(self.get_input(commitment).await?);
        }
        Ok(results)
    }
}
