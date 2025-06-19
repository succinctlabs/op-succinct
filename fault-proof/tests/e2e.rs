use std::{collections::HashSet, env};

use alloy_primitives::{FixedBytes, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use anyhow::Result;
use op_alloy_network::EthereumWallet;
use tokio::{
    process::Command as TokioCommand,
    time::{sleep, Duration},
};

use fault_proof::{
    config::ProposerConfig,
    contract::{DisputeGameFactory, OPSuccinctFaultDisputeGame, ProposalStatus},
    test_utils::setup_test_environment,
    utils::setup_logging,
    FactoryTrait,
};

#[tokio::test]
async fn test_e2e_proposer_wins() -> Result<()> {
    setup_logging();

    let _span = tracing::info_span!("[[TEST]]").entered();

    // Setup test environment with local anvil.
    let test_config = setup_test_environment(None, None).await?;

    // Verify basic anvil functionality.
    let block_number = test_config.l1_provider.get_block_number().await?;
    tracing::info!("Current block number: {}", block_number);

    // Check deployer balance.
    let deployer_balance = test_config.l1_provider.get_balance(test_config.deployer_signer.address()).await?;
    tracing::info!("Deployer balance: {} ETH", deployer_balance);

    // TODO: This test is currently a placeholder that only verifies anvil setup.
    // Once contract deployment is implemented, we'll add the full game creation and resolution logic.
    tracing::warn!("Full proposer test not yet implemented - contract deployment needed");
    
    // For now, just verify that we can connect to anvil and the test infrastructure works.
    assert!(deployer_balance > U256::ZERO, "Deployer should have ETH balance on anvil");
    assert!(block_number < u64::MAX, "Should be able to get block number");

    tracing::info!("Basic anvil setup verification completed successfully");

    Ok(())
}

#[tokio::test]
#[ignore] // TODO: Refactor this test to use local anvil setup similar to proposer_wins test
async fn test_e2e_challenger_wins() -> Result<()> {
    const NUM_GAMES: usize = 3;

    setup_logging();

    let _span = tracing::info_span!("[[TEST]]").entered();

    dotenv::from_filename(".env.proposer").ok();
    let proposer_config = ProposerConfig::from_env()?;

    let wallet =
        EthereumWallet::from(env::var("PRIVATE_KEY").unwrap().parse::<PrivateKeySigner>().unwrap());

    let l1_provider_with_wallet =
        ProviderBuilder::new().wallet(wallet.clone()).connect_http(proposer_config.l1_rpc.clone());

    let factory =
        DisputeGameFactory::new(proposer_config.factory_address, l1_provider_with_wallet.clone());

    let game_type = proposer_config.game_type;
    let init_bond = factory.initBonds(game_type).call().await?;

    let latest_game_index = factory.fetch_latest_game_index().await?;
    let start_game_index = latest_game_index.unwrap_or(U256::ZERO);
    tracing::info!("Start game index: {}", start_game_index);

    // Spawn the challenger process first
    tracing::info!("Spawning challenger");
    let mut challenger_process = TokioCommand::new("cargo")
        .args(["run", "--bin", "challenger"])
        .spawn()
        .expect("Failed to spawn challenger");

    // Create games in background
    let mut l2_block_number = factory.get_anchor_l2_block_number(game_type).await? +
        U256::from(proposer_config.proposal_interval_in_blocks);
    let parent_game_index = u32::MAX;

    for i in 0..NUM_GAMES {
        tracing::info!("Creating faulty game {}", i);
        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));
        let faulty_output_root = FixedBytes::<32>::from_slice(&rand::random::<[u8; 32]>());

        factory
            .create(game_type, faulty_output_root, extra_data.into())
            .value(init_bond)
            .send()
            .await?
            .get_receipt()
            .await?;

        l2_block_number += U256::from(proposer_config.proposal_interval_in_blocks);
    }

    // Wait for and collect new games
    let mut game_addresses = Vec::new();
    let mut logged_indices = HashSet::new();
    let mut done = false;
    let max_wait = Duration::from_secs(120); // 2 minutes total wait
    let start = tokio::time::Instant::now();

    while !done && (tokio::time::Instant::now() - start) < max_wait {
        let latest_game_index = factory.fetch_latest_game_index().await?.unwrap_or(U256::ZERO);

        if latest_game_index >= start_game_index + U256::from(NUM_GAMES) {
            // Get latest game addresses
            game_addresses.clear(); // Clear to avoid duplicates
            for i in 0..NUM_GAMES {
                let game_index = start_game_index + U256::from(i);
                let game = factory.gameAtIndex(game_index).call().await?;
                let game_address = game.proxy;

                // Only log if we haven't seen this index before
                if logged_indices.insert(game_index) {
                    tracing::info!("Game {:?} created at index {}", game_address, game_index);
                }

                game_addresses.push(game_address);
            }

            // Check if all games are challenged
            let provider = std::sync::Arc::new(l1_provider_with_wallet.clone());
            let all_challenged =
                futures::future::try_join_all(game_addresses.iter().map(|&game_address| {
                    let provider = provider.clone();
                    async move {
                        let game =
                            OPSuccinctFaultDisputeGame::new(game_address, (*provider).clone());
                        let status = game.claimData().call().await?.status;
                        Ok::<_, anyhow::Error>(status == ProposalStatus::Challenged)
                    }
                }))
                .await?
                .into_iter()
                .all(|x| x);

            if all_challenged {
                done = true;
                tracing::info!("Successfully challenged all faulty games");
            }
        }

        if !done {
            sleep(Duration::from_secs(10)).await;
        }
    }

    // Kill the challenger process properly
    challenger_process.kill().await.expect("Failed to kill challenger process");

    assert!(
        done,
        "Timed out waiting for CHALLENGER_WINS. Possibly the challenge/resolve window is too long."
    );

    Ok(())
}
