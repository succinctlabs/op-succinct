use std::{env, time::Duration};

use fault_proof::{
    config::ChallengerConfig,
    contract::{
        DisputeGameFactory::{self, DisputeGameFactoryInstance},
        OPSuccinctFaultDisputeGame,
    },
    utils::setup_logging,
    FactoryTrait, L1Provider, L1ProviderWithWallet, L2Provider, Mode,
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
use anyhow::{Context, Result};
use op_alloy_network::EthereumWallet;
use tokio::time;

struct OPSuccinctChallenger<F, P, T>
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

impl<F, P, T> OPSuccinctChallenger<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    /// Creates a new challenger instance with the provided L1 provider with wallet and factory contract instance.
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

    /// Challenges a specific game at the given address.
    async fn challenge_game(&self, game_address: Address) -> Result<()> {
        let game =
            OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider_with_wallet.clone());

        let receipt = game
            .challenge()
            .value(self.proof_reward)
            .send()
            .await
            .context("Failed to send challenge transaction")?
            .get_receipt()
            .await
            .context("Failed to get transaction receipt for challenge")?;

        tracing::info!(
            "Successfully challenged game {:?} with tx {:?}",
            game_address,
            receipt.transaction_hash
        );

        Ok(())
    }

    /// Handles challenging of invalid games by scanning recent games for potential challenges.
    async fn handle_game_challenging(&self) -> Result<()> {
        let _span = tracing::info_span!("[[Challenging]]").entered();

        if let Some(game_address) = self
            .factory
            .get_oldest_challengable_game_address(
                self.config.max_games_to_check_for_challenge,
                self.l1_provider.clone(),
                self.l2_provider.clone(),
            )
            .await?
        {
            tracing::info!("Attempting to challenge game {:?}", game_address);
            self.challenge_game(game_address).await?;
        }

        Ok(())
    }

    /// Handles resolution of challenged games that are ready to be resolved.
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

    /// Runs the challenger in an infinite loop, periodically checking for games to challenge and resolve.
    async fn run(&mut self) -> Result<()> {
        tracing::info!("OP Succinct Challenger running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        // Each loop, check the oldest challengeable game and challenge it if it exists.
        // Eventually, all games will be challenged (as long as the rate at which games are being created is slower than the fetch interval).
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

    let mut challenger = OPSuccinctChallenger::new(l1_provider_with_wallet, factory)
        .await
        .unwrap();
    challenger.run().await.expect("Runs in an infinite loop");
}
