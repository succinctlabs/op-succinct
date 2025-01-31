#[cfg(test)]
mod e2e_on_sepolia {
    use anyhow::Result;
    use std::{env, process::Command};
    use tokio::{
        process::Command as TokioCommand,
        spawn,
        time::{sleep, Duration},
    };

    use alloy::{
        eips::BlockNumberOrTag,
        primitives::{keccak256, Address, FixedBytes, U256},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
        sol_types::SolValue,
        transports::http::reqwest::Url,
    };
    use op_alloy_network::EthereumWallet;
    use op_succinct_fp::{
        compute_output_root_at_block, config::ProposerConfig, fetch_game_address_by_index,
        fetch_latest_game_index, get_l2_block_by_number, Claim, DisputeGameFactory, GameStatus,
        L1Provider, L2Provider, OPSuccinctFaultDisputeGame, ProposalStatus,
    };

    #[tokio::test]
    async fn test_e2e_challenger_wins() -> Result<()> {
        // Initialize logging with default level info
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into()),
            )
            .init();

        // Propose a faulty proposal
        let proposer_config = ProposerConfig::from_env()?;
        let l1_provider: L1Provider =
            ProviderBuilder::default().on_http(proposer_config.l1_rpc.clone());
        let l2_provider: L2Provider =
            ProviderBuilder::default().on_http(proposer_config.l2_rpc.clone());
        let wallet = EthereumWallet::from(
            env::var("PRIVATE_KEY")
                .unwrap()
                .parse::<PrivateKeySigner>()
                .unwrap(),
        );
        let l1_provider_with_wallet = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet.clone())
            .on_http(env::var("L1_RPC").unwrap().parse::<Url>().unwrap());

        let l2_block_number = U256::from(
            get_l2_block_by_number(l2_provider.clone(), BlockNumberOrTag::Latest)
                .await?
                .header
                .number,
        );
        let parent_game_index = u32::MAX;
        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let factory = DisputeGameFactory::new(
            proposer_config.factory_address,
            l1_provider_with_wallet.clone(),
        );

        let game_type = env::var("GAME_TYPE")
            .expect("GAME_TYPE not set")
            .parse::<u32>()
            .unwrap();
        let init_bond = factory.initBonds(game_type).call().await?._0;

        let receipt = factory
            .create(
                game_type,
                FixedBytes::<32>::ZERO, // Faulty output root
                extra_data.into(),
            )
            .value(init_bond)
            .send()
            .await?
            .get_receipt()
            .await?;

        tracing::info!("Game created with tx: {:?}", receipt.transaction_hash);

        // Spawn the challenger
        let mut challenger_process = TokioCommand::new("cargo")
            .args(&["run", "--bin", "challenger", "--", "--test"])
            .spawn()
            .expect("Failed to spawn challenger");

        let challenger_handle = spawn(async move {
            challenger_process.wait().await?;
            Ok::<_, anyhow::Error>(())
        });

        let mut done = false;
        let max_wait = Duration::from_secs(60); // 1 minute total wait
        let start = tokio::time::Instant::now();

        while !done && (tokio::time::Instant::now() - start) < max_wait {
            // Sleep some seconds between checks
            sleep(Duration::from_secs(15)).await;

            //     let game =
            //         OPSuccinctFaultDisputeGame::new(game_address, l1_provider_with_wallet.clone());
            //     let proof_reward = game.proofReward().call().await?.proofReward_;
            //     let res = game
            //         .challenge()
            //         .value(proof_reward)
            //         .send()
            //         .await?
            //         .get_receipt()
            //         .await?;

            //     let status = game.claimData().call().await?.claimData_.status;
            //     if status == ProposalStatus::Challenged {
            //         done = true;
            //         println!("[TEST] Challenge successful");
            //     }
            // }

            // === 5) Cancel tasks and assert the final condition
            // ----------------------------------------------------------
            challenger_handle.abort();

            // assert!(
            //     done,
            //     "Timed out waiting for CHALLENGER_WINS. Possibly the challenge/resolve window is too long."
            // );
        }

        Ok(())
    }
}
