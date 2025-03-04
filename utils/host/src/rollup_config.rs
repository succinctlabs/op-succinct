use anyhow::Result;
use kona_genesis::RollupConfig;
use std::fs;
use std::path::PathBuf;

/// Read rollup config from the rollup config file.
pub fn read_rollup_config(rollup_config_path: PathBuf) -> Result<RollupConfig> {
    let rollup_config_str = fs::read_to_string(rollup_config_path)?;
    let rollup_config: RollupConfig = serde_json::from_str(&rollup_config_str)?;
    Ok(rollup_config)
}
