use std::{env, time::Duration};

use fp::{
    compute_output_root_at_block, fetch_game_address_by_index, fetch_latest_game_index,
    get_l2_block_by_number, get_latest_valid_proposal, DisputeGameFactory, GameStatus, L1Provider,
    L2Provider, OPSuccinctFaultDisputeGame, ProposalStatus,
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
    sol_types::SolValue,
    transports::{http::reqwest::Url, Transport},
};
use anyhow::Result;
use op_alloy_network::EthereumWallet;
use tokio::time;

#[derive(Debug, Clone)]
pub struct ProposerConfig {
    pub l1_rpc: Url,
    pub l2_rpc: Url,
    pub factory_address: Address,
    pub proposal_interval_in_blocks: u64,
    pub fetch_interval: u64,
    pub game_type: u32,
    pub enable_game_resolution: bool,
    pub max_games_to_check_for_resolution: u64,
}

impl ProposerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            l2_rpc: env::var("L2_RPC")?.parse().expect("L2_RPC not set"),
            factory_address: env::var("FACTORY_ADDRESS")?
                .parse()
                .expect("FACTORY_ADDRESS not set"),
            proposal_interval_in_blocks: env::var("PROPOSAL_INTERVAL_IN_BLOCKS")
                .unwrap_or("1000".to_string())
                .parse()?,
            fetch_interval: env::var("FETCH_INTERVAL")
                .unwrap_or("30".to_string())
                .parse()?,
            game_type: env::var("GAME_TYPE").expect("GAME_TYPE not set").parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or("false".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or("100".to_string())
                .parse()?,
        })
    }
}

struct OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    config: ProposerConfig,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    l1_provider_with_wallet: FillProvider<F, P, T, Ethereum>,
}

impl<F, P, T> OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    pub async fn new(fill_provider: FillProvider<F, P, T, Ethereum>) -> Result<Self> {
        let config = ProposerConfig::from_env()?;

        Ok(Self {
            config: config.clone(),
            l1_provider: ProviderBuilder::default().on_http(config.l1_rpc.clone()),
            l2_provider: ProviderBuilder::default().on_http(config.l2_rpc),
            l1_provider_with_wallet: fill_provider,
        })
    }

    async fn fetch_init_bond(&self) -> Result<U256> {
        let factory =
            DisputeGameFactory::new(self.config.factory_address, self.l1_provider.clone());
        let init_bond = factory.initBonds(self.config.game_type).call().await?;
        Ok(init_bond._0)
    }

    async fn create_game(&self, l2_block_number: U256, parent_game_index: u32) -> Result<()> {
        let factory = DisputeGameFactory::new(
            self.config.factory_address,
            self.l1_provider_with_wallet.clone(),
        );

        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let receipt = factory
            .create(
                self.config.game_type,
                compute_output_root_at_block(self.l2_provider.clone(), l2_block_number).await?,
                extra_data.into(),
            )
            .value(self.fetch_init_bond().await?)
            .send()
            .await?
            .get_receipt()
            .await?;

        tracing::info!("New game created at tx: {:?}", receipt.transaction_hash);

        Ok(())
    }

    async fn should_attempt_resolution(&self, oldest_game_index: U256) -> Result<(bool, Address)> {
        let oldest_game_address = fetch_game_address_by_index(
            self.l1_provider.clone(),
            self.config.factory_address,
            oldest_game_index,
        )
        .await?;
        let oldest_game =
            OPSuccinctFaultDisputeGame::new(oldest_game_address, self.l1_provider.clone());
        let parent_game_index = oldest_game.claimData().call().await?.claimData_.parentIndex;

        // Always attempt resolution for first games (those with parent_game_index == u32::MAX)
        // For other games, only attempt if the oldest game's parent game is resolved
        if parent_game_index == u32::MAX {
            Ok((true, oldest_game_address))
        } else {
            let parent_game_address = fetch_game_address_by_index(
                self.l1_provider.clone(),
                self.config.factory_address,
                U256::from(parent_game_index),
            )
            .await?;
            let parent_game =
                OPSuccinctFaultDisputeGame::new(parent_game_address, self.l1_provider.clone());

            Ok((
                parent_game.status().call().await?.status_ != GameStatus::IN_PROGRESS,
                oldest_game_address,
            ))
        }
    }

    async fn try_resolve_unchallenged_game(&self, index: U256) -> Result<()> {
        let game_address = fetch_game_address_by_index(
            self.l1_provider.clone(),
            self.config.factory_address,
            index,
        )
        .await?;
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

        let current_timestamp =
            get_l2_block_by_number(self.l2_provider.clone(), BlockNumberOrTag::Latest)
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

    async fn resolve_unchallenged_games(&self) -> Result<()> {
        // Find latest game index, return early if no games exist
        let Some(latest_game_index) =
            fetch_latest_game_index(self.l1_provider.clone(), self.config.factory_address).await?
        else {
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

    async fn run(&self) -> Result<()> {
        tracing::debug!("Some debug message");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            let safe_l2_head_block_number =
                get_l2_block_by_number(self.l2_provider.clone(), BlockNumberOrTag::Safe)
                    .await?
                    .header
                    .number;
            tracing::info!("Safe L2 head block number: {:?}", safe_l2_head_block_number);

            let latest_valid_proposal = get_latest_valid_proposal(
                self.l1_provider.clone(),
                self.l2_provider.clone(),
                self.config.factory_address,
            )
            .await?;

            let (next_l2_block_number_for_proposal, parent_game_index) = match latest_valid_proposal
            {
                Some((latest_block, latest_game_idx)) => (
                    latest_block + U256::from(self.config.proposal_interval_in_blocks),
                    latest_game_idx.to::<u32>(),
                ),
                None => (
                    // For first game, start from safe head minus proposal interval
                    U256::from(safe_l2_head_block_number)
                        .saturating_sub(U256::from(self.config.proposal_interval_in_blocks)),
                    u32::MAX, // Use max value for first game's parent index
                ),
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

    let proposer = OPSuccinctProposer::new(l1_provider_with_wallet)
        .await
        .unwrap();
    proposer.run().await.expect("Runs in an infinite loop");
}
