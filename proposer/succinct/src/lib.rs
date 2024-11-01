use alloy_primitives::B256;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Deserializer, Serialize};
use sp1_sdk::{network::proto::network::ProofStatus as SP1ProofStatus, SP1VerifyingKey};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidateConfigRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidateConfigResponse {
    pub rollup_config_hash_valid: bool,
    pub agg_vkey_valid: bool,
    pub range_vkey_valid: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpanProofRequest {
    pub start: u64,
    pub end: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AggProofRequest {
    #[serde(deserialize_with = "deserialize_base64_vec")]
    pub subproofs: Vec<Vec<u8>>,
    pub head: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProofResponse {
    pub proof_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
/// The type of error that occurred when unclaiming a proof. Based off of the `unclaim_description`
/// field in the `ProofStatus` struct.
pub enum UnclaimDescription {
    UnexpectedProverError = 0,
    ProgramExecutionError = 1,
    CycleLimitExceeded = 2,
    Other = 3,
}

/// Convert a string to an `UnclaimDescription`. These cover the common reasons why a proof might
/// be unclaimed.
impl From<String> for UnclaimDescription {
    fn from(description: String) -> Self {
        match description.as_str().to_lowercase().as_str() {
            "unexpected prover error" => UnclaimDescription::UnexpectedProverError,
            "program execution error" => UnclaimDescription::ProgramExecutionError,
            "cycle limit exceeded" => UnclaimDescription::CycleLimitExceeded,
            _ => UnclaimDescription::Other,
        }
    }
}

#[derive(Serialize)]
/// The status of a proof request.
pub struct ProofStatus {
    pub status: SP1ProofStatus,
    pub proof: Vec<u8>,
    pub unclaim_description: Option<UnclaimDescription>,
}

/// Configuration of the L2 Output Oracle contract. Created once at server start-up, monitors if there are any changes
/// to the contract's configuration.
#[derive(Clone)]
pub struct ContractConfig {
    pub range_vk: SP1VerifyingKey,
    pub agg_vkey_hash: B256,
    pub range_vkey_commitment: B256,
    pub rollup_config_hash: B256,
}

/// Deserialize a vector of base64 strings into a vector of vectors of bytes. Go serializes
/// the subproofs as base64 strings.
fn deserialize_base64_vec<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    s.into_iter()
        .map(|base64_str| {
            general_purpose::STANDARD
                .decode(base64_str)
                .map_err(serde::de::Error::custom)
        })
        .collect()
}
