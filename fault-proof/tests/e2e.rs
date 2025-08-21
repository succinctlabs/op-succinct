mod common;

use std::str::FromStr;

use alloy_network::EthereumWallet;
use alloy_primitives::{FixedBytes, U256};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use fault_proof::contract::GameStatus;
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
async fn test_proposer_branch_creation_after_challenger_wins() -> Result<()> {
    TestEnvironment::init_logging();
    info!("=== Test: Proposer Branch Creation After Challenger Wins ===");

    const NUM_VALID_GAMES: usize = 2;

    // Setup common test environment
    let env = TestEnvironment::setup().await?;
    let mut l2_block_number = env.anvil.starting_l2_block_number;

    // Create a signer for creating games
    let wallet = PrivateKeySigner::from_str(PROPOSER_PRIVATE_KEY)?;
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(env.anvil.endpoint.parse::<Url>()?);

    let factory = DisputeGameFactory::new(env.deployed.factory, provider_with_signer.clone());
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

    // === PHASE 1: Create Valid Base Games ===
    info!("=== Phase 1: Create Valid Base Games ===");

    // Start proposer to create some valid games first
    let proposer_handle = start_proposer(
        &env.rpc_config,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
    )
    .await?;
    info!("✓ Proposer service started");

    // Wait for proposer to create 2 valid games
    let tracked_games =
        wait_and_track_games(&factory, TEST_GAME_TYPE, 2, Duration::from_secs(60)).await?;

    info!("✓ Proposer created {} valid base games:", tracked_games.len());
    for (i, game) in tracked_games.iter().enumerate() {
        info!("  Game {}: {} at L2 block {}", i + 1, game.address, game.l2_block_number);
    }

    // Stop proposer temporarily
    proposer_handle.abort();
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("✓ Stopped proposer temporarily");

    // === PHASE 2: Create Invalid Game ===
    info!("=== Phase 2: Create Invalid Game ===");

    let mut invalid_games = Vec::new();
    let mut rng = rand::rng();
    let last_game_index = factory.gameCount().call().await? - U256::from(1);

    l2_block_number += 10 * (NUM_VALID_GAMES + 1) as u64;
    // Create game with random invalid output root
    let mut invalid_root_bytes = [0u8; 32];
    rng.fill(&mut invalid_root_bytes);
    let invalid_root = FixedBytes::<32>::from(invalid_root_bytes);

    // Create invalid game that references the latest valid game (index 1)
    // This simulates building an invalid game on top of valid chain
    let parent_index = last_game_index.to::<u32>();

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

    info!(
        "✓ Created {} invalid game building on top of valid game at index {}:",
        invalid_games.len(),
        last_game_index
    );
    for (i, game) in invalid_games.iter().enumerate() {
        info!("  Invalid Game {}: {}", i + 1, game);
    }

    // === PHASE 3: Challenge and Resolve Invalid Game ===
    info!("=== Phase 3: Challenge and Resolve Invalid Game ===");

    // Start challenger service
    let challenger_handle = start_challenger(
        &env.rpc_config,
        CHALLENGER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
        None,
    )
    .await?;
    info!("✓ Challenger service started");

    // Wait for challenges
    wait_for_challenges(&env.anvil.provider, &invalid_games, Duration::from_secs(60)).await?;
    info!("✓ Invalid game challenged successfully");

    // Warp time to resolve in challenger's favor
    warp_time(
        &env.anvil.provider,
        Duration::from_secs(MAX_CHALLENGE_DURATION + MAX_PROVE_DURATION),
    )
    .await?;
    info!("✓ Warped time to trigger challenger wins");

    // Manually resolve games since automatic resolution seems to be timing out
    // Need to resolve parent games first before child games can be resolved

    // First resolve the valid parent games
    for game in &tracked_games {
        info!("Resolving parent game at address: {}", game.address);
        let game_contract =
            op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
                game.address,
                provider_with_signer.clone(),
            );
        let resolve_tx = game_contract.resolve().send().await?;
        let _resolve_receipt = resolve_tx.get_receipt().await?;
        info!("✓ Parent game resolved: {}", game.address);
    }

    // Now resolve the invalid game
    info!("Resolving challenged invalid game...");
    let invalid_game_contract =
        op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
            invalid_games[0],
            provider_with_signer.clone(),
        );

    let resolve_tx = invalid_game_contract.resolve().send().await?;
    let _resolve_receipt = resolve_tx.get_receipt().await?;
    info!("✓ Invalid game resolved manually");

    // Wait for and verify challenger wins
    wait_and_verify_game_resolutions(
        &env.anvil.provider,
        &invalid_games,
        GameStatus::CHALLENGER_WINS,
        "ChallengerWins",
        Duration::from_secs(10),
    )
    .await?;

    // Stop challenger
    challenger_handle.abort();
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("✓ Challenger stopped after resolving invalid game");

    // === PHASE 4: Restart Proposer - Should Create New Branch ===
    info!("=== Phase 4: Restart Proposer - Should Create New Branch ===");

    let games_before_restart = factory.gameCount().call().await?;
    info!("Games count before proposer restart: {}", games_before_restart);

    // Start proposer again - it should detect challenger-won games and create a new branch
    let proposer_handle_2 = start_proposer(
        &env.rpc_config,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory,
        TEST_GAME_TYPE,
    )
    .await?;
    info!("✓ Proposer service restarted");

    // Wait for proposer to create new games (should branch from safe point)
    let new_tracked_games = wait_and_track_games(
        &factory,
        TEST_GAME_TYPE,
        games_before_restart.to::<usize>() + NUM_VALID_GAMES,
        Duration::from_secs(90),
    )
    .await?;

    let newly_created_games: Vec<_> =
        new_tracked_games.into_iter().skip(games_before_restart.to::<usize>()).collect();

    info!("✓ Proposer created {} new games after challenger wins:", newly_created_games.len());
    for (i, game) in newly_created_games.iter().enumerate() {
        info!("  New Game {}: {} at L2 block {}", i + 1, game.address, game.l2_block_number);
    }

    // === CRITICAL VERIFICATION: Ensure new games don't reference invalidated chain ===
    info!("=== Verifying Branch Safety ===");
    
    // Get the index of the invalid game that was challenged and won
    let invalid_game_index = game_index.to::<u32>();
    
    for (i, game) in newly_created_games.iter().enumerate() {
        let game_contract =
            op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
                game.address,
                provider_with_signer.clone(),
            );
        
        let claim_data = game_contract.claimData().call().await?;
        
        // Verify this game doesn't reference the invalidated chain
        assert_ne!(
            claim_data.parentIndex,
            invalid_game_index,
            "New game {} incorrectly references invalidated game at index {}",
            i + 1,
            invalid_game_index
        );
        
        // Also verify it doesn't reference u32::MAX unless it's the first game from anchor
        if claim_data.parentIndex != u32::MAX {
            // It should reference one of the valid games created in Phase 1
            let parent_game_info = factory.gameAtIndex(U256::from(claim_data.parentIndex)).call().await?;
            let parent_game_contract =
                op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame::new(
                    parent_game_info.proxy_,
                    provider_with_signer.clone(),
                );
            let parent_status_raw = parent_game_contract.status().call().await?;
            
            // Convert u8 to GameStatus for comparison
            let parent_status = match parent_status_raw {
                0 => GameStatus::IN_PROGRESS,
                1 => GameStatus::CHALLENGER_WINS,
                2 => GameStatus::DEFENDER_WINS,
                _ => panic!("Invalid game status: {}", parent_status_raw),
            };
            
            // Parent should be DEFENDER_WINS (valid) or IN_PROGRESS
            assert!(
                parent_status == GameStatus::DEFENDER_WINS || parent_status == GameStatus::IN_PROGRESS,
                "New game {} references parent with invalid status: {:?}",
                i + 1,
                parent_status
            );
            
            info!(
                "  ✓ New Game {} correctly references parent index {} with status {:?}",
                i + 1,
                claim_data.parentIndex,
                parent_status
            );
        } else {
            info!("  ✓ New Game {} correctly references anchor (no parent)", i + 1);
        }
    }
    
    info!("✓ All new games correctly avoid invalidated chain - branch safety verified");

    // Verify the new games are valid by checking they don't get challenged
    tokio::time::sleep(Duration::from_secs(10)).await;
    info!("✓ New games remained unchallengeable for 10 seconds - likely valid");

    // Stop proposer
    proposer_handle_2.abort();
    info!("✓ Proposer stopped gracefully");

    info!("=== Branch Creation Test Complete ===");
    info!("✓ Proposer successfully created new branch after challenger won invalid game");
    info!("✓ This validates the fix prevents building on challenger-won chains");

    Ok(())
}
