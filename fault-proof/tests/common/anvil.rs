//! Anvil fork management utilities for E2E tests.

use std::{sync::Mutex, time::Duration};

use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_provider::Provider;
use alloy_rpc_types_eth::BlockNumberOrTag;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
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

/// Setup an Anvil fork with automatic fork block calculation.
///
/// # Arguments
/// * `fork_url` - The RPC URL to fork from (e.g., Sepolia)
///
/// Returns AnvilFork with provider and endpoint
pub async fn setup_anvil_fork(fork_url: &str) -> Result<AnvilFork> {
    // Calculate the appropriate fork block
    let fork_block = calculate_fork_block().await?;

    info!("Starting Anvil fork from block {} on {}", fork_block, fork_url);

    // Create Anvil instance with 1 second block time as default
    let anvil = Anvil::new()
        .fork(fork_url)
        .fork_block_number(fork_block)
        .arg("--disable-code-size-limit")
        .block_time(1);

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

/// Calculate the fork block based on L2 state.
/// This function determines the appropriate L1 block to fork from based on the L2 finalized block.
async fn calculate_fork_block() -> Result<u64> {
    let l2_rpc_url = std::env::var("L2_RPC").context("L2_RPC must be set")?;
    let l2_provider = L2Provider::new_http(l2_rpc_url.parse()?);

    let l2_finalized = l2_provider
        .get_block_by_number(BlockNumberOrTag::Finalized)
        .await?
        .context("Failed to get L2 finalized block")?
        .header
        .number;

    // Use finalized - 100 for testing
    let target_l2 = l2_finalized.saturating_sub(100);

    // Use the fetcher to get the safe L1 block for this L2 block
    let fetcher = OPSuccinctDataFetcher::new();

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
