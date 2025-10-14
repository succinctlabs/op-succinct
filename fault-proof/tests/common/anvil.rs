//! Anvil fork management utilities for E2E tests.

use std::{sync::Mutex, time::Duration};

use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_provider::Provider;
use alloy_rpc_types_eth::BlockNumberOrTag;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCMode};
use serde_json::Value;
use tracing::info;

use fault_proof::L1Provider;

use super::constants::L2_BLOCK_OFFSET_FROM_FINALIZED;

// An Anvil instance that is kept alive for the duration of the program.
lazy_static! {
    static ref ANVIL: Mutex<Option<AnvilInstance>> = Mutex::new(None);
}

/// Container for Anvil fork information
pub struct AnvilFork {
    /// Provider for the forked chain
    pub provider: L1Provider,
    /// RPC URL for the forked chain
    pub endpoint: String,
    /// Starting l2 block number
    pub starting_l2_block_number: u64,
    /// Starting root
    pub starting_root: String,
}

/// Setup an Anvil fork with automatic fork block calculation.
///
/// # Arguments
/// * `fork_url` - The RPC URL to fork from (e.g., Sepolia)
///
/// Returns AnvilFork with provider and endpoint
pub async fn setup_anvil_fork(fork_url: &str) -> Result<AnvilFork> {
    // Calculate the appropriate fork block

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

    let starting_block_number_hex = format!("0x{target_l2:x}");
    let optimism_output_data: Value = fetcher
        .fetch_rpc_data_with_mode(
            RPCMode::L2Node,
            "optimism_outputAtBlock",
            vec![starting_block_number_hex.into()],
        )
        .await?;

    let starting_root = optimism_output_data["outputRoot"].as_str().unwrap().to_string();

    let (_, l1_block_number) = fetcher
        .get_safe_l1_block_for_l2_block(target_l2)
        .await
        .context("Failed to get safe L1 block")?;

    info!("Starting Anvil fork from block {} on {}", l1_block_number, fork_url);

    // Create Anvil instance
    let anvil = Anvil::new()
        .fork(fork_url)
        .fork_block_number(l1_block_number)
        .block_time(1)
        .arg("--disable-code-size-limit");

    let anvil_instance = anvil.spawn();
    let endpoint = anvil_instance.endpoint();
    info!("Anvil fork started at: {}", endpoint);

    // Store the instance to keep it alive
    *ANVIL.lock().unwrap() = Some(anvil_instance);

    // Create provider
    let provider = L1Provider::new_http(endpoint.parse()?);

    Ok(AnvilFork {
        provider,
        endpoint: endpoint.to_string(),
        starting_l2_block_number: target_l2,
        starting_root,
    })
}

/// Time manipulation utility for the forked chain
pub async fn warp_time<P: Provider>(provider: &P, duration: Duration) -> Result<()> {
    let seconds = duration.as_secs();
    info!("Warping time by {} seconds", seconds);

    let client = provider.client();

    // Advance the timestamp.
    let _: serde_json::Value =
        client.request("evm_increaseTime", vec![serde_json::json!(seconds)]).await?;

    // Mine a block to apply the timestamp
    let _: String = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;

    Ok(())
}
