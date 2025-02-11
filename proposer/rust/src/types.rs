use ethers::types::{Address, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRequest {
    pub id: u64,
    pub start_block: u64,
    pub end_block: u64,
    pub proof_type: ProofType,
    pub status: ProofStatus,
    pub proof: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofType {
    Span,
    Aggregation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    Pending,
    WitnessGeneration,
    Proving,
    Complete,
    Failed,
}
