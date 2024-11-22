use alloy::{eips::BlockId, hex, signers::local::PrivateKeySigner};
use alloy_primitives::Address;
use anyhow::Result;
use op_succinct_client_utils::{boot::hash_rollup_config, types::u32_to_u8};
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp1_sdk::{HashableKey, ProverClient};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub const AGG_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");
pub const RANGE_ELF: &[u8] = include_bytes!("../../../elf/range-elf");

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The config for deploying the OPSuccinctL2OutputOracle.
/// Note: The fields should be in alphabetical order for Solidity to parse it correctly.
struct L2OOConfig {
    challenger: String,
    finalization_period: u64,
    l2_block_time: u64,
    proposer: String,
    rollup_config_hash: String,
    starting_block_number: u64,
    starting_output_root: String,
    starting_timestamp: u64,
    submission_interval: u64,
    verifier_gateway: String,
    aggregation_vkey: String,
    range_vkey_commitment: String,
}

/// Update the L2OO config with the rollup config hash and other relevant data before the contract is deployed.
///
/// Specifically, updates the following fields in `opsuccinctl2ooconfig.json`:
/// - rollup_config_hash: Get the hash of the rollup config from the rollup config file.
/// - l2_block_time: Get the block time from the rollup config.
/// - starting_block_number: If `USE_CACHED_STARTING_BLOCK` is `false`, set starting_block_number to 10 blocks before the latest block on L2.
/// - starting_output_root: Set to the output root of the starting block number.
/// - starting_timestamp: Set to the timestamp of the starting block number.
/// - chain_id: Get the chain id from the rollup config.
/// - vkey: Get the vkey from the aggregation program ELF.
/// - owner: Set to the address associated with the private key.
async fn update_l2oo_config() -> Result<()> {
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config()
        .await
        .unwrap();
    // Get the workspace root with cargo metadata to make the paths.
    let workspace_root = PathBuf::from(
        cargo_metadata::MetadataCommand::new()
            .exec()?
            .workspace_root,
    );

    // Set the verifier address.
    let verifier_gateway = if env::var("VERIFIER_ADDRESS").is_ok() {
        env::var("VERIFIER_ADDRESS").unwrap()
    } else {
        // Set the verifier gateway to the address of the Groth16 VerifierGateway contract.
        // Source: https://docs.succinct.xyz/verification/onchain/contract-addresses
        "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B".to_string()
    };

    let starting_block_number = if env::var("STARTING_BLOCK_NUMBER").is_ok() {
        env::var("STARTING_BLOCK_NUMBER").unwrap().parse().unwrap()
    } else {
        // If we are not using a cached starting block number, set it to the finalized block number on L2.
        data_fetcher
            .get_l2_header(BlockId::finalized())
            .await?
            .number
    };

    // Convert the starting block number to a hex string for the optimism_outputAtBlock RPC call.
    let starting_block_number_hex = format!("0x{:x}", starting_block_number);
    let optimism_output_data: Value = data_fetcher
        .fetch_rpc_data_with_mode(
            RPCMode::L2Node,
            "optimism_outputAtBlock",
            vec![starting_block_number_hex.into()],
        )
        .await?;
    // Set the starting output root and starting timestamp.
    let starting_output_root = optimism_output_data["outputRoot"]
        .as_str()
        .unwrap()
        .to_string();
    let starting_timestamp = optimism_output_data["blockRef"]["timestamp"]
        .as_u64()
        .unwrap();

    let rollup_config_hash = format!(
        "0x{:x}",
        hash_rollup_config(data_fetcher.rollup_config.as_ref().unwrap())
    );

    // Set the L2 block time from the rollup config.
    let l2_block_time = data_fetcher.rollup_config.as_ref().unwrap().block_time;

    // Set the submission interval.
    // The order of precedence is:
    // 1. SUBMISSION_INTERVAL environment variable
    // 2. 1000 (default)
    let submission_interval: u64 = env::var("SUBMISSION_INTERVAL")
        .unwrap_or("1000".to_string())
        .parse()?;

    let finalization_period = if env::var("FINALIZATION_PERIOD").is_ok() {
        env::var("FINALIZATION_PERIOD").unwrap().parse().unwrap()
    } else {
        0
    };

    let proposer = if let Ok(proposer) = env::var("PROPOSER") {
        proposer
    } else {
        // Get the account associated with the private key.
        let private_key = env::var("PRIVATE_KEY").unwrap();
        let signer: PrivateKeySigner = private_key.parse().expect("Failed to parse private key");
        signer.address().to_string()
    };

    let challenger = if let Ok(challenger) = env::var("CHALLENGER") {
        challenger
    } else {
        Address::ZERO.to_string()
    };

    // Set the vkey.
    let prover = ProverClient::new();
    let (_, vkey) = prover.setup(AGG_ELF);
    let aggregation_vkey = vkey.vk.bytes32();

    let (_, range_vkey) = prover.setup(RANGE_ELF);
    let range_vkey_commitment = format!("0x{}", hex::encode(u32_to_u8(range_vkey.vk.hash_u32())));

    let l2oo_config = L2OOConfig {
        challenger,
        finalization_period,
        l2_block_time,
        proposer,
        rollup_config_hash,
        starting_block_number,
        starting_output_root,
        starting_timestamp,
        submission_interval,
        verifier_gateway,
        aggregation_vkey,
        range_vkey_commitment,
    };

    // Write the L2OO rollup config to the opsuccinctl2ooconfig.json file.
    write_l2oo_config(l2oo_config, &workspace_root)?;

    Ok(())
}

/// Write the L2OO rollup config to `contracts/opsuccinctl2ooconfig.json`.
fn write_l2oo_config(config: L2OOConfig, workspace_root: &Path) -> Result<()> {
    let opsuccinct_config_path = workspace_root
        .join("contracts/opsuccinctl2ooconfig.json")
        .canonicalize()?;
    // Write the L2OO rollup config to the opsuccinctl2ooconfig.json file.
    fs::write(
        opsuccinct_config_path,
        serde_json::to_string_pretty(&config)?,
    )?;
    Ok(())
}

fn find_project_root() -> Option<PathBuf> {
    let mut path = std::env::current_dir().ok()?;
    while !path.join(".git").exists() {
        if !path.pop() {
            return None;
        }
    }
    Some(path)
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// L2 chain ID
    #[arg(long, default_value = ".env")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // This fetches the .env file from the project root. If the command is invoked in the contracts/ directory,
    // the .env file in the root of the repo is used.
    if let Some(root) = find_project_root() {
        dotenv::from_path(root.join(args.env_file)).ok();
    } else {
        eprintln!(
            "Warning: Could not find project root. {} file not loaded.",
            args.env_file
        );
    }

    update_l2oo_config().await?;

    Ok(())
}
