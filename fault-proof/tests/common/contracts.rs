//! Contract deployment utilities for E2E tests.

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{address, Address, FixedBytes, U256};
use alloy_sol_types::SolCall;
use anyhow::Result;
use bindings::{
    access_manager::AccessManager, anchor_state_registry::AnchorStateRegistry,
    dispute_game_factory::DisputeGameFactory, erc1967_proxy::ERC1967Proxy,
    mock_optimism_portal2::MockOptimismPortal2,
    op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame, sp1_mock_verifier::SP1MockVerifier,
    superchain_config::SuperchainConfig,
};
use tracing::{debug, info};

use fault_proof::{L1Provider, L2Provider, L2ProviderTrait};

/// Container for deployed contracts
pub struct DeployedContracts {
    pub factory: Address,
    pub portal: Address,
    pub access_manager: Address,
    pub game_implementation: Address,
}

/// Test configuration constants
pub const TEST_GAME_TYPE: u32 = 42; // Must match OP_SUCCINCT_FAULT_DISPUTE_GAME_TYPE in contracts
pub const INIT_BOND: U256 = U256::from_limbs([10_000_000_000_000_000, 0, 0, 0]); // 0.01 ETH
pub const CHALLENGER_BOND: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]); // 1 ETH

// Time constants
pub const AIRGAP: u64 = 60 * 60 * 24 * 7; // 7 days
pub const MAX_CHALLENGE_DURATION: u64 = 60 * 60; // 1 hour
pub const MAX_PROVE_DURATION: u64 = 60 * 60 * 12; // 12 hours
pub const FALLBACK_TIMEOUT: U256 = U256::from_limbs([1209600, 0, 0, 0]); // 2 weeks

// Configuration hashes for OPSuccinctFaultDisputeGame
pub const ROLLUP_CONFIG_HASH: FixedBytes<32> = FixedBytes::ZERO; // Mock value for testing
pub const AGGREGATION_VKEY: FixedBytes<32> = FixedBytes::ZERO; // Mock value for testing
pub const RANGE_VKEY_COMMITMENT: FixedBytes<32> = FixedBytes::ZERO; // Mock value for testing

