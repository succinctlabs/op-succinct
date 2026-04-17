//! Configuration for selecting which L1 block the proposer and helper tooling reason about
//! when deciding what to prove, what to range over, and what to cap the head at.
//!
//! The default behavior is to use Ethereum's finalized tag (no offset), which preserves the
//! historical op-succinct behavior. Operators can opt into faster, less conservative selections
//! via the `L1_BLOCK_TAG` / `L1_CONFIRMATIONS` environment variables.
//!
//! # Safety note
//!
//! Any non-default selection (tag != `finalized` or `confirmations != 0`) breaks the implicit
//! assumption that the chosen L1 block cannot be reorged. Downstream code has a single entry
//! point that maps the selected L1 block back to an L2 block number
//! (`optimism_safeHeadAtL1Block`), which requires SafeDB to be activated on the op-node. The
//! shared `enforce_l1_selection_supported` helper enforces this requirement at startup, and is
//! invoked by both proposer binaries and operator-facing utility scripts that initialize a host.
use alloy_eips::BlockId;
use anyhow::{anyhow, bail, Context, Result};
use std::{env, str::FromStr};

/// Environment variable for the L1 block tag to use (finalized | safe | latest).
pub const L1_BLOCK_TAG_ENV: &str = "L1_BLOCK_TAG";

/// Environment variable for the number of additional confirmations to wait behind the selected
/// tag.
pub const L1_CONFIRMATIONS_ENV: &str = "L1_CONFIRMATIONS";

/// Tag identifying which L1 block the proposer treats as its reference point before applying
/// an optional confirmation offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L1BlockTag {
    /// Cryptoeconomic finality via Casper FFG. Default. Lags the chain tip by ~2 epochs.
    Finalized,
    /// Justified checkpoint. Lags by ~1 epoch. Reverting requires >=1/3 validator collusion.
    Safe,
    /// Chain tip. No confirmation guarantee without additional offset.
    Latest,
}

impl L1BlockTag {
    /// The alloy `BlockId` that corresponds to this tag when no further resolution is needed
    /// (i.e., `confirmations == 0`).
    pub fn to_block_id(self) -> BlockId {
        match self {
            L1BlockTag::Finalized => BlockId::finalized(),
            L1BlockTag::Safe => BlockId::safe(),
            L1BlockTag::Latest => BlockId::latest(),
        }
    }
}

impl FromStr for L1BlockTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "finalized" => Ok(L1BlockTag::Finalized),
            "safe" => Ok(L1BlockTag::Safe),
            "latest" => Ok(L1BlockTag::Latest),
            other => Err(anyhow!(
                "invalid {L1_BLOCK_TAG_ENV} value {other:?}: expected one of \
                 'finalized', 'safe', 'latest'"
            )),
        }
    }
}

/// Configuration for selecting the reference L1 block used across the proposer pipeline.
///
/// `default()` preserves the historical behavior: `finalized` tag with zero additional
/// confirmations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct L1BlockSelectionConfig {
    pub tag: L1BlockTag,
    pub confirmations: u64,
}

impl Default for L1BlockSelectionConfig {
    fn default() -> Self {
        Self { tag: L1BlockTag::Finalized, confirmations: 0 }
    }
}

impl L1BlockSelectionConfig {
    /// Whether this config matches the historical default (finalized, 0 confirmations).
    ///
    /// Callers use this to route to the original, byte-identical code path; any non-default
    /// config takes the new L1 -> L2 resolution path via `optimism_safeHeadAtL1Block`.
    pub fn is_default(&self) -> bool {
        matches!(self.tag, L1BlockTag::Finalized) && self.confirmations == 0
    }

    /// Parse the selection config from environment variables. Returns an `Err` with a clear
    /// message referencing the offending env var and value on malformed input.
    ///
    /// Intended for use at proposer entry points where a clean startup error is required.
    pub fn from_env() -> Result<Self> {
        let tag = match env::var(L1_BLOCK_TAG_ENV) {
            Ok(raw) => raw
                .parse::<L1BlockTag>()
                .with_context(|| format!("failed to parse {L1_BLOCK_TAG_ENV} from environment"))?,
            Err(env::VarError::NotPresent) => L1BlockTag::Finalized,
            Err(env::VarError::NotUnicode(_)) => {
                bail!("{L1_BLOCK_TAG_ENV} is set but is not valid UTF-8");
            }
        };

        let confirmations = match env::var(L1_CONFIRMATIONS_ENV) {
            Ok(raw) => raw.trim().parse::<u64>().map_err(|e| {
                anyhow!(
                    "invalid {L1_CONFIRMATIONS_ENV} value {raw:?}: \
                     expected non-negative integer ({e})"
                )
            })?,
            Err(env::VarError::NotPresent) => 0,
            Err(env::VarError::NotUnicode(_)) => {
                bail!("{L1_CONFIRMATIONS_ENV} is set but is not valid UTF-8");
            }
        };

        Ok(Self { tag, confirmations })
    }

