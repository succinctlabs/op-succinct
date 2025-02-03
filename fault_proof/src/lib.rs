pub mod config;
pub mod sol;
use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::{address, keccak256, Address, FixedBytes, B256, U256},
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider, RootProvider,
    },
    rpc::types::eth::Block,
    sol_types::SolValue,
    transports::{
        http::{Client, Http},
        Transport,
    },
};
use anyhow::{bail, Result};
use op_alloy_network::{primitives::BlockTransactionsKind, Optimism};
use op_alloy_rpc_types::Transaction;

use crate::sol::{
    DisputeGameFactory::DisputeGameFactoryInstance, L2Output, OPSuccinctFaultDisputeGame,
};

pub type L1Provider = RootProvider<Http<Client>, Ethereum>;
pub type L2Provider = RootProvider<Http<Client>, Optimism>;
pub type L1ProviderWithWallet<F, P, T> = FillProvider<F, P, T, Ethereum>;

/// Fetches the bond required to create a game
pub async fn fetch_init_bond<F, P, T>(
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    game_type: u32,
) -> Result<U256>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    let init_bond = factory.initBonds(game_type).call().await?;
    Ok(init_bond._0)
}
pub async fn fetch_latest_game_index<F, P, T>(
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
) -> Result<Option<U256>>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    let game_count = factory.gameCount().call().await?;

    if game_count.gameCount_ == U256::ZERO {
        tracing::info!("No games exist yet");
        return Ok(None);
    }

    let latest_game_index = game_count.gameCount_ - U256::from(1);
    tracing::info!("Latest game index: {:?}", latest_game_index);

    Ok(Some(latest_game_index))
}

pub async fn fetch_game_address_by_index<F, P, T>(
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    game_index: U256,
) -> Result<Address>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    let game = factory.gameAtIndex(game_index).call().await?;
    Ok(game.proxy)
}

pub async fn get_genesis_l2_block_number<F, P, T>(
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    game_type: u32,
    l1_provider: L1Provider,
) -> Result<U256>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    let game_impl_address = factory.gameImpls(game_type).call().await?._0;
    let game_impl = OPSuccinctFaultDisputeGame::new(game_impl_address, l1_provider);
    let genesis_l2_block_number = game_impl
        .genesisL2BlockNumber()
        .call()
        .await?
        .genesisL2BlockNumber_;
    Ok(genesis_l2_block_number)
}

pub async fn get_l2_block_by_number(
    l2_provider: L2Provider,
    block_number: BlockNumberOrTag,
) -> Result<Block<Transaction>> {
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
    l2_provider: L2Provider,
    address: Address,
    block_number: BlockNumberOrTag,
) -> Result<B256> {
    let storage_root = l2_provider
        .get_proof(address, Vec::new())
        .block_id(block_number.into())
        .await?
        .storage_hash;
    Ok(storage_root)
}

pub async fn compute_output_root_at_block(
    l2_provider: L2Provider,
    l2_block_number: U256,
) -> Result<FixedBytes<32>> {
    let l2_block = get_l2_block_by_number(
        l2_provider.clone(),
        BlockNumberOrTag::Number(l2_block_number.to::<u64>()),
    )
    .await?;
    let l2_state_root = l2_block.header.state_root;
    let l2_claim_hash = l2_block.header.hash;
    let l2_storage_root = get_l2_storage_root(
        l2_provider.clone(),
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

pub async fn get_latest_valid_proposal<F, P, T>(
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
) -> Result<Option<(U256, U256)>>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    // Get latest game index, return None if no games exist
    let Some(mut game_index) = fetch_latest_game_index(factory.clone()).await? else {
        tracing::info!("No games exist yet");
        return Ok(None);
    };

    let mut block_number;

    loop {
        let game_address = fetch_game_address_by_index(factory.clone(), game_index).await?;
        let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider.clone());
        block_number = game.l2BlockNumber().call().await?.l2BlockNumber_;
        tracing::info!("Checking if proposal for block {:?} is valid", block_number);
        let game_claim = game.rootClaim().call().await?.rootClaim_;

        let output_root = compute_output_root_at_block(l2_provider.clone(), block_number).await?;

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
