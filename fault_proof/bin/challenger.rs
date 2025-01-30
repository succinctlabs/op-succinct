use std::{env, time::Duration};

use fp::{
    compute_output_root_at_block, fetch_game_address_by_index, fetch_latest_game_index, L1Provider,
    L2Provider, OPSuccinctFaultDisputeGame, ProposalStatus,
};

use alloy::{
    network::Ethereum,
    primitives::{Address, U256},
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider, ProviderBuilder,
    },
    signers::local::PrivateKeySigner,
    transports::{http::reqwest::Url, Transport},
};
use anyhow::Result;
use op_alloy_network::EthereumWallet;
use tokio::time;

#[derive(Debug, Clone)]
pub struct ChallengerConfig {
    pub l1_rpc: Url,
    pub l2_rpc: Url,
    pub factory_address: Address,
    pub fetch_interval: u64,
    pub max_games_to_check_for_challenge: u64,
}

impl ChallengerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            l2_rpc: env::var("L2_RPC")?.parse().expect("L2_RPC not set"),
            factory_address: env::var("FACTORY_ADDRESS")?
                .parse()
                .expect("FACTORY_ADDRESS not set"),
            fetch_interval: env::var("FETCH_INTERVAL")
                .unwrap_or("30".to_string())
                .parse()?,
            max_games_to_check_for_challenge: env::var("MAX_GAMES_TO_CHECK_FOR_CHALLENGE")
                .unwrap_or("100".to_string())
                .parse()?,
        })
    }
}

struct OPSuccicntChallenger<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    config: ChallengerConfig,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    l1_provider_with_wallet: FillProvider<F, P, T, Ethereum>,
}

impl<F, P, T> OPSuccicntChallenger<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    pub async fn new(l1_provider_with_wallet: FillProvider<F, P, T, Ethereum>) -> Result<Self> {
        let config = ChallengerConfig::from_env()?;

        Ok(Self {
            config: config.clone(),
            l1_provider: ProviderBuilder::default().on_http(config.l1_rpc.clone()),
            l2_provider: ProviderBuilder::default().on_http(config.l2_rpc.clone()),
            l1_provider_with_wallet: l1_provider_with_wallet.clone(),
        })
    }

    async fn fetch_proof_reward(&self, game_address: Address) -> Result<U256> {
        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let proof_reward = game.proofReward().call().await?.proofReward_;
        Ok(proof_reward)
    }

    async fn challenge_game(&self, game_address: Address) -> Result<()> {
        const NUM_CONFIRMATIONS: u64 = 3;
        const TIMEOUT_SECONDS: u64 = 60;

        let game =
            OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider_with_wallet.clone());

        let receipt = game
            .challenge()
            .value(self.fetch_proof_reward(game_address).await?)
            .send()
            .await?
            .with_required_confirmations(NUM_CONFIRMATIONS)
            .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
            .get_receipt()
            .await?;

        tracing::info!(
            "Successfully challenged game {:?} with tx {:?}",
            game_address,
            receipt.transaction_hash
        );

        Ok(())
    }

    pub async fn get_oldest_unchallenged_invalid_game(&self) -> Result<Option<Address>> {
        // Get latest game index, return None if no games exist
        let Some(latest_game_index) =
            fetch_latest_game_index(self.l1_provider.clone(), self.config.factory_address).await?
        else {
            tracing::info!("No games exist yet");
            return Ok(None);
        };

        // Start from the latest game index - max_games_to_check_for_challenge
        let mut game_index = latest_game_index
            .saturating_sub(U256::from(self.config.max_games_to_check_for_challenge));
        let mut game_address;
        let mut block_number;

        loop {
            game_address = fetch_game_address_by_index(
                self.l1_provider.clone(),
                self.config.factory_address,
                game_index,
            )
            .await?;
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

            let claim_data = game.claimData().call().await?.claimData_;
            if claim_data.status != ProposalStatus::Unchallenged {
                tracing::info!(
                    "Game {:?} at index {:?} is not unchallenged, not attempting challenge",
                    game_address,
                    game_index
                );
                return Ok(None);
            }

            block_number = game.l2BlockNumber().call().await?.l2BlockNumber_;
            tracing::info!(
                "Checking if game {:?} at index {:?} for block {:?} is invalid",
                game_address,
                game_index,
                block_number
            );
            let game_claim = game.rootClaim().call().await?.rootClaim_;

            let output_root =
                compute_output_root_at_block(self.l2_provider.clone(), block_number).await?;

            if output_root != game_claim {
                tracing::info!(
                    "Output root {:?} at block {:?} is not same as game claim {:?}",
                    output_root,
                    block_number,
                    game_claim
                );
                break;
            }

            // If we've reached index 0 and still haven't found a valid proposal
            if game_index == latest_game_index {
                tracing::warn!("No invalid proposals found after checking all games");
                return Ok(None);
            }

            game_index += U256::from(1);
        }

        tracing::info!(
            "Oldest invalid game {:?} at game index {:?} with l2 block number: {:?}",
            game_address,
            game_index,
            block_number
        );

        Ok(Some(game_address))
    }

    async fn run(&mut self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            if let Some(game_address) = self.get_oldest_unchallenged_invalid_game().await? {
                tracing::info!("Attempting to challenge game {:?}", game_address);
                self.challenge_game(game_address).await?;
            } else {
                tracing::info!("No invalid games found, not attempting to challenge");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging with default level info
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    dotenv::dotenv().ok();

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

    let mut challenger = OPSuccicntChallenger::new(l1_provider_with_wallet)
        .await
        .unwrap();
    challenger.run().await.expect("Runs in an infinite loop");
}
