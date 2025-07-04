//! Anvil fork management utilities for E2E tests.

use std::{sync::Mutex, time::Duration};

use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_primitives::U256;
use alloy_provider::Provider;
use alloy_rpc_types_eth::BlockNumberOrTag;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use tracing::info;

use fault_proof::{L1Provider, L2Provider};

// An Anvil instance that is kept alive for the duration of the program.
lazy_static! {
    static ref ANVIL: Mutex<Option<AnvilInstance>> = Mutex::new(None);
}

/// Container for Anvil fork information
pub struct AnvilFork {
    pub provider: L1Provider,
    pub endpoint: String,
}

/// Setup an Anvil fork with the given configuration.
///
/// # Arguments
/// * `fork_url` - The RPC URL to fork from (e.g., Sepolia)
/// * `fork_block` - The block number to fork at
/// * `block_time` - Optional block time for auto-mining
///
/// Returns AnvilFork with provider and endpoint
pub async fn setup_anvil_fork(
    fork_url: &str,
    fork_block: u64,
    block_time: Option<Duration>,
) -> Result<AnvilFork> {
    info!("Starting Anvil fork from block {} on {}", fork_block, fork_url);

    // Create Anvil instance
    let mut anvil =
        Anvil::new().fork(fork_url).fork_block_number(fork_block).arg("--disable-code-size-limit");

    if let Some(bt) = block_time {
        anvil = anvil.block_time(bt.as_secs());
    } else {
        // Default to 1 second block time for faster tests
        anvil = anvil.block_time(1);
    }

    let anvil_instance = anvil.spawn();
    let endpoint = anvil_instance.endpoint();
    info!("Anvil fork started at: {}", endpoint);

    // Store the instance to keep it alive
    *ANVIL.lock().unwrap() = Some(anvil_instance);

    // Create provider
    let provider = L1Provider::new_http(endpoint.parse()?);

    Ok(AnvilFork { provider, endpoint: endpoint.to_string() })
}

/// Time manipulation utilities for the forked chain
pub async fn warp_time<P: Provider>(provider: &P, duration: Duration) -> Result<()> {
    let current_block = provider.get_block_number().await?;
    let current_timestamp = provider
        .get_block_by_number(current_block.into())
        .await?
        .context("Failed to get block")?
        .header
        .timestamp;

    let new_timestamp = current_timestamp + duration.as_secs();

    info!(
        "Warping time by {} seconds: {} -> {}",
        duration.as_secs(),
        current_timestamp,
        new_timestamp
    );

    // Use Anvil's time manipulation via direct RPC calls
    let client = provider.client();

    // evm_setNextBlockTimestamp returns null on success
    let _: Option<serde_json::Value> =
        client.request("evm_setNextBlockTimestamp", vec![serde_json::json!(new_timestamp)]).await?;

    // Mine a block to apply the timestamp
    let _: Option<String> = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;

    Ok(())
}

/// Mine a specific number of blocks
pub async fn _mine_blocks<P: Provider>(provider: &P, count: u64) -> Result<()> {
    info!("Mining {} blocks", count);
    let client = provider.client();

    for _ in 0..count {
        let _: Option<String> = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;
    }

    Ok(())
}

/// Create a snapshot of the current state
pub async fn _snapshot<P: Provider>(provider: &P) -> Result<U256> {
    let client = provider.client();
    let id: U256 = client.request("evm_snapshot", Vec::<serde_json::Value>::new()).await?;
    info!("Created snapshot with id: {}", id);
    Ok(id)
}

/// Revert to a previous snapshot
pub async fn _revert_to_snapshot<P: Provider>(provider: &P, snapshot_id: U256) -> Result<()> {
    let client = provider.client();
    let success: bool = client.request("evm_revert", vec![serde_json::json!(snapshot_id)]).await?;

    if !success {
        anyhow::bail!("Failed to revert to snapshot {}", snapshot_id);
    }
    info!("Reverted to snapshot {}", snapshot_id);
    Ok(())
}

/// Calculate the fork block based on L2 state.
/// This function determines the appropriate L1 block to fork from based on the L2 finalized block.
pub async fn _calculate_fork_block() -> Result<u64> {
    let l2_rpc_url = std::env::var("L2_RPC").context("L2_RPC must be set")?;
    let l2_provider = L2Provider::new_http(l2_rpc_url.parse()?);

    let l2_finalized = l2_provider
        .get_block_by_number(BlockNumberOrTag::Finalized)
        .await?
        .context("Failed to get L2 finalized block")?
        .header
        .number;

    // Buffer for 3 games with 100 block intervals
    let target_l2 = l2_finalized.saturating_sub(300);

    // Use the fetcher to get the safe L1 block for this L2 block
    let fetcher =
        op_succinct_host_utils::fetcher::OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let (_, l1_block_number) = fetcher
        .get_safe_l1_block_for_l2_block(target_l2)
        .await
        .context("Failed to get safe L1 block")?;

    Ok(l1_block_number)
}

/// Cleanup the Anvil instance
pub fn cleanup_anvil() {
    *ANVIL.lock().unwrap() = None;
}
