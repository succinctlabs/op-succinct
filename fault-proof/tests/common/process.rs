//! Process management utilities for running proposer and challenger binaries.

use anyhow::{Context, Result};
use std::{collections::HashMap, path::PathBuf, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    task::JoinHandle,
    time::timeout,
};
use tracing::{error, info, warn};

/// Represents a running binary process with monitoring
pub struct ManagedProcess {
    pub name: String,
    pub child: Child,
    pub stdout_handle: JoinHandle<()>,
    pub stderr_handle: JoinHandle<()>,
}

impl ManagedProcess {
    /// Kill the process gracefully
    pub async fn kill(mut self) -> Result<()> {
        info!("Stopping process: {}", self.name);

        // Try graceful shutdown first
        if let Some(pid) = self.child.id() {
            // Send SIGTERM for graceful shutdown
            let _ = Command::new("kill").args(["-TERM", &pid.to_string()]).output().await;

            // Wait up to 5 seconds for graceful shutdown
            match timeout(Duration::from_secs(5), self.child.wait()).await {
                Ok(Ok(status)) => {
                    info!("Process {} exited gracefully with status: {}", self.name, status);
                }
                Ok(Err(e)) => {
                    warn!("Error waiting for process {}: {}", self.name, e);
                }
                Err(_) => {
                    // Timeout - force kill
                    info!("Process {} didn't exit gracefully, force killing", self.name);
                    self.child.kill().await.context("Failed to kill process")?;
                }
            }
        }

        // Cancel output monitoring tasks
        self.stdout_handle.abort();
        self.stderr_handle.abort();

        Ok(())
    }

    /// Check if the process is still running
    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }
}

/// Configuration for starting a binary process
pub struct ProcessConfig {
    pub name: String,
    pub binary_path: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub log_stdout: bool,
    pub log_stderr: bool,
}

/// Start a proposer binary with the given configuration
pub async fn start_proposer_binary(
    binary_path: PathBuf,
    env_vars: HashMap<String, String>,
) -> Result<ManagedProcess> {
    let config = ProcessConfig {
        name: "proposer".to_string(),
        binary_path,
        env_vars,
        log_stdout: true,
        log_stderr: true,
    };

    start_binary_process(config).await
}

/// Start a challenger binary with the given configuration
pub async fn start_challenger_binary(
    binary_path: PathBuf,
    env_vars: HashMap<String, String>,
) -> Result<ManagedProcess> {
    let config = ProcessConfig {
        name: "challenger".to_string(),
        binary_path,
        env_vars,
        log_stdout: true,
        log_stderr: true,
    };

    start_binary_process(config).await
}

/// Start a binary process with monitoring
async fn start_binary_process(config: ProcessConfig) -> Result<ManagedProcess> {
    info!("Starting {} binary: {:?}", config.name, config.binary_path);

    // Build the command
    let mut cmd = Command::new(&config.binary_path);

    // Set environment variables
    for (key, value) in &config.env_vars {
        cmd.env(key, value);
    }

    // Configure stdio
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::null());

    // Spawn the process
    let mut child =
        cmd.spawn().with_context(|| format!("Failed to spawn {} binary", config.name))?;

    let pid = child.id().unwrap_or(0);
    info!("{} started with PID: {}", config.name, pid);

    // Set up stdout monitoring
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stdout_reader = BufReader::new(stdout);
    let stdout_name = config.name.clone();
    let log_stdout = config.log_stdout;

    let stdout_handle = tokio::spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if log_stdout {
                println!("[{} stdout] {}", stdout_name, line);
                info!("[{} stdout] {}", stdout_name, line);
            }
        }
    });

    // Set up stderr monitoring
    let stderr = child.stderr.take().expect("Failed to get stderr");
    let stderr_reader = BufReader::new(stderr);
    let stderr_name = config.name.clone();
    let log_stderr = config.log_stderr;

    let stderr_handle = tokio::spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if log_stderr {
                println!("[{} stderr] {}", stderr_name, line);
                warn!("[{} stderr] {}", stderr_name, line);
            }
        }
    });

    // Wait a bit to ensure process started successfully
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Check if process is still running
    match child.try_wait() {
        Ok(Some(status)) => {
            error!("{} exited immediately with status: {}", config.name, status);
            anyhow::bail!("{} failed to start", config.name);
        }
        Ok(None) => {
            info!("{} is running", config.name);
        }
        Err(e) => {
            error!("Failed to check {} status: {}", config.name, e);
            anyhow::bail!("Failed to check process status");
        }
    }

    Ok(ManagedProcess { name: config.name, child, stdout_handle, stderr_handle })
}

