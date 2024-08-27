use alloy_primitives::B256;
use anyhow::{bail, Context, Result};
use client_utils::boot::hash_rollup_config;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::process::Command;

fn fetch_rollup_config(rollup_rpc: Option<String>, l2_rpc: Option<String>) -> Result<()> {
    // Determine RPC URLs
    let rollup_rpc = rollup_rpc
        .or_else(|| env::var("ROLLUP_RPC").ok())
        .context("Must provide rollup rpc as argument or env variable (ROLLUP_RPC)")?;

    let l2_rpc = l2_rpc
        .or_else(|| env::var("L2_RPC").ok())
        .context("Must provide L2 rpc as argument or env variable (L2_RPC)")?;

    // Fetch rollup config
    let rollup_config = Command::new("cast")
        .args(["rpc", "--rpc-url", &rollup_rpc, "optimism_rollupConfig"])
        .output()?;
    fs::write("rollup-config.json", &rollup_config.stdout)?;

    // Fetch chain config
    let chain_config = Command::new("cast")
        .args(["rpc", "--rpc-url", &l2_rpc, "debug_chainConfig"])
        .output()?;
    fs::write("chain-config.json", &chain_config.stdout)?;

    // Read and parse JSON files
    let rollup_json: Value = serde_json::from_str(&fs::read_to_string("rollup-config.json")?)?;
    let chain_json: Value = serde_json::from_str(&fs::read_to_string("chain-config.json")?)?;

    // Process and merge configs
    let merged_config = merge_configs(&rollup_json, &chain_json)?;

    // Write merged config to file
    let merged_config_str = serde_json::to_string_pretty(&merged_config)?;
    fs::write("rollup-config.json", &merged_config_str)?;

    // Clean up
    fs::remove_file("chain-config.json")?;

    println!("Updated rollup config saved to ./rollup-config.json");

    // Generate hash
    let hash: B256 = hash_rollup_config(&merged_config_str.as_bytes().to_vec());
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

fn merge_configs(rollup: &Value, chain: &Value) -> Result<Value> {
    let elasticity = chain["optimism"]["eip1559Elasticity"]
        .as_u64()
        .context("Missing eip1559Elasticity in chain config")?;
    let denominator = chain["optimism"]["eip1559Denominator"]
        .as_u64()
        .context("Missing eip1559Denominator in chain config")?;

    let mut merged = rollup.clone();

    // Merge time fields
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

    // Set base_fee_params
    merged["base_fee_params"] = json!({
        "elasticity_multiplier": elasticity,
        "max_change_denominator": denominator
    });

    // Set canyon_base_fee_params if present
    if let Some(canyon_denominator) = chain["optimism"]["eip1559DenominatorCanyon"].as_u64() {
        merged["canyon_base_fee_params"] = json!({
            "elasticity_multiplier": elasticity,
            "max_change_denominator": canyon_denominator
        });
    }

    // Rename batcherAddr to batcherAddress
    if let Some(system_config) = merged["genesis"]["system_config"].as_object_mut() {
        if let Some(batcher_addr) = system_config.remove("batcherAddr") {
            system_config.insert("batcherAddress".to_string(), batcher_addr);
        } else {
            bail!("Missing batcherAddr in rollup config");
        }
    } else {
        bail!("Invalid structure in rollup config: missing system_config");
    }

    Ok(merged)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let rollup_rpc = args.get(1).cloned();
    let l2_rpc = args.get(2).cloned();

    fetch_rollup_config(rollup_rpc, l2_rpc)
}
