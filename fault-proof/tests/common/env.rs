//! Common test environment setup utilities.

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use op_succinct_host_utils::fetcher::{get_rpcs_from_env, RPCConfig};
use tracing::info;

use fault_proof::{config::FaultDisputeGameConfig, L2Provider, L2ProviderTrait};

use crate::common::constants::{
    AGGREGATION_VKEY, CHALLENGER_ADDRESS, CHALLENGER_BOND, DEPLOYER_PRIVATE_KEY,
    DISPUTE_GAME_FINALITY_DELAY_SECONDS, FALLBACK_TIMEOUT, INIT_BOND, MAX_CHALLENGE_DURATION,
    MAX_PROVE_DURATION, PROPOSER_ADDRESS, RANGE_VKEY_COMMITMENT, ROLLUP_CONFIG_HASH,
    TEST_GAME_TYPE,
};

use super::{
    anvil::{setup_anvil_fork, AnvilFork},
    contracts::{deploy_test_contracts, DeployedContracts},
};

/// Common test environment setup
pub struct TestEnvironment {
    #[allow(dead_code)]
    pub rpc_config: RPCConfig,
    pub anvil: AnvilFork,
    pub deployed: DeployedContracts,
    pub l2_provider: L2Provider,
}

/// The test configuration, used for integration tests.
pub fn test_config(starting_l2_block_number: u64, starting_root: String) -> FaultDisputeGameConfig {
    FaultDisputeGameConfig {
        aggregation_vkey: AGGREGATION_VKEY.to_string(),
        challenger_addresses: vec![CHALLENGER_ADDRESS.to_string()],
        challenger_bond_wei: CHALLENGER_BOND.to::<u64>(),
        dispute_game_finality_delay_seconds: DISPUTE_GAME_FINALITY_DELAY_SECONDS,
        fallback_timeout_fp_secs: FALLBACK_TIMEOUT.to::<u64>(),
        game_type: TEST_GAME_TYPE,
        initial_bond_wei: INIT_BOND.to::<u64>(),
        max_challenge_duration: MAX_CHALLENGE_DURATION,
        max_prove_duration: MAX_PROVE_DURATION,
        optimism_portal2_address: Address::ZERO.to_string(),
        permissionless_mode: false,
        proposer_addresses: vec![PROPOSER_ADDRESS.to_string()],
        range_vkey_commitment: RANGE_VKEY_COMMITMENT.to_string(),
        rollup_config_hash: ROLLUP_CONFIG_HASH.to_string(),
        starting_l2_block_number,
        starting_root,
        use_sp1_mock_verifier: true,
        verifier_address: Address::ZERO.to_string(),
    }
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

        // Put the test config into ../contracts/opsuccinctfdgconfig.json

        let test_config: FaultDisputeGameConfig =
            test_config(anvil.starting_l2_block_number, anvil.starting_root.clone());
        let json = serde_json::to_string_pretty(&test_config)?;
        std::fs::write("../contracts/opsuccinctfdgconfig.json", json)?;

        // Create L2 provider
        let l2_provider = ProviderBuilder::default().connect_http(rpc_config.l2_rpc.clone());

        // Update RPC config with Anvil endpoint
        rpc_config.l1_rpc = Url::parse(&anvil.endpoint.clone())?;

        // Deploy contracts
        info!("\n=== Deploying Contracts ===");
        let deployed = deploy_test_contracts(&anvil.endpoint, DEPLOYER_PRIVATE_KEY).await?;
        info!("✓ Contracts deployed");
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
