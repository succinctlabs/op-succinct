mod common;

use std::str::FromStr;

use alloy_network::EthereumWallet;
use alloy_primitives::{FixedBytes, U256};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use fault_proof::{contract::GameStatus, L2ProviderTrait};
use op_succinct_bindings::dispute_game_factory::DisputeGameFactory;
use rand::Rng;
use tokio::time::Duration;
use tracing::info;

use common::{
    constants::{
        CHALLENGER_ADDRESS, CHALLENGER_PRIVATE_KEY, DISPUTE_GAME_FINALITY_DELAY_SECONDS,
        MAX_CHALLENGE_DURATION, MAX_PROVE_DURATION, PROPOSER_ADDRESS, PROPOSER_PRIVATE_KEY,
        TEST_GAME_TYPE,
    },
    monitor::{
        verify_all_resolved_correctly, wait_and_track_games, wait_and_verify_game_resolutions,
        wait_for_challenges, wait_for_resolutions, TrackedGame,
    },
    warp_time, TestEnvironment,
};

use crate::common::{monitor::wait_for_bond_claims, start_challenger, start_proposer};

#[tokio::test(flavor = "multi_thread")]
async fn test_honest_proposer_native() -> Result<()> {
    TestEnvironment::init_logging();
    info!("=== Test: Honest Proposer Full Lifecycle (Create → Resolve → Claim) ===");

    // Setup common test environment
    let env = TestEnvironment::setup().await?;

    // Start proposer
    let proposer_handle = start_proposer(
        &env.rpc_config,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
    )
    .await?;
    info!("✓ Proposer service started");

    // Wait for proposer to create games
    info!("=== Waiting for Game Creation ===");
    let factory = DisputeGameFactory::new(env.deployed.factory, env.anvil.provider.clone());

    // Track first 3 games (L2 finalized head won't advance far enough for 3)
    let tracked_games =
        wait_and_track_games(&factory, TEST_GAME_TYPE, 3, Duration::from_secs(60)).await?;

    info!("✓ Proposer created {} games:", tracked_games.len());
    for (i, game) in tracked_games.iter().enumerate() {
        info!("  Game {}: {} at L2 block {}", i + 1, game.address, game.l2_block_number);
    }

    // Verify proposer is still running
    assert!(!proposer_handle.is_finished(), "Proposer should still be running");
    info!("✓ Proposer is still running successfully");

    // === PHASE 2: Challenge Period ===
    info!("=== Phase 2: Challenge Period ===");
    info!("Warping time to near end of max challenge duration...");

    // Warp by max challenge duration
    warp_time(&env.anvil.provider, Duration::from_secs(MAX_CHALLENGE_DURATION)).await?;
    info!("✓ Warped time by max challenge duration ({MAX_CHALLENGE_DURATION} seconds) to trigger resolution");

    // Verify proposer is still running
    assert!(!proposer_handle.is_finished(), "Proposer should still be running");
    info!("✓ Proposer is still running successfully");

    // === PHASE 3: Resolution ===
    info!("=== Phase 3: Resolution ===");

    // Wait for games to be resolved
    let resolutions =
        wait_for_resolutions(&env.anvil.provider, &tracked_games, Duration::from_secs(30)).await?;

    // Verify all games resolved correctly (proposer wins)
    verify_all_resolved_correctly(&resolutions)?;

    // Warp past DISPUTE_GAME_FINALITY_DELAY_SECONDS
    warp_time(&env.anvil.provider, Duration::from_secs(DISPUTE_GAME_FINALITY_DELAY_SECONDS))
        .await?;
    info!("✓ Warped time by DISPUTE_GAME_FINALITY_DELAY_SECONDS ({DISPUTE_GAME_FINALITY_DELAY_SECONDS} seconds) to trigger bond claims");

    // Verify proposer is still running
    assert!(!proposer_handle.is_finished(), "Proposer should still be running");
    info!("✓ Proposer is still running successfully");

    // === PHASE 4: Bond Claims ===
    info!("=== Phase 4: Bond Claims ===");

    // Wait for proposer to claim bonds
    wait_for_bond_claims(
        &env.anvil.provider,
        &tracked_games,
        PROPOSER_ADDRESS,
        Duration::from_secs(30),
    )
    .await?;

    // Stop proposer
    info!("=== Stopping Proposer ===");
    proposer_handle.abort();
    info!("✓ Proposer stopped gracefully");

    info!("=== Full Lifecycle Test Complete ===");
    info!("✓ Games created, resolved, and bonds claimed successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_honest_challenger_native() -> Result<()> {
    TestEnvironment::init_logging();
    info!("=== Test: Honest Challenger Full Lifecycle (Challenge → Resolve → Claim) ===");

    const NUM_INVALID_GAMES: usize = 3;

    // Setup common test environment
    let env = TestEnvironment::setup().await?;
    let mut l2_block_number = env.anvil.starting_l2_block_number;

    // Start challenger service
    info!("=== Starting Challenger Service ===");
    let challenger_handle = start_challenger(
        &env.rpc_config,
        CHALLENGER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
        None,
    )
    .await?;
    info!("✓ Challenger service started");

    // === PHASE 1: Create Invalid Games ===
    info!("=== Phase 1: Create Invalid Games ===");

    // Create a signer for permissioned account 0
    let wallet = PrivateKeySigner::from_str(PROPOSER_PRIVATE_KEY)?;
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(env.anvil.endpoint.parse::<Url>()?);

    let factory = DisputeGameFactory::new(env.deployed.factory, provider_with_signer.clone());
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

    let mut invalid_games = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..NUM_INVALID_GAMES {
        l2_block_number += 10;
        // Create game with random invalid output root
        let mut invalid_root_bytes = [0u8; 32];
        rng.fill(&mut invalid_root_bytes);
        let invalid_root = FixedBytes::<32>::from(invalid_root_bytes);

        let parent_index = u32::MAX;
        let extra_data = (U256::from(l2_block_number), parent_index).abi_encode_packed();

        let tx = factory
            .create(TEST_GAME_TYPE, invalid_root, extra_data.into())
            .value(init_bond)
            .send()
            .await?;

        let _receipt = tx.get_receipt().await?;
        let new_game_count = factory.gameCount().call().await?;
        let game_index = new_game_count - U256::from(1);
        let game_info = factory.gameAtIndex(game_index).call().await?;
        let game_address = game_info.proxy_;

        invalid_games.push(game_address);
    }

    info!("✓ Created {} invalid games:", invalid_games.len());
    for (i, game) in invalid_games.iter().enumerate() {
        info!("  Game {}: {}", i + 1, game);
    }

    // Verify challenger is still running
    assert!(!challenger_handle.is_finished(), "Challenger should still be running");
    info!("✓ Challenger is still running successfully");

    // === PHASE 2: Challenge Period ===
    info!("=== Phase 2: Challenge Period ===");
    wait_for_challenges(&env.anvil.provider, &invalid_games, Duration::from_secs(60)).await?;
    info!("✓ All games challenged successfully");

    // === PHASE 3: Resolution ===
    info!("=== Phase 3: Resolution ===");
    info!("Warping time past prove deadline to trigger challenger wins...");
    warp_time(
        &env.anvil.provider,
        Duration::from_secs(MAX_CHALLENGE_DURATION + MAX_PROVE_DURATION),
    )
    .await?;
    info!(
        "✓ Warped time by {} seconds (max challenge duration + max prove duration)",
        MAX_CHALLENGE_DURATION + MAX_PROVE_DURATION
    );

    // Wait for and verify challenger wins
    wait_and_verify_game_resolutions(
        &env.anvil.provider,
        &invalid_games,
        GameStatus::CHALLENGER_WINS,
        "ChallengerWins",
        Duration::from_secs(30),
    )
    .await?;

    // Warp DISPUTE_GAME_FINALITY_DELAY_SECONDS + 1 for bond claims
    warp_time(&env.anvil.provider, Duration::from_secs(DISPUTE_GAME_FINALITY_DELAY_SECONDS + 1))
        .await?;
    info!(
        "✓ Warped time to enable bond claims ({} seconds)",
        DISPUTE_GAME_FINALITY_DELAY_SECONDS + 1
    );

    // Verify challenger is still running
    assert!(!challenger_handle.is_finished(), "Challenger should still be running");
    info!("✓ Challenger is still running successfully");

    // === PHASE 4: Bond Claims ===
    info!("=== Phase 4: Bond Claims ===");

    // Wait for challenger to claim bonds
    let tracked_games: Vec<_> = invalid_games
        .iter()
        .map(|&address| TrackedGame {
            address,
            l2_block_number: U256::ZERO, // Not needed for bond claim check
        })
        .collect();

    wait_for_bond_claims(
        &env.anvil.provider,
        &tracked_games,
        CHALLENGER_ADDRESS,
        Duration::from_secs(60),
    )
    .await?;

    // Stop challenger
    info!("=== Stopping Challenger ===");
    challenger_handle.abort();
    info!("✓ Challenger stopped gracefully");

    info!("=== Full Lifecycle Test Complete ===");
    info!("✓ Invalid games challenged, won by challenger, and bonds claimed successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_game_chain_validation_invalid_parent() -> Result<()> {
    TestEnvironment::init_logging();
    info!("=== Test: Game Chain Validation - Invalid Parent Chain ===");

    // Setup common test environment
    let env = TestEnvironment::setup().await?;

    // Create a signer for creating games
    let wallet = PrivateKeySigner::from_str(PROPOSER_PRIVATE_KEY)?;
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(env.anvil.endpoint.parse::<Url>()?);

    let factory = DisputeGameFactory::new(env.deployed.factory, provider_with_signer.clone());
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

    // === PHASE 1: Create Invalid Parent Chain ===
    info!("=== Phase 1: Creating Invalid Parent Chain ===");
    
    // Step 1: Create a valid anchor game (parentIndex = u32::MAX)
    let anchor_block = env.anvil.starting_l2_block_number + 100;
    
    // Create L2 provider to compute output roots
    let fetcher = op_succinct_host_utils::fetcher::OPSuccinctDataFetcher::new();
    let anchor_root = fetcher.l2_provider.compute_output_root_at_block(U256::from(anchor_block)).await?;
    let anchor_extra_data = (U256::from(anchor_block), u32::MAX).abi_encode_packed();
    
    let tx = factory
        .create(TEST_GAME_TYPE, anchor_root, anchor_extra_data.into())
        .value(init_bond)
        .send()
        .await?;
    tx.get_receipt().await?;
    
    let anchor_game_count = factory.gameCount().call().await?;
    let anchor_game_index = anchor_game_count - U256::from(1);
    let anchor_game_info = factory.gameAtIndex(anchor_game_index).call().await?;
    info!("✓ Created valid anchor game at index {} (address: {})", anchor_game_index, anchor_game_info.proxy_);

    // Step 2: Create an invalid middle game with wrong output root
    let middle_block = anchor_block + 100;
    let mut rng = rand::rng();
    let mut invalid_root_bytes = [0u8; 32];
    rng.fill(&mut invalid_root_bytes);
    let invalid_root = FixedBytes::<32>::from(invalid_root_bytes);
    let middle_extra_data = (U256::from(middle_block), anchor_game_index.to::<u32>()).abi_encode_packed();
    
    let tx = factory
        .create(TEST_GAME_TYPE, invalid_root, middle_extra_data.into())
        .value(init_bond)
        .send()
        .await?;
    tx.get_receipt().await?;
    
    let middle_game_count = factory.gameCount().call().await?;
    let middle_game_index = middle_game_count - U256::from(1);
    let middle_game_info = factory.gameAtIndex(middle_game_index).call().await?;
    info!("✓ Created invalid middle game at index {} (address: {})", middle_game_index, middle_game_info.proxy_);

    // Step 3: Create a valid child game pointing to invalid parent
    let child_block = middle_block + 100;
    let child_root = fetcher.l2_provider.compute_output_root_at_block(U256::from(child_block)).await?;
    let child_extra_data = (U256::from(child_block), middle_game_index.to::<u32>()).abi_encode_packed();
    
    let tx = factory
        .create(TEST_GAME_TYPE, child_root, child_extra_data.into())
        .value(init_bond)
        .send()
        .await?;
    tx.get_receipt().await?;
    
    let child_game_count = factory.gameCount().call().await?;
    let child_game_index = child_game_count - U256::from(1);
    let child_game_info = factory.gameAtIndex(child_game_index).call().await?;
    info!("✓ Created valid child game at index {} (address: {})", child_game_index, child_game_info.proxy_);

    // === PHASE 2: Start Proposer and Verify Chain Rejection ===
    info!("=== Phase 2: Starting Proposer to Validate Chain ===");
    
    // Start proposer
    let proposer_handle = start_proposer(
        &env.rpc_config,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
    )
    .await?;
    info!("✓ Proposer service started");

    // Wait for proposer to create a new game (it should skip the invalid chain)
    let initial_game_count = child_game_count;
    let mut new_game_created = false;
    
    for _ in 0..30 {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let current_game_count = factory.gameCount().call().await?;
        
        if current_game_count > initial_game_count {
            new_game_created = true;
            let new_game_index = current_game_count - U256::from(1);
            let new_game_info = factory.gameAtIndex(new_game_index).call().await?;
            
            // Check that the new game doesn't build on the invalid chain
            let new_game = op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
                new_game_info.proxy_,
                env.anvil.provider.clone(),
            );
            let claim_data = new_game.claimData().call().await?;
            
            // The new game should either be an anchor game or build on the valid anchor game
            assert!(
                claim_data.parentIndex == u32::MAX || U256::from(claim_data.parentIndex) <= anchor_game_index,
                "Proposer should not build on invalid chain"
            );
            
            info!("✓ Proposer correctly skipped invalid chain and created new game at index {}", new_game_index);
            info!("  New game parent index: {}", claim_data.parentIndex);
            break;
        }
    }
    
    assert!(new_game_created, "Proposer should have created a new game");
    
    // Stop proposer
    proposer_handle.abort();
    info!("✓ Test complete: Proposer correctly rejected invalid parent chain");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_game_chain_validation_challenged_parent() -> Result<()> {
    TestEnvironment::init_logging();
    info!("=== Test: Game Chain Validation - Challenged Parent ===");

    // Setup common test environment
    let env = TestEnvironment::setup().await?;

    // Create a signer for creating games
    let wallet = PrivateKeySigner::from_str(PROPOSER_PRIVATE_KEY)?;
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(env.anvil.endpoint.parse::<Url>()?);

    let factory = DisputeGameFactory::new(env.deployed.factory, provider_with_signer.clone());
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

    // === PHASE 1: Create Valid Parent Game ===
    info!("=== Phase 1: Creating Valid Parent Game ===");
    
    let parent_block = env.anvil.starting_l2_block_number + 100;
    
    // Create L2 provider to compute output roots
    let fetcher = op_succinct_host_utils::fetcher::OPSuccinctDataFetcher::new();
    let parent_root = fetcher.l2_provider.compute_output_root_at_block(U256::from(parent_block)).await?;
    let parent_extra_data = (U256::from(parent_block), u32::MAX).abi_encode_packed();
    
    let tx = factory
        .create(TEST_GAME_TYPE, parent_root, parent_extra_data.into())
        .value(init_bond)
        .send()
        .await?;
    tx.get_receipt().await?;
    
    let parent_game_count = factory.gameCount().call().await?;
    let parent_game_index = parent_game_count - U256::from(1);
    let parent_game_info = factory.gameAtIndex(parent_game_index).call().await?;
    let parent_game_address = parent_game_info.proxy_;
    info!("✓ Created valid parent game at index {} (address: {})", parent_game_index, parent_game_address);

    // === PHASE 2: Challenge and Resolve Parent as CHALLENGER_WINS ===
    info!("=== Phase 2: Challenging Parent Game ===");
    
    // Start challenger to challenge the game (using malicious mode to challenge valid game)
    let challenger_handle = start_challenger(
        &env.rpc_config,
        CHALLENGER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
        Some(100.0), // Challenge all games maliciously for testing
    )
    .await?;
    info!("✓ Challenger service started in malicious mode");

    // Wait for challenge
    wait_for_challenges(&env.anvil.provider, &[parent_game_address], Duration::from_secs(30)).await?;
    info!("✓ Parent game challenged");

    // Warp time to resolve as CHALLENGER_WINS (no proof submitted)
    warp_time(
        &env.anvil.provider,
        Duration::from_secs(MAX_CHALLENGE_DURATION + MAX_PROVE_DURATION),
    )
    .await?;
    
    // Wait for resolution
    wait_and_verify_game_resolutions(
        &env.anvil.provider,
        &[parent_game_address],
        GameStatus::CHALLENGER_WINS,
        "ChallengerWins",
        Duration::from_secs(30),
    )
    .await?;
    info!("✓ Parent game resolved as CHALLENGER_WINS");

    // Stop challenger
    challenger_handle.abort();

    // === PHASE 3: Create Child Game Referencing Challenged Parent ===
    info!("=== Phase 3: Creating Child Game with Challenged Parent ===");
    
    let child_block = parent_block + 100;
    let child_root = fetcher.l2_provider.compute_output_root_at_block(U256::from(child_block)).await?;
    let child_extra_data = (U256::from(child_block), parent_game_index.to::<u32>()).abi_encode_packed();
    
    let tx = factory
        .create(TEST_GAME_TYPE, child_root, child_extra_data.into())
        .value(init_bond)
        .send()
        .await?;
    tx.get_receipt().await?;
    
    let child_game_count = factory.gameCount().call().await?;
    let child_game_index = child_game_count - U256::from(1);
    let child_game_info = factory.gameAtIndex(child_game_index).call().await?;
    info!("✓ Created child game at index {} (address: {}) with challenged parent", 
          child_game_index, child_game_info.proxy_);

    // === PHASE 4: Start Proposer and Verify Chain Rejection ===
    info!("=== Phase 4: Starting Proposer to Validate Chain ===");
    
    // Start proposer
    let proposer_handle = start_proposer(
        &env.rpc_config,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
    )
    .await?;
    info!("✓ Proposer service started");

    // Wait for proposer to create a new game (it should skip the chain with challenged parent)
    let initial_game_count = child_game_count;
    let mut new_game_created = false;
    
    for _ in 0..30 {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let current_game_count = factory.gameCount().call().await?;
        
        if current_game_count > initial_game_count {
            new_game_created = true;
            let new_game_index = current_game_count - U256::from(1);
            let new_game_info = factory.gameAtIndex(new_game_index).call().await?;
            
            // Check that the new game doesn't build on the challenged parent chain
            let new_game = op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
                new_game_info.proxy_,
                env.anvil.provider.clone(),
            );
            let claim_data = new_game.claimData().call().await?;
            
            // The new game should not reference the challenged parent or its child
            assert!(
                U256::from(claim_data.parentIndex) != parent_game_index && 
                U256::from(claim_data.parentIndex) != child_game_index,
                "Proposer should not build on chain with challenged parent"
            );
            
            info!("✓ Proposer correctly skipped chain with challenged parent and created new game at index {}", new_game_index);
            info!("  New game parent index: {}", claim_data.parentIndex);
            break;
        }
    }
    
    assert!(new_game_created, "Proposer should have created a new game");
    
    // Stop proposer
    proposer_handle.abort();
    info!("✓ Test complete: Proposer correctly rejected chain with challenged parent");

    Ok(())
}
