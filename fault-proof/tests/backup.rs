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

    use crate::common::{new_proposer, new_proposer_with_confirmations, TestEnvironment};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_proposer_backup_persistence() -> Result<()> {
        info!("=== Test: Proposer Backup Persistence ===");

        let env = TestEnvironment::setup().await?;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Start proposer with backup enabled
        let proposer = Arc::new(
            new_proposer(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
            )
            .await?,
        );

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
        let proposer2 = Arc::new(
            new_proposer(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
            )
            .await?,
        );

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

    /// Tests that backup-restored games above the pinned latest index are pruned during sync.
    ///
    /// Simulates a restart with SYNC_L1_CONFIRMATIONS > 0 where the pinned block is behind
    /// the newest restored games. Verifies that sync_state succeeds and the pruned games
    /// don't appear in the snapshot or affect canonical head selection.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_backup_prune_future_games_with_confirmations() -> Result<()> {
        info!("=== Test: Backup Prune Future Games With Confirmations ===");

        let env = TestEnvironment::setup().await?;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Create games and persist backup with confirmations=0.
        let proposer = Arc::new(
            new_proposer(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
            )
            .await?,
        );

        let proposer_clone = proposer.clone();
        let proposer_handle = tokio::spawn(async move { proposer_clone.run().await });

        env.wait_and_track_games(2, 60).await?;

        for _ in 0..30 {
            if proposer.state_snapshot().await.games.len() >= 2 {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
        let snapshot_before = proposer.state_snapshot().await;
        let game_count_before = snapshot_before.games.len();
        assert!(game_count_before >= 2, "Need at least 2 games for this test");
        info!("Phase 1: {} games in state", game_count_before);

        sleep(Duration::from_secs(2)).await;
        proposer_handle.abort();

        // Phase 2: Restart from backup with a large confirmation offset so the pinned
        // block is behind the newest game(s).
        let proposer2 = Arc::new(
            new_proposer_with_confirmations(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
                1_000_000, // Large offset: pinned block will be 0, before all games.
            )
            .await?,
        );

        proposer2.try_init().await?;
        proposer2.sync_state().await?;

        let snapshot_after = proposer2.state_snapshot().await;
        info!(
            "Phase 2: {} games after sync with large confirmation offset",
            snapshot_after.games.len()
        );

        // All games should have been pruned since pinned block is before any game.
        assert!(
            snapshot_after.games.is_empty(),
            "All games should be pruned when pinned block is before first game"
        );
        assert!(
            snapshot_after.anchor_index.is_none(),
            "Anchor should be cleared when all games are pruned"
        );
        assert!(
            snapshot_after.canonical_head_index.is_none(),
            "Canonical head should be None when all games are pruned"
        );

        info!("Backup prune future games test complete");
        Ok(())
    }

    /// Tests that backup restore with pinned block having zero games clears all state.
    ///
    /// Creates games, backs up, then restarts with confirmations large enough that the
    /// pinned block predates all game creation. Verifies the cache is fully cleared.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_backup_restore_pinned_block_no_games() -> Result<()> {
        info!("=== Test: Backup Restore With Pinned Block Having Zero Games ===");

        let env = TestEnvironment::setup().await?;

        let backup_dir = TempDir::new()?;
        let backup_path = backup_dir.path().join("proposer_backup.json");

        // Phase 1: Create at least one game and persist backup.
        let proposer = Arc::new(
            new_proposer(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
            )
            .await?,
        );

        let proposer_clone = proposer.clone();
        let proposer_handle = tokio::spawn(async move { proposer_clone.run().await });

        env.wait_and_track_games(1, 30).await?;

        for _ in 0..30 {
            if !proposer.state_snapshot().await.games.is_empty() {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
        assert!(
            !proposer.state_snapshot().await.games.is_empty(),
            "Proposer should have at least one game"
        );

        sleep(Duration::from_secs(2)).await;
        proposer_handle.abort();

        // Phase 2: Restart with huge confirmations so pinned block is before genesis games.
        let proposer2 = Arc::new(
            new_proposer_with_confirmations(
                &env.rpc_config,
                env.private_keys.proposer,
                &env.deployed.anchor_state_registry,
                &env.deployed.factory,
                env.game_type,
                Some(backup_path.clone()),
                1_000_000,
            )
            .await?,
        );

        proposer2.try_init().await?;
        proposer2.sync_state().await?;

        let snapshot = proposer2.state_snapshot().await;
        assert!(snapshot.games.is_empty(), "Games should be empty");
        assert!(snapshot.anchor_index.is_none(), "Anchor should be None");
        assert!(snapshot.canonical_head_index.is_none(), "Canonical head should be None");
        // Note: canonical_head_l2_block is intentionally not asserted — current semantics
        // only clear canonical_head_index, not the stored baseline L2 block.

        info!("Backup restore pinned block no games test complete");
        Ok(())
    }
}