    /// Parse the selection config from environment variables, panicking on invalid values.
    ///
    /// Intended for library-level constructors (`OPSuccinctDataFetcher::new`,
    /// `OPSuccinctDataFetcher::default`) where fallibility would require touching every
    /// call site across the workspace. Utility scripts inherit this behavior.
    ///
    /// Panic messages include the offending env var name and value so that failures are
    /// easy to diagnose; the full error chain is rendered via `{:#}` so that root-cause
    /// context (e.g. the offending value) is not lost behind an outer wrapper.
    pub fn from_env_or_default() -> Self {
        match Self::from_env() {
            Ok(cfg) => cfg,
            Err(e) => panic!("{e:#}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Env var access is process-global; serialize tests that mutate env vars.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_env() {
        std::env::remove_var(L1_BLOCK_TAG_ENV);
        std::env::remove_var(L1_CONFIRMATIONS_ENV);
    }

    #[test]
    fn default_is_finalized_zero() {
        let cfg = L1BlockSelectionConfig::default();
        assert_eq!(cfg.tag, L1BlockTag::Finalized);
        assert_eq!(cfg.confirmations, 0);
        assert!(cfg.is_default());
    }

    #[test]
    fn from_env_empty_env_yields_default() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        let cfg = L1BlockSelectionConfig::from_env().unwrap();
        assert!(cfg.is_default());
    }

    #[test]
    fn from_env_parses_safe() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_BLOCK_TAG_ENV, "safe");
        let cfg = L1BlockSelectionConfig::from_env().unwrap();
        assert_eq!(cfg.tag, L1BlockTag::Safe);
        assert_eq!(cfg.confirmations, 0);
        assert!(!cfg.is_default());
        clear_env();
    }

    #[test]
    fn from_env_parses_latest_with_confirmations() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_BLOCK_TAG_ENV, "latest");
        std::env::set_var(L1_CONFIRMATIONS_ENV, "4");
        let cfg = L1BlockSelectionConfig::from_env().unwrap();
        assert_eq!(cfg.tag, L1BlockTag::Latest);
        assert_eq!(cfg.confirmations, 4);
        assert!(!cfg.is_default());
        clear_env();
    }

    #[test]
    fn from_env_is_case_insensitive() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_BLOCK_TAG_ENV, "SAFE");
        assert_eq!(L1BlockSelectionConfig::from_env().unwrap().tag, L1BlockTag::Safe);
        std::env::set_var(L1_BLOCK_TAG_ENV, "FiNaLiZeD");
        assert_eq!(L1BlockSelectionConfig::from_env().unwrap().tag, L1BlockTag::Finalized);
        clear_env();
    }

    #[test]
    fn from_env_rejects_invalid_tag() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_BLOCK_TAG_ENV, "xyz");
        let err = L1BlockSelectionConfig::from_env().unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains(L1_BLOCK_TAG_ENV), "error should name env var: {msg}");
        assert!(msg.contains("xyz"), "error should include offending value: {msg}");
        clear_env();
    }

    #[test]
    fn from_env_rejects_non_numeric_confirmations() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_CONFIRMATIONS_ENV, "abc");
        let err = L1BlockSelectionConfig::from_env().unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains(L1_CONFIRMATIONS_ENV), "error should name env var: {msg}");
        assert!(msg.contains("abc"), "error should include offending value: {msg}");
        clear_env();
    }

    #[test]
    fn from_env_or_default_panics_on_invalid_with_context() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_env();
        std::env::set_var(L1_BLOCK_TAG_ENV, "garbage");
        let result = std::panic::catch_unwind(L1BlockSelectionConfig::from_env_or_default);
        clear_env();
        let payload = result.expect_err("should panic on invalid tag");
        let msg = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        assert!(msg.contains(L1_BLOCK_TAG_ENV), "panic should name env var: {msg}");
        assert!(msg.contains("garbage"), "panic should include value: {msg}");
    }

    #[test]
    fn to_block_id_matches_tag() {
        assert_eq!(L1BlockTag::Finalized.to_block_id(), BlockId::finalized());
        assert_eq!(L1BlockTag::Safe.to_block_id(), BlockId::safe());
        assert_eq!(L1BlockTag::Latest.to_block_id(), BlockId::latest());
    }
}
