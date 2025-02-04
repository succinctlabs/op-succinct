use anyhow::Result;
use std::env;
use tokio::{process::Command as TokioCommand, time::Duration};

use alloy::{
    primitives::{FixedBytes, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use fault_proof::{
    config::ProposerConfig,
    sol::{DisputeGameFactory, OPSuccinctFaultDisputeGame, ProposalStatus},
};
use op_alloy_network::EthereumWallet;

#[tokio::test]
async fn test_e2e_challenger_wins() -> Result<()> {
    // Initialize logging with default level info
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Spawn the challenger
    let mut challenger_process = TokioCommand::new("cargo")
        .args(["run", "--bin", "challenger"])
        .spawn()
        .expect("Failed to spawn challenger");

    // Propose a faulty proposal
    let proposer_config = ProposerConfig::from_env()?;

    let wallet = EthereumWallet::from(
        env::var("PRIVATE_KEY")
            .unwrap()
            .parse::<PrivateKeySigner>()
            .unwrap(),
    );

    let l1_provider_with_wallet = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(proposer_config.l1_rpc.clone());

    let factory = DisputeGameFactory::new(
        proposer_config.factory_address,
        l1_provider_with_wallet.clone(),
    );

    let game_type = proposer_config.game_type;
    let init_bond = factory.initBonds(game_type).call().await?._0;

    let game_impl = OPSuccinctFaultDisputeGame::new(
        factory.gameImpls(game_type).call().await?._0,
        l1_provider_with_wallet.clone(),
    );
    let mut l2_block_number = game_impl
        .genesisL2BlockNumber()
        .call()
        .await?
        .genesisL2BlockNumber_
        + U256::from(proposer_config.proposal_interval_in_blocks);
    let parent_game_index = u32::MAX;

    let mut game_addresses = Vec::new();
    for _ in 0..3 {
        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        const NUM_CONFIRMATIONS: u64 = 1;
        const TIMEOUT_SECONDS: u64 = 60;

        let faulty_output_root = FixedBytes::<32>::from_slice(&rand::random::<[u8; 32]>());

        let receipt = factory
            .create(
                game_type,
                faulty_output_root, // Random faulty output root
                extra_data.into(),
            )
            .value(init_bond)
            .send()
            .await?
            .with_required_confirmations(NUM_CONFIRMATIONS)
            .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
            .get_receipt()
            .await?;

        let game_address = receipt.inner.logs()[0].address();

        tracing::info!(
            "Game \x1B]8;;https://sepolia.etherscan.io/address/{:?}\x07{:?}\x1B]8;;\x07 created with tx: \x1B]8;;https://sepolia.etherscan.io/tx/{:?}\x07{:?}\x1B]8;;\x07",
            game_address,
            game_address,
            receipt.transaction_hash,
            receipt.transaction_hash
        );

        game_addresses.push(game_address);

        l2_block_number += U256::from(proposer_config.proposal_interval_in_blocks);
    }

    let mut done = false;
    let max_wait = Duration::from_secs(120); // 2 minutes total wait
    let start = tokio::time::Instant::now();

    while !done && (tokio::time::Instant::now() - start) < max_wait {
        // Sleep some seconds between checks
        // sleep(Duration::from_secs(15)).await;

        // If all games' statuses are challenged, we're done
        let provider = std::sync::Arc::new(l1_provider_with_wallet.clone());
        let all_challenged =
            futures::future::try_join_all(game_addresses.iter().map(|&game_address| {
                let provider = provider.clone();
                async move {
                    let game = OPSuccinctFaultDisputeGame::new(game_address, (*provider).clone());
                    let status = game.claimData().call().await?.claimData_.status;
                    Ok::<_, anyhow::Error>(status == ProposalStatus::Challenged)
                }
            }))
            .await?
            .into_iter()
            .all(|x| x);

        if all_challenged {
            done = true;
            println!("[TEST] Successfully challenged all faulty games");
        }
    }

    // === 5) Cancel tasks and assert the final condition
    // ----------------------------------------------------------
    // Kill the challenger process properly
    challenger_process
        .kill()
        .await
        .expect("Failed to kill challenger process");

    assert!(
        done,
        "Timed out waiting for CHALLENGER_WINS. Possibly the challenge/resolve window is too long."
    );

    Ok(())
}
