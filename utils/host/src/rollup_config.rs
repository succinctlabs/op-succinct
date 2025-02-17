use anyhow::Result;
use op_alloy_genesis::RollupConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MantleEigenDaConfig {
    pub proxy_url: Option<String>,
    pub retrieve_timeout: Duration,
}

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

/// Read eigen da config from the env params.
pub fn get_eigen_da_config() -> Result<MantleEigenDaConfig> {
    let proxy_url = env::var("EIGEN_DA_PROXY_URL").expect("EIGEN_DA_PROXY_URL is not set");
    let retrieve_timeout_string =
        env::var("EIGEN_DA_RETRIEVE_TIMEOUT").unwrap_or_else(|_| "120".into());
    Ok(MantleEigenDaConfig {
        proxy_url: Some(proxy_url),
        retrieve_timeout: Duration::from_secs(retrieve_timeout_string.parse()?),
    })
}
