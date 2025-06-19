use anyhow::Result;
use op_succinct_scripts::config_common::{
    find_project_root, get_shared_config_data, get_workspace_root, parse_addresses,
    write_config_file, TWO_WEEKS_IN_SECONDS,
};
use serde::{Deserialize, Serialize};
use std::env;

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
    optimism_portal2_address: String,
    permissionless_mode: bool,
    proposer_addresses: Vec<String>,
    range_vkey_commitment: String,
    rollup_config_hash: String,
    starting_l2_block_number: u64,
    starting_root: String,
    use_sp1_mock_verifier: bool,
    verifier_address: String,
}

async fn update_fdg_config() -> Result<()> {
    let shared_config = get_shared_config_data().await?;
    let workspace_root = get_workspace_root()?;

    // Game configuration.
    let game_type = env::var("GAME_TYPE").unwrap_or("42".to_string()).parse().unwrap();

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
    let permissionless_mode =
        env::var("PERMISSIONLESS_MODE").unwrap_or("false".to_string()).parse().unwrap();

    let proposer_addresses =
        if permissionless_mode { vec![] } else { parse_addresses("PROPOSER_ADDRESSES") };

    let challenger_addresses =
        if permissionless_mode { vec![] } else { parse_addresses("CHALLENGER_ADDRESSES") };

    // OptimismPortal2 configuration.
    let optimism_portal2_address = env::var("OPTIMISM_PORTAL2_ADDRESS").unwrap_or_else(|_| {
        // Default to zero address if not provided - will deploy MockOptimismPortal2
        "0x0000000000000000000000000000000000000000".to_string()
    });

    let fdg_config = FaultDisputeGameConfig {
        aggregation_vkey: shared_config.aggregation_vkey,
        challenger_addresses,
        challenger_bond_wei,
        dispute_game_finality_delay_seconds,
        fallback_timeout_fp_secs,
        game_type,
        initial_bond_wei,
        max_challenge_duration,
        max_prove_duration,
        optimism_portal2_address,
        permissionless_mode,
        proposer_addresses,
        range_vkey_commitment: shared_config.range_vkey_commitment,
        rollup_config_hash: shared_config.rollup_config_hash,
        starting_l2_block_number: shared_config.starting_l2_block_number,
        starting_root: shared_config.starting_output_root,
        use_sp1_mock_verifier: shared_config.use_sp1_mock_verifier,
        verifier_address: shared_config.verifier_address,
    };

    let config_path = workspace_root.join("contracts/opsuccinctfdgconfig.json");
    write_config_file(&fdg_config, &config_path, "Fault Dispute Game")?;

    Ok(())
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
