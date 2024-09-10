use alloy_primitives::B256;
use anyhow::{bail, Context, Result};
use log::info;
use op_succinct_client_utils::boot::hash_rollup_config;
use op_succinct_host_utils::fetcher::{ChainMode, OPSuccinctDataFetcher};
use serde_json::{json, Value};
use sp1_sdk::{block_on, HashableKey, ProverClient};
use std::{env, fs, path::PathBuf};

pub const AGG_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");

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
        l2oo_config["startingBlockNumber"] = json!(latest_block.number - 10);
    }
    let starting_block_number = l2oo_config["startingBlockNumber"].as_u64().unwrap();

    // Fetch the rollup config from the rollup node.
    let rollup_json =
        fetch_rpc_data(&sp1_kona_data_fetcher.l2_node_rpc, "optimism_rollupConfig", vec![])?;
    // Get the L2 block time from the rollup config.
    let l2_block_time = rollup_json["block_time"].as_u64().unwrap();

    // Convert the starting block number to a hex string as that's what the optimism_outputAtBlock RPC call expects.
    let starting_block_number_hex = format!("0x{:x}", starting_block_number);
    let optimism_output_data = fetch_rpc_data(
        &sp1_kona_data_fetcher.l2_node_rpc,
        "optimism_outputAtBlock",
        vec![starting_block_number_hex.into()],
    )?;
    let chain_json = fetch_rpc_data(&sp1_kona_data_fetcher.l2_rpc, "debug_chainConfig", vec![])?;

    // Canonicalize the paths.
    let rollup_config_path =
        PathBuf::from(workspace_root.join("rollup-config.json")).canonicalize()?;

    let merged_config = merge_configs(&rollup_json, &chain_json)?;
    let merged_config_str = serde_json::to_string_pretty(&merged_config)?;

    // Write the merged config to the rollup-config.json file.
    fs::write(rollup_config_path, &merged_config_str)?;

    info!("Updated rollup config saved to ./rollup-config.json");

    // Hash the rollup config.
    let hash: B256 = hash_rollup_config(&merged_config_str.as_bytes().to_vec());

    // Set the L2 block time from the rollup config.
    l2oo_config["l2BlockTime"] = json!(l2_block_time);

    // Set the rollup config hash.
    let hash_str = format!("0x{:x}", hash);
    l2oo_config["rollupConfigHash"] = json!(hash_str);

    // Set the starting output root and starting timestamp.
    let timestamp = optimism_output_data["blockRef"]["timestamp"].as_u64().unwrap();
    let output_root = optimism_output_data["outputRoot"].clone();
    l2oo_config["startingOutputRoot"] = output_root;
    l2oo_config["startingTimestamp"] = json!(timestamp);

    // Set the chain id.
    l2oo_config["chainId"] = json!(block_on(sp1_kona_data_fetcher.get_chain_id(ChainMode::L2))?);

    // Set the vkey.
    let prover = ProverClient::new();
    let (_, vkey) = prover.setup(AGG_ELF);
    l2oo_config["vkey"] = json!(vkey.vk.bytes32());

    // Write the L2OO rollup config to the zkconfig.json file.
    write_l2oo_config_to_zkconfig(l2oo_config, &workspace_root)?;

    Ok(())
}

/// Fetch data from the RPC.
fn fetch_rpc_data(rpc_url: &str, method: &str, params: Vec<Value>) -> Result<Value> {
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

    Ok(response["result"].clone())
}

/// Get the L2OO rollup config from the contracts directory.
fn get_l2oo_config_from_contracts(workspace_root: &PathBuf) -> Result<Value> {
    let zkconfig_path = workspace_root.join("contracts/zkconfig.json").canonicalize()?;
    if fs::metadata(&zkconfig_path).is_ok() {
        let zkconfig_str = fs::read_to_string(zkconfig_path)?;
        Ok(serde_json::from_str(&zkconfig_str)?)
    } else {
        bail!("Missing zkconfig.json");
    }
}

/// Write the L2OO rollup config to the zkconfig.json file.
fn write_l2oo_config_to_zkconfig(config: Value, workspace_root: &PathBuf) -> Result<()> {
    let zkconfig_path = workspace_root.join("contracts/zkconfig.json").canonicalize()?;
    fs::write(zkconfig_path, serde_json::to_string_pretty(&config)?)?;
    Ok(())
}

/// Merge the rollup and chain configs.
fn merge_configs(rollup: &Value, chain: &Value) -> Result<Value> {
    let elasticity = chain["optimism"]["eip1559Elasticity"]
        .as_u64()
        .context("Missing eip1559Elasticity in chain config")?;
    let denominator = chain["optimism"]["eip1559Denominator"]
        .as_u64()
        .context("Missing eip1559Denominator in chain config")?;

    let mut merged = rollup.clone();

    merge_time_fields(&mut merged, chain);
    set_base_fee_params(&mut merged, elasticity, denominator);
    set_canyon_base_fee_params(&mut merged, chain, elasticity);
    rename_batcher_addr(&mut merged)?;

    Ok(merged)
}

/// Merge the time fields from the chain config into the rollup config.
fn merge_time_fields(merged: &mut Value, chain: &Value) {
    for field in &[
        "regolithTime",
        "canyonTime",
        "deltaTime",
        "ecotoneTime",
        "fjordTime",
        "graniteTime",
        "holoceneTime",
    ] {
        if let Some(value) = chain[field].as_str() {
            merged[field] = json!(value);
        }
    }
}

/// Set the base fee params in the rollup config.
fn set_base_fee_params(merged: &mut Value, elasticity: u64, denominator: u64) {
    merged["base_fee_params"] = json!({
        "elasticity_multiplier": elasticity,
        "max_change_denominator": denominator
    });
}

/// Set the canyon base fee params in the rollup config.
fn set_canyon_base_fee_params(merged: &mut Value, chain: &Value, elasticity: u64) {
    if let Some(canyon_denominator) = chain["optimism"]["eip1559DenominatorCanyon"].as_u64() {
        merged["canyon_base_fee_params"] = json!({
            "elasticity_multiplier": elasticity,
            "max_change_denominator": canyon_denominator
        });
    }
}

/// Rename the batcher address in the rollup config.
fn rename_batcher_addr(merged: &mut Value) -> Result<()> {
    if let Some(system_config) = merged["genesis"]["system_config"].as_object_mut() {
        if let Some(batcher_addr) = system_config.remove("batcherAddr") {
            system_config.insert("batcherAddress".to_string(), batcher_addr);
        } else {
            bail!("Missing batcherAddr in rollup config");
        }
    } else {
        bail!("Invalid structure in rollup config: missing system_config");
    }
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    save_rollup_config_to_zkconfig()
}
