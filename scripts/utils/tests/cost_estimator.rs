//! Exercise the cost estimator binary with a cached witness that panics in the guest.

use std::{fs, process::Command};

use anyhow::Result;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use sp1_sdk::SP1Stdin;

#[tokio::test]
async fn cost_estimator_rejects_invalid_cached_witness() -> Result<()> {
    let fetcher = OPSuccinctDataFetcher::new();
    let chain_id = fetcher.get_l2_chain_id().await?;
    let dir = tempfile::tempdir()?;
    // Isolate the cache and execution report from other commands and test runs.
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"[package]
name = "cost-estimator-fixture"
version = "0.0.0"
[workspace]
[lib]
path = "lib.rs"
"#,
    )?;
    fs::write(dir.path().join("lib.rs"), "")?;
    let cache_dir = dir.path().join(format!("data/{chain_id}/witness-cache"));
    fs::create_dir_all(&cache_dir)?;
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&[]);
    fs::write(cache_dir.join("1-6-stdin.bin"), bincode::serialize(&stdin)?)?;

    let output = Command::new(env!("CARGO_BIN_EXE_cost-estimator"))
        .current_dir(dir.path())
        .args(["--start", "1", "--end", "6", "--no-safe-head-split", "--cache"])
        .output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to deserialize witness data"), "{stderr}");
    assert!(!output.status.success(), "Estimator accepted a panicking guest: {stderr}");
    assert!(stderr.contains("Range program exited with code 1"), "{stderr}");
    Ok(())
}
