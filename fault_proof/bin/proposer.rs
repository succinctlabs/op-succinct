use std::{env, time::Duration};

use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::{Address, U256},
    providers::{fillers::TxFiller, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
    transports::{http::reqwest::Url, Transport},
};
use anyhow::Result;
use op_alloy_network::EthereumWallet;
use tokio::time;

use fault_proof::{
    config::ProposerConfig,
    sol::{
        DisputeGameFactory, DisputeGameFactory::DisputeGameFactoryInstance, GameStatus,
        OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    FactoryTrait, L1Provider, L1ProviderWithWallet, L2Provider, L2ProviderTrait,
};

struct OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    config: ProposerConfig,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    l1_provider_with_wallet: L1ProviderWithWallet<F, P, T>,
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    init_bond: U256,
}

impl<F, P, T> OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    /// Creates a new challenger instance with the provided L1 provider with wallet and factory contract instance
    pub async fn new(
        l1_provider_with_wallet: L1ProviderWithWallet<F, P, T>,
        factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    ) -> Result<Self> {
        let config = ProposerConfig::from_env()?;

        Ok(Self {
            config: config.clone(),
            l1_provider: ProviderBuilder::default().on_http(config.l1_rpc.clone()),
            l2_provider: ProviderBuilder::default().on_http(config.l2_rpc),
            l1_provider_with_wallet: l1_provider_with_wallet.clone(),
            factory: factory.clone(),
            init_bond: factory.fetch_init_bond(config.game_type).await?,
        })
    }

    /// Creates a new game with the given parameters.
    ///
    /// `l2_block_number`: the L2 block number we are proposing the output root for.
    /// `parent_game_index`: the index of the parent game.
    async fn create_game(&self, l2_block_number: U256, parent_game_index: u32) -> Result<()> {
        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let receipt = self
            .factory
            .create(
                self.config.game_type,
                self.l2_provider
                    .compute_output_root_at_block(l2_block_number)
                    .await?,
                extra_data.into(),
            )
            .value(self.init_bond)
            .send()
            .await?
            .get_receipt()
            .await?;

        tracing::info!("New game created at tx: {:?}", receipt.transaction_hash);

        Ok(())
    }

    /// Determines if we should attempt resolution or not. The `oldest_game_index` is configured
    /// to be `latest_game_index`` - `max_games_to_check_for_resolution`.
    ///
    /// If the oldest game has no parent (i.e., it's a first game), we always attempt resolution.
    /// For other games, we only attempt resolution if the parent game is not in progress.
    ///
    /// NOTE(fakedev9999): Needs to be updated considering more complex cases where there are
    ///                    multiple branches of games.
    async fn should_attempt_resolution(&self, oldest_game_index: U256) -> Result<(bool, Address)> {
        let oldest_game_address = self
            .factory
            .fetch_game_address_by_index(oldest_game_index)
            .await?;
        let oldest_game =
            OPSuccinctFaultDisputeGame::new(oldest_game_address, self.l1_provider.clone());
        let parent_game_index = oldest_game.claimData().call().await?.claimData_.parentIndex;

        // Always attempt resolution for first games (those with parent_game_index == u32::MAX)
        // For other games, only attempt if the oldest game's parent game is resolved
        if parent_game_index == u32::MAX {
            Ok((true, oldest_game_address))
        } else {
            let parent_game_address = self
                .factory
                .fetch_game_address_by_index(U256::from(parent_game_index))
                .await?;
            let parent_game =
                OPSuccinctFaultDisputeGame::new(parent_game_address, self.l1_provider.clone());

            Ok((
                parent_game.status().call().await?.status_ != GameStatus::IN_PROGRESS,
                oldest_game_address,
            ))
        }
    }

    /// Attempts to resolve an unchallenged game.
    ///
    /// This function checks if the game is in progress and unchallenged, and if so, attempts to resolve it.
    async fn try_resolve_unchallenged_game(&self, index: U256) -> Result<()> {
        let game_address = self.factory.fetch_game_address_by_index(index).await?;
        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        if game.status().call().await?.status_ != GameStatus::IN_PROGRESS {
            tracing::info!(
                "Game {:?} at index {:?} is not in progress, not attempting resolution",
                game_address,
                index
            );
            return Ok(());
        }

        let claim_data = game.claimData().call().await?.claimData_;
        if claim_data.status != ProposalStatus::Unchallenged {
            tracing::info!(
                "Game {:?} at index {:?} is not unchallenged, not attempting resolution",
                game_address,
                index
            );
            return Ok(());
        }

        let current_timestamp = self
            .l2_provider
            .get_l2_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .header
            .timestamp;
        let deadline = U256::from(claim_data.deadline).to::<u64>();
        if deadline >= current_timestamp {
            tracing::info!(
                "Game {:?} at index {:?} deadline {:?} has not passed, not attempting resolution",
                game_address,
                index,
                deadline
            );
            return Ok(());
        }

        let contract =
            OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider_with_wallet.clone());
        let receipt = contract.resolve().send().await?.get_receipt().await?;
        tracing::info!(
            "Successfully resolved unchallenged game {:?} at index {:?} with tx {:?}",
            game_address,
            index,
            receipt.transaction_hash
        );
        Ok(())
    }

    /// Attempts to resolve all unchallenged games, up to `max_games_to_check_for_resolution`.
    async fn resolve_unchallenged_games(&self) -> Result<()> {
        // Find latest game index, return early if no games exist
        let Some(latest_game_index) = self.factory.fetch_latest_game_index().await? else {
            tracing::info!("No games exist, skipping resolution");
            return Ok(());
        };

        // If the oldest game's parent game is not resolved, we'll not attempt resolution.
        // Except for the game without a parent, which are first games.
        let oldest_game_index = latest_game_index
            .saturating_sub(U256::from(self.config.max_games_to_check_for_resolution));
        let games_to_check =
            latest_game_index.min(U256::from(self.config.max_games_to_check_for_resolution));

        let (should_attempt_resolution, game_address) =
            self.should_attempt_resolution(oldest_game_index).await?;

        if should_attempt_resolution {
            for i in 0..games_to_check.to::<u64>() {
                let index = oldest_game_index + U256::from(i);
                self.try_resolve_unchallenged_game(index).await?;
            }
        } else {
            tracing::info!(
                "Oldest game {:?} at index {:?} is not resolved, not attempting resolution",
                game_address,
                oldest_game_index
            );
        }

        Ok(())
    }

    /// Runs the proposer indefinitely.
    async fn run(&self) -> Result<()> {
        tracing::info!("OP Succinct Proposer running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            let safe_l2_head_block_number = self
                .l2_provider
                .get_l2_block_by_number(BlockNumberOrTag::Safe)
                .await?
                .header
                .number;
            tracing::info!("Safe L2 head block number: {:?}", safe_l2_head_block_number);

            let latest_valid_proposal = self
                .factory
                .get_latest_valid_proposal(self.l1_provider.clone(), self.l2_provider.clone())
                .await?;

            let (next_l2_block_number_for_proposal, parent_game_index) = match latest_valid_proposal
            {
                Some((latest_block, latest_game_idx)) => (
                    latest_block + U256::from(self.config.proposal_interval_in_blocks),
                    latest_game_idx.to::<u32>(),
                ),
                None => {
                    // For first game, start from genesis L2 block number + proposal interval
                    let genesis_l2_block_number = self
                        .factory
                        .get_genesis_l2_block_number(
                            self.config.game_type,
                            self.l1_provider.clone(),
                        )
                        .await?;

                    (
                        genesis_l2_block_number
                            .checked_add(U256::from(self.config.proposal_interval_in_blocks))
                            .unwrap(),
                        u32::MAX, // Use max value for first game's parent index
                    )
                }
            };

            if U256::from(safe_l2_head_block_number) > next_l2_block_number_for_proposal {
                self.create_game(next_l2_block_number_for_proposal, parent_game_index)
                    .await?;
            }

            // Only attempt game resolution if enabled
            if self.config.enable_game_resolution {
                if let Err(e) = self.resolve_unchallenged_games().await {
                    tracing::warn!("Failed to resolve unchallenged games: {:?}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging using RUST_LOG environment variable, defaulting to INFO level
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into())
            }),
        )
        .init();

    dotenv::from_filename(".env.proposer").ok();

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

    let proposer = OPSuccinctProposer::new(l1_provider_with_wallet, factory)
        .await
        .unwrap();
    proposer.run().await.expect("Runs in an infinite loop");
}
