pub mod common;

use std::path::PathBuf;

use alloy_primitives::{Address, B256, U256};
use fault_proof::{
    backup::{ProposerBackup, BACKUP_VERSION},
    contract::{GameStatus, ProposalStatus},
    proposer::Game,
};
use tempfile::TempDir;

/// Create a test game with the given index and parent.
fn test_game(index: u64, parent_index: u32) -> Game {
    Game {
        index: U256::from(index),
        address: Address::ZERO,
        parent_index,
        l2_block: U256::from(index + 100),
        status: GameStatus::IN_PROGRESS,
        proposal_status: ProposalStatus::Unchallenged,
        deadline: 0,
        should_attempt_to_resolve: false,
        should_attempt_to_claim_bond: false,
        aggregation_vkey: B256::ZERO,
        range_vkey_commitment: B256::ZERO,
        rollup_config_hash: B256::ZERO,
    }
}

mod validation {
    use super::*;
    use rstest::rstest;

    const M: u32 = u32::MAX;

    #[rstest]
    // Valid cases
    #[case::empty(None, &[], None, true)]
    #[case::cursor_zero_no_games(Some(0), &[], None, true)]
    #[case::single_genesis_game(Some(0), &[(0, M)], None, true)]
    #[case::chain_with_anchor(Some(1), &[(0, M), (1, 0)], Some(1), true)]
    // Orphan cases — valid because anchor-based fetch and ASR filtering create partial DAGs
    #[case::orphaned_parent(Some(1), &[(0, M), (1, 99)], None, true)]
    #[case::anchor_fetch_orphan(Some(5), &[(3, 2), (4, 3), (5, 4)], Some(3), true)]
    #[case::asr_filtered_gap(Some(4), &[(2, 1), (4, 3)], Some(4), true)]
    #[case::multiple_orphan_roots(Some(5), &[(2, 1), (5, 4)], None, true)]
    #[case::single_orphan(Some(5), &[(5, 3)], None, true)]
    #[case::all_genesis_rooted(Some(1), &[(0, M), (1, M)], Some(0), true)]
    // Invalid cases — real corruption
    #[case::cursor_without_games(Some(5), &[], None, false)]
    #[case::invalid_anchor_index(Some(0), &[(0, M)], Some(99), false)]
    fn test_validation(
        #[case] cursor: Option<u64>,
        #[case] games: &[(u64, u32)],
        #[case] anchor: Option<u64>,
        #[case] valid: bool,
    ) {
        let backup = ProposerBackup::new(
            cursor.map(U256::from),
            games.iter().map(|(idx, parent)| test_game(*idx, *parent)).collect(),
            anchor.map(U256::from),
        );

        assert_eq!(backup.validate().is_ok(), valid);
    }
}

mod persistence {
    use super::*;

    fn temp_backup_path() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("backup.json");
        (dir, path)
    }

    #[test]
    fn save_and_load_roundtrip() {
        let (_dir, path) = temp_backup_path();

        let original = ProposerBackup::new(
            Some(U256::from(5)),
            vec![test_game(0, u32::MAX), test_game(1, 0), test_game(2, 1)],
            Some(U256::from(2)),
        );

        original.save(&path).unwrap();
        let loaded = ProposerBackup::load(&path).unwrap();

        assert_eq!(loaded.version, BACKUP_VERSION);
        assert_eq!(loaded.cursor, original.cursor);
        assert_eq!(loaded.games.len(), 3);
        assert_eq!(loaded.anchor_game_index, original.anchor_game_index);
    }

    #[test]
    fn load_nonexistent_returns_none() {
        let path = PathBuf::from("/nonexistent/backup.json");
        assert!(ProposerBackup::load(&path).is_none());
    }

    #[test]
    fn load_invalid_json_returns_none() {
        let (_dir, path) = temp_backup_path();
        std::fs::write(&path, "not valid json").unwrap();
        assert!(ProposerBackup::load(&path).is_none());
    }

    #[test]
    fn load_version_mismatch_returns_none() {
        let (_dir, path) = temp_backup_path();

        let json = serde_json::json!({
            "version": BACKUP_VERSION + 1,
            "cursor": null,
            "games": [],
            "anchor_game_index": null
        });
        std::fs::write(&path, json.to_string()).unwrap();

        assert!(ProposerBackup::load(&path).is_none());
    }

    #[test]
    fn load_validation_failure_returns_none() {
        let (_dir, path) = temp_backup_path();

        let json = serde_json::json!({
            "version": BACKUP_VERSION,
            "cursor": "0x5",
            "games": [],
            "anchor_game_index": null
        });
        std::fs::write(&path, json.to_string()).unwrap();

        assert!(ProposerBackup::load(&path).is_none());
    }
}

#[cfg(feature = "integration")]
mod integration {
    use std::sync::Arc;

    use anyhow::Result;
    use tempfile::TempDir;
    use tokio::time::{sleep, Duration};
    use tracing::info;

