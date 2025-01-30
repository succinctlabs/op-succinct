use std::{env, time::Duration};

use fault_proof::{
    config::ChallengerConfig,
    contract::{
        DisputeGameFactory::{self, DisputeGameFactoryInstance},
        OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    utils::setup_logging,
    FactoryTrait, L1Provider, L1ProviderWithWallet, L2Provider, L2ProviderTrait, Mode,
};

use alloy::{
    eips::BlockNumberOrTag,
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
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    proof_reward: U256,
}

impl<F, P, T> OPSuccicntChallenger<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    pub async fn new(
        l1_provider_with_wallet: FillProvider<F, P, T, Ethereum>,
        factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    ) -> Result<Self> {
        let config = ChallengerConfig::from_env()?;
        let l1_provider = ProviderBuilder::default().on_http(config.l1_rpc.clone());

        Ok(Self {
            config: config.clone(),
            l1_provider: l1_provider.clone(),
            l2_provider: ProviderBuilder::default().on_http(config.l2_rpc.clone()),
            l1_provider_with_wallet: l1_provider_with_wallet.clone(),
            factory: factory.clone(),
            proof_reward: factory
                .fetch_proof_reward(config.game_type, l1_provider)
                .await?,
        })
    }

    async fn challenge_game(&self, game_address: Address) -> Result<()> {
        let game =
            OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider_with_wallet.clone());

        let receipt = game
            .challenge()
            .value(self.proof_reward)
            .send()
            .await?
            .get_receipt()
            .await?;

        tracing::info!(
            "Successfully challenged game {:?} with tx {:?}",
            game_address,
            receipt.transaction_hash
        );

        Ok(())
    }

    pub async fn get_oldest_challengable_game_address(&self) -> Result<Option<Address>> {
        // Get latest game index, return None if no games exist
        let Some(latest_game_index) = self.factory.fetch_latest_game_index().await? else {
            tracing::info!("No games exist yet");
            return Ok(None);
        };

        // Start from the latest game index - max_games_to_check_for_challenge
        let mut game_index = latest_game_index
            .saturating_sub(U256::from(self.config.max_games_to_check_for_challenge));
        let mut game_address;
        let mut block_number;

        loop {
            // If we've reached last index and still haven't found a valid proposal
            if game_index > latest_game_index {
                tracing::info!("No invalid proposals found after checking all games");
                return Ok(None);
            }

            game_address = self.factory.fetch_game_address_by_index(game_index).await?;
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

            let claim_data = game.claimData().call().await?.claimData_;
            if claim_data.status != ProposalStatus::Unchallenged {
                tracing::info!(
                    "Game {:?} at index {:?} is not unchallenged, not attempting to challenge",
                    game_address,
                    game_index
                );
                // return Ok(None);
                game_index += U256::from(1);
                continue;
            }

            // Check if the the game is still in the challenge window
            let current_timestamp = self
                .l2_provider
                .get_l2_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .header
                .timestamp;
            let deadline = U256::from(claim_data.deadline).to::<u64>();
            if deadline < current_timestamp {
                tracing::info!(
                    "Game {:?} at index {:?} deadline {:?} has passed, not attempting to challenge",
                    game_address,
                    game_index,
                    deadline
                );
                game_index += U256::from(1);
                continue;
            }

            block_number = game.l2BlockNumber().call().await?.l2BlockNumber_;
            tracing::info!(
                "Checking if game {:?} at index {:?} for block {:?} is invalid",
                game_address,
                game_index,
                block_number
            );
            let game_claim = game.rootClaim().call().await?.rootClaim_;

            let output_root = self
                .l2_provider
                .compute_output_root_at_block(block_number)
                .await?;

            if output_root != game_claim {
                tracing::info!(
                    "Output root {:?} at block {:?} is not same as game claim {:?}",
                    output_root,
                    block_number,
                    game_claim
                );
                break;
            }

            game_index += U256::from(1);
        }

        tracing::info!(
            "Oldest challengable game {:?} at game index {:?} with l2 block number: {:?}",
            game_address,
            game_index,
            block_number
        );

        Ok(Some(game_address))
    }

    /// Handles challenging of invalid games
    async fn handle_game_challenging(&self) -> Result<()> {
        let _span = tracing::info_span!("[[Challenging]]").entered();

        if let Some(game_address) = self.get_oldest_challengable_game_address().await? {
            tracing::info!("Attempting to challenge game {:?}", game_address);
            self.challenge_game(game_address).await?;
        }

        Ok(())
    }

    /// Handles resolution of challenged games
    async fn handle_game_resolution(&self) -> Result<()> {
        // Only resolve games if the config is enabled
        if !self.config.enable_game_resolution {
            return Ok(());
        }

        let _span = tracing::info_span!("[[Resolving]]").entered();

        self.factory
            .resolve_games(
                Mode::Challenger,
                self.config.max_games_to_check_for_resolution,
                self.l1_provider_with_wallet.clone(),
                self.l2_provider.clone(),
            )
            .await
    }

    async fn run(&mut self) -> Result<()> {
        tracing::info!("OP Succinct Challenger running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            if let Err(e) = self.handle_game_challenging().await {
                tracing::warn!("Failed to handle game challenging: {:?}", e);
            }

            if let Err(e) = self.handle_game_resolution().await {
                tracing::warn!("Failed to handle game resolution: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    setup_logging();

    dotenv::from_filename(".env.challenger").ok();

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

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")
            .unwrap()
            .parse::<Address>()
            .unwrap(),
        l1_provider_with_wallet.clone(),
    );

    let mut challenger = OPSuccicntChallenger::new(l1_provider_with_wallet, factory)
        .await
        .unwrap();
    challenger.run().await.expect("Runs in an infinite loop");
}
