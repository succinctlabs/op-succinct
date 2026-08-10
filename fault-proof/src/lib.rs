// The SP1 cluster proof types produce deeply nested async
// futures (e.g. `proposer::prove_game`, `prover::generate_range_proof`) whose
// layout exceeds the default recursion limit of 128. Raise it so the layout
// query can complete.
#![recursion_limit = "256"]

pub mod backup;
pub mod challenger;
pub mod config;
pub mod contract;
pub mod prometheus;
pub mod proposer;
pub mod prover;

use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::{address, keccak256, Address, FixedBytes, B256, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types_eth::{Block, Header};
use alloy_sol_types::SolValue;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use op_alloy_network::Optimism;
use op_alloy_rpc_types::Transaction;

use crate::contract::{
    DisputeGameFactory::DisputeGameFactoryInstance, GameStatus, IDisputeGame, L2Output,
    OPSuccinctFaultDisputeGame,
};

pub type L1Provider = RootProvider;
pub type L2Provider = RootProvider<Optimism>;
pub type L2NodeProvider = RootProvider<Optimism>;

/// L2ToL1MessagePasser predeploy address (OP Stack).
/// Ref: `op_alloy_consensus::L2_TO_L1_MESSAGE_PASSER_ADDRESS` (available from op-alloy v0.23+).
const L2_TO_L1_MESSAGE_PASSER: Address = address!("0x4200000000000000000000000000000000000016");

pub const NUM_CONFIRMATIONS: u64 = 3;
pub const TIMEOUT_SECONDS: u64 = 60;

/// Converts a contract-provided L2 block number without truncating values that cannot be
/// represented by the execution-layer RPC.
pub(crate) fn checked_l2_block_number(l2_block_number: U256) -> Result<u64> {
    u64::try_from(l2_block_number)
        .map_err(|_| anyhow!("L2 block number exceeds u64::MAX: {l2_block_number}"))
}

#[async_trait]
pub trait L2ProviderTrait {
    /// Get the L2 block by number.
    async fn get_l2_block_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> Result<Block<Transaction>>;

    /// Get the L2 block header by number.
    ///
    /// The default preserves compatibility for existing trait implementations. The production
    /// provider overrides it to use the header-only RPC.
    async fn get_l2_header_by_number(&self, block_number: BlockNumberOrTag) -> Result<Header> {
        Ok(self.get_l2_block_by_number(block_number).await?.header)
    }

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

    /// Get the L2 block header by number.
    async fn get_l2_header_by_number(&self, block_number: BlockNumberOrTag) -> Result<Header> {
        self.get_header_by_number(block_number)
            .await?
            .ok_or_else(|| anyhow!("Failed to get L2 block header by number"))
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
        let l2_block_number = checked_l2_block_number(l2_block_number)?;
        let l2_header =
            self.get_l2_header_by_number(BlockNumberOrTag::Number(l2_block_number)).await?;
        let l2_state_root = l2_header.state_root;
        let l2_claim_hash = l2_header.hash;
        // Post-Isthmus: withdrawals_root carries the L2ToL1MessagePasser storage root.
        // Pre-Isthmus: it's nil or EMPTY_ROOT_HASH, so fall back to eth_getProof.
        let l2_storage_root = match l2_header.withdrawals_root {
            Some(root) if root != alloy_trie::EMPTY_ROOT_HASH => root,
            _ => {
                self.get_l2_storage_root(
                    L2_TO_L1_MESSAGE_PASSER,
                    BlockNumberOrTag::Number(l2_block_number),
                )
                .await?
            }
        };

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
    /// Returns the game implementation for the given game type.
    /// Errors if the game type is not registered (zero address).
    async fn game_impl(
        &self,
        game_type: u32,
    ) -> Result<OPSuccinctFaultDisputeGame::OPSuccinctFaultDisputeGameInstance<P>>;

    /// Fetches the bond required to create a game.
    async fn fetch_init_bond(&self, game_type: u32) -> Result<U256>;

    /// Fetches the latest game index.
    async fn fetch_latest_game_index(&self, block: BlockId) -> Result<Option<U256>>;
}

#[async_trait]
impl<P> FactoryTrait<P> for DisputeGameFactoryInstance<P>
where
    P: Provider + Clone,
{
    /// Returns the game implementation for the given game type.
    /// Errors if the game type is not registered (zero address).
    async fn game_impl(
        &self,
        game_type: u32,
    ) -> Result<OPSuccinctFaultDisputeGame::OPSuccinctFaultDisputeGameInstance<P>> {
        let game_impl_address = self.gameImpls(game_type).call().await?;
        if game_impl_address == Address::ZERO {
            bail!("Game type {game_type} is not registered in the factory");
        }
        Ok(OPSuccinctFaultDisputeGame::new(game_impl_address, self.provider().clone()))
    }

    /// Fetches the bond required to create a game.
    async fn fetch_init_bond(&self, game_type: u32) -> Result<U256> {
        let init_bond = self.initBonds(game_type).call().await?;
        Ok(init_bond)
    }

    /// Fetches the latest game index.
    async fn fetch_latest_game_index(&self, block: BlockId) -> Result<Option<U256>> {
        let game_count = self.gameCount().block(block).call().await?;

        if game_count == U256::ZERO {
            tracing::debug!("No games exist yet");
            return Ok(None);
        }

        let latest_game_index = game_count - U256::from(1);
        tracing::debug!("Latest game index: {:?}", latest_game_index);

        Ok(Some(latest_game_index))
    }
}

async fn is_parent_resolved<P>(
    parent_index: u32,
    factory: &DisputeGameFactoryInstance<P>,
    pinned_block: BlockId,
) -> Result<bool>
where
    P: Provider + Clone,
{
    if parent_index == u32::MAX {
        return Ok(true);
    }

    let parent_game_address =
        factory.gameAtIndex(U256::from(parent_index)).block(pinned_block).call().await?.proxy;
    let parent_game_contract = IDisputeGame::new(parent_game_address, factory.provider());

    Ok(parent_game_contract.status().block(pinned_block).call().await? != GameStatus::IN_PROGRESS)
}

async fn is_parent_challenger_wins<P>(
    parent_index: u32,
    factory: &DisputeGameFactoryInstance<P>,
    pinned_block: BlockId,
) -> Result<bool>
where
    P: Provider + Clone,
{
    if parent_index == u32::MAX {
        return Ok(false);
    }

    let parent_game_address =
        factory.gameAtIndex(U256::from(parent_index)).block(pinned_block).call().await?.proxy;
    let parent_game_contract = IDisputeGame::new(parent_game_address, factory.provider());

    Ok(parent_game_contract.status().block(pinned_block).call().await? ==
        GameStatus::CHALLENGER_WINS)
}

/// Prefix used for transaction revert errors.
pub const TX_REVERTED_PREFIX: &str = "transaction reverted:";

/// Extension trait for checking transaction error types.
pub trait TxErrorExt {
    /// Returns true if this error indicates a transaction revert (definitive failure).
    fn is_revert(&self) -> bool;
}

impl TxErrorExt for anyhow::Error {
    fn is_revert(&self) -> bool {
        self.to_string().starts_with(TX_REVERTED_PREFIX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{rpc::client::RpcClient, transports::mock::Asserter};
    use alloy_rpc_types_eth::EIP1186AccountProofResponse;

    fn mock_l2_provider(asserter: Asserter) -> L2Provider {
        RootProvider::new(RpcClient::mocked(asserter))
    }

    fn expected_output_root(header: &Header, storage_root: B256) -> B256 {
        keccak256(
            L2Output {
                zero: 0,
                l2_state_root: header.state_root.0.into(),
                l2_storage_hash: storage_root.0.into(),
                l2_claim_hash: header.hash.0.into(),
            }
            .abi_encode(),
        )
    }

    #[test]
    fn checked_l2_block_number_accepts_u64_max() {
        assert_eq!(checked_l2_block_number(U256::from(u64::MAX)).unwrap(), u64::MAX);
    }

    #[test]
    fn checked_l2_block_number_rejects_values_above_u64_max() {
        let oversized = U256::from(u64::MAX) + U256::from(1);
        let error = checked_l2_block_number(oversized).unwrap_err();

        assert!(error.to_string().contains("exceeds u64::MAX"));
    }

    #[tokio::test]
    async fn output_root_uses_header_withdrawals_root_post_isthmus() {
        let asserter = Asserter::new();
        let provider = mock_l2_provider(asserter.clone());
        let storage_root = B256::repeat_byte(0x33);
        let mut header: Header = Header::default();
        header.hash = B256::repeat_byte(0x11);
        header.state_root = B256::repeat_byte(0x22);
        header.withdrawals_root = Some(storage_root);
        asserter.push_success(&Some(header.clone()));

        let actual = provider.compute_output_root_at_block(U256::from(7)).await.unwrap();

        assert_eq!(actual, expected_output_root(&header, storage_root));
        assert!(
            asserter.read_q().is_empty(),
            "post-Isthmus output root should only consume the header response"
        );
    }

    #[tokio::test]
    async fn output_root_fetches_proof_when_withdrawals_root_is_unavailable() {
        for withdrawals_root in [None, Some(alloy_trie::EMPTY_ROOT_HASH)] {
            let asserter = Asserter::new();
            let provider = mock_l2_provider(asserter.clone());
            let storage_root = B256::repeat_byte(0x44);
            let mut header: Header = Header::default();
            header.hash = B256::repeat_byte(0x11);
            header.state_root = B256::repeat_byte(0x22);
            header.withdrawals_root = withdrawals_root;
            let proof =
                EIP1186AccountProofResponse { storage_hash: storage_root, ..Default::default() };
            asserter.push_success(&Some(header.clone()));
            asserter.push_success(&proof);

            let actual = provider.compute_output_root_at_block(U256::from(7)).await.unwrap();

            assert_eq!(actual, expected_output_root(&header, storage_root));
            assert!(asserter.read_q().is_empty(), "pre-Isthmus proof response was not consumed");
        }
    }
}
