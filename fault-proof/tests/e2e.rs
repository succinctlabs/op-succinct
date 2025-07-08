mod common;

use std::str::FromStr;

use alloy_network::EthereumWallet;
use alloy_primitives::{FixedBytes, U256};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use bindings::dispute_game_factory::DisputeGameFactory;
use rand::Rng;
use tokio::time::Duration;
use tracing::info;

use common::{
    constants::{
        CHALLENGER_ADDRESS, CHALLENGER_PRIVATE_KEY, DISPUTE_GAME_FINALITY_DELAY_SECONDS,
        MAX_CHALLENGE_DURATION, MAX_PROVE_DURATION, PROPOSER_ADDRESS, PROPOSER_PRIVATE_KEY,
        TEST_GAME_TYPE,
    },
    find_binary_path, generate_challenger_env, generate_proposer_env,
    monitor::{
        verify_all_bonds_claimed, verify_all_resolved_correctly, wait_and_track_games,
        wait_and_verify_game_resolutions, wait_for_bond_claims, wait_for_challenges,
        wait_for_resolutions, TrackedGame, GAME_STATUS_CHALLENGER_WINS,
    },
    start_challenger_binary, start_proposer_binary, warp_time, TestEnvironment,
};

#[tokio::test]
async fn test_honest_proposer() -> Result<()> {
    TestEnvironment::init_logging();
    info!("\n=== Test: Honest Proposer Full Lifecycle (Create → Resolve → Claim) ===");

    // Setup common test environment
    let env = TestEnvironment::setup().await?;

    // Find proposer binary
    info!("\n=== Starting Proposer Service ===");
    let proposer_binary = find_binary_path("proposer")?;
    info!("✓ Found proposer binary: {:?}", proposer_binary);

    // Generate proposer environment
    let proposer_env = generate_proposer_env(
        &env.anvil.endpoint,
        &env.l2_rpc,
        &env.l2_node_rpc,
        &env.l1_beacon_rpc,
        PROPOSER_PRIVATE_KEY,
        &env.deployed.factory.to_string(),
        TEST_GAME_TYPE,
        None,
    );

    // Start proposer
    let mut proposer = start_proposer_binary(proposer_binary, proposer_env).await?;
    info!("✓ Proposer service started");

    // Wait for proposer to create games
    info!("\n=== Waiting for Game Creation ===");
    let factory = DisputeGameFactory::new(env.deployed.factory, env.anvil.provider.clone());

    // Track first 3 games (L2 finalized head won't advance far enough for 3)
    let tracked_games =
        wait_and_track_games(&factory, TEST_GAME_TYPE, 3, Duration::from_secs(60)).await?;

    info!("✓ Proposer created {} games:", tracked_games.len());
    for (i, game) in tracked_games.iter().enumerate() {
        info!("  Game {}: {} at L2 block {}", i + 1, game.address, game.l2_block_number);
    }

    // Verify proposer is still running
    assert!(proposer.is_running(), "Proposer should still be running");
    info!("\n✓ Proposer is still running successfully");

    // === PHASE 2: Challenge Period ===
    info!("\n=== Phase 2: Challenge Period ===");
    info!("Warping time to near end of max challenge duration...");

    // Warp by max challenge duration
    warp_time(&env.anvil.provider, Duration::from_secs(MAX_CHALLENGE_DURATION)).await?;
    info!("✓ Warped time by max challenge duration ({MAX_CHALLENGE_DURATION} seconds) to trigger resolution");

    // Verify proposer is still running
    assert!(proposer.is_running(), "Proposer should still be running");
    info!("\n✓ Proposer is still running successfully");

    // === PHASE 3: Resolution ===
    info!("\n=== Phase 3: Resolution ===");

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
    assert!(proposer.is_running(), "Proposer should still be running");
    info!("\n✓ Proposer is still running successfully");

    // === PHASE 4: Bond Claims ===
    info!("\n=== Phase 4: Bond Claims ===");

    // Wait for proposer to claim bonds
    let claims = wait_for_bond_claims(
        &env.anvil.provider,
        &tracked_games,
        PROPOSER_ADDRESS,
        Duration::from_secs(30),
    )
    .await?;

    // Verify all bonds were claimed
    verify_all_bonds_claimed(&claims)?;

    // Stop proposer
    info!("\n=== Stopping Proposer ===");
    proposer.kill().await?;
    info!("✓ Proposer stopped gracefully");

    info!("\n=== Full Lifecycle Test Complete ===");
    info!("✓ Games created, resolved, and bonds claimed successfully");

    Ok(())
}

