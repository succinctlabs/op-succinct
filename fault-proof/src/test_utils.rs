use std::{sync::Mutex, time::Duration};

use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_primitives::Address;
use alloy_provider::{
    fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    ProviderBuilder, RootProvider,
};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use lazy_static::lazy_static;
use op_alloy_network::{Ethereum, EthereumWallet};

// An Anvil instance that is kept alive for the duration of the program.
lazy_static! {
    static ref ANVIL: Mutex<AnvilWrapper> = Mutex::new(AnvilWrapper(None));
}

/// Wrapper struct for AnvilInstance that implements Drop for cleanup.
struct AnvilWrapper(Option<AnvilInstance>);

impl Drop for AnvilWrapper {
    fn drop(&mut self) {
        if let Some(anvil) = self.0.take() {
            drop(anvil);
        }
    }
}

/// Guard struct to ensure Anvil cleanup when it goes out of scope
pub struct CleanupGuard;

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        cleanup_anvil();
    }
}

/// Test account constants (standard Anvil accounts).
pub const ANVIL_PRIVATE_KEY_0: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const ANVIL_PRIVATE_KEY_1: &str =
    "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

/// Get the endpoint of the Anvil instance.
pub fn get_anvil_endpoint() -> String {
    ANVIL.lock().unwrap().0.as_ref().unwrap().endpoint().to_string()
}

/// Connects to an existing Ethereum node, or creates and connects to a new local Anvil instance.
///
/// For debugging, it is recommended to run an external Anvil instance and pass in the `rpc_url` so
/// that logs persist after the tests are finished. Transactions can then be replayed using the
/// 'cast run' command.
///
/// Returns a tuple of (TestProvider, CleanupGuard) to ensure proper cleanup of resources.
pub async fn setup_eth_backend(
    rpc_url: Option<&str>,
    block_time: Option<Duration>,
) -> Result<(TestProvider, CleanupGuard)> {
    let endpoint: String;
    if let Some(rpc_url) = rpc_url {
        // Use the provided endpoint.
        endpoint = rpc_url.to_string();
    } else {
        // Initialize the Anvil instance and use it as the endpoint.
        initialize_anvil(block_time);
        endpoint = get_anvil_endpoint();
    }

    // Initialize the RPC client.
    let eth_client = ProviderBuilder::new().connect_http(endpoint.parse()?);

    Ok((eth_client, CleanupGuard))
}

/// Initialize the global Anvil instance.
pub fn initialize_anvil(block_time: Option<Duration>) {
    let mut anvil = ANVIL.lock().unwrap();
    if anvil.0.is_none() {
        anvil.0 = Some(create_anvil(block_time));
    }
}

/// Creates a local Anvil node.
///
/// When block time is given, new blocks are mined periodically. Otherwise, a new block is mined per
/// transaction.
pub fn create_anvil(block_time: Option<Duration>) -> AnvilInstance {
    let mut anvil = Anvil::new();

    if let Some(bt) = block_time {
        anvil = anvil.block_time(bt.as_secs());
    }
    // Let anvil choose an available port automatically
    anvil = anvil.arg("--disable-code-size-limit");

    anvil.spawn()
}

/// Cleans up the global Anvil instance by dropping it.
pub fn cleanup_anvil() {
    let mut anvil = ANVIL.lock().unwrap();
    anvil.0 = None;
}

// Type alias for the complex provider type
pub type TestProvider = FillProvider<
    JoinFill<
        alloy_provider::Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider<Ethereum>,
>;

/// Test configuration for fault proof tests.
pub struct TestConfig {
    pub deployer_signer: PrivateKeySigner,
    pub deployer_wallet: EthereumWallet,
    pub user_signer: PrivateKeySigner,
    pub factory_address: Address,
    pub l1_provider: TestProvider,
    pub _cleanup_guard: CleanupGuard,
}

/// Sets up a complete test environment with deployed contracts.
pub async fn setup_test_environment(
    rpc_url: Option<&str>,
    block_time: Option<Duration>,
) -> Result<TestConfig> {
    // Setup anvil provider.
    let (l1_provider, cleanup_guard) = setup_eth_backend(rpc_url, block_time).await?;

    // Initialize signers.
    let deployer_signer = ANVIL_PRIVATE_KEY_0.parse::<PrivateKeySigner>()?;
    let deployer_wallet = EthereumWallet::from(deployer_signer.clone());
    let user_signer = ANVIL_PRIVATE_KEY_1.parse::<PrivateKeySigner>()?;

    // Create provider with wallet for contract deployment.
    let _deployer_provider = ProviderBuilder::new()
        .wallet(deployer_wallet.clone())
        .connect_provider(l1_provider.clone());

    // TODO: Deploy DisputeGameFactory and other required contracts.
    // For now, we'll use a placeholder address.
    let factory_address = Address::ZERO;

    Ok(TestConfig {
        deployer_signer,
        deployer_wallet,
        user_signer,
        factory_address,
        l1_provider,
        _cleanup_guard: cleanup_guard,
    })
}
