//! Common test environment setup utilities.

use alloy_eips::BlockNumberOrTag;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use tracing::info;

use fault_proof::{L2Provider, L2ProviderTrait};

use super::{
    anvil::{setup_anvil_fork, AnvilFork},
    contracts::{configure_contracts, deploy_test_contracts, DeployedContracts},
};

/// Common test environment setup
pub struct TestEnvironment {
    #[allow(dead_code)]
    pub l1_rpc: String,
    pub l2_rpc: String,
    pub l2_node_rpc: String,
    pub l1_beacon_rpc: String,
    pub anvil: AnvilFork,
    pub deployed: DeployedContracts,
    pub l2_provider: L2Provider,
}

impl TestEnvironment {
    /// Initialize logging for tests
    pub fn init_logging() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .try_init();
    }

    /// Create a new test environment with common setup
    pub async fn setup() -> Result<Self> {
        // Get environment variables
        let l1_rpc = std::env::var("L1_RPC").expect("L1_RPC must be set");
        let l2_rpc = std::env::var("L2_RPC").expect("L2_RPC must be set");
        let l2_node_rpc = std::env::var("L2_NODE_RPC").unwrap_or_else(|_| l2_rpc.clone());
        let l1_beacon_rpc = std::env::var("L1_BEACON_RPC").expect("L1_BEACON_RPC must be set");

        // Setup Anvil fork
        let anvil = setup_anvil_fork(&l1_rpc).await?;

        // Create L2 provider
        let l2_provider =
            ProviderBuilder::default().connect_http(l2_rpc.clone().parse::<Url>().unwrap());

        // Deploy contracts
        info!("\n=== Deploying Contracts ===");
        let deployed = deploy_test_contracts(anvil.provider.clone(), l2_provider.clone()).await?;
        configure_contracts(anvil.provider.clone(), &deployed).await?;
        info!("✓ Contracts deployed and configured");
        info!("  Factory: {}", deployed.factory);

        Ok(Self { l1_rpc, l2_rpc, l2_node_rpc, l1_beacon_rpc, anvil, deployed, l2_provider })
    }

    /// Get the initial L2 block number for testing (finalized - 100)
    pub async fn get_initial_l2_block_number(&self) -> Result<u64> {
        Ok(self
            .l2_provider
            .get_l2_block_by_number(BlockNumberOrTag::Finalized)
            .await?
            .header
            .number -
            100)
    }
}
