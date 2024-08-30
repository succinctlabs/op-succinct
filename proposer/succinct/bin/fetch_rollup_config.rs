use alloy_primitives::B256;
use anyhow::{bail, Context, Result};
use client_utils::boot::hash_rollup_config;
use serde_json::{json, Value};
use sp1_sdk::install::block_on;
use std::env;
use std::fs;

/// Fetch the rollup config from the rollup node and save it to a file.
fn fetch_rollup_config() -> Result<()> {
    let (rollup_rpc, l2_rpc) = get_rpc_urls();

    let rollup_config = fetch_rpc_data(&rollup_rpc, "optimism_rollupConfig")?;
    let chain_config = fetch_rpc_data(&l2_rpc, "debug_chainConfig")?;

    save_config_to_file("rollup-config.json", &rollup_config)?;
    save_config_to_file("chain-config.json", &chain_config)?;

    let rollup_json: Value = serde_json::from_str(&rollup_config)?;
    let chain_json: Value = serde_json::from_str(&chain_config)?;

    let merged_config = merge_configs(&rollup_json, &chain_json)?;
    let merged_config_str = serde_json::to_string_pretty(&merged_config)?;

    save_config_to_file("rollup-config.json", &merged_config_str)?;
    fs::remove_file("chain-config.json")?;

    println!("Updated rollup config saved to ./rollup-config.json");

    let hash: B256 = hash_rollup_config(&merged_config_str.as_bytes().to_vec());
    update_zkconfig_rollup_config_hash(hash)
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

fn save_config_to_file(filename: &str, content: &str) -> Result<()> {
    fs::write(filename, content)?;
    Ok(())
}

/// Update the rollup config hash in the zkconfig.json file.
fn update_zkconfig_rollup_config_hash(hash: B256) -> Result<()> {
    let hash_str = format!("0x{:x}", hash);

    // Update zkconfig.json with the new hash
    let zkconfig_path = "contracts/zkconfig.json";
    let zkconfig = if fs::metadata(zkconfig_path).is_ok() {
        let mut config: Value = serde_json::from_str(&fs::read_to_string(zkconfig_path)?)?;
        config["rollupConfigHash"] = json!(hash_str);
        config
    } else {
        json!({ "rollupConfigHash": hash_str })
    };

    fs::write(zkconfig_path, serde_json::to_string_pretty(&zkconfig)?)?;

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
    fetch_rollup_config()
}
