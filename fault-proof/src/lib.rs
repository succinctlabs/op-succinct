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

    /// Fetches the game address by index.
    async fn fetch_game_address_by_index(&self, game_index: U256) -> Result<Address>;

    /// Get the latest valid proposal.
    ///
    /// This function checks from the latest game to the earliest game, returning the latest valid
    /// proposal.
    async fn get_latest_valid_proposal(
        &self,
        l2_provider: L2Provider,
    ) -> Result<Option<(U256, U256)>>;

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
    ) -> Result<bool>;

    /// Get the oldest game address with a given condition.
    async fn get_oldest_game_address<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        status_check: S,
        output_root_check: O,
        log_message: &str,
    ) -> Result<Option<Address>>
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
    ) -> Result<Option<Address>>;

    /// Get the oldest defensible game address.
    ///
    /// Defensible games are games with valid claims that have been challenged but have not been
    /// proven yet.
    ///
    /// This function checks a window of recent games, starting from
    /// (latest_game_index - max_games_to_check_for_defense) up to latest_game_index.
    async fn get_oldest_defensible_game_address(
        &self,
        max_games_to_check_for_defense: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
    ) -> Result<Option<Address>>;

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
    ) -> Result<Option<Address>>;

    /// Determines whether to attempt resolution or not. The `oldest_game_index` is configured
    /// to be `latest_game_index` - `max_games_to_check_for_resolution`.
    ///
    /// If the oldest game has no parent (i.e., it's a first game), we always attempt resolution.
    /// For other games, we only attempt resolution if the parent game is not in progress.
    ///
    /// NOTE(fakedev9999): Needs to be updated considering more complex cases where there are
    ///                    multiple branches of games.
    async fn should_attempt_resolution(&self, oldest_game_index: U256) -> Result<(bool, Address)>;

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

    /// Fetches the game address by index.
    async fn fetch_game_address_by_index(&self, game_index: U256) -> Result<Address> {
        let game = self.gameAtIndex(game_index).call().await?.proxy;
        Ok(game)
    }

    /// Get the latest valid proposal.
    ///
    /// This function checks from the latest game to the earliest game, returning the latest valid
    /// proposal.
    async fn get_latest_valid_proposal(
        &self,
        l2_provider: L2Provider,
    ) -> Result<Option<(U256, U256)>> {
        // Get latest game index, return None if no games exist.
        let Some(mut game_index) = self.fetch_latest_game_index().await? else {
            return Ok(None);
        };

        let mut block_number;

        // Loop through games in reverse order (latest to earliest) to find the most recent valid
        // game.
        loop {
            // Get the game contract for the current index.
            let game_address = self.fetch_game_address_by_index(game_index).await?;
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.provider());

            // Get the L2 block number the game is proposing output for.
            block_number = game.l2BlockNumber().call().await?;
            tracing::debug!(
                "Checking if game {:?} at block {:?} is valid",
                game_address,
                block_number
            );

            // Get the output root the game is proposing.
            let game_claim = game.rootClaim().call().await?;

            // Compute the actual output root at the L2 block number.
            let output_root = l2_provider.compute_output_root_at_block(block_number).await?;

            // If the output root matches the game claim, we've found the latest valid proposal.
            if output_root == game_claim {
                break;
            }

            // If the output root doesn't match the game claim, we need to find earlier games.
            tracing::info!(
                "Output root {:?} is not same as game claim {:?}",
                output_root,
                game_claim
            );

            // If we've reached index 0 (the earliest game) and still haven't found a valid
            // proposal. Return `None` as no valid proposals were found.
            if game_index == U256::ZERO {
                tracing::info!("No valid proposals found after checking all games");
                return Ok(None);
            }

            // Decrement the game index to check the previous game.
            game_index -= U256::from(1);
        }

        tracing::info!(
            "Latest valid proposal at game index {:?} with l2 block number: {:?}",
            game_index,
            block_number
        );

        Ok(Some((block_number, game_index)))
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

    async fn get_oldest_game_address<S, O>(
        &self,
        max_games_to_check: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
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
            let game_address = self.fetch_game_address_by_index(game_index).await?;
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.provider());
            let claim_data = game.claimData().call().await?;

            if !status_check(claim_data.status) {
                tracing::debug!(
                    "Game {:?} at index {:?} does not match status criteria, skipping",
                    game_address,
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
                    game_address,
                    game_index,
                    deadline
                );
                game_index += U256::from(1);
                continue;
            }

            let block_number = game.l2BlockNumber().call().await?;
            let game_claim = game.rootClaim().call().await?;
            let output_root = l2_provider.compute_output_root_at_block(block_number).await?;

            if output_root_check(output_root, game_claim) {
                tracing::info!(
                    "{} {:?} at game index {:?} with L2 block number: {:?}",
                    log_message,
                    game_address,
                    game_index,
                    block_number
                );
                return Ok(Some(game_address));
            }

            game_index += U256::from(1);
        }

        Ok(None)
    }

    /// Get the oldest challengable game address.
    async fn get_oldest_challengable_game_address(
        &self,
        max_games_to_check_for_challenge: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
    ) -> Result<Option<Address>> {
        self.get_oldest_game_address(
            max_games_to_check_for_challenge,
            l1_provider,
            l2_provider,
            |status| status == ProposalStatus::Unchallenged,
            |output_root, game_claim| output_root != game_claim,
            "Oldest challengable game",
        )
        .await
    }

    /// Get the oldest defensible game address.
    async fn get_oldest_defensible_game_address(
        &self,
        max_games_to_check_for_defense: u64,
        l1_provider: L1Provider,
        l2_provider: L2Provider,
    ) -> Result<Option<Address>> {
        self.get_oldest_game_address(
            max_games_to_check_for_defense,
            l1_provider,
            l2_provider,
            |status| status == ProposalStatus::Challenged,
            |output_root, game_claim| output_root == game_claim,
            "Oldest defensible game",
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
            let game_address = self.fetch_game_address_by_index(index).await?;
            if self.is_claimable(game_type, game_address, claimant).await? {
                return Ok(Some(game_address));
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
    async fn should_attempt_resolution(&self, oldest_game_index: U256) -> Result<(bool, Address)> {
        let oldest_game_address = self.fetch_game_address_by_index(oldest_game_index).await?;
        let oldest_game = OPSuccinctFaultDisputeGame::new(oldest_game_address, self.provider());
        let parent_game_index = oldest_game.claimData().call().await?.parentIndex;

        // Always attempt resolution for first games (those with parent_game_index == u32::MAX).
        // For other games, only attempt if the oldest game's parent game is resolved.
        if parent_game_index == u32::MAX {
            Ok((true, oldest_game_address))
        } else {
            let parent_game_address =
                self.fetch_game_address_by_index(U256::from(parent_game_index)).await?;
            let parent_game = OPSuccinctFaultDisputeGame::new(parent_game_address, self.provider());

            Ok((parent_game.status().call().await? != GameStatus::IN_PROGRESS, oldest_game_address))
        }
    }

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
    ) -> Result<Action> {
        let game_address = self.fetch_game_address_by_index(index).await?;
        let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider.clone());
        if game.status().call().await? != GameStatus::IN_PROGRESS {
            tracing::debug!(
                "Game {:?} at index {:?} is not in progress, not attempting resolution",
                game_address,
                index
            );
            return Ok(Action::Skipped);
        }

        let claim_data = game.claimData().call().await?;
        match mode {
            Mode::Proposer => {
                if claim_data.status != ProposalStatus::Unchallenged {
                    tracing::debug!(
                        "Game {:?} at index {:?} is not unchallenged, not attempting resolution",
                        game_address,
                        index
                    );
                    return Ok(Action::Skipped);
                }
            }
            Mode::Challenger => {
                if claim_data.status != ProposalStatus::Challenged {
                    tracing::debug!(
                        "Game {:?} at index {:?} is not challenged, not attempting resolution",
                        game_address,
                        index
                    );
                    return Ok(Action::Skipped);
                }
            }
        }

        let current_timestamp = l1_provider
            .get_block(alloy_eips::BlockId::Number(BlockNumberOrTag::Latest))
            .await?
            .unwrap()
            .header
            .timestamp;
        let deadline = U256::from(claim_data.deadline).to::<u64>();
        if deadline >= current_timestamp {
            tracing::debug!(
                "Game {:?} at index {:?} deadline {:?} has not passed, not attempting resolution",
                game_address,
                index,
                deadline
            );
            return Ok(Action::Skipped);
        }

        let contract = OPSuccinctFaultDisputeGame::new(game_address, self.provider());
        let transaction_request = contract.resolve().into_transaction_request();
        let receipt = signer.send_transaction_request(l1_rpc, transaction_request).await?;
        tracing::info!(
            "\x1b[1mSuccessfully resolved game {:?} at index {:?} with tx {:?}\x1b[0m",
            game_address,
            index,
            receipt.transaction_hash
        );
        Ok(Action::Performed)
    }

    /// Attempts to resolve games, up to `max_games_to_check_for_resolution`.
    async fn resolve_games(
        &self,
        mode: Mode,
        max_games_to_check_for_resolution: u64,
        signer: Signer,
        l1_rpc: Url,
        l1_provider: L1Provider,
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

        let (should_attempt_resolution, game_address) =
            self.should_attempt_resolution(oldest_game_index).await?;

        if should_attempt_resolution {
            for i in 0..games_to_check.to::<u64>() {
                let index = oldest_game_index + U256::from(i);
                if let Ok(Action::Performed) = self
                    .try_resolve_games(
                        index,
                        mode,
                        signer.clone(),
                        l1_rpc.clone(),
                        l1_provider.clone(),
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
        } else {
            tracing::info!(
                "Oldest game {:?} at index {:?} has unresolved parent, not attempting resolution",
                game_address,
                oldest_game_index
            );
        }

        Ok(())
    }
}
