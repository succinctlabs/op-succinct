mod config;

use config::Config;

use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{address, keccak256, Address, FixedBytes, B256, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::eth::Block,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
    transports::http::{reqwest::Url, Client, Http},
};
use anyhow::{bail, Result};
use op_alloy_network::{primitives::BlockTransactionsKind, EthereumWallet, Optimism};
use op_alloy_rpc_types::Transaction;
use tokio::time;

sol! {
    type GameType is uint32;
    type Claim is bytes32;
    type Timestamp is uint64;

    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    contract DisputeGameFactory {
        mapping(GameType => uint256) public initBonds;

        function gameCount() external view returns (uint256 gameCount_);

        function gameAtIndex(uint256 _index) external view returns (GameType gameType, Timestamp timestamp, IDisputeGame proxy);

        // extraData is a bytes array that contains the l2BlockNumber and parentIndex, and has length of 32 bytes and 4 bytes respectively
        function create(GameType gameType, Claim rootClaim, bytes extraData) external;
    }

    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IDisputeGame {}

    #[allow(missing_docs)]
    #[sol(rpc)]
    contract OPSuccinctFaultDisputeGame {
        function l2BlockNumber() public pure returns (uint256 l2BlockNumber_);
        function rootClaim() public pure returns (Claim rootClaim_);
        function status() public view returns (GameStatus status_);
        function claimData() public view returns (ClaimData memory claimData_);
        function resolve() external returns (GameStatus status_);
    }

    #[derive(Debug, PartialEq)]
    enum GameStatus {
        IN_PROGRESS,
        DEFENDER_WINS,
        CHALLENGER_WINS
    }

    #[derive(Debug, PartialEq)]
    enum ProposalStatus {
        Unchallenged,
        Challenged,
        UnchallengedAndValidProofProvided,
        ChallengedAndValidProofProvided,
        Resolved
    }

    #[derive(Debug)]
    struct ClaimData {
        uint32 parentIndex;
        address counteredBy;
        address prover;
        Claim claim;
        ProposalStatus status;
        uint64 deadline;
    }

    struct L2Output {
        uint64 zero;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}

async fn fetch_latest_game_index(l1_rpc: Url, factory_address: Address) -> Result<Option<U256>> {
    let l1_provider: RootProvider<Http<Client>> =
        ProviderBuilder::default().on_http(l1_rpc.clone());
    let factory = DisputeGameFactory::new(factory_address, l1_provider.clone());
    let game_count = factory.gameCount().call().await?;

    if game_count.gameCount_ == U256::ZERO {
        tracing::info!("No games exist yet");
        return Ok(None);
    }

    let latest_game_index = game_count.gameCount_ - U256::from(1);
    tracing::info!("Latest game index: {:?}", latest_game_index);

    Ok(Some(latest_game_index))
}

async fn fetch_game_address(
    l1_rpc: Url,
    factory_address: Address,
    game_index: U256,
) -> Result<Address> {
    let l1_provider: RootProvider<Http<Client>> =
        ProviderBuilder::default().on_http(l1_rpc.clone());
    let factory = DisputeGameFactory::new(factory_address, l1_provider.clone());
    let game = factory.gameAtIndex(game_index).call().await?;
    Ok(game.proxy)
}

pub async fn get_l2_block_by_number(
    l2_rpc: Url,
    block_number: BlockNumberOrTag,
) -> Result<Block<Transaction>> {
    let l2_provider: RootProvider<Http<Client>, Optimism> =
        ProviderBuilder::default().on_http(l2_rpc.clone());
    let block = l2_provider
        .get_block_by_number(block_number, BlockTransactionsKind::Hashes)
        .await?;
    if let Some(block) = block {
        Ok(block)
    } else {
        bail!("Failed to get L2 block by number");
    }
}

pub async fn get_l2_storage_root(
    l2_rpc: Url,
    address: Address,
    block_number: BlockNumberOrTag,
) -> Result<B256> {
    let l2_provider: RootProvider<Http<Client>, Optimism> =
        ProviderBuilder::default().on_http(l2_rpc.clone());
    let storage_root = l2_provider
        .get_proof(address, Vec::new())
        .block_id(block_number.into())
        .await?
        .storage_hash;
    Ok(storage_root)
}

pub async fn compute_output_root_at_block(
    l2_rpc: Url,
    l2_block_number: U256,
) -> Result<FixedBytes<32>> {
    let l2_block = get_l2_block_by_number(
        l2_rpc.clone(),
        BlockNumberOrTag::Number(l2_block_number.to::<u64>()),
    )
    .await?;
    let l2_state_root = l2_block.header.state_root;
    let l2_claim_hash = l2_block.header.hash;
    let l2_storage_root = get_l2_storage_root(
        l2_rpc.clone(),
        address!("0x4200000000000000000000000000000000000016"),
        BlockNumberOrTag::Number(l2_block_number.to::<u64>()),
    )
    .await?;

    let l2_claim_encoded = L2Output {
        zero: 0,
        l2_state_root: l2_state_root.0.into(),
        l2_storage_hash: l2_storage_root.0.into(),
        l2_claim_hash: l2_claim_hash.0.into(),
    };
    let l2_output_root = keccak256(l2_claim_encoded.abi_encode());
    Ok(l2_output_root)
}

pub async fn get_latest_valid_proposal(
    l1_rpc: Url,
    l2_rpc: Url,
    factory_address: Address,
) -> Result<Option<(U256, U256)>> {
    let provider: RootProvider<Http<Client>> = ProviderBuilder::default().on_http(l1_rpc.clone());

    // Get latest game index, return None if no games exist
    let Some(mut game_index) = fetch_latest_game_index(l1_rpc.clone(), factory_address).await?
    else {
        tracing::info!("No games exist yet");
        return Ok(None);
    };

    let mut block_number;

    loop {
        let game_address = fetch_game_address(l1_rpc.clone(), factory_address, game_index).await?;
        let game = OPSuccinctFaultDisputeGame::new(game_address, provider.clone());
        block_number = game.l2BlockNumber().call().await?.l2BlockNumber_;
        tracing::info!("Checking if proposal for block {:?} is valid", block_number);
        let game_claim = game.rootClaim().call().await?.rootClaim_;

        let output_root = compute_output_root_at_block(l2_rpc.clone(), block_number).await?;

        if output_root == game_claim {
            break;
        }
        tracing::info!(
            "Output root {:?} is not same as game claim {:?}",
            output_root,
            game_claim
        );

        // If we've reached index 0 and still haven't found a valid proposal
        if game_index == U256::ZERO {
            tracing::warn!("No valid proposals found after checking all games");
            return Ok(None);
        }

        game_index -= U256::from(1);
    }

    tracing::info!(
        "Latest valid proposal at game index {:?} with l2 block number: {:?}",
        game_index,
        block_number
    );

    Ok(Some((block_number, game_index)))
}

struct OPSuccicntProposer {
    config: Config,
    wallet: EthereumWallet,
}

impl OPSuccicntProposer {
    pub async fn new() -> Result<Self> {
        let config = Config::from_env()?;

        Ok(Self {
            config: config.clone(),
            wallet: EthereumWallet::from(
                config
                    .clone()
                    .private_key
                    .parse::<PrivateKeySigner>()
                    .unwrap(),
            ),
        })
    }

    async fn fetch_init_bond(&self) -> Result<U256> {
        let l1_provider: RootProvider<Http<Client>> =
            ProviderBuilder::default().on_http(self.config.l1_rpc.clone());
        let factory = DisputeGameFactory::new(self.config.factory_address, l1_provider.clone());
        let init_bond = factory.initBonds(self.config.game_type).call().await?;
        Ok(init_bond._0)
    }

    async fn create_game(&self, l2_block_number: U256, parent_game_index: u32) -> Result<()> {
        const NUM_CONFIRMATIONS: u64 = 3;
        const TIMEOUT_SECONDS: u64 = 60;

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(self.wallet.clone())
            .on_http(self.config.l1_rpc.clone());
        let contract = DisputeGameFactory::new(self.config.factory_address, provider.clone());

        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let receipt = contract
            .create(
                self.config.game_type,
                compute_output_root_at_block(self.config.l2_rpc.clone(), l2_block_number).await?,
                extra_data.into(),
            )
            .value(self.fetch_init_bond().await?)
            .send()
            .await?
            .with_required_confirmations(NUM_CONFIRMATIONS)
            .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
            .get_receipt()
            .await?;

        tracing::info!("New game created at tx: {:?}", receipt.transaction_hash);

        Ok(())
    }

    async fn should_attempt_resolution(&self, oldest_game_index: U256) -> Result<(bool, Address)> {
        let provider = ProviderBuilder::new().on_http(self.config.l1_rpc.clone());
        let factory = DisputeGameFactory::new(self.config.factory_address, provider.clone());
        let oldest_game_address = factory.gameAtIndex(oldest_game_index).call().await?.proxy;
        let oldest_game = OPSuccinctFaultDisputeGame::new(oldest_game_address, provider.clone());
        let parent_game_index = oldest_game.claimData().call().await?.claimData_.parentIndex;

        // Always attempt resolution for first games (those with parent_game_index == u32::MAX)
        // For other games, only attempt if the oldest game's parent game is resolved
        if parent_game_index == u32::MAX {
            Ok((true, oldest_game_address))
        } else {
            let parent_game_address = factory
                .gameAtIndex(U256::from(parent_game_index))
                .call()
                .await?
                .proxy;
            let parent_game =
                OPSuccinctFaultDisputeGame::new(parent_game_address, provider.clone());

            Ok((
                parent_game.status().call().await?.status_ != GameStatus::IN_PROGRESS,
                oldest_game_address,
            ))
        }
    }

    async fn try_resolve_unchallenged_game(&self, index: U256) -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(self.wallet.clone())
            .on_http(self.config.l1_rpc.clone());
        let factory = DisputeGameFactory::new(self.config.factory_address, provider.clone());
        let game_address = factory.gameAtIndex(index).call().await?.proxy;
        let game = OPSuccinctFaultDisputeGame::new(game_address, provider.clone());

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

        let current_block_number = provider.get_block_number().await?;
        let current_timestamp = provider
            .get_block(current_block_number.into(), BlockTransactionsKind::Hashes)
            .await?
            .unwrap()
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

        let receipt = game.resolve().send().await?.get_receipt().await?;
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
            fetch_latest_game_index(self.config.l1_rpc.clone(), self.config.factory_address)
                .await?
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

    async fn run(&mut self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            let safe_l2_head_block_number =
                get_l2_block_by_number(self.config.l2_rpc.clone(), BlockNumberOrTag::Finalized)
                    .await?
                    .header
                    .number;
            tracing::info!("Safe L2 head block number: {:?}", safe_l2_head_block_number);

            let latest_valid_proposal = get_latest_valid_proposal(
                self.config.l1_rpc.clone(),
                self.config.l2_rpc.clone(),
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
    // Initialize logging with default level info
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    dotenv::dotenv().ok();

    let mut proposer = OPSuccicntProposer::new().await.unwrap();
    proposer.run().await.expect("Runs in an infinite loop");
}
