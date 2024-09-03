use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;

use kona_host::HostCli;

/// Convert the HostCli to a vector of arguments that can be passed to a command.
pub fn convert_host_cli_to_args(host_cli: &HostCli) -> Vec<String> {
    let mut args = vec![
        format!("--l1-head={}", host_cli.l1_head),
        format!("--l2-head={}", host_cli.l2_head),
        format!("--l2-output-root={}", host_cli.l2_output_root),
        format!("--l2-claim={}", host_cli.l2_claim),
        format!("--l2-block-number={}", host_cli.l2_block_number),
        format!("--l2-chain-id={}", host_cli.l2_chain_id),
    ];
    if let Some(addr) = &host_cli.l2_node_address {
        args.push("--l2-node-address".to_string());
        args.push(addr.to_string());
    }
    if let Some(addr) = &host_cli.l1_node_address {
        args.push("--l1-node-address".to_string());
        args.push(addr.to_string());
    }
    if let Some(addr) = &host_cli.l1_beacon_address {
        args.push("--l1-beacon-address".to_string());
        args.push(addr.to_string());
    }
    if let Some(dir) = &host_cli.data_dir {
        args.push("--data-dir".to_string());
        args.push(dir.to_string_lossy().into_owned());
    }
    if let Some(exec) = &host_cli.exec {
        args.push("--exec".to_string());
        args.push(exec.to_string());
    }
    if host_cli.server {
        args.push("--server".to_string());
    }
    args
}

/// Run the native host with a timeout. Use a binary to execute the native host, as opposed to
/// spawning a new thread in the same process due to the static cursors employed by the host.
pub async fn run_witnessgen(host_cli: &HostCli, timeout_duration: Duration) -> Result<()> {
    let metadata =
        cargo_metadata::MetadataCommand::new().exec().expect("Failed to get cargo metadata");
    let target_dir =
        metadata.target_directory.join("native_host_runner/release/native_host_runner");
    let args = convert_host_cli_to_args(host_cli);

    // Run the native host runner.
    let mut child =
        tokio::process::Command::new(target_dir).args(&args).env("RUST_LOG", "info").spawn()?;

    // Set up a timeout that will kill the process if it exceeds the duration
    match tokio::time::timeout(timeout_duration, child.wait()).await {
        Ok(result) => {
            let result = result.unwrap();
            if result.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Process failed with exit code: {}", result.code().unwrap()))
            }
        }
        Err(_) => {
            // Timeout occurred, kill the process
            child.kill().await?;
            Err(anyhow::anyhow!("Process timed out and was killed."))
        }
    }
}

pub async fn run_parallel_witnessgen(
    host_clis: Vec<HostCli>,
    timeout_duration: Duration,
) -> Result<()> {
    let metadata =
        cargo_metadata::MetadataCommand::new().exec().expect("Failed to get cargo metadata");
    let target_dir =
        metadata.target_directory.join("native_host_runner/release/native_host_runner");

    let mut command_args = Vec::new();
    for host_cli in host_clis.clone() {
        let args = convert_host_cli_to_args(&host_cli);
        command_args.push(args);
    }

    let mut children = Vec::new();
    for args in command_args {
        let child = tokio::process::Command::new(&target_dir)
            .args(&args)
            .env("RUST_LOG", "info")
            .spawn()?;
        children.push(child);
    }

    let mut any_failed = false;
    for child in &mut children {
        match timeout(timeout_duration, child.wait()).await {
            Ok(Ok(status)) if !status.success() => {
                any_failed = true;
                break;
            }
            Ok(Err(e)) => {
                any_failed = true;
                eprintln!("Child process error: {}", e);
                break;
            }
            Err(_) => {
                any_failed = true;
                eprintln!("Child process timed out");
                break;
            }
            _ => {}
        }
    }

    if any_failed {
        // Terminate remaining children
        for mut child in children {
            let _ = child.kill().await?;
        }
        // Kill the spawned witness gen program.
        let binary_name = host_clis[0].exec.as_ref().unwrap().split('/').last().unwrap();
        std::process::Command::new("pkill").arg("-f").arg(binary_name).output()?;
        Err(anyhow::anyhow!("One or more child processes failed or timed out"))
    } else {
        Ok(())
    }
}
