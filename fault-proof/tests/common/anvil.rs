//! Anvil fork management utilities for E2E tests.

use std::{sync::Mutex, time::Duration};

use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_provider::Provider;
use alloy_rpc_types_eth::BlockNumberOrTag;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use tracing::info;

use fault_proof::L1Provider;

use super::constants::L2_BLOCK_OFFSET_FROM_FINALIZED;

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

    // Create Anvil instance
    let anvil =
        Anvil::new().fork(fork_url).fork_block_number(fork_block).arg("--disable-code-size-limit");

    let anvil_instance = anvil.spawn();
    let endpoint = anvil_instance.endpoint();
    info!("Anvil fork started at: {}", endpoint);

    // Store the instance to keep it alive
    *ANVIL.lock().unwrap() = Some(anvil_instance);

    // Create provider
    let provider = L1Provider::new_http(endpoint.parse()?);

    Ok(AnvilFork { provider, endpoint: endpoint.to_string() })
}

/// Time manipulation utility for the forked chain
pub async fn warp_time<P: Provider>(provider: &P, duration: Duration) -> Result<()> {
    let seconds = duration.as_secs();
    info!("Warping time by {} seconds", seconds);

    let client = provider.client();

    // Use evm_increaseTime which is simpler than calculating timestamps manually
    let _: serde_json::Value =
        client.request("evm_increaseTime", vec![serde_json::json!(seconds)]).await?;

    // Mine a block to apply the timestamp
    let _: String = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;

    Ok(())
}

/// Calculate the fork block based on L2 state.
/// This function determines the appropriate L1 block to fork from based on the L2 finalized block.
async fn calculate_fork_block() -> Result<u64> {
    let fetcher = OPSuccinctDataFetcher::new();

    let l2_finalized = fetcher
        .l2_provider
        .get_block_by_number(BlockNumberOrTag::Finalized)
        .await?
        .context("Failed to get L2 finalized block")?
        .header
        .number;

    // Use finalized - L2_BLOCK_OFFSET_FROM_FINALIZED for testing
    let target_l2 = l2_finalized.saturating_sub(L2_BLOCK_OFFSET_FROM_FINALIZED);

    let (_, l1_block_number) = fetcher
        .get_safe_l1_block_for_l2_block(target_l2)
        .await
        .context("Failed to get safe L1 block")?;

    Ok(l1_block_number)
}
