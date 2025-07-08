//! Contract deployment utilities for E2E tests.

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;
use anyhow::{anyhow, Result};
use bindings::{
    access_manager::AccessManager, anchor_state_registry::AnchorStateRegistry,
    dispute_game_factory::DisputeGameFactory, erc1967_proxy::ERC1967Proxy,
    mock_optimism_portal2::MockOptimismPortal2,
    op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame, sp1_mock_verifier::SP1MockVerifier,
    superchain_config::SuperchainConfig,
};
use tracing::{debug, info};

use fault_proof::{L1Provider, L2Provider, L2ProviderTrait};

use super::constants::{
    AGGREGATION_VKEY, CHALLENGER_ADDRESS, CHALLENGER_BOND, DEPLOYER_ADDRESS,
    DISPUTE_GAME_FINALITY_DELAY_SECONDS, FALLBACK_TIMEOUT, INIT_BOND,
    L2_BLOCK_OFFSET_FROM_FINALIZED, MAX_CHALLENGE_DURATION, MAX_PROVE_DURATION, PROPOSER_ADDRESS,
    RANGE_VKEY_COMMITMENT, ROLLUP_CONFIG_HASH, TEST_GAME_TYPE,
};

/// Container for deployed contracts
pub struct DeployedContracts {
    pub factory: Address,
    pub portal: Address,
    pub access_manager: Address,
    pub game_implementation: Address,
}

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
    let init_data = DisputeGameFactory::initializeCall { _owner: DEPLOYER_ADDRESS }.abi_encode();

    let factory_proxy =
        ERC1967Proxy::deploy(provider.clone(), factory_impl_addr, init_data.into()).await?;
    let factory = *factory_proxy.address();
    info!("✓ DisputeGameFactory proxy deployed at: {}", factory);

    // 2. Deploy MockOptimismPortal2 (needed for AnchorStateRegistry)
    debug!("Deploying MockOptimismPortal2...");
    let portal_instance = MockOptimismPortal2::deploy(
        provider.clone(),
        TEST_GAME_TYPE,
        U256::from(DISPUTE_GAME_FINALITY_DELAY_SECONDS),
    )
    .await?;
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
    // NOTE: We use the finalized L2 block number - L2_BLOCK_OFFSET_FROM_FINALIZED as the starting
    // anchor root and test 3 games with proposal interval of 10 blocks.
    let l2_block_number = U256::from(
        l2_provider.get_l2_block_by_number(BlockNumberOrTag::Finalized).await?.header.number,
    ) - U256::from(L2_BLOCK_OFFSET_FROM_FINALIZED);
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
    // NOTE(fakedev9999): AccessManager deployment requires manual gas specification
    // because Anvil's gas estimator fails to properly handle the Ownable constructor
    // logic when running on a forked network.
    let access_manager_receipt =
        AccessManager::deploy_builder(provider.clone(), FALLBACK_TIMEOUT, factory)
            .gas(3_000_000)
            .send()
            .await?
            .get_receipt()
            .await?;
    let access_manager = access_manager_receipt
        .contract_address
        .ok_or(anyhow!("AccessManager deployment failed"))?;
    info!("✓ AccessManager deployed at: {}", access_manager);

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
    let tx = access_manager.setProposer(PROPOSER_ADDRESS, true).send().await?;
    tx.get_receipt().await?;
    info!("✓ Proposer permission set for {}", PROPOSER_ADDRESS);

    // Set challenger permission (Anvil account 1)
    let tx = access_manager.setChallenger(CHALLENGER_ADDRESS, true).send().await?;
    tx.get_receipt().await?;
    info!("✓ Challenger permission set for {}", CHALLENGER_ADDRESS);

    info!("✓ All contracts configured successfully");
    Ok(())
}
