pub mod config;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{address, keccak256, Address, FixedBytes, B256, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::eth::Block,
    sol,
    sol_types::SolValue,
    transports::http::{reqwest::Url, Client, Http},
};
use anyhow::{bail, Result};
use op_alloy_network::{primitives::BlockTransactionsKind, Optimism};
use op_alloy_rpc_types::Transaction;

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

pub async fn fetch_latest_game_index(
    l1_rpc: Url,
    factory_address: Address,
) -> Result<Option<U256>> {
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

pub async fn fetch_game_address_by_index(
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
        let game_address =
            fetch_game_address_by_index(l1_rpc.clone(), factory_address, game_index).await?;
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
