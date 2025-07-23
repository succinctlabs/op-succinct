use anyhow::Result;
use kona_genesis::RollupConfig;
use std::{env, fs, path::PathBuf};

/// Get the path to the rollup config file for the env.
pub fn get_rollup_config_path() -> Result<PathBuf> {
    let rollup_config_path = env::var("ROLLUP_CONFIG_PATH").expect("ROLLUP_CONFIG_PATH is not set");
    Ok(rollup_config_path.into())
}

/// Read rollup config from the rollup config file.
pub fn read_rollup_config() -> Result<RollupConfig> {
    let rollup_config_path = env::var("ROLLUP_CONFIG_PATH").expect("ROLLUP_CONFIG_PATH is not set");
    let rollup_config_str = fs::read_to_string(rollup_config_path)?;
    let rollup_config: RollupConfig = serde_json::from_str(&rollup_config_str)?;
    Ok(rollup_config)
}
