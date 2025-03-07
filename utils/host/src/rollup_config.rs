use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use kona_genesis::RollupConfig;

use crate::fetcher::RunContext;

/// Get the path to the rollup config file for the given chain id.
pub fn get_rollup_config_path(l2_chain_id: u64, run_context: RunContext) -> Result<PathBuf> {
    match run_context {
        RunContext::Dev => {
            let workspace_root = cargo_metadata::MetadataCommand::new()
                .exec()
                .expect("Failed to get workspace root")
                .workspace_root;
            let rollup_config_path =
                workspace_root.join(format!("configs/{}/rollup.json", l2_chain_id));
            Ok(rollup_config_path.into())
        }
        RunContext::Docker => {
            let rollup_config_path =
                PathBuf::from(format!("/usr/local/configs/{}/rollup.json", l2_chain_id));
            Ok(rollup_config_path)
        }
    }
}

/// Read rollup config from the rollup config file.
pub fn read_rollup_config(l2_chain_id: u64, run_context: RunContext) -> Result<RollupConfig> {
    let rollup_config_path = get_rollup_config_path(l2_chain_id, run_context)?;
    let rollup_config_str = fs::read_to_string(rollup_config_path)?;
    let rollup_config: RollupConfig = serde_json::from_str(&rollup_config_str)?;
    Ok(rollup_config)
}
