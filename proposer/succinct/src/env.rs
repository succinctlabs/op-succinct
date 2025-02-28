use std::env;

use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use sp1_sdk::{network::FulfillmentStrategy, SP1ProofMode};

pub struct EnvironmentConfig {
    pub db_url: String,
    pub metrics_port: u16,
    pub l1_rpc: String,
    pub private_key: PrivateKeySigner,
    pub loop_interval: Option<u64>,
    pub range_proof_strategy: FulfillmentStrategy,
    pub agg_proof_strategy: FulfillmentStrategy,
    pub agg_proof_mode: SP1ProofMode,
    pub l2oo_address: Address,
    pub range_proof_interval: u64,
    pub max_concurrent_witness_gen: u64,
    pub max_concurrent_proof_requests: u64,
    pub submission_interval: u64,
    pub mock: bool,
}

pub fn read_env() -> Result<EnvironmentConfig> {
    let range_proof_strategy = match env::var("RANGE_PROOF_STRATEGY") {
        Ok(v) => {
            if v.to_lowercase() == "hosted" {
                FulfillmentStrategy::Hosted
            } else {
                FulfillmentStrategy::Reserved
            }
        }
        Err(_) => FulfillmentStrategy::Reserved,
    };
    let agg_proof_strategy = match env::var("AGG_PROOF_STRATEGY") {
        Ok(v) => {
            if v.to_lowercase() == "hosted" {
                FulfillmentStrategy::Hosted
            } else {
                FulfillmentStrategy::Reserved
            }
        }
        Err(_) => FulfillmentStrategy::Reserved,
    };
    let agg_proof_mode = match env::var("AGG_PROOF_MODE") {
        Ok(v) => {
            if v.to_lowercase() == "plonk" {
                SP1ProofMode::Plonk
            } else {
                SP1ProofMode::Groth16
            }
        }
        Err(_) => SP1ProofMode::Groth16,
    };

    let config = EnvironmentConfig {
        metrics_port: env::var("METRICS_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or_else(|_| 8080),
        loop_interval: env::var("LOOP_INTERVAL")
            .map(|v| v.parse::<u64>().expect("Failed to parse LOOP_INTERVAL"))
            .ok(),
        l1_rpc: env::var("L1_RPC").expect("L1_RPC is not set"),
        private_key: env::var("PRIVATE_KEY")
            .expect("PRIVATE_KEY is not set")
            .parse()
            .expect("Failed to parse PRIVATE_KEY"),
        db_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
        range_proof_strategy,
        agg_proof_strategy,
        agg_proof_mode,
        l2oo_address: env::var("L2OO_ADDRESS")
            .expect("L2OO_ADDRESS is not set")
            .parse()?,
        range_proof_interval: env::var("RANGE_PROOF_INTERVAL")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()?,
        max_concurrent_witness_gen: env::var("MAX_CONCURRENT_WITNESS_GEN")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()?,
        max_concurrent_proof_requests: env::var("MAX_CONCURRENT_PROOF_REQUESTS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()?,
        submission_interval: env::var("SUBMISSION_INTERVAL")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()?,
        mock: env::var("OP_SUCCINCT_MOCK")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()?,
    };

    Ok(config)
}
