mod common;

use alloy_eips::BlockNumberOrTag;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use bindings::{access_manager::AccessManager, dispute_game_factory::DisputeGameFactory};
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use tokio::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::{address, FixedBytes, U256};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame;
use common::*;
use fault_proof::L2ProviderTrait;
use rand::Rng;
use std::str::FromStr;

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

#[tokio::test]
async fn test_honest_challenger() -> Result<()> {
    // Initialize logging
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    println!("\n=== Test: Invalid Games (Challenger Wins) ===");

    const NUM_INVALID_GAMES: usize = 3;

    // Get environment variables
    let l1_rpc = std::env::var("L1_RPC").expect("L1_RPC must be set");
    let l2_rpc = std::env::var("L2_RPC").expect("L2_RPC must be set");
    let l2_node_rpc = std::env::var("L2_NODE_RPC").unwrap_or_else(|_| l2_rpc.clone());
    let l1_beacon_rpc = std::env::var("L1_BEACON_RPC").expect("L1_BEACON_RPC must be set");
    let fetcher = OPSuccinctDataFetcher::new();

    // Setup Anvil fork
    let l2_provider =
        ProviderBuilder::default().connect_http(l2_rpc.clone().parse::<Url>().unwrap());
    let mut l2_block_number =
        l2_provider.get_l2_block_by_number(BlockNumberOrTag::Finalized).await?.header.number - 100;
    let fork_block = fetcher.get_safe_l1_block_for_l2_block(l2_block_number).await?.1;
    let anvil = setup_anvil_fork(&l1_rpc, fork_block, Some(Duration::from_secs(1))).await?;
    println!("✓ Anvil fork started at: {}", anvil.endpoint);

    // Warp Anvil time to current time to avoid deadline issues
    let current_time =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let current_block = anvil.provider.get_block_number().await?;
    let block_info = anvil.provider.get_block_by_number(current_block.into()).await?.unwrap();
    let anvil_timestamp = block_info.header.timestamp;

    if current_time > anvil_timestamp {
        let warp_seconds = current_time - anvil_timestamp;
        println!("\n=== Warping Anvil Time ===");
        println!("  Current Anvil timestamp: {}", anvil_timestamp);
        println!("  Current real timestamp: {}", current_time);
        println!("  Warping forward {} seconds...", warp_seconds);
        warp_time(&anvil.provider, Duration::from_secs(warp_seconds)).await?;
        println!("  ✓ Time warped to current");
    }

    // Deploy contracts
    println!("\n=== Deploying Contracts ===");
    let deployed = deploy_test_contracts(anvil.provider.clone(), l2_provider.clone()).await?;

    // Configure contracts
    configure_contracts(anvil.provider.clone(), &deployed).await?;

    println!("✓ Contracts deployed and configured");
    println!("  Factory: {}", deployed.factory);
    println!("  Game Type: {}", TEST_GAME_TYPE);

    // Start challenger first
    println!("\n=== Starting Challenger Service ===");
    let challenger_binary = find_binary_path("challenger")?;
    println!("✓ Found challenger binary: {:?}", challenger_binary);

    // Generate challenger environment
    let challenger_key = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"; // Anvil account 1 private key
    let challenger_env = generate_challenger_env(
        &anvil.endpoint,               // L1 RPC (Anvil fork)
        &l2_rpc,                       // L2 RPC
        &l2_node_rpc,                  // L2 Node RPC
        &l1_beacon_rpc,                // L1 Beacon RPC
        challenger_key,                // Private key
        &deployed.factory.to_string(), // Factory address
        TEST_GAME_TYPE,                // Game type
        None,                          // No prover network for test
        None,                          // No malicious percentage
    );

    let mut challenger = start_challenger_binary(challenger_binary, challenger_env).await?;
    println!("✓ Challenger service started");

    // Give challenger time to initialize
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Verify challenger is running
    if !challenger.is_running() {
        anyhow::bail!("Challenger service stopped unexpectedly");
    }
    println!("✓ Challenger is running");

    // Also fund the challenger with some ETH for bonds
    let tx = anvil
        .provider
        .send_transaction(
            alloy_rpc_types_eth::request::TransactionRequest::default()
                .from(address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")) // Anvil account 0
                .to(address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8")) // Challenger
                .value(U256::from_str("1000000000000000000")?),
        ) // 1 ETH
        .await?;
    tx.get_receipt().await?;
    println!("✓ Funded challenger with 1 ETH");

    // Manually create invalid games using factory
    println!("\n=== Creating Invalid Games ===");

    // Create a signer for permissioned account 0
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // Anvil account 0
    let wallet = PrivateKeySigner::from_str(private_key)?;
    let address = wallet.address();
    println!("Using anvil account 0: {}", address);

    // Create provider with signer
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(anvil.endpoint.parse::<Url>()?);

    let factory = DisputeGameFactory::new(deployed.factory, provider_with_signer.clone());
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

    // The AnchorStateRegistry is initialized with an anchor at finalized block - 100
    // So we can create games directly without needing to create an anchor game
    let initial_game_count = factory.gameCount().call().await?;
    println!("Initial game count: {}", initial_game_count);

    let mut invalid_games = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..NUM_INVALID_GAMES {
        l2_block_number += 10;
        // Create game with random invalid output root
        let mut invalid_root_bytes = [0u8; 32];
        rng.fill(&mut invalid_root_bytes);
        let invalid_root = FixedBytes::<32>::from(invalid_root_bytes);

        // Encode extra data with L2 block number and parent game index
        // All games will use u32::MAX as parent since they reference the anchor state
        let parent_index = u32::MAX;
        let extra_data = (U256::from(l2_block_number), parent_index).abi_encode_packed();

        println!(
            "Creating invalid game {} with output root: 0x{}",
            i + 1,
            hex::encode(&invalid_root)
        );

        println!("  L2 block: {}, Parent index: {}", l2_block_number, parent_index);

        let tx = factory
            .create(TEST_GAME_TYPE, invalid_root, extra_data.into())
            .value(init_bond)
            .send()
            .await?;

        let _receipt = tx.get_receipt().await?;

        // Get the game address by fetching the latest game index
        // Since we just created a game, the latest index should be our game
        let new_game_count = factory.gameCount().call().await?;

        // Ensure game count actually increased
        if new_game_count <= initial_game_count + U256::from(i) {
            anyhow::bail!(
                "Game creation failed - game count did not increase. Expected > {}, got {}",
                initial_game_count + U256::from(i),
                new_game_count
            );
        }

        let game_index = new_game_count - U256::from(1);
        println!("  New game count: {}, Game index: {}", new_game_count, game_index);

        let game_info = factory.gameAtIndex(game_index).call().await?;
        let game_address = game_info.proxy_;

        invalid_games.push(game_address);

        println!("✓ Created invalid game {}: {}", i + 1, game_address);
    }

    // Verify challenger is still running
    if !challenger.is_running() {
        anyhow::bail!("Challenger service stopped unexpectedly");
    }
    println!("✓ Challenger is running");

    // Wait for challenger to detect and challenge
    println!("\n=== Waiting for Challenger to Challenge Invalid Games ===");

    // Give challenger time to process the created games
    println!("Giving challenger time to process games...");
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Check game status before waiting for challenges
    println!("\n=== Debugging Game States ===");

    // Get current L2 timestamp for comparison
    let current_l2_block = l2_provider.get_l2_block_by_number(BlockNumberOrTag::Latest).await?;
    let current_timestamp = current_l2_block.header.timestamp;
    println!("Current L2 timestamp: {}", current_timestamp);

    for (i, &game_address) in invalid_games.iter().enumerate() {
        let game = OPSuccinctFaultDisputeGame::new(game_address, &anvil.provider);
        let claim_data = game.claimData().call().await?;
        let l2_block_number = game.l2BlockNumber().call().await?;
        let root_claim = game.rootClaim().call().await?;

        println!("\nGame {} ({})", i + 1, game_address);
        println!("  L2 Block: {}", l2_block_number);
        println!("  Root Claim: 0x{}", hex::encode(&root_claim));
        println!("  Status: {} (0=Unchallenged, 1=Challenged)", claim_data.status);
        println!(
            "  Deadline: {} ({})",
            claim_data.deadline,
            if claim_data.deadline > current_timestamp { "future" } else { "past" }
        );
        println!(
            "  Time until deadline: {} seconds",
            if claim_data.deadline > current_timestamp {
                claim_data.deadline - current_timestamp
            } else {
                0
            }
        );

        // Try to compute the actual output root
        match l2_provider.compute_output_root_at_block(l2_block_number).await {
            Ok(actual_root) => {
                println!("  Actual Root: 0x{}", hex::encode(&actual_root));
                println!("  Invalid: {} (roots don't match)", actual_root != root_claim);
            }
            Err(e) => {
                println!("  Failed to compute actual root: {}", e);
            }
        }
    }

    // Check permissions
    println!("\n=== Checking Permissions ===");
    let access_manager = AccessManager::new(deployed.access_manager, anvil.provider.clone());
    let is_challenger = access_manager
        .isAllowedChallenger(address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8"))
        .call()
        .await?;
    println!("  Challenger has permission: {}", is_challenger);

    // Check challenger bond and balance
    let challenger_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;
    println!("  Challenger bond required: {} wei", challenger_bond);

    // Check challenger balance
    let challenger_address = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let challenger_balance = anvil.provider.get_balance(challenger_address).await?;
    println!("  Challenger balance: {} wei", challenger_balance);
    println!("  Challenger has enough for bond: {}", challenger_balance >= challenger_bond);

    // Give more time for challenger to process
    println!("\nWaiting additional 20 seconds for challenger to detect games...");
    tokio::time::sleep(Duration::from_secs(20)).await;

    // Check status again
    println!("\n=== Rechecking Game Statuses ===");
    for (i, &game_address) in invalid_games.iter().enumerate() {
        let game = OPSuccinctFaultDisputeGame::new(game_address, &anvil.provider);
        let claim_data = game.claimData().call().await?;
        println!("  Game {} status: {} (0=Unchallenged, 1=Challenged)", i + 1, claim_data.status);
    }

    // Check total game count and latest game
    let total_count = factory.gameCount().call().await?;
    println!("\n=== Factory State ===");
    println!("  Total game count: {}", total_count);
    if total_count > U256::ZERO {
        let latest_index = total_count - U256::from(1);
        let latest_game = factory.gameAtIndex(latest_index).call().await?;
        println!("  Latest game index: {}", latest_index);
        println!("  Latest game address: {}", latest_game.proxy_);
        println!("  Latest game type: {}", latest_game.gameType_);
    }

    let challenges =
        wait_for_challenges(&anvil.provider, &invalid_games, Duration::from_secs(300)).await?;

    println!("✓ All games challenged:");
    for (i, status) in challenges.iter().enumerate() {
        println!("  Game {}: status {}", i + 1, status);
    }

    // Time warp past max prove duration (challenger wins if no proof)
    println!("\n=== Warping Past Max Prove Duration ===");
    let prove_warp = MAX_PROVE_DURATION + 1;
    warp_time(&anvil.provider, Duration::from_secs(prove_warp)).await?;
    println!("✓ Warped time by {} seconds (past prove deadline)", prove_warp);

    // Give challenger time to detect and resolve
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify challenger wins all games
    println!("\n=== Verifying Challenger Wins ===");
    for (i, &game_address) in invalid_games.iter().enumerate() {
        let game = OPSuccinctFaultDisputeGame::new(game_address, &anvil.provider);
        let status = game.status().call().await?;

        // Status 1 = CHALLENGER_WINS
        assert_eq!(status, 1, "Game {} should have ChallengerWins status", i + 1);
        println!("✓ Game {} resolved with ChallengerWins status", i + 1);
    }

    // Stop challenger
    println!("\n=== Stopping Challenger ===");
    challenger.kill().await?;
    println!("✓ Challenger stopped gracefully");

    println!("\n=== Test Complete ===");
    println!("✓ All invalid games were successfully challenged and won by challenger");

    // Cleanup
    cleanup_anvil();
    Ok(())
}
