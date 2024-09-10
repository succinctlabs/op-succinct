use alloy::eips::eip1559::BaseFeeParams;
use alloy_primitives::{Address, B256};
use anyhow::{bail, Result};
use kona_primitives::{ChainGenesis, RollupConfig};
use log::info;
use op_succinct_client_utils::boot::hash_rollup_config;
use op_succinct_host_utils::fetcher::{ChainMode, OPSuccinctDataFetcher};
use serde_json::{json, Value};
use sp1_sdk::{block_on, HashableKey, ProverClient};
use std::{env, fs, path::PathBuf};

pub const AGG_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");

// Matches the output of the optimism_rollupConfig RPC call.
#[derive(Debug, Deserialize, Serialize)]
struct OptimismRollupConfigRPC {
    genesis: ChainGenesis,
    block_time: u64,
    max_sequencer_drift: u64,
    seq_window_size: u64,
    channel_timeout: u64,
    l1_chain_id: u64,
    l2_chain_id: u64,
    regolith_time: Option<u64>,
    canyon_time: Option<u64>,
    delta_time: Option<u64>,
    ecotone_time: Option<u64>,
    fjord_time: Option<u64>,
    granite_time: Option<u64>,
    holocene_time: Option<u64>,
    batch_inbox_address: Address,
    deposit_contract_address: Address,
    l1_system_config_address: Address,
    protocol_versions_address: Address,
    da_challenge_contract_address: Option<Address>,
}

/// The chain config returned by the `debug_chainConfig` RPC call.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainConfig {
    chain_id: u64,
    homestead_block: u64,
    eip150_block: u64,
    eip155_block: u64,
    eip158_block: u64,
    byzantium_block: u64,
    constantinople_block: u64,
    petersburg_block: u64,
    istanbul_block: u64,
    muir_glacier_block: u64,
    berlin_block: u64,
    london_block: u64,
    arrow_glacier_block: u64,
    gray_glacier_block: u64,
    merge_netsplit_block: u64,
    shanghai_time: u64,
    cancun_time: u64,
    bedrock_block: u64,
    regolith_time: u64,
    canyon_time: u64,
    ecotone_time: u64,
    fjord_time: u64,
    terminal_total_difficulty: u64,
    terminal_total_difficulty_passed: bool,
    optimism: OptimismConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OptimismConfig {
    eip1559_elasticity: u128,
    eip1559_denominator: u128,
    eip1559_denominator_canyon: u128,
}

/// Fetch the rollup config from the rollup node as well as the relevant config for the L200 and save it to a file.
///
fn save_rollup_config_to_zkconfig() -> Result<()> {
    let sp1_kona_data_fetcher = OPSuccinctDataFetcher::default();

    // Get the workspace root with cargo metadata to make the paths.
    let workspace_root =
        PathBuf::from(cargo_metadata::MetadataCommand::new().exec()?.workspace_root);

    // Read the L2OO config from the contracts directory.
    let mut l2oo_config = get_l2oo_config_from_contracts(&workspace_root)?;

    // If we are not using a cached starting block number, set it to 10 blocks before the latest block on L2.
    if env::var("USE_CACHED_STARTING_BLOCK").unwrap_or("false".to_string()) != "true" {
        // Set the starting block number to 10 blocks before the latest block on L2.
        let latest_block = block_on(sp1_kona_data_fetcher.get_head(ChainMode::L2))?;
        l2oo_config.starting_block_number = latest_block.number - 10;
    }
    let starting_block_number = l2oo_config.starting_block_number;

    // Fetch the rollup config from the rollup node.
    let rollup_config: OptimismRollupConfigRPC =
        fetch_rpc_data(&sp1_kona_data_fetcher.l2_node_rpc, "optimism_rollupConfig", vec![])?;
    // Get the L2 block time from the rollup config.
    let l2_block_time = rollup_config.block_time;

    // Convert the starting block number to a hex string as that's what the optimism_outputAtBlock RPC call expects.
    let starting_block_number_hex = format!("0x{:x}", starting_block_number);
    let optimism_output_data: Value = fetch_rpc_data(
        &sp1_kona_data_fetcher.l2_node_rpc,
        "optimism_outputAtBlock",
        vec![starting_block_number_hex.into()],
    )?;
    let chain_config: ChainConfig =
        fetch_rpc_data(&sp1_kona_data_fetcher.l2_rpc, "debug_chainConfig", vec![])?;

    // Canonicalize the paths.
    let rollup_config_path =
        PathBuf::from(workspace_root.join("rollup-config.json")).canonicalize()?;

    let merged_config = get_rollup_config(&rollup_config, &chain_config)?;
    let merged_config_str = serde_json::to_string_pretty(&merged_config)?;

    // Write the merged config to the rollup-config.json file.
    fs::write(rollup_config_path, &merged_config_str)?;

    info!("Updated rollup config saved to ./rollup-config.json");

    // Hash the rollup config.
    let hash: B256 = hash_rollup_config(&merged_config_str.as_bytes().to_vec());

    // Set the L2 block time from the rollup config.
    l2oo_config.l2_block_time = l2_block_time;

    // Set the rollup config hash.
    let hash_str = format!("0x{:x}", hash);
    l2oo_config.rollup_config_hash = hash_str;

    // Set the starting output root and starting timestamp.
    let timestamp = optimism_output_data["blockRef"]["timestamp"].as_u64().unwrap();
    let output_root = optimism_output_data["outputRoot"].as_str().unwrap();
    l2oo_config.starting_output_root = output_root.to_string();
    l2oo_config.starting_timestamp = timestamp;

    // Set the chain id.
    l2oo_config.chain_id = block_on(sp1_kona_data_fetcher.get_chain_id(ChainMode::L2))?;

    // Set the vkey.
    let prover = ProverClient::new();
    let (_, vkey) = prover.setup(AGG_ELF);
    l2oo_config.vkey = vkey.vk.bytes32();

    // Write the L2OO rollup config to the zkconfig.json file.
    write_l2oo_config_to_zkconfig(l2oo_config, &workspace_root)?;

    Ok(())
}

