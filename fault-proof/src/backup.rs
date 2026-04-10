//! Simple file-based state persistence for proposer and challenger recovery.
//!
//! On restart, the proposer/challenger can restore its cursor and game cache from a backup file,
//! avoiding a full re-sync from the factory contract.

use std::{io::Write, path::Path};

use tempfile::NamedTempFile;

use alloy_primitives::U256;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::{challenger::Game as ChallengerGame, proposer::Game};

/// Atomically save a serializable value as pretty-printed JSON (temp file + fsync + rename).
fn save_json(value: &impl Serialize, path: &Path, label: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(value)
        .with_context(|| format!("failed to serialize {label} backup"))?;
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut temp = NamedTempFile::new_in(dir)
        .with_context(|| format!("failed to create {label} backup temp file"))?;
    temp.write_all(json.as_bytes())
        .with_context(|| format!("failed to write {label} backup temp file"))?;
    temp.as_file()
        .sync_all()
        .with_context(|| format!("failed to sync {label} backup temp file"))?;
    temp.persist(path).with_context(|| format!("failed to persist {label} backup file"))?;
    Ok(())
}

/// Current backup format version. Increment when making breaking changes.
pub const BACKUP_VERSION: u32 = 1;

/// Serializable backup of the proposer state.
#[derive(Serialize, Deserialize)]
pub struct ProposerBackup {
    pub version: u32,
    pub cursor: Option<U256>,
    pub games: Vec<Game>,
    pub anchor_game_index: Option<U256>,
}

impl ProposerBackup {
    /// Create a new backup with the current version.
    pub fn new(cursor: Option<U256>, games: Vec<Game>, anchor_game_index: Option<U256>) -> Self {
        Self { version: BACKUP_VERSION, cursor, games, anchor_game_index }
    }

    /// Validate backup integrity. Rejects stale/corrupted backups but allows orphaned parent
    /// references, which are normal when anchor-based fetching or ASR filtering produce partial
    /// DAGs.
    pub fn validate(&self) -> Result<()> {
        // Cursor with no games indicates a stale or corrupted backup.
        if let Some(cursor) = self.cursor {
            if self.games.is_empty() && cursor > U256::ZERO {
                bail!("cursor exists but no games");
            }
        }

        // Anchor must reference a game that exists in the backup.
        if let Some(anchor_idx) = self.anchor_game_index {
            if !self.games.iter().any(|g| g.index == anchor_idx) {
                bail!("anchor game index references non-existent game");
            }
        }

        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        save_json(self, path, "proposer")?;
        tracing::debug!(?path, games = self.games.len(), "Proposer state backed up");
        Ok(())
    }

    /// Load and validate a backup from file.
    ///
    /// Returns None and logs a warning if:
    /// - File doesn't exist or can't be read
    /// - JSON parsing fails
    /// - Version mismatch
    /// - Validation fails (stale/corrupted data)
    pub fn load(path: &Path) -> Option<Self> {
        let json = std::fs::read_to_string(path).ok()?;

        let backup = match serde_json::from_str::<Self>(&json) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!(?path, error = %e, "Failed to parse backup, starting fresh");
                return None;
            }
        };

        if backup.version != BACKUP_VERSION {
            tracing::warn!(
                ?path,
                backup_version = backup.version,
                current_version = BACKUP_VERSION,
                "Backup version mismatch, starting fresh"
            );
            return None;
        }

        if let Err(e) = backup.validate() {
            tracing::warn!(?path, error = %e, "Backup validation failed, starting fresh");
            return None;
        }

        tracing::info!(?path, games = backup.games.len(), "Proposer backup loaded");
        Some(backup)
    }
}

// ==================== Challenger Backup ====================

/// Current challenger backup format version. Increment when making breaking changes.
pub const CHALLENGER_BACKUP_VERSION: u32 = 1;

/// Serializable backup of the challenger state.
#[derive(Serialize, Deserialize)]
pub struct ChallengerBackup {
    pub version: u32,
    pub cursor: U256,
    pub games: Vec<ChallengerGame>,
}

impl ChallengerBackup {
    pub fn new(cursor: U256, games: Vec<ChallengerGame>) -> Self {
        Self { version: CHALLENGER_BACKUP_VERSION, cursor, games }
    }

