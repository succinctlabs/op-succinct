use alloy_primitives::B256;
use anyhow::{bail, Result};
use op_succinct_client_utils::boot::hash_rollup_config;
use op_succinct_host_utils::{
    fetcher::{OPSuccinctDataFetcher, RPCMode},
    rollup_config::save_rollup_config,
};
use serde_json::Value;
use sp1_sdk::{HashableKey, ProverClient};
use std::{env, fs, path::PathBuf};

pub const AGG_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");

/// Fetch the rollup config from the rollup node as well as the relevant config for the L200 and save it to a file.
async fn save_rollup_config_to_zkconfig() -> Result<()> {
    let data_fetcher = OPSuccinctDataFetcher::default();
    let rollup_config = data_fetcher.fetch_rollup_config().await?;

    // Get the workspace root with cargo metadata to make the paths.
    let workspace_root =
        PathBuf::from(cargo_metadata::MetadataCommand::new().exec()?.workspace_root);

    // Read the L2OO config from the contracts directory.
    let mut l2oo_config = get_l2oo_config_from_contracts(&workspace_root)?;

    // If we are not using a cached starting block number, set it to 10 blocks before the latest block on L2.
    if env::var("USE_CACHED_STARTING_BLOCK").unwrap_or("false".to_string()) != "true" {
        // Set the starting block number to 10 blocks before the latest block on L2.
        let latest_block = data_fetcher.get_head(RPCMode::L2).await?;
        l2oo_config.starting_block_number = latest_block.number - 10;
    }
    // Convert the starting block number to a hex string as that's what the optimism_outputAtBlock RPC call expects.
    let starting_block_number_hex = format!("0x{:x}", l2oo_config.starting_block_number);
    let optimism_output_data: Value = data_fetcher
        .fetch_rpc_data(
            RPCMode::L2Node,
            "optimism_outputAtBlock",
            vec![starting_block_number_hex.into()],
        )
        .await?;

    // Write the rollup config to rollup-configs/<chain_id>.json file.
    save_rollup_config(&rollup_config)?;

    // Hash the rollup config.
    let hash: B256 = hash_rollup_config(&rollup_config);
    // Set the rollup config hash.
    let hash_str = format!("0x{:x}", hash);
    l2oo_config.rollup_config_hash = hash_str;

    // Set the L2 block time from the rollup config.
    l2oo_config.l2_block_time = rollup_config.block_time;

    // Set the starting output root and starting timestamp.
    l2oo_config.starting_output_root = optimism_output_data["outputRoot"].as_str().unwrap().to_string();
    l2oo_config.starting_timestamp = optimism_output_data["blockRef"]["timestamp"].as_u64().unwrap();

    // Set the chain id.
    l2oo_config.chain_id = data_fetcher.get_chain_id(RPCMode::L2).await?;

    // Set the vkey.
    let prover = ProverClient::new();
    let (_, vkey) = prover.setup(AGG_ELF);
    l2oo_config.vkey = vkey.vk.bytes32();

    // Write the L2OO rollup config to the zkconfig.json file.
    write_l2oo_config_to_zkconfig(l2oo_config, &workspace_root)?;

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct L2OOConfig {
    chain_id: u64,
    challenger: String,
    finalization_period: u64,
    l2_block_time: u64,
    l2_output_oracle_proxy: String,
    owner: String,
    proposer: String,
    rollup_config_hash: String,
    starting_block_number: u64,
    starting_output_root: String,
    starting_timestamp: u64,
    submission_interval: u64,
    verifier_gateway: String,
    vkey: String,
}

/// Get the L2OO rollup config from the contracts directory.
fn get_l2oo_config_from_contracts(workspace_root: &PathBuf) -> Result<L2OOConfig> {
    let zkconfig_path = workspace_root.join("contracts/zkconfig.json").canonicalize()?;
    if fs::metadata(&zkconfig_path).is_ok() {
        let zkconfig_str = fs::read_to_string(zkconfig_path)?;
        Ok(serde_json::from_str(&zkconfig_str)?)
    } else {
        bail!("Missing zkconfig.json");
    }
}

/// Write the L2OO rollup config to the zkconfig.json file.
fn write_l2oo_config_to_zkconfig(config: L2OOConfig, workspace_root: &PathBuf) -> Result<()> {
    let zkconfig_path = workspace_root.join("contracts/zkconfig.json").canonicalize()?;
    // Write the L2OO rollup config to the zkconfig.json file.
    fs::write(zkconfig_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    save_rollup_config_to_zkconfig().await
}
