mod common;

use alloy_eips::BlockNumberOrTag;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use bindings::dispute_game_factory::DisputeGameFactory;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use tokio::time::Duration;

use common::*;
use fault_proof::L2ProviderTrait;

#[tokio::test]
async fn test_honest_proposer() -> Result<()> {
    // Initialize logging
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    println!("\n=== Test: Proposer Full Lifecycle (Create → Resolve → Claim) ===");

    // Get environment variables
    let l1_rpc = std::env::var("L1_RPC").expect("L1_RPC must be set");
    let l2_rpc = std::env::var("L2_RPC").expect("L2_RPC must be set");
    let l2_node_rpc = std::env::var("L2_NODE_RPC").unwrap_or_else(|_| l2_rpc.clone());
    let l1_beacon_rpc = std::env::var("L1_BEACON_RPC").expect("L1_BEACON_RPC must be set");
    let fetcher = OPSuccinctDataFetcher::new();

    // Setup Anvil fork
    let l2_provider =
        ProviderBuilder::default().connect_http(l2_rpc.clone().parse::<Url>().unwrap());
    let l2_block_number =
        l2_provider.get_l2_block_by_number(BlockNumberOrTag::Finalized).await?.header.number - 100;
    let fork_block = fetcher.get_safe_l1_block_for_l2_block(l2_block_number).await?.1;
    let anvil = setup_anvil_fork(&l1_rpc, fork_block, Some(Duration::from_secs(1))).await?;
    println!("✓ Anvil fork started at: {}", anvil.endpoint);

    // Deploy contracts
    println!("\n=== Deploying Contracts ===");
    let deployed = deploy_test_contracts(anvil.provider.clone(), l2_provider).await?;

    // Configure contracts
    configure_contracts(anvil.provider.clone(), &deployed).await?;

    println!("✓ Contracts deployed and configured");
    println!("  Factory: {}", deployed.factory);
    println!("  Game Type: {}", TEST_GAME_TYPE);

    // Find proposer binary
    println!("\n=== Starting Proposer Service ===");
    let proposer_binary = find_binary_path("proposer")?;
    println!("✓ Found proposer binary: {:?}", proposer_binary);

    // Generate proposer environment
    let proposer_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // Anvil account 0
    let proposer_env = generate_proposer_env(
        &anvil.endpoint,               // L1 RPC (Anvil fork)
        &l2_rpc,                       // L2 RPC
        &l2_node_rpc,                  // L2 Node RPC
        &l1_beacon_rpc,                // L1 Beacon RPC
        proposer_key,                  // Private key
        &deployed.factory.to_string(), // Factory address
        TEST_GAME_TYPE,                // Game type
        None,                          // No prover network for test
    );

    // Start proposer
    let mut proposer = start_proposer_binary(proposer_binary, proposer_env).await?;
    println!("✓ Proposer service started");

    // Wait for proposer to create games
    println!("\n=== Waiting for Game Creation ===");
    let factory = DisputeGameFactory::new(deployed.factory, anvil.provider.clone());

    // Track first 2 games (L2 finalized head won't advance far enough for 3)
    let tracked_games =
        wait_and_track_games(&factory, TEST_GAME_TYPE, 3, Duration::from_secs(60)).await?;

    println!("✓ Proposer created {} games:", tracked_games.len());
    for (i, game) in tracked_games.iter().enumerate() {
        println!("  Game {}: {} at L2 block {}", i + 1, game.address, game.l2_block_number);
    }

    // Verify proposer is still running
    assert!(proposer.is_running(), "Proposer should still be running");
    println!("\n✓ Proposer is still running successfully");

    // === PHASE 2: Challenge Period ===
    println!("\n=== Phase 2: Challenge Period ===");
    println!("Warping time to near end of max challenge duration...");

    // Warp to near the end of max challenge duration (120s - 10s)
    let warp_duration = MAX_CHALLENGE_DURATION - 10;
    warp_time(&anvil.provider, Duration::from_secs(warp_duration)).await?;
    println!("✓ Warped time by {} seconds", warp_duration);

    // Give proposer time to react
    tokio::time::sleep(Duration::from_secs(5)).await;

    // === PHASE 3: Resolution ===
    println!("\n=== Phase 3: Resolution ===");
    println!("Warping past max challenge duration to trigger resolution...");

    // Warp past max challenge duration to trigger resolution
    warp_time(&anvil.provider, Duration::from_secs(15)).await?;
    println!("✓ Warped time by 15 seconds (past challenge deadline)");

    // Wait for games to be resolved
    let resolutions =
        wait_for_resolutions(&anvil.provider, &tracked_games, Duration::from_secs(30)).await?;

    // Verify all games resolved correctly (proposer wins)
    verify_all_resolved_correctly(&resolutions)?;

    // === PHASE 4: Bond Claims ===
    println!("\n=== Phase 4: Bond Claims ===");
    println!("Warping past delay period to enable bond claims...");

    // Warp past delay period (60s + 1s)
    let delay_warp = DELAY_PERIOD + 1;
    warp_time(&anvil.provider, Duration::from_secs(delay_warp)).await?;
    println!("✓ Warped time by {} seconds (past delay period)", delay_warp);

    // Wait for proposer to claim bonds
    let claims =
        wait_for_bond_claims(&anvil.provider, &tracked_games, Duration::from_secs(30)).await?;

    // Verify all bonds were claimed
    verify_all_bonds_claimed(&claims)?;

    // Stop proposer
    println!("\n=== Stopping Proposer ===");
    proposer.kill().await?;
    println!("✓ Proposer stopped gracefully");

    println!("\n=== Full Lifecycle Test Complete ===");
    println!("✓ Games created, resolved, and bonds claimed successfully");
    println!("✓ Total test time: ~{} seconds", warp_duration + 15 + delay_warp + 10); // Approximate total time

    // Cleanup
    cleanup_anvil();
    Ok(())
}