#[tokio::test]
async fn test_honest_challenger() -> Result<()> {
    TestEnvironment::init_logging();
    info!("\n=== Test: Honest Challenger Full Lifecycle (Challenge → Resolve → Claim) ===");

    const NUM_INVALID_GAMES: usize = 3;

    // Setup common test environment
    let env = TestEnvironment::setup().await?;
    let mut l2_block_number = env.get_initial_l2_block_number().await?;

    // Start challenger service
    info!("\n=== Starting Challenger Service ===");
    let challenger_binary = find_binary_path("challenger")?;
    info!("✓ Found challenger binary: {:?}", challenger_binary);

    // Generate challenger environment
    let challenger_env = generate_challenger_env(
        &env.anvil.endpoint,
        &env.l2_rpc,
        &env.l2_node_rpc,
        &env.l1_beacon_rpc,
        CHALLENGER_PRIVATE_KEY,
        &env.deployed.factory.to_string(),
        TEST_GAME_TYPE,
        None,
        None,
    );

    let mut challenger = start_challenger_binary(challenger_binary, challenger_env).await?;
    info!("✓ Challenger service started");

    // === PHASE 1: Create Invalid Games ===
    info!("\n=== Phase 1: Create Invalid Games ===");

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
    assert!(challenger.is_running(), "Challenger should still be running");
    info!("\n✓ Challenger is still running successfully");

    // === PHASE 2: Challenge Period ===
    info!("\n=== Phase 2: Challenge Period ===");
    wait_for_challenges(&env.anvil.provider, &invalid_games, Duration::from_secs(60)).await?;
    info!("✓ All games challenged successfully");

    // === PHASE 3: Resolution ===
    info!("\n=== Phase 3: Resolution ===");
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
        GAME_STATUS_CHALLENGER_WINS,
        "ChallengerWins",
        Duration::from_secs(30),
    )
    .await?;

    // Warp past DISPUTE_GAME_FINALITY_DELAY_SECONDS for bond claims
    // NOTE(fakedev9999): +1 to ensure we're *past* the finalization time
    warp_time(&env.anvil.provider, Duration::from_secs(DISPUTE_GAME_FINALITY_DELAY_SECONDS + 1))
        .await?;
    info!(
        "✓ Warped time by DISPUTE_GAME_FINALITY_DELAY_SECONDS ({} seconds) to enable bond claims",
        DISPUTE_GAME_FINALITY_DELAY_SECONDS + 1
    );

    // Verify challenger is still running
    assert!(challenger.is_running(), "Challenger should still be running");
    info!("\n✓ Challenger is still running successfully");

    // === PHASE 4: Bond Claims ===
    info!("\n=== Phase 4: Bond Claims ===");

    // Wait for challenger to claim bonds
    let tracked_games: Vec<_> = invalid_games
        .iter()
        .map(|&address| TrackedGame {
            address,
            l2_block_number: U256::ZERO, // Not needed for bond claim check
            output_root: FixedBytes::default(),
            created_at_block: 0,
        })
        .collect();

    let claims = wait_for_bond_claims(
        &env.anvil.provider,
        &tracked_games,
        CHALLENGER_ADDRESS,
        Duration::from_secs(60),
    )
    .await?;

    // Verify all bonds were claimed
    verify_all_bonds_claimed(&claims)?;

    // Stop challenger
    info!("\n=== Stopping Challenger ===");
    challenger.kill().await?;
    info!("✓ Challenger stopped gracefully");

    info!("\n=== Full Lifecycle Test Complete ===");
    info!("✓ Invalid games challenged, won by challenger, and bonds claimed successfully");

    Ok(())
}