    use alloy_primitives::U256;
    use fault_proof::backup::ProposerBackup;

    use crate::common::TestEnvironment;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_proposer_backup_persistence() -> Result<()> {
        info!("=== Test: Proposer Backup Persistence ===");

        let env = TestEnvironment::setup().await?;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Start proposer with backup enabled
        let proposer = Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 0).await?);

        let proposer_clone = proposer.clone();
        let proposer_handle = tokio::spawn(async move { proposer_clone.run().await });

        let tracked_games = env.wait_and_track_games(1, 30).await?;
        info!("Proposer created {} game(s)", tracked_games.len());

        // Wait for proposer to sync the game into its internal state
        for _ in 0..30 {
            if !proposer.state_snapshot().await.games.is_empty() {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
        let snapshot_before = proposer.state_snapshot().await;
        assert!(!snapshot_before.games.is_empty(), "Proposer should have games in state");

        // Allow backup to complete
        sleep(Duration::from_secs(2)).await;
        proposer_handle.abort();

        // Phase 2: Restart and verify backup load
        let proposer2 =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 0).await?);

        proposer2.try_init().await?;

        let snapshot_after = proposer2.state_snapshot().await;

        assert!(!snapshot_after.games.is_empty(), "Restored state should contain games");
        assert_eq!(
            snapshot_before.games.len(),
            snapshot_after.games.len(),
            "Game count should match"
        );

        info!("Backup persistence test complete");
        Ok(())
    }

    /// Tests that backup-restored games above the pinned latest index are pruned during
    /// sync, while older games survive.
    ///
    /// Deterministic setup: create game 0, mine an empty L1 block, create game 1. Then
    /// restart from backup with sync_l1_confirmations=1 so the pinned block is between
    /// game 0 and game 1. Game 0 should survive; game 1 should be pruned.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_backup_partial_prune_with_confirmations() -> Result<()> {
        info!("=== Test: Backup Partial Prune With Confirmations ===");

        let env = TestEnvironment::setup().await?;
        let starting_l2_block = env.anvil.starting_l2_block_number;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Create two games with an empty L1 block between them.
        let factory = env.factory()?;
        let init_bond = factory.initBonds(env.game_type).call().await?;

        // Game 0 at L1 block B.
        let block_0 = starting_l2_block + 1;
        let root_claim_0 = env.compute_output_root_at_block(block_0).await?;
        env.create_game(root_claim_0, block_0, u32::MAX, init_bond).await?;
        info!("✓ Created game 0 at L2 block {}", block_0);

        // Mine an empty L1 block to create a gap.
        env.warp_time(0).await?;

        // Game 1 at L1 block B+2 (after the gap).
        let block_1 = starting_l2_block + 2;
        let root_claim_1 = env.compute_output_root_at_block(block_1).await?;
        env.create_game(root_claim_1, block_1, 0, init_bond).await?;
        info!("✓ Created game 1 at L2 block {}", block_1);

        // Phase 2: Construct and save backup containing both games.
        // We build the backup directly rather than running the proposer loop, so the
        // backup content is deterministic and guaranteed to exist.
        let proposer_phase2 =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 0).await?);
        proposer_phase2.try_init().await?;
        proposer_phase2.sync_state().await?;

        let snapshot = proposer_phase2.state_snapshot().await;
        assert_eq!(snapshot.games.len(), 2, "Both games should be cached");

        // Save backup from real cached games (with correct on-chain addresses).
        let game_0 = proposer_phase2.get_game(U256::from(0)).await.expect("game 0 in cache");
        let game_1 = proposer_phase2.get_game(U256::from(1)).await.expect("game 1 in cache");

        let backup = ProposerBackup::new(
            Some(U256::from(1)), // cursor at game 1
            vec![game_0, game_1],
            None, // no anchor yet
        );
        backup.save(&backup_path)?;
        info!("Phase 2: backup saved with {} games", snapshot.games.len());

        // Phase 3: Restart from backup with confirmations=1.
        // latest is the block containing game 1. latest-1 is the gap block,
        // where only game 0 exists in the factory.
        let proposer3 =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 1).await?);
        proposer3.try_init().await?;

        // Verify: backup was restored with both games before sync prunes.
        let snapshot_restored = proposer3.state_snapshot().await;
        assert_eq!(
            snapshot_restored.games.len(),
            2,
            "Both games should be restored from backup before sync"
        );

        proposer3.sync_state().await?;

        let snapshot_after = proposer3.state_snapshot().await;
        info!("Phase 3: {} games after sync with confirmations=1", snapshot_after.games.len());

        // Game 0 should survive (exists at pinned block).
        assert_eq!(snapshot_after.games.len(), 1, "Only game 0 should survive");
        assert!(
            snapshot_after.games.iter().any(|(idx, _)| *idx == U256::from(0)),
            "Game 0 should be retained"
        );

        // Canonical head should be game 0, not game 1.
        assert_eq!(
            snapshot_after.canonical_head_index,
            Some(U256::from(0)),
            "Canonical head should be game 0"
        );

        info!("Backup partial prune test complete");
        Ok(())
    }

    /// Tests that backup restore with pinned block having zero games clears all state.
    ///
    /// Saves a backup containing a game, then restarts with confirmations large enough
    /// that the pinned block predates all game creation. Verifies the cache is fully cleared.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_backup_restore_pinned_block_no_games() -> Result<()> {
        info!("=== Test: Backup Restore With Pinned Block Having Zero Games ===");

        let env = TestEnvironment::setup().await?;
        let starting_l2_block = env.anvil.starting_l2_block_number;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Create a game on-chain and sync it into the proposer.
        let factory = env.factory()?;
        let init_bond = factory.initBonds(env.game_type).call().await?;
        let block = starting_l2_block + 1;
        let root_claim = env.compute_output_root_at_block(block).await?;
        env.create_game(root_claim, block, u32::MAX, init_bond).await?;
        info!("✓ Created game 0");

        let proposer_phase1 =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 0).await?);
        proposer_phase1.try_init().await?;
        proposer_phase1.sync_state().await?;
        let game_0 = proposer_phase1.get_game(U256::from(0)).await.expect("game 0 in cache");

        // Phase 2: Save backup with real game data.
        let backup = ProposerBackup::new(Some(U256::from(0)), vec![game_0], None);
        backup.save(&backup_path)?;

        // Phase 3: Restart with huge confirmations so pinned block is before all games.
        let proposer =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 1_000_000).await?);
        proposer.try_init().await?;

        // Verify: backup was restored with game 0.
        let snapshot_restored = proposer.state_snapshot().await;
        assert_eq!(snapshot_restored.games.len(), 1, "Game 0 should be restored from backup");

        proposer.sync_state().await?;

        let snapshot = proposer.state_snapshot().await;
        assert!(snapshot.games.is_empty(), "Games should be empty");
        assert!(snapshot.anchor_index.is_none(), "Anchor should be None");
        assert!(snapshot.canonical_head_index.is_none(), "Canonical head should be None");
        // Note: canonical_head_l2_block is intentionally not asserted — current semantics
        // only clear canonical_head_index, not the stored baseline L2 block.

        info!("Backup restore pinned block no games test complete");
        Ok(())
    }

    /// Tests that games pruned due to confirmation lag are rediscovered once confirmed.
    ///
    /// With sync_l1_confirmations=1, the first sync pins to the block before game creation
    /// — cache is empty and cursor is reset. After mining one more block, the second sync
    /// pins to the game creation block and rediscovers the game.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_backup_prune_then_rediscover_after_confirmation() -> Result<()> {
        info!("=== Test: Prune Then Rediscover After Confirmation ===");

        let env = TestEnvironment::setup().await?;
        let starting_l2_block = env.anvil.starting_l2_block_number;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Create game 0 on-chain.
        let factory = env.factory()?;
        let init_bond = factory.initBonds(env.game_type).call().await?;
        let block_0 = starting_l2_block + 1;
        let root_claim = env.compute_output_root_at_block(block_0).await?;
        env.create_game(root_claim, block_0, u32::MAX, init_bond).await?;
        info!("✓ Created game 0 at L2 block {}", block_0);

        // Sync game 0 into proposer and save real backup.
        let proposer_setup =
            Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 0).await?);
        proposer_setup.try_init().await?;
        proposer_setup.sync_state().await?;
        let game_0 = proposer_setup.get_game(U256::from(0)).await.expect("game 0");

        let backup = ProposerBackup::new(Some(U256::from(0)), vec![game_0], None);
        backup.save(&backup_path)?;

        // Restart with confirmations=1. Latest is the game creation block,
        // so pinned = latest - 1 = block before game 0. Game gets pruned.
        let proposer = Arc::new(env.new_proposer_with_options(Some(backup_path.clone()), 1).await?);
        proposer.try_init().await?;

        let snapshot_restored = proposer.state_snapshot().await;
        assert_eq!(snapshot_restored.games.len(), 1, "Game 0 restored from backup");

        proposer.sync_state().await?;

        let snapshot_pruned = proposer.state_snapshot().await;
        assert!(snapshot_pruned.games.is_empty(), "Game 0 pruned (not yet confirmed)");
        assert!(snapshot_pruned.canonical_head_index.is_none(), "No canonical head");

        // Mine one empty block so pinned = latest - 1 = game creation block.
        env.warp_time(0).await?;

        proposer.sync_state().await?;

        let snapshot_rediscovered = proposer.state_snapshot().await;
        assert_eq!(
            snapshot_rediscovered.games.len(),
            1,
            "Game 0 should be rediscovered after confirmation"
        );
        assert_eq!(
            snapshot_rediscovered.canonical_head_index,
            Some(U256::from(0)),
            "Canonical head should be game 0"
        );

        info!("Prune then rediscover test complete");
        Ok(())
    }
}