    /// Validate backup integrity.
    pub fn validate(&self) -> Result<()> {
        // Cursor with no games indicates a stale or corrupted backup.
        if self.games.is_empty() && self.cursor > U256::ZERO {
            bail!("cursor exists but no games");
        }
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        save_json(self, path, "challenger")?;
        tracing::debug!(?path, games = self.games.len(), "Challenger state backed up");
        Ok(())
    }

    /// Load and validate a backup from file. Returns None if unavailable or invalid.
    pub fn load(path: &Path) -> Option<Self> {
        let json = std::fs::read_to_string(path).ok()?;

        let backup = match serde_json::from_str::<Self>(&json) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!(?path, error = %e, "Failed to parse challenger backup, starting fresh");
                return None;
            }
        };

        if backup.version != CHALLENGER_BACKUP_VERSION {
            tracing::warn!(
                ?path,
                backup_version = backup.version,
                current_version = CHALLENGER_BACKUP_VERSION,
                "Challenger backup version mismatch, starting fresh"
            );
            return None;
        }

        if let Err(e) = backup.validate() {
            tracing::warn!(?path, error = %e, "Challenger backup validation failed, starting fresh");
            return None;
        }

        tracing::info!(?path, games = backup.games.len(), "Challenger backup loaded");
        Some(backup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Schema guard: if this test fails, you likely need to bump BACKUP_VERSION.
    /// This catches accidental schema changes that would break backup compatibility.
    #[test]
    fn backup_schema_guard() {
        use crate::contract::{GameStatus, ProposalStatus};
        use alloy_primitives::{Address, B256};

        // If Game fields change, this won't compile or the JSON keys will differ
        let game = Game {
            index: U256::ZERO,
            address: Address::ZERO,
            parent_index: 0,
            l2_block: U256::ZERO,
            status: GameStatus::IN_PROGRESS,
            proposal_status: ProposalStatus::Unchallenged,
            deadline: 0,
            should_attempt_to_resolve: false,
            should_attempt_to_claim_bond: false,
            aggregation_vkey: B256::ZERO,
            range_vkey_commitment: B256::ZERO,
            rollup_config_hash: B256::ZERO,
        };

        let json = serde_json::to_value(&game).unwrap();
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();

        // If this assertion fails, Game schema changed - bump BACKUP_VERSION!
        assert_eq!(
            keys,
            vec![
                "address",
                "aggregation_vkey",
                "deadline",
                "index",
                "l2_block",
                "parent_index",
                "proposal_status",
                "range_vkey_commitment",
                "rollup_config_hash",
                "should_attempt_to_claim_bond",
                "should_attempt_to_resolve",
                "status",
            ],
            "Game schema changed! Bump BACKUP_VERSION in backup.rs"
        );

        // Check ProposerBackup fields
        let backup = ProposerBackup::new(None, vec![], None);
        let json = serde_json::to_value(&backup).unwrap();
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();

        assert_eq!(
            keys,
            vec!["anchor_game_index", "cursor", "games", "version"],
            "ProposerBackup schema changed! Bump BACKUP_VERSION in backup.rs"
        );
    }

    #[test]
    fn challenger_backup_schema_guard() {
        use crate::contract::{GameStatus, ProposalStatus};
        use alloy_primitives::Address;

        let game = ChallengerGame {
            index: U256::ZERO,
            address: Address::ZERO,
            parent_index: 0,
            l2_block_number: U256::ZERO,
            is_invalid: false,
            status: GameStatus::IN_PROGRESS,
            proposal_status: ProposalStatus::Unchallenged,
            should_attempt_to_challenge: false,
            should_attempt_to_resolve: false,
            should_attempt_to_claim_bond: false,
        };

        let json = serde_json::to_value(&game).unwrap();
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();

        assert_eq!(
            keys,
            vec![
                "address",
                "index",
                "is_invalid",
                "l2_block_number",
                "parent_index",
                "proposal_status",
                "should_attempt_to_challenge",
                "should_attempt_to_claim_bond",
                "should_attempt_to_resolve",
                "status",
            ],
            "ChallengerGame schema changed! Bump CHALLENGER_BACKUP_VERSION in backup.rs"
        );

        let backup = ChallengerBackup::new(U256::ZERO, vec![]);
        let json = serde_json::to_value(&backup).unwrap();
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();

        assert_eq!(
            keys,
            vec!["cursor", "games", "version"],
            "ChallengerBackup schema changed! Bump CHALLENGER_BACKUP_VERSION in backup.rs"
        );
    }
}
