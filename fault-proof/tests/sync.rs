use crate::common::{constants::TEST_GAME_TYPE, TestEnvironment};
use alloy_primitives::{FixedBytes, Uint, U256};
use anyhow::Result;
use fault_proof::proposer::{GameFetchResult, OPSuccinctProposer};
use op_succinct_host_utils::host::OPSuccinctHost;
use rand::Rng;
use rstest::rstest;

mod common;

async fn setup() -> Result<(
    TestEnvironment,
    OPSuccinctProposer<fault_proof::L1Provider, impl OPSuccinctHost + Clone>,
    Uint<256, 4>,
)> {
    let env = TestEnvironment::setup().await?;
    let factory = env.factory()?;
    let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;
    let proposer = env.init_proposer().await?;
    Ok((env, proposer, init_bond))
}

const M: u32 = u32::MAX;

#[rstest]
#[case::zero_games(0, &[M], &[], None, 0)]
#[case::single_game_default_interval(1, &[M], &[], Some(0), 1)]
#[case::single_game_large_interval(1, &[M], &[10], Some(0), 10)]
#[case::two_games_same_parent(2, &[M, M], &[], Some(1), 2)]
#[case::two_games_same_branch(2, &[M, 0], &[], Some(1), 2)]
#[case::three_games_same_branch(3, &[M, 0, 1], &[], Some(2), 3)]
#[case::three_games_same_parent_varying_intervals(3, &[M, M, M], &[1, 3, 2], Some(2), 3)]
#[case::three_games_same_branch_varying_intervals(3, &[M, 0, 1], &[1, 2, 3], Some(2), 6)]
#[tokio::test]
async fn test_sync_state_happy_paths(
    #[case] num_games: usize,
    #[case] parent_ids: &[u32],
    #[case] intervals: &[u64],
    #[case] expected_canonical_head_index: Option<u64>,
    #[case] expected_canonical_head_l2_block: u64,
) -> Result<()> {
    let (env, proposer, init_bond) = setup().await?;

    let mut block = env.anvil.starting_l2_block_number + intervals.first().unwrap_or(&1);
    for (i, _) in parent_ids.iter().take(num_games).enumerate() {
        let root_claim = env.compute_output_root_at_block(block).await?;
        env.create_game(root_claim, block, parent_ids[i], init_bond).await?;
        block += intervals.get(i).unwrap_or(&1);
    }

    proposer.sync_state().await?;

    let snapshot = proposer.state_snapshot().await;

    let expected_canonical_head_index = expected_canonical_head_index.map(|idx| U256::from(idx));

    assert_eq!(snapshot.games.len(), num_games, "Number of synced games should match");
    assert_eq!(
        snapshot.canonical_head_index, expected_canonical_head_index,
        "Canonical head index should match"
    );
    assert_eq!(
        U256::from(expected_canonical_head_l2_block),
        snapshot.canonical_head_l2_block - U256::from(env.anvil.starting_l2_block_number),
        "Canonical head L2 block should match"
    );

    Ok(())
}

#[tokio::test]
async fn test_sync_state_with_game_already_exists() -> Result<()> {
    let (env, proposer, init_bond) = setup().await?;

    let mut parent_id = M;
    let mut block = env.anvil.starting_l2_block_number + 1;
    for _ in 0..10 {
        let root_claim = env.compute_output_root_at_block(block).await?;
        env.create_game(root_claim, block, parent_id, init_bond).await?;
        parent_id = if parent_id == M { 0 } else { parent_id + 1 };
        block += 1;
    }

    proposer.sync_state().await?;

    for i in 0..10 {
        let fetch_result = proposer.fetch_game(U256::from(i)).await?;
        assert!(matches!(fetch_result, GameFetchResult::AlreadyExists));
    }

    Ok(())
}

#[tokio::test]
async fn test_sync_state_with_invalid_claim() -> Result<()> {
    let (env, proposer, init_bond) = setup().await?;

    let block = env.anvil.starting_l2_block_number + 1;
    let mut rng = rand::rng();
    let mut invalid_root_bytes = [0u8; 32];
    rng.fill(&mut invalid_root_bytes);
    let invalid_root = FixedBytes::<32>::from(invalid_root_bytes);
    env.create_game(invalid_root, block, M, init_bond).await?;

    proposer.sync_state().await?;

    let fetch_result = proposer.fetch_game(U256::from(0)).await?;
    assert!(matches!(fetch_result, GameFetchResult::Dropped { .. }));

    Ok(())
}
