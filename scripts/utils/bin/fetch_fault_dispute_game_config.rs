use alloy_eips::BlockId;
use alloy_primitives::{hex, Address};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use op_succinct_client_utils::{boot::hash_rollup_config, types::u32_to_u8};
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCMode};
use op_succinct_proof_utils::get_range_elf_embedded;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp1_sdk::{HashableKey, Prover, ProverClient};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const TWO_WEEKS_IN_SECONDS: u64 = 14 * 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The config for deploying the OPSuccinctFaultDisputeGame.
/// Note: The fields should be in alphabetical order for Solidity to parse it correctly.
struct FaultDisputeGameConfig {
    aggregation_vkey: String,
    challenger_addresses: Vec<String>,
    challenger_bond_wei: u64,
    dispute_game_finality_delay_seconds: u64,
    fallback_timeout_fp_secs: u64,
    game_type: u32,
    initial_bond_wei: u64,
    max_challenge_duration: u64,
    max_prove_duration: u64,
    permissionless_mode: bool,
    proposer_addresses: Vec<String>,
    range_vkey_commitment: String,
    rollup_config_hash: String,
    starting_l2_block_number: u64,
    starting_root: String,
    use_sp1_mock_verifier: bool,
    verifier_address: String,
}


/// Parse comma-separated addresses from environment variable.
fn parse_addresses(env_var: &str) -> Vec<String> {
    env::var(env_var)
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}

/// Update the FDG config with the rollup config hash and other relevant data before the contract
/// is deployed.
async fn update_fdg_config() -> Result<()> {
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let workspace_root = cargo_metadata::MetadataCommand::new().exec()?.workspace_root;

    // Determine if we're using mock verifier.
    let use_sp1_mock_verifier = env::var("USE_SP1_MOCK_VERIFIER")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Set the verifier address.
    let verifier_address = if use_sp1_mock_verifier {
        // Mock verifier will be deployed, so use zero address as placeholder.
        Address::ZERO.to_string()
    } else {
        env::var("VERIFIER_ADDRESS").unwrap_or_else(|_| {
            // Default to Groth16 VerifierGateway contract address.
            // Source: https://docs.succinct.xyz/docs/sp1/verification/contract-addresses
            "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B".to_string()
        })
    };

    // Get starting block number - use latest finalized if not set.
    let starting_l2_block_number = match env::var("STARTING_L2_BLOCK_NUMBER") {
        Ok(n) => n.parse().unwrap(),
        Err(_) => {
            println!("STARTING_L2_BLOCK_NUMBER not set, fetching latest finalized L2 block...");
            data_fetcher.get_l2_header(BlockId::finalized()).await.unwrap().number
        }
    };

    let starting_block_number_hex = format!("0x{starting_l2_block_number:x}");
    let optimism_output_data: Value = data_fetcher
        .fetch_rpc_data_with_mode(
            RPCMode::L2Node,
            "optimism_outputAtBlock",
            vec![starting_block_number_hex.into()],
        )
        .await?;

    let starting_root = optimism_output_data["outputRoot"].as_str().unwrap().to_string();

    let rollup_config = data_fetcher.rollup_config.as_ref().unwrap();
    let rollup_config_hash = format!("0x{:x}", hash_rollup_config(rollup_config));

    // Game configuration.
    let game_type = env::var("GAME_TYPE")
        .unwrap_or("42".to_string())
        .parse()
        .unwrap();

    // Timing configuration.
    let dispute_game_finality_delay_seconds = env::var("DISPUTE_GAME_FINALITY_DELAY_SECONDS")
        .unwrap_or("604800".to_string()) // 7 days default
        .parse()
        .unwrap();

    let max_challenge_duration = env::var("MAX_CHALLENGE_DURATION")
        .unwrap_or("604800".to_string()) // 7 days default
        .parse()
        .unwrap();

    let max_prove_duration = env::var("MAX_PROVE_DURATION")
        .unwrap_or("86400".to_string()) // 1 day default
        .parse()
        .unwrap();

    let fallback_timeout_fp_secs = env::var("FALLBACK_TIMEOUT_FP_SECS")
        .map(|p| p.parse().unwrap())
        .unwrap_or(TWO_WEEKS_IN_SECONDS);

    // Bond configuration.
    let initial_bond_wei = env::var("INITIAL_BOND_WEI")
        .unwrap_or("1000000000000000".to_string()) // 0.001 ETH default
        .parse()
        .unwrap();

    let challenger_bond_wei = env::var("CHALLENGER_BOND_WEI")
        .unwrap_or("1000000000000000".to_string()) // 0.001 ETH default
        .parse()
        .unwrap();

    // Access control configuration.
    let permissionless_mode = env::var("PERMISSIONLESS_MODE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap();

    let proposer_addresses = if permissionless_mode {
        vec![]
    } else {
        parse_addresses("PROPOSER_ADDRESSES")
    };

    let challenger_addresses = if permissionless_mode {
        vec![]
    } else {
        parse_addresses("CHALLENGER_ADDRESSES")
    };

    // Calculate verification keys.
    let (aggregation_vkey, range_vkey_commitment) = if use_sp1_mock_verifier {
        // Use zero values for mock verifier.
        ("0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
         "0x0000000000000000000000000000000000000000000000000000000000000000".to_string())
    } else {
        let prover = ProverClient::builder().cpu().build();
        let (_, agg_vkey) = prover.setup(AGGREGATION_ELF);
        let aggregation_vkey = agg_vkey.vk.bytes32();

        let (_, range_vkey) = prover.setup(get_range_elf_embedded());
        let range_vkey_commitment = format!("0x{}", hex::encode(u32_to_u8(range_vkey.vk.hash_u32())));

        (aggregation_vkey, range_vkey_commitment)
    };

    let fdg_config = FaultDisputeGameConfig {
        aggregation_vkey,
        challenger_addresses,
        challenger_bond_wei,
        dispute_game_finality_delay_seconds,
        fallback_timeout_fp_secs,
        game_type,
        initial_bond_wei,
        max_challenge_duration,
        max_prove_duration,
        permissionless_mode,
        proposer_addresses,
        range_vkey_commitment,
        rollup_config_hash,
        starting_l2_block_number,
        starting_root,
        use_sp1_mock_verifier,
        verifier_address,
    };

    write_fdg_config(fdg_config, workspace_root.as_std_path())?;

    Ok(())
}

/// Write the FDG config to `contracts/opsuccinctfdgconfig.json`.
fn write_fdg_config(config: FaultDisputeGameConfig, workspace_root: &Path) -> Result<()> {
    let opsuccinct_config_path = workspace_root.join("contracts/opsuccinctfdgconfig.json");
    // Create parent directories if they don't exist.
    if let Some(parent) = opsuccinct_config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Write the FDG config to the opsuccinctfdgconfig.json file.
    fs::write(&opsuccinct_config_path, serde_json::to_string_pretty(&config)?)?;
    
    println!("Fault Dispute Game configuration written to: {}", opsuccinct_config_path.display());
    println!("Starting L2 block number: {}", config.starting_l2_block_number);
    println!("Starting root: {}", config.starting_root);
    
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
    /// Environment file to load
    #[arg(long, default_value = ".env")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // This fetches the .env file from the project root. If the command is invoked in the contracts/
    // directory, the .env file in the root of the repo is used.
    if let Some(root) = find_project_root() {
        dotenv::from_path(root.join(args.env_file)).ok();
    } else {
        eprintln!("Warning: Could not find project root. {} file not loaded.", args.env_file);
    }

    update_fdg_config().await?;

    Ok(())
}