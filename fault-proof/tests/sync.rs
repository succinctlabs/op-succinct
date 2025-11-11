mod common;

#[cfg(feature = "e2e")]
mod sync {
    use std::{collections::HashMap, time::Duration};

    use crate::common::{
        constants::{
            DISPUTE_GAME_FINALITY_DELAY_SECONDS, MAX_CHALLENGE_DURATION, MAX_PROVE_DURATION,
            TEST_GAME_TYPE,
        },
        TestEnvironment,
    };
    use alloy_primitives::{FixedBytes, Uint, U256};
    use anyhow::Result;
    use fault_proof::proposer::{GameFetchResult, OPSuccinctProposer};
    use op_succinct_host_utils::host::OPSuccinctHost;
    use rand::Rng;
    use rstest::rstest;

    const M: u32 = u32::MAX;

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

    // Naming guide used in case names:
    // - "*_noanch_*": anchored game absent.
    // - "*_anch_*": anchored game present.

    #[rstest]
    #[case::zero_games(0, &[], &[M], &[], None, 0)]
    #[case::noanch_single_game_default_interval(1, &[], &[M], &[], Some(0), 1)] // Branch: M->0, Blocks: 0->1
    #[case::noanch_single_game_large_interval(1, &[], &[M], &[10], Some(0), 10)] // Branch: M->0, Blocks: 0->10
    #[case::noanch_two_games_same_branch(2, &[], &[M, 0], &[], Some(1), 2)] // Branch: M->0->1, Blocks: 0->1->2
    #[case::noanch_two_games_same_parent_diff_intervals(2, &[], &[M, M], &[1, 2], Some(1), 2)] // Branches: M->0, M->1, Blocks: 0->1, 0->2
    #[case::noanch_three_games_same_branch(3,&[], &[M, 0, 1], &[], Some(2), 3)] // Branch: M->0->1->2, Blocks: 0->1->2->3
    #[case::noanch_three_games_same_parent_diff_intervals_1(3,&[], &[M, M, M], &[1, 2, 3], Some(2), 3)] // Branches: M->0, M->1, M->2, Blocks: 0->1, 0->2, 0->3
    #[case::noanch_three_games_same_parent_diff_intervals_2(3,&[], &[M, M, M], &[1, 3, 2], Some(1), 3)] // Branches: M->0, M->1, M->2, Blocks: 0->1, 0->3, 0->2
    #[case::noanch_three_games_same_branch_diff_intervals_1(3,&[], &[M, 0, 1], &[1, 2, 3], Some(2), 6)] // Branch: M->0->1->2, Blocks: 0->1->3->6
    #[case::noanch_three_games_same_branch_diff_intervals_2(3, &[], &[M, 0, 1], &[1, 3, 2], Some(2), 6)] // Branch: M->0->1->2, Blocks: 0->1->4->6
    #[case::noanch_three_games_two_branches_diff_intervals(3,&[], &[M, 0, M], &[1, 3, 2], Some(1), 4)] // Branches: M->0->1, M->2, Blocks: 0->1->4 and 0->2
    #[case::noanch_five_games_two_branches(5, &[], &[M, 0, 1, 0, 3], &[1, 1, 1, 2, 2], Some(4), 5)] // Branches: M->0->1->2, 0->3->4, Blocks: 0->1->2->3 and 1->3->5
    #[case::noanch_five_games_three_branches(5, &[], &[M, 0, 1, 0, 0], &[1, 1, 1, 4, 3], Some(3), 5)] // Branches: M->0->1->2, 0->3, 0->4, Blocks: 0->1->2->3, 1->5, 1->4
    #[case::anch_single_game_default_interval(1, &[0], &[M], &[], Some(0), 1)]
    #[case::anch_two_games_same_branch(2, &[0, 1], &[M, 0], &[], Some(1), 2)]
    #[case::anch_two_games_same_parent_diff_intervals(2, &[0], &[M, M], &[1, 2], Some(0), 1)]
    #[case::anch_five_games_two_branches(5, &[0, 1], &[M, 0, 1, 0, 3], &[1, 1, 1, 2, 2], Some(2), 3)]
    #[tokio::test]
    async fn test_sync_state_happy_paths(
        #[case] num_games: usize,
        #[case] anchor_ids: &[usize],
        #[case] parent_ids: &[u32],
        #[case] intervals: &[u64],
        #[case] expected_canonical_head_index: Option<u64>,
        #[case] expected_canonical_head_l2_block: u64,
    ) -> Result<()> {
        let (env, proposer, init_bond) = setup().await?;

        let mut starting_blocks: HashMap<u32, u64> = HashMap::new();

        let mut block = env.anvil.starting_l2_block_number;
        for (i, _) in parent_ids.iter().take(num_games).enumerate() {
            let cur_parent_id = parent_ids[i];
            starting_blocks.insert(cur_parent_id, block);

            let end_block = block + intervals.get(i).unwrap_or(&1);
            let root_claim = env.compute_output_root_at_block(end_block).await?;
            env.create_game(root_claim, end_block, cur_parent_id, init_bond).await?;

            let (index, address) = env.last_game_info().await?;
            tracing::info!("Created game {index} with parent {cur_parent_id}");

            if anchor_ids.contains(&i) {
                env.warp_time(MAX_CHALLENGE_DURATION + 60).await?;
                env.resolve_game(address).await?;
                env.warp_time(DISPUTE_GAME_FINALITY_DELAY_SECONDS + 60).await?;
                env.set_anchor_state(address).await?;
                tracing::info!("Anchor game set to index {index}");
            }

            // Determine the starting block for the next game
            //
            // If the next game's parent is the current game, the next game's starting block
            // is the end block of the current game.
            // Otherwise, look up the starting block from the map.
            let next_parent_id = parent_ids.get(i + 1).copied().unwrap_or(M);
            if cur_parent_id.wrapping_add(1) == next_parent_id {
                block = end_block;
            } else {
                block = *starting_blocks.get(&next_parent_id).unwrap_or(&end_block);
            }
        }

        proposer.sync_state().await?;

        let snapshot = proposer.state_snapshot().await;
        let expected_canonical_head_index =
            expected_canonical_head_index.map(|idx| U256::from(idx));
        let expected_anchor_index = if anchor_ids.is_empty() {
            None
        } else {
            Some(U256::from(*anchor_ids.iter().max().unwrap()))
        };

        assert_eq!(snapshot.anchor_index, expected_anchor_index, "Anchor index should match");
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
}