/// Deploy all contracts required for E2E testing
pub async fn deploy_test_contracts(
    provider: L1Provider,
    l2_provider: L2Provider,
) -> Result<DeployedContracts> {
    info!("Deploying all contracts for E2E testing");

    // 1. Deploy DisputeGameFactory as proxy (matching production pattern)
    debug!("Deploying DisputeGameFactory implementation...");
    let factory_impl = DisputeGameFactory::deploy(provider.clone()).await?;
    let factory_impl_addr = *factory_impl.address();
    info!("✓ DisputeGameFactory implementation deployed at: {}", factory_impl_addr);

    // Deploy factory proxy with initialization
    debug!("Deploying DisputeGameFactory proxy...");
    let deployer = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"); // Anvil account 0
    let init_data = DisputeGameFactory::initializeCall { _owner: deployer }.abi_encode();

    let factory_proxy =
        ERC1967Proxy::deploy(provider.clone(), factory_impl_addr, init_data.into()).await?;
    let factory = *factory_proxy.address();
    info!("✓ DisputeGameFactory proxy deployed at: {}", factory);

    // 2. Deploy MockOptimismPortal2 (needed for AnchorStateRegistry)
    debug!("Deploying MockOptimismPortal2...");
    let portal_instance =
        MockOptimismPortal2::deploy(provider.clone(), TEST_GAME_TYPE, U256::from(AIRGAP)).await?;
    let portal = *portal_instance.address();
    info!("✓ MockOptimismPortal2 deployed at: {}", portal);

    // 3. Deploy AnchorStateRegistry as proxy (before AccessManager)
    debug!("Deploying SuperchainConfig...");
    let superchain_config = SuperchainConfig::deploy(provider.clone()).await?;
    let superchain_config_addr = *superchain_config.address();
    info!("✓ SuperchainConfig deployed at: {}", superchain_config_addr);

    // Deploy AnchorStateRegistry implementation
    debug!("Deploying AnchorStateRegistry implementation...");
    let anchor_impl = AnchorStateRegistry::deploy(provider.clone()).await?;
    let anchor_impl_addr = *anchor_impl.address();
    info!("✓ AnchorStateRegistry implementation deployed at: {}", anchor_impl_addr);

    // Prepare initialization data with expected test values
    // NOTE: We use the finalized L2 block number - 100 as the starting anchor root and test 3 games
    // with proposal interval of 10 blocks.
    let l2_block_number = U256::from(
        l2_provider.get_l2_block_by_number(BlockNumberOrTag::Finalized).await?.header.number,
    ) - U256::from(100u64);
    let output_root = l2_provider.compute_output_root_at_block(l2_block_number).await?;
    let starting_anchor_root = bindings::anchor_state_registry::AnchorStateRegistry::OutputRoot {
        root: output_root,
        l2BlockNumber: l2_block_number,
    };

    let init_data = bindings::anchor_state_registry::AnchorStateRegistry::initializeCall {
        _superchainConfig: superchain_config_addr,
        _disputeGameFactory: factory,
        _portal: portal,
        _startingAnchorRoot: starting_anchor_root,
    }
    .abi_encode();

    // Deploy AnchorStateRegistry proxy
    debug!("Deploying AnchorStateRegistry proxy...");
    let anchor_proxy =
        ERC1967Proxy::deploy(provider.clone(), anchor_impl_addr, init_data.into()).await?;
    let anchor_state_registry = *anchor_proxy.address();
    info!("✓ AnchorStateRegistry proxy deployed at: {}", anchor_state_registry);

    // 4. Deploy AccessManager
    debug!("Deploying AccessManager...");
    // Note: AccessManager deployment may fail on Anvil forks, so we handle it gracefully
    let access_manager = match AccessManager::deploy_builder(
        provider.clone(),
        FALLBACK_TIMEOUT,
        factory,
    )
    .gas(3_000_000)
    .send()
    .await
    {
        Ok(pending_tx) => match pending_tx.get_receipt().await {
            Ok(receipt) => {
                if receipt.status() {
                    if let Some(addr) = receipt.contract_address {
                        info!("✓ AccessManager deployed at: {}", addr);
                        addr
                    } else {
                        info!("⚠️ AccessManager deployment succeeded but no contract address returned");
                        Address::ZERO
                    }
                } else {
                    info!("⚠️ AccessManager deployment reverted (transaction status: false)");
                    Address::ZERO
                }
            }
            Err(e) => {
                info!("⚠️ Failed to get AccessManager deployment receipt: {}", e);
                Address::ZERO
            }
        },
        Err(e) => {
            info!("⚠️ Failed to send AccessManager deployment transaction: {}", e);
            Address::ZERO
        }
    };

    // 5. Deploy SP1MockVerifier
    debug!("Deploying SP1MockVerifier...");
    let verifier_instance = SP1MockVerifier::deploy(provider.clone()).await?;
    let verifier = *verifier_instance.address();
    info!("✓ SP1MockVerifier deployed at: {}", verifier);

    // 6. Deploy OPSuccinctFaultDisputeGame implementation
    debug!("Deploying OPSuccinctFaultDisputeGame...");
    let game_implementation_instance = OPSuccinctFaultDisputeGame::deploy(
        provider.clone(),
        MAX_CHALLENGE_DURATION,
        MAX_PROVE_DURATION,
        factory,  // factory address
        verifier, // SP1 verifier
        ROLLUP_CONFIG_HASH,
        AGGREGATION_VKEY,
        RANGE_VKEY_COMMITMENT,
        CHALLENGER_BOND,
        anchor_state_registry,
        access_manager,
    )
    .await?;
    let game_implementation = *game_implementation_instance.address();
    info!("✓ OPSuccinctFaultDisputeGame deployed at: {}", game_implementation);

    Ok(DeployedContracts { factory, portal, access_manager, game_implementation })
}

