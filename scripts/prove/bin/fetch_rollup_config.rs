use alloy_primitives::B256;
use anyhow::{bail, Context, Result};
use op_succinct_client_utils::boot::hash_rollup_config;
use op_succinct_host_utils::fetcher::{ChainMode, OPSuccinctDataFetcher};
use serde_json::{json, Value};
use sp1_sdk::block_on;
use std::{env, fs, path::PathBuf};

/// Fetch the rollup config from the rollup node and save it to a file.
fn save_rollup_config_to_zkconfig() -> Result<()> {
    let (rollup_rpc, l2_rpc) = get_rpc_urls();

    let rollup_config = fetch_rpc_data(&rollup_rpc, "optimism_rollupConfig")?;
    let chain_config = fetch_rpc_data(&l2_rpc, "debug_chainConfig")?;

    // Get the workspace root with cargo metadata to make the paths.
    let workspace_root = cargo_metadata::MetadataCommand::new().exec()?.workspace_root;

    // Canonicalize the paths
    let rollup_config_path =
        PathBuf::from(workspace_root.join("rollup-config.json")).canonicalize()?;

    let rollup_json: Value = serde_json::from_str(&rollup_config)?;
    let chain_json: Value = serde_json::from_str(&chain_config)?;

    let merged_config = merge_configs(&rollup_json, &chain_json)?;
    let merged_config_str = serde_json::to_string_pretty(&merged_config)?;

    fs::write(rollup_config_path, &merged_config_str)?;

    println!("Updated rollup config saved to ./rollup-config.json");

    let hash: B256 = hash_rollup_config(&merged_config_str.as_bytes().to_vec());
    update_zkconfig_rollup_config_hash(hash, &workspace_root.into_std_path_buf())?;

    Ok(())
}

/// Get the rollup RPC URLs.
fn get_rpc_urls() -> (String, String) {
    let rollup_rpc = env::var("L2_NODE_RPC")
        .expect("Must provide rollup rpc as argument or env variable (L2_NODE_RPC)");
    let l2_rpc =
        env::var("L2_RPC").expect("Must provide L2 rpc as argument or env variable (L2_RPC)");

    (rollup_rpc, l2_rpc)
}

/// Fetch data from the RPC.
fn fetch_rpc_data(rpc_url: &str, method: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = block_on(async {
        client
            .post(rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": [],
                "id": 1
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await
    })?;

    Ok(response["result"].to_string())
}

/// Update the rollup config hash in the zkconfig.json file.
fn update_zkconfig_rollup_config_hash(hash: B256, workspace_root: &PathBuf) -> Result<()> {
    let hash_str = format!("0x{:x}", hash);

    // Update zkconfig.json with the new hash
    let zkconfig_path = workspace_root.join("contracts/zkconfig.json").canonicalize()?;
    let zkconfig = if fs::metadata(&zkconfig_path).is_ok() {
        let mut config: Value = serde_json::from_str(&fs::read_to_string(&zkconfig_path)?)?;
        config["rollupConfigHash"] = json!(hash_str);

        // If the starting block number is not set, set it to the latest block number from the L2 RPC.
        if config["startingBlockNumber"].as_u64().unwrap_or(0) == 0 {
            // Get the latest block number from the L2 RPC.
            let sp1_kona_data_fetcher = OPSuccinctDataFetcher::default();
            let latest_block = block_on(sp1_kona_data_fetcher.get_head(ChainMode::L2))?;
            config["startingBlockNumber"] = json!(latest_block.number - 10);
        }

        config
    } else {
        json!({ "rollupConfigHash": hash_str })
    };

    fs::write(zkconfig_path, serde_json::to_string_pretty(&zkconfig)?)?;

    Ok(())
}

/// Merge the rollup and chain configs.
fn merge_configs(rollup: &Value, chain: &Value) -> Result<Value> {
    println!("rollup: {:?}", rollup);
    println!("chain: {:?}", chain);
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
