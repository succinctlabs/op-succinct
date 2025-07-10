//! Common test environment setup utilities.

use alloy_eips::BlockNumberOrTag;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use op_succinct_host_utils::fetcher::{get_rpcs_from_env, RPCConfig};
use tracing::info;

use fault_proof::{L2Provider, L2ProviderTrait};

use super::{
    anvil::{setup_anvil_fork, AnvilFork},
    contracts::{configure_contracts, deploy_test_contracts, DeployedContracts},
};

/// Common test environment setup
pub struct TestEnvironment {
    #[allow(dead_code)]
    pub rpc_config: RPCConfig,
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
        let mut rpc_config = get_rpcs_from_env();

        // Setup Anvil fork
        let anvil = setup_anvil_fork(&rpc_config.l1_rpc.to_string()).await?;

        // Create L2 provider
        let l2_provider = ProviderBuilder::default().connect_http(rpc_config.l2_rpc.clone());

        // Update RPC config with Anvil endpoint
        rpc_config.l1_rpc = Url::parse(&anvil.endpoint.clone())?;

        // Deploy contracts
        info!("\n=== Deploying Contracts ===");
        let deployed = deploy_test_contracts(anvil.provider.clone(), l2_provider.clone()).await?;
        configure_contracts(anvil.provider.clone(), &deployed).await?;
        info!("✓ Contracts deployed and configured");
        info!("  Factory: {}", deployed.factory);

        Ok(Self { rpc_config, anvil, deployed, l2_provider })
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