/// Fetch data from the RPC.
fn fetch_rpc_data<T>(rpc_url: &str, method: &str, params: Vec<Value>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = block_on(async {
        client
            .post(rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
                "id": 1
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await
    })?;

    serde_json::from_value(response["result"].clone()).map_err(Into::into)
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

/// Merge the rollup and chain configs. TODO: Simplify this more, use better types.
fn get_rollup_config(
    op_rollup_config_rpc: &OptimismRollupConfigRPC,
    chain: &ChainConfig,
) -> Result<RollupConfig> {
    let mut rollup_config = RollupConfig {
        genesis: op_rollup_config_rpc.genesis.clone(),
        block_time: op_rollup_config_rpc.block_time,
        max_sequencer_drift: op_rollup_config_rpc.max_sequencer_drift,
        seq_window_size: op_rollup_config_rpc.seq_window_size,
        channel_timeout: op_rollup_config_rpc.channel_timeout,
        l1_chain_id: op_rollup_config_rpc.l1_chain_id,
        l2_chain_id: op_rollup_config_rpc.l2_chain_id,
        regolith_time: op_rollup_config_rpc.regolith_time,
        canyon_time: op_rollup_config_rpc.canyon_time,
        delta_time: op_rollup_config_rpc.delta_time,
        ecotone_time: op_rollup_config_rpc.ecotone_time,
        fjord_time: op_rollup_config_rpc.fjord_time,
        granite_time: op_rollup_config_rpc.granite_time,
        holocene_time: op_rollup_config_rpc.holocene_time,
        batch_inbox_address: op_rollup_config_rpc.batch_inbox_address,
        deposit_contract_address: op_rollup_config_rpc.deposit_contract_address,
        l1_system_config_address: op_rollup_config_rpc.l1_system_config_address,
        protocol_versions_address: op_rollup_config_rpc.protocol_versions_address,
        da_challenge_address: op_rollup_config_rpc.da_challenge_contract_address,
        ..Default::default()
    };

    // Add the base fee params from the chain config.
    rollup_config.base_fee_params = BaseFeeParams {
        elasticity_multiplier: chain.optimism.eip1559_elasticity,
        max_change_denominator: chain.optimism.eip1559_denominator,
    };

    // Add the canyon base fee params from the chain config.
    rollup_config.canyon_base_fee_params = BaseFeeParams {
        elasticity_multiplier: chain.optimism.eip1559_elasticity,
        max_change_denominator: chain.optimism.eip1559_denominator_canyon,
    };

    Ok(rollup_config)
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    save_rollup_config_to_zkconfig()
}
