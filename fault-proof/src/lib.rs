pub mod challenger;
pub mod config;
pub mod contract;
pub mod prometheus;
pub mod proposer;

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{address, keccak256, Address, FixedBytes, B256, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types_eth::Block;
use alloy_sol_types::SolValue;
use alloy_transport_http::reqwest::Url;
use anyhow::{bail, Result};
use async_trait::async_trait;
use op_alloy_network::Optimism;
use op_alloy_rpc_types::Transaction;
use op_succinct_signer_utils::Signer;

use crate::{
    contract::{
        AnchorStateRegistry, DisputeGameFactory::DisputeGameFactoryInstance, GameStatus, L2Output,
        OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    prometheus::{ChallengerGauge, ProposerGauge},
};
use op_succinct_host_utils::metrics::MetricsGauge;

pub type L1Provider = RootProvider;
pub type L2Provider = RootProvider<Optimism>;
pub type L2NodeProvider = RootProvider<Optimism>;

pub const NUM_CONFIRMATIONS: u64 = 3;
pub const TIMEOUT_SECONDS: u64 = 60;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Proposer,
    Challenger,
}

#[derive(Debug)]
pub enum Action {
    Performed,
    Skipped,
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub game_type: u32,
    pub address: Address,
}

#[async_trait]
pub trait L2ProviderTrait {
    /// Get the L2 block by number.
    async fn get_l2_block_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> Result<Block<Transaction>>;

    /// Get the L2 storage root for an address at a given block number.
    async fn get_l2_storage_root(
        &self,
        address: Address,
        block_number: BlockNumberOrTag,
    ) -> Result<B256>;

    /// Compute the output root at a given L2 block number.
    async fn compute_output_root_at_block(&self, l2_block_number: U256) -> Result<FixedBytes<32>>;
}

#[async_trait]
impl L2ProviderTrait for L2Provider {
    /// Get the L2 block by number.
    async fn get_l2_block_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> Result<Block<Transaction>> {
        let block = self.get_block_by_number(block_number).await?;
        if let Some(block) = block {
            Ok(block)
        } else {
            bail!("Failed to get L2 block by number");
        }
    }

    /// Get the L2 storage root for an address at a given block number.
    async fn get_l2_storage_root(
        &self,
        address: Address,
        block_number: BlockNumberOrTag,
    ) -> Result<B256> {
        let storage_root =
            self.get_proof(address, Vec::new()).block_id(block_number.into()).await?.storage_hash;
        Ok(storage_root)
    }

    /// Compute the output root at a given L2 block number.
    ///
    /// Local implementation is used because the RPC method `optimism_outputAtBlock` can fail for
    /// older blocks if the L2 node isn't fully synced or has pruned historical state data.
    ///
    /// Common error: "missing trie node ... state is not available".
    async fn compute_output_root_at_block(&self, l2_block_number: U256) -> Result<FixedBytes<32>> {
        let l2_block = self
            .get_l2_block_by_number(BlockNumberOrTag::Number(l2_block_number.to::<u64>()))
            .await?;
        let l2_state_root = l2_block.header.state_root;
        let l2_claim_hash = l2_block.header.hash;
        let l2_storage_root = self
            .get_l2_storage_root(
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
}

#[async_trait]
pub trait FactoryTrait<P>
where
    P: Provider + Clone,
{
    /// Fetches the bond required to create a game.
    async fn fetch_init_bond(&self, game_type: u32) -> Result<U256>;

    /// Fetches the challenger bond required to challenge a game.
    async fn fetch_challenger_bond(&self, game_type: u32) -> Result<U256>;

    /// Fetches the latest game index.
    async fn fetch_latest_game_index(&self) -> Result<Option<U256>>;

    /// Fetches the game by index.
    async fn fetch_game_by_index(&self, game_index: U256) -> Result<Game>;

    /// Get the anchor state registry address.
    async fn get_anchor_state_registry_address(&self, game_type: u32) -> Result<Address>;

    /// Get the anchor L2 block number.
    ///
    /// This function returns the L2 block number of the anchor game for a given game type.
    async fn get_anchor_l2_block_number(&self, game_type: u32) -> Result<U256>;

    /// Check if a game is finalized.
    async fn is_game_finalized(&self, game_type: u32, game_address: Address) -> Result<bool>;

    /// Check if a game is claimable.
    async fn is_claimable(
        &self,
        game_type: u32,
        game_address: Address,
        claimant: Address,
        mode: Mode,
    ) -> Result<bool>;

    /// Get the oldest game address with a given condition.
    #[allow(clippy::too_many_arguments)]
    async fn get_oldest_game_address<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
        status_check: S,
        output_root_check: O,
        log_message: &str,
    ) -> Result<Option<Address>>
    where
        S: Fn(ProposalStatus) -> bool + Send + Sync,
        O: Fn(B256, B256) -> bool + Send + Sync;

    /// Get all game addresses with a given condition.
    #[allow(clippy::too_many_arguments)]
    async fn get_game_addresses<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
        status_check: S,
        output_root_check: O,
        log_message: &str,
    ) -> Result<Vec<Address>>
    where
        S: Fn(ProposalStatus) -> bool + Send + Sync,
        O: Fn(B256, B256) -> bool + Send + Sync;

    /// Get the oldest challengable game address.
    ///
    /// This function checks a window of recent games, starting from.
    /// (latest_game_index - max_games_to_check_for_challenge) up to latest_game_index.
    async fn get_oldest_challengable_game_address(
        &self,
        max_games_to_check_for_challenge: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
    ) -> Result<Option<Address>>;

    /// Get the oldest game address with claimable bonds.
    ///
    /// Claimable games are games that have been finalized and have a determined bond distribution
    /// mode. Check if the game is finalized by checking if it's not in progress (status is
    /// Resolved).
    ///
    /// This function checks a window of recent games, starting from
    /// (latest_game_index - max_games_to_check_for_bond_claiming) up to latest_game_index.
    ///
    /// The mode parameter determines which games are claimable:
    /// - Proposer mode: only games where DEFENDER_WINS
    /// - Challenger mode: only games where CHALLENGER_WINS
    async fn get_oldest_claimable_bond_game_address(
        &self,
        game_type: u32,
        max_games_to_check_for_bond_claiming: u64,
        claimant: Address,
        mode: Mode,
        expected_game_type: u32,
    ) -> Result<Option<Address>>;

    /// Determines whether to attempt resolution or not. The `oldest_game_index` is configured
    /// to be `latest_game_index` - `max_games_to_check_for_resolution`.
    ///
    /// If the oldest game has no parent (i.e., it's a first game), we always attempt resolution.
    /// For other games, we only attempt resolution if the parent game is not in progress.
    ///
    /// NOTE(fakedev9999): Needs to be updated considering more complex cases where there are
    ///                    multiple branches of games.
    async fn should_attempt_resolution(
        &self,
        oldest_game_index: U256,
        expected_game_type: u32,
    ) -> Result<bool>;

    /// Attempts to resolve a challenged game.
    ///
    /// This function checks if the game is in progress and challenged, and if so, attempts to
    /// resolve it.
    async fn try_resolve_games(
        &self,
        index: U256,
        mode: Mode,
        signer: Signer,
        l1_rpc: Url,
        l1_provider: L1Provider,
        expected_game_type: u32,
    ) -> Result<Action>;

    /// Attempts to resolve all challenged games that the challenger won, up to
    /// `max_games_to_check_for_resolution`.
    async fn resolve_games(
        &self,
        mode: Mode,
        max_games_to_check_for_resolution: u64,
        signer: Signer,
        l1_rpc: Url,
        l1_provider: L1Provider,
        expected_game_type: u32,
    ) -> Result<()>;
}

#[async_trait]
impl<P> FactoryTrait<P> for DisputeGameFactoryInstance<P>
where
    P: Provider + Clone,
{
    /// Fetches the bond required to create a game.
    async fn fetch_init_bond(&self, game_type: u32) -> Result<U256> {
        let init_bond = self.initBonds(game_type).call().await?;
        Ok(init_bond)
    }

    /// Fetches the challenger bond required to challenge a game.
    async fn fetch_challenger_bond(&self, game_type: u32) -> Result<U256> {
        let game_impl_address = self.gameImpls(game_type).call().await?;
        let game_impl = OPSuccinctFaultDisputeGame::new(game_impl_address, self.provider());
        let challenger_bond = game_impl.challengerBond().call().await?;
        Ok(challenger_bond)
    }

    /// Fetches the latest game index.
    async fn fetch_latest_game_index(&self) -> Result<Option<U256>> {
        let game_count = self.gameCount().call().await?;

        if game_count == U256::ZERO {
            tracing::debug!("No games exist yet");
            return Ok(None);
        }

        let latest_game_index = game_count - U256::from(1);
        tracing::debug!("Latest game index: {:?}", latest_game_index);

        Ok(Some(latest_game_index))
    }

    /// Fetches the game by index.
    async fn fetch_game_by_index(&self, game_index: U256) -> Result<Game> {
        let game = self.gameAtIndex(game_index).call().await?;
        Ok(Game { game_type: game.gameType, address: game.proxy })
    }

    /// Get the anchor state registry address.
    async fn get_anchor_state_registry_address(&self, game_type: u32) -> Result<Address> {
        let game_impl_address = self.gameImpls(game_type).call().await?;
        let game_impl = OPSuccinctFaultDisputeGame::new(game_impl_address, self.provider());
        let anchor_state_registry_address = game_impl.anchorStateRegistry().call().await?;
        Ok(anchor_state_registry_address)
    }

    /// Get the anchor L2 block number.
    ///
    /// This function returns the L2 block number of the anchor game for a given game type.
    async fn get_anchor_l2_block_number(&self, game_type: u32) -> Result<U256> {
        let anchor_state_registry_address =
            self.get_anchor_state_registry_address(game_type).await?;
        let anchor_state_registry =
            AnchorStateRegistry::new(anchor_state_registry_address, self.provider());
        let anchor_l2_block_number = anchor_state_registry.getAnchorRoot().call().await?._1;
        Ok(anchor_l2_block_number)
    }

    /// Check if a game is finalized.
    async fn is_game_finalized(&self, game_type: u32, game_address: Address) -> Result<bool> {
        let anchor_state_registry_address =
            self.get_anchor_state_registry_address(game_type).await?;
        let anchor_state_registry =
            AnchorStateRegistry::new(anchor_state_registry_address, self.provider());
        let is_finalized = anchor_state_registry.isGameFinalized(game_address).call().await?;
        Ok(is_finalized)
    }

    /// Check if a game is claimable.
    async fn is_claimable(
        &self,
        game_type: u32,
        game_address: Address,
        claimant: Address,
        mode: Mode,
    ) -> Result<bool> {
        let game = OPSuccinctFaultDisputeGame::new(game_address, self.provider());
        let claim_data = game.claimData().call().await?;

        // NOTE(fakedev9999): This is a redundant check with the is_game_finalized check below,
        // but is useful for better logging.
        if claim_data.status != ProposalStatus::Resolved {
            tracing::debug!("Game {:?} is not resolved yet", game_address);
            return Ok(false);
        }

        // Game must be finalized before claiming credit.
        if !self.is_game_finalized(game_type, game_address).await? {
            tracing::debug!("Game {:?} is resolved but not finalized", game_address);
            return Ok(false);
        }

        // Check if the game outcome matches the mode
        let game_status = game.status().call().await?;
        let is_correct_outcome = matches!(
            (mode, game_status),
            (Mode::Proposer, GameStatus::DEFENDER_WINS) |
                (Mode::Challenger, GameStatus::CHALLENGER_WINS)
        );

        if !is_correct_outcome {
            tracing::debug!(
                "Game {:?} outcome {:?} doesn't match mode {:?}",
                game_address,
                game_status,
                mode
            );
            return Ok(false);
        }

        // Claimant must have credit left to claim.
        if game.credit(claimant).call().await? == U256::ZERO {
            tracing::debug!(
                "Claimant {:?} has no credit to claim from game {:?}",
                claimant,
                game_address
            );
            return Ok(false);
        }

        Ok(true)
    }

    #[allow(clippy::too_many_arguments)]
    async fn get_oldest_game_address<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
        status_check: S,
        output_root_check: O,
        log_message: &str,
    ) -> Result<Option<Address>>
    where
        S: Fn(ProposalStatus) -> bool + Send + Sync,
        O: Fn(B256, B256) -> bool + Send + Sync,
    {
        let Some(latest_game_index) = self.fetch_latest_game_index().await? else {
            tracing::info!("No games exist yet");
            return Ok(None);
        };

        let mut game_index = latest_game_index.saturating_sub(U256::from(max_games_to_check));

        while game_index <= latest_game_index {
            let game = self.fetch_game_by_index(game_index).await?;

            if game.game_type != expected_game_type {
                tracing::debug!(
                    game_index = %game_index,
                    game_type = game.game_type,
                    expected_game_type,
                    "Skipping game with unexpected type"
                );
                game_index += U256::from(1);
                continue;
            }

            let game_contract: OPSuccinctFaultDisputeGame::OPSuccinctFaultDisputeGameInstance<&P> =
                OPSuccinctFaultDisputeGame::new(game.address, self.provider());
            let claim_data = game_contract.claimData().call().await?;

            if !status_check(claim_data.status) {
                tracing::debug!(
                    "Game {:?} at index {:?} does not match status criteria, skipping",
                    game.address,
                    game_index
                );
                game_index += U256::from(1);
                continue;
            }

            let current_timestamp = l1_provider
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .unwrap()
                .header
                .timestamp;
            let deadline = U256::from(claim_data.deadline).to::<u64>();
            if deadline < current_timestamp {
                tracing::info!(
                    "Game {:?} at index {:?} deadline {:?} has passed, skipping",
                    game.address,
                    game_index,
                    deadline
                );
                game_index += U256::from(1);
                continue;
            }

            let block_number = game_contract.l2BlockNumber().call().await?;
            let game_claim = game_contract.rootClaim().call().await?;
            let output_root = l2_provider.compute_output_root_at_block(block_number).await?;

            if output_root_check(output_root, game_claim) {
                tracing::info!(
                    "{} {:?} at game index {:?} with L2 block number: {:?}",
                    log_message,
                    game.address,
                    game_index,
                    block_number
                );
                return Ok(Some(game.address));
            }

            game_index += U256::from(1);
        }

        Ok(None)
    }

    #[allow(clippy::too_many_arguments)]
    async fn get_game_addresses<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
        status_check: S,
        output_root_check: O,
        log_message: &str,
    ) -> Result<Vec<Address>>
    where
        S: Fn(ProposalStatus) -> bool + Send + Sync,
        O: Fn(B256, B256) -> bool + Send + Sync,
    {
        let Some(latest_game_index) = self.fetch_latest_game_index().await? else {
            tracing::info!("No games exist yet");
            return Ok(vec![]);
        };

        let mut addresses = vec![];
        let mut game_index = latest_game_index.saturating_sub(U256::from(max_games_to_check));

        while game_index <= latest_game_index {
            let game = self.fetch_game_by_index(game_index).await?;
            if game.game_type != expected_game_type {
                tracing::debug!(
                    game_index = %game_index,
                    game_type = game.game_type,
                    expected_game_type,
                    "Skipping game with unexpected type"
                );
                game_index += U256::from(1);
                continue;
            }

            let game_contract = OPSuccinctFaultDisputeGame::new(game.address, self.provider());
            let claim_data = game_contract.claimData().call().await?;

            if !status_check(claim_data.status) {
                tracing::debug!(
                    "Game {:?} at index {:?} does not match status criteria, skipping",
                    game.address,
                    game_index
                );
                game_index += U256::from(1);
                continue;
            }

            let current_timestamp = l1_provider
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .unwrap()
                .header
                .timestamp;
            let deadline = U256::from(claim_data.deadline).to::<u64>();
            if deadline < current_timestamp {
                tracing::info!(
                    "Game {:?} at index {:?} deadline {:?} has passed, skipping",
                    game.address,
                    game_index,
                    deadline
                );
                game_index += U256::from(1);
                continue;
            }

            let block_number = game_contract.l2BlockNumber().call().await?;
            let game_claim = game_contract.rootClaim().call().await?;
            let output_root = l2_provider.compute_output_root_at_block(block_number).await?;

            if output_root_check(output_root, game_claim) {
                tracing::info!(
                    "{} {:?} at game index {:?} with L2 block number: {:?}",
                    log_message,
                    game.address,
                    game_index,
                    block_number
                );
                addresses.push(game.address);
            }

            game_index += U256::from(1);
        }

        Ok(addresses)
    }

    /// Get the oldest challengable game address.
    async fn get_oldest_challengable_game_address(
        &self,
        max_games_to_check_for_challenge: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        expected_game_type: u32,
    ) -> Result<Option<Address>> {
        self.get_oldest_game_address(
            max_games_to_check_for_challenge,
            l1_provider,
            l2_provider,
            expected_game_type,
            |status| status == ProposalStatus::Unchallenged,
            |output_root, game_claim| output_root != game_claim,
            "Oldest challengable game",
        )
        .await
    }

    /// Get the oldest game address with claimable bonds.
    ///
    /// Claimable games are games that have been finalized and have a determined bond distribution
    /// mode. Check if the game is finalized by checking if it's not in progress (status is
    /// Resolved).
    ///
    /// This function checks a window of recent games, starting from
    /// (latest_game_index - max_games_to_check_for_bond_claiming) up to latest_game_index.
    async fn get_oldest_claimable_bond_game_address(
        &self,
        game_type: u32,
        max_games_to_check_for_bond_claiming: u64,
        claimant: Address,
        mode: Mode,
        expected_game_type: u32,
    ) -> Result<Option<Address>> {
        let latest_game_index = match self.fetch_latest_game_index().await? {
            Some(index) => index,
            None => {
                tracing::info!("No games exist yet for bond claiming");
                return Ok(None);
            }
        };

        let oldest_game_index =
            latest_game_index.saturating_sub(U256::from(max_games_to_check_for_bond_claiming));
        let games_to_check = (latest_game_index - oldest_game_index + U256::from(1))
            .min(U256::from(max_games_to_check_for_bond_claiming))
            .to::<u64>();

        for i in 0..games_to_check {
            let index = oldest_game_index + U256::from(i);
            let game = self.fetch_game_by_index(index).await?;

            if game.game_type != expected_game_type {
                tracing::debug!(
                    game_index = %index,
                    game_type = game.game_type,
                    expected_game_type,
                    "Skipping game with unexpected type while checking bond claims"
                );
                continue;
            }

            if self.is_claimable(game_type, game.address, claimant, mode).await? {
                return Ok(Some(game.address));
            }
        }

        Ok(None)
    }

    /// Determines whether to attempt resolution or not. The `oldest_game_index` is configured
    /// to be `latest_game_index` - `max_games_to_check_for_resolution`.
    ///
    /// If the oldest game has no parent (i.e., it's a first game), we always attempt resolution.
    /// For other games, we only attempt resolution if the parent game is not in progress.
    ///
    /// NOTE(fakedev9999): Needs to be updated considering more complex cases where there are
    ///                    multiple branches of games.
    async fn should_attempt_resolution(
        &self,
        oldest_game_index: U256,
        expected_game_type: u32,
    ) -> Result<bool> {
        let oldest_game = self.fetch_game_by_index(oldest_game_index).await?;

        if oldest_game.game_type != expected_game_type {
            tracing::debug!(
                game_index = %oldest_game_index,
                game_type = oldest_game.game_type,
                expected_game_type,
                "Oldest game has unexpected type; proceeding with resolution attempts for matching games"
            );
            return Ok(true);
        }

        let oldest_game_contract =
            OPSuccinctFaultDisputeGame::new(oldest_game.address, self.provider());
        let parent_game_index = oldest_game_contract.claimData().call().await?.parentIndex;

        if parent_game_index == u32::MAX {
            return Ok(true);
        }

        let parent_game = self.fetch_game_by_index(U256::from(parent_game_index)).await?;

        if parent_game.game_type != expected_game_type {
            tracing::debug!(
                parent_index = %parent_game_index,
                game_type = parent_game.game_type,
                expected_game_type,
                "Parent game has unexpected type; treating as resolved for gating purposes"
            );
            return Ok(true);
        }

        let parent_game_contract =
            OPSuccinctFaultDisputeGame::new(parent_game.address, self.provider());
        Ok(parent_game_contract.status().call().await? != GameStatus::IN_PROGRESS)
    }

    /// Attempts to resolve a game.
    ///
    /// This function checks if the game is in progress and can be resolved by the current mode.
    async fn try_resolve_games(
        &self,
        index: U256,
        mode: Mode,
        signer: Signer,
        l1_rpc: Url,
        l1_provider: L1Provider,
        expected_game_type: u32,
    ) -> Result<Action> {
        let game = self.fetch_game_by_index(index).await?;

        if game.game_type != expected_game_type {
            tracing::debug!(
                game_index = %index,
                game_type = game.game_type,
                expected_game_type,
                "Skipping resolution for game with unexpected type"
            );
            return Ok(Action::Skipped);
        }

        let game_contract = OPSuccinctFaultDisputeGame::new(game.address, l1_provider.clone());

        // Early exit if game is not in progress
        if game_contract.status().call().await? != GameStatus::IN_PROGRESS {
            tracing::debug!(
                game_address = ?game.address,
                game_index = %index,
                "Game is not in progress, skipping"
            );
            return Ok(Action::Skipped);
        }

        let claim_data = game_contract.claimData().call().await?;
        let is_proven = matches!(
            claim_data.status,
            ProposalStatus::UnchallengedAndValidProofProvided |
                ProposalStatus::ChallengedAndValidProofProvided
        );

        // Check if this mode can resolve the game
        let can_resolve = match (mode, &claim_data.status) {
            // Proposer can resolve unchallenged or any proven games
            (Mode::Proposer, ProposalStatus::Unchallenged) => true,
            (Mode::Proposer, _) if is_proven => true,

            // Challenger can only resolve challenged games
            (Mode::Challenger, ProposalStatus::Challenged) => true,

            _ => false,
        };

        if !can_resolve {
            tracing::debug!(
                game_address = ?game.address,
                game_index = %index,
                mode = ?mode,
                status = ?claim_data.status,
                "Game cannot be resolved by current mode"
            );
            return Ok(Action::Skipped);
        }

        // Check parent game status (except for the first game, which has no parent)
        if claim_data.parentIndex != u32::MAX {
            let parent_index = U256::from(claim_data.parentIndex);
            let parent_game = self.fetch_game_by_index(parent_index).await?;

            if parent_game.game_type != expected_game_type {
                tracing::debug!(
                    game_address = ?game.address,
                    game_index = %index,
                    parent_index = %claim_data.parentIndex,
                    parent_game_type = parent_game.game_type,
                    expected_game_type,
                    "Skipping parent status check for unexpected game type"
                );
            } else {
                let parent_game_contract =
                    OPSuccinctFaultDisputeGame::new(parent_game.address, l1_provider.clone());
                let parent_status = parent_game_contract.status().call().await?;

                if parent_status == GameStatus::IN_PROGRESS {
                    tracing::debug!(
                        game_address = ?game.address,
                        game_index = %index,
                        parent_index = %claim_data.parentIndex,
                        "Cannot resolve game - parent game is still in progress"
                    );
                    return Ok(Action::Skipped);
                }
            }
        }

        // For proven games, resolve immediately without deadline check
        if is_proven {
            tracing::info!(
                game_address = ?game.address,
                game_index = %index,
                status = ?claim_data.status,
                "Game is proven, resolving immediately"
            );
        } else {
            // Check deadline for unproven games
            let current_timestamp = l1_provider
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .unwrap()
                .header
                .timestamp;

            if U256::from(claim_data.deadline).to::<u64>() >= current_timestamp {
                tracing::debug!(
                    game_address = ?game.address,
                    game_index = %index,
                    deadline = %claim_data.deadline,
                    current_timestamp = %current_timestamp,
                    "Game deadline has not passed"
                );
                return Ok(Action::Skipped);
            }
        }

        // Attempt resolution
        tracing::info!(
            game_address = ?game.address,
            game_index = %index,
            status = ?claim_data.status,
            deadline = %claim_data.deadline,
            mode = ?mode,
            action = "attempting_resolution",
            "Attempting to resolve game"
        );
        let game_contract = OPSuccinctFaultDisputeGame::new(game.address, self.provider());

        // Get L2 block number for context
        let l2_block_number = game_contract.l2BlockNumber().call().await?;

        let transaction_request = game_contract.resolve().into_transaction_request();
        match signer.send_transaction_request(l1_rpc.clone(), transaction_request).await {
            Ok(receipt) => {
                tracing::info!(
                    game_index = %index,
                    game_address = ?game.address,
                    l2_block_end = %l2_block_number,
                    tx_hash = ?receipt.transaction_hash,
                    "Game resolved successfully"
                );
                Ok(Action::Performed)
            }
            Err(e) => {
                tracing::error!(
                    game_index = %index,
                    game_address = ?game.address,
                    l2_block_end = %l2_block_number,
                    error = ?e,
                    "Game resolution failed"
                );
                Err(e)
            }
        }
    }

    /// Attempts to resolve games, up to `max_games_to_check_for_resolution`.
    async fn resolve_games(
        &self,
        mode: Mode,
        max_games_to_check_for_resolution: u64,
        signer: Signer,
        l1_rpc: Url,
        l1_provider: L1Provider,
        expected_game_type: u32,
    ) -> Result<()> {
        // Find latest game index, return early if no games exist.
        let Some(latest_game_index) = self.fetch_latest_game_index().await? else {
            tracing::info!("No games exist, skipping resolution");
            return Ok(());
        };

        // If the oldest game's parent game is not resolved, we'll not attempt resolution.
        // Except for the game without a parent, which are first games.
        let oldest_game_index =
            latest_game_index.saturating_sub(U256::from(max_games_to_check_for_resolution));
        let games_to_check = (latest_game_index - oldest_game_index + U256::from(1))
            .min(U256::from(max_games_to_check_for_resolution));

        if !self.should_attempt_resolution(oldest_game_index, expected_game_type).await? {
            tracing::info!(
                oldest_game_index = %oldest_game_index,
                "Skipping resolution: oldest game has parent still in progress"
            );
            return Ok(());
        }

        for i in 0..games_to_check.to::<u64>() {
            let index = oldest_game_index + U256::from(i);
            if let Ok(Action::Performed) = self
                .try_resolve_games(
                    index,
                    mode,
                    signer.clone(),
                    l1_rpc.clone(),
                    l1_provider.clone(),
                    expected_game_type,
                )
                .await
            {
                // Use mode-specific metrics to avoid cross-contamination
                match mode {
                    Mode::Proposer => ProposerGauge::GamesResolved.increment(1.0),
                    Mode::Challenger => ChallengerGauge::GamesResolved.increment(1.0),
                }
            }
        }

        Ok(())
    }
}