/// Configure contracts after deployment
pub async fn configure_contracts(
    provider: L1Provider,
    contracts: &DeployedContracts,
) -> Result<()> {
    info!("Configuring contracts for E2E testing");

    // Only configure if all contracts were deployed
    if contracts.access_manager == Address::ZERO {
        info!("⚠️  Skipping contract configuration - not all contracts deployed");
        return Ok(());
    }

    // 1. Configure MockOptimismPortal2 - Set respected game type
    info!("Configuring MockOptimismPortal2...");
    let portal = MockOptimismPortal2::new(contracts.portal, provider.clone());
    let tx = portal.setRespectedGameType(TEST_GAME_TYPE).send().await?;
    tx.get_receipt().await?;
    info!("✓ Respected game type set to {}", TEST_GAME_TYPE);

    // 2. Configure DisputeGameFactory
    info!("Configuring DisputeGameFactory...");
    let factory = DisputeGameFactory::new(contracts.factory, provider.clone());

    // Set game implementation
    let tx =
        factory.setImplementation(TEST_GAME_TYPE, contracts.game_implementation).send().await?;
    tx.get_receipt().await?;
    info!("✓ Game implementation set for type {}", TEST_GAME_TYPE);

    // Set init bond
    let tx = factory.setInitBond(TEST_GAME_TYPE, INIT_BOND).send().await?;
    tx.get_receipt().await?;
    info!("✓ Init bond set to {}", INIT_BOND);

    // 3. Configure AccessManager
    info!("Configuring AccessManager...");
    let access_manager = AccessManager::new(contracts.access_manager, provider.clone());

    // Set proposer permission (Anvil account 0)
    let proposer_address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let tx = access_manager.setProposer(proposer_address, true).send().await?;
    tx.get_receipt().await?;
    info!("✓ Proposer permission set for {}", proposer_address);

    // Set challenger permission (Anvil account 1)
    let challenger_address = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let tx = access_manager.setChallenger(challenger_address, true).send().await?;
    tx.get_receipt().await?;
    info!("✓ Challenger permission set for {}", challenger_address);

    // The AnchorStateRegistry is already initialized with a starting anchor state
    // during deployment (starting at finalized L2 block - 30 with output root at the block).
    info!("✓ AnchorStateRegistry already configured with initial anchor state");

    info!("✓ All contracts configured successfully");
    Ok(())
}

/// Create a game through the factory (simplified version for testing)
#[allow(dead_code)]
pub async fn create_test_game(
    provider: L1Provider,
    factory_address: Address,
    game_type: u32,
    l2_block_number: U256,
    output_root: FixedBytes<32>,
) -> Result<Address> {
    let factory = DisputeGameFactory::new(factory_address, provider.clone());

    // First check if game is registered
    let game_impl = factory.gameImpls(game_type).call().await?;
    if game_impl == Address::ZERO {
        anyhow::bail!("Game type {} not registered in factory", game_type);
    }

    // Get init bond
    let init_bond = factory.initBonds(game_type).call().await?;
    info!("Init bond for game type {}: {}", game_type, init_bond);

    // Encode extra data (l2 block number and parent game index)
    let extra_data = alloy_sol_types::SolValue::abi_encode_packed(&(l2_block_number, u32::MAX));

    // Create the game
    let tx =
        factory.create(game_type, output_root, extra_data.into()).value(init_bond).send().await?;

    let receipt = tx.get_receipt().await?;
    info!("✓ Game created in tx: {}", receipt.transaction_hash);

    // Get game address from the latest game
    let game_count = factory.gameCount().call().await?;
    let game_index = game_count - U256::from(1);
    let game_info = factory.gameAtIndex(game_index).call().await?;
    let game_address = game_info.proxy_;

    info!("✓ Game deployed at: {}", game_address);
    Ok(game_address)
}