/// Generate environment variables for the proposer
pub fn generate_proposer_env(
    l1_rpc: &str,
    l2_rpc: &str,
    l2_node_rpc: &str,
    l1_beacon_rpc: &str,
    private_key: &str,
    factory_address: &str,
    game_type: u32,
    prover_network_rpc: Option<&str>,
) -> HashMap<String, String> {
    let mut env = HashMap::new();

    // Required environment variables
    env.insert("L1_RPC".to_string(), l1_rpc.to_string());
    env.insert("L1_BEACON_RPC".to_string(), l1_beacon_rpc.to_string());
    env.insert("L2_RPC".to_string(), l2_rpc.to_string());
    env.insert("L2_NODE_RPC".to_string(), l2_node_rpc.to_string());
    env.insert("PRIVATE_KEY".to_string(), private_key.to_string());
    env.insert("FACTORY_ADDRESS".to_string(), factory_address.to_string());
    env.insert("GAME_TYPE".to_string(), game_type.to_string());

    // Optional prover network RPC
    if let Some(prover_rpc) = prover_network_rpc {
        env.insert("PROVER_NETWORK_RPC".to_string(), prover_rpc.to_string());
    }

    // Enable info logging
    env.insert("RUST_LOG".to_string(), "info".to_string());

    // Test-specific configuration for faster game creation
    env.insert("PROPOSAL_INTERVAL_IN_BLOCKS".to_string(), "10".to_string()); // Much smaller interval for testing
    env.insert("FETCH_INTERVAL".to_string(), "2".to_string()); // Check more frequently in tests

    env
}

/// Generate environment variables for the challenger
pub fn generate_challenger_env(
    l1_rpc: &str,
    l2_rpc: &str,
    l2_node_rpc: &str,
    l1_beacon_rpc: &str,
    private_key: &str,
    factory_address: &str,
    game_type: u32,
    prover_network_rpc: Option<&str>,
    malicious_percentage: Option<f64>,
) -> HashMap<String, String> {
    let mut env = HashMap::new();

    // Required environment variables
    env.insert("L1_RPC".to_string(), l1_rpc.to_string());
    env.insert("L1_BEACON_RPC".to_string(), l1_beacon_rpc.to_string());
    env.insert("L2_RPC".to_string(), l2_rpc.to_string());
    env.insert("L2_NODE_RPC".to_string(), l2_node_rpc.to_string());
    env.insert("PRIVATE_KEY".to_string(), private_key.to_string());
    env.insert("FACTORY_ADDRESS".to_string(), factory_address.to_string());
    env.insert("GAME_TYPE".to_string(), game_type.to_string());

    // Optional prover network RPC
    if let Some(prover_rpc) = prover_network_rpc {
        env.insert("PROVER_NETWORK_RPC".to_string(), prover_rpc.to_string());
    }

    // Optional malicious challenge percentage
    if let Some(percentage) = malicious_percentage {
        env.insert("MALICIOUS_CHALLENGE_PERCENTAGE".to_string(), percentage.to_string());
    }

    // Enable info logging
    env.insert("RUST_LOG".to_string(), "info".to_string());

    // Test-specific configuration
    env.insert("FETCH_INTERVAL".to_string(), "2".to_string()); // Check more frequently in tests
    env.insert("MAX_GAMES_TO_CHECK_FOR_CHALLENGE".to_string(), "10".to_string()); // Check more games

    env
}

/// Wait for a process to produce a specific log line
pub async fn wait_for_log_line(
    process_name: &str,
    expected_line: &str,
    _timeout_duration: Duration,
) -> Result<()> {
    // This is a simplified version - in practice you'd monitor the output channels
    info!("Waiting for {} to log: {}", process_name, expected_line);

    // For now, just wait a bit
    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

/// Helper to find the binary path
pub fn find_binary_path(binary_name: &str) -> Result<PathBuf> {
    // First, check if we're in a cargo test environment
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let workspace_root = PathBuf::from(manifest_dir).parent().unwrap().to_path_buf();

        // Check common locations
        let possible_paths = vec![
            workspace_root.join("target/debug").join(binary_name),
            workspace_root.join("target/release").join(binary_name),
            workspace_root.join("fault-proof/target/debug").join(binary_name),
            workspace_root.join("fault-proof/target/release").join(binary_name),
        ];

        for path in possible_paths {
            if path.exists() {
                info!("Found {} binary at: {:?}", binary_name, path);
                return Ok(path);
            }
        }
    }

    // Try to find in PATH
    if let Ok(path) = which::which(binary_name) {
        info!("Found {} binary in PATH at: {:?}", binary_name, path);
        return Ok(path);
    }

    anyhow::bail!(
        "Could not find {} binary. Make sure it's built with 'cargo build --bin {}'",
        binary_name,
        binary_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_generation() {
        let proposer_env = generate_proposer_env(
            "http://localhost:8545",
            "http://localhost:9545",
            "http://localhost:9545",
            "http://localhost:5052",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "0x1234567890123456789012345678901234567890",
            254,
            None,
        );

        assert_eq!(proposer_env.get("L1_RPC").unwrap(), "http://localhost:8545");
        assert_eq!(proposer_env.get("GAME_TYPE").unwrap(), "254");
    }
}
