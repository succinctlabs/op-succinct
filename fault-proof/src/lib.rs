pub mod challenger;
pub mod config;
pub mod contract;
pub mod prometheus;
pub mod proposer;

use std::collections::HashMap;

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{address, keccak256, Address, FixedBytes, B256, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types_eth::Block;
use alloy_sol_types::SolValue;
use anyhow::{bail, Result};
use async_trait::async_trait;
use op_alloy_network::Optimism;
use op_alloy_rpc_types::Transaction;
use tokio::sync::Mutex;

use crate::contract::{
    AnchorStateRegistry, DisputeGameFactory::DisputeGameFactoryInstance, GameStatus, IDisputeGame,
    L2Output, OPSuccinctFaultDisputeGame, ProposalStatus,
};

pub type L1Provider = RootProvider;
pub type L2Provider = RootProvider<Optimism>;
pub type L2NodeProvider = RootProvider<Optimism>;

pub const NUM_CONFIRMATIONS: u64 = 3;
pub const TIMEOUT_SECONDS: u64 = 60;
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

    /// Get the anchor state registry address.
    async fn get_anchor_state_registry_address(&self, game_type: u32) -> Result<Address>;

    /// Get the anchor L2 block number.
    ///
    /// This function returns the L2 block number of the anchor game for a given game type.
    async fn get_anchor_l2_block_number(&self, game_type: u32) -> Result<U256>;

    /// Check if a game is finalized.
    async fn is_game_finalized(&self, game_type: u32, game_address: Address) -> Result<bool>;
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
}

#[derive(Clone)]
struct Game {
    index: U256,
    address: Address,
    game_type: u32,
    l2_block_number: U256,
    status: GameStatus,
    proposal_status: ProposalStatus,
    should_attempt_to_challenge: bool,
    should_attempt_to_resolve: bool,
    should_attempt_to_claim_bond: bool,
}

pub struct ChallengerState {
    cursor: Option<U256>,
    games: HashMap<U256, Game>,
}

// TODO(fakedev9999): traitify state.
// TODO(fakedev9999): check if there's any corner cases where the game we're checking is not
// opsuccinct fault dispute game.
async fn is_parent_resolved(
    state: &Mutex<ChallengerState>,
    game_index: U256,
    l1_provider: L1Provider,
) -> Result<bool> {
    let state = state.lock().await;
    let Some(game) = state.games.get(&game_index) else {
        return Ok(false);
    };

    let game_contract = OPSuccinctFaultDisputeGame::new(game.address, l1_provider.clone());
    let parent_index = game_contract.claimData().call().await?.parentIndex;

    let parent_game = state.games.get(&U256::from(parent_index));
    // If parent_game is Some, instantiate the IDisputeGame contract.
    // If parent_game is None return false.
    let parent_game_contract = if let Some(parent_game) = parent_game {
        IDisputeGame::new(parent_game.address, l1_provider)
    } else {
        return Ok(false);
    };

    if parent_index == u32::MAX ||
        parent_game_contract.status().call().await? != GameStatus::IN_PROGRESS
    {
        Ok(true)
    } else {
        Ok(false)
    }
}
