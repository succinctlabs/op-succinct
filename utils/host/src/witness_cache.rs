//! Witness caching module for saving/loading WitnessData to/from disk.
//!
//! This module provides functions to cache witness data, keyed by (chain_id, start_block,
//! end_block). Caching allows skipping the time-consuming witness generation step (`host.run()`)
//! on subsequent runs.
//!
//! Note: Cache files are NOT compatible across DA types. A cache file created with
//! `--features eigenda` cannot be loaded without that feature, and vice versa.

use std::{fs, path::PathBuf};

use anyhow::Result;

// Select witness type based on DA feature flag (matches pattern in utils/proof/src/lib.rs)
cfg_if::cfg_if! {
    if #[cfg(feature = "eigenda")] {
        /// The witness data type used for caching, selected based on DA feature flag.
        pub type WitnessDataType = op_succinct_client_utils::witness::EigenDAWitnessData;
    } else {
        // Both Ethereum DA and Celestia DA use DefaultWitnessData
        /// The witness data type used for caching, selected based on DA feature flag.
        pub type WitnessDataType = op_succinct_client_utils::witness::DefaultWitnessData;
    }
}

/// Returns the cache directory path for a given chain ID.
pub fn get_cache_dir(chain_id: u64) -> PathBuf {
    PathBuf::from(format!("data/{}/witness-cache", chain_id))
}

/// Returns the cache file path for a given block range.
pub fn get_cache_path(chain_id: u64, start_block: u64, end_block: u64) -> PathBuf {
    get_cache_dir(chain_id).join(format!("{}-{}.bin", start_block, end_block))
}

/// Save witness data to cache.
///
/// Creates the cache directory if it doesn't exist and serializes the witness data using rkyv.
pub fn save_witness_to_cache(
    chain_id: u64,
    start_block: u64,
    end_block: u64,
    witness: &WitnessDataType,
) -> Result<PathBuf> {
    let cache_dir = get_cache_dir(chain_id);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }

    let cache_path = get_cache_path(chain_id, start_block, end_block);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(witness)?;
    fs::write(&cache_path, &bytes)?;

    Ok(cache_path)
}

/// Load witness data from cache if it exists.
///
/// Returns `Ok(Some(witness))` if the cache file exists and was successfully deserialized,
/// `Ok(None)` if the cache file doesn't exist, or an error if deserialization failed.
pub fn load_witness_from_cache(
    chain_id: u64,
    start_block: u64,
    end_block: u64,
) -> Result<Option<WitnessDataType>> {
    let cache_path = get_cache_path(chain_id, start_block, end_block);

    if !cache_path.exists() {
        return Ok(None);
    }

    let bytes = fs::read(&cache_path)?;
    let witness = rkyv::from_bytes::<WitnessDataType, rkyv::rancor::Error>(&bytes)?;

    Ok(Some(witness))
}

/// Check if cache exists for a given block range.
pub fn cache_exists(chain_id: u64, start_block: u64, end_block: u64) -> bool {
    get_cache_path(chain_id, start_block, end_block).exists()
}

// ============================================================================
// SP1Stdin caching (DA-agnostic)
// ============================================================================

use sp1_sdk::SP1Stdin;

/// Returns the stdin cache file path for a given block range.
pub fn get_stdin_cache_path(chain_id: u64, start_block: u64, end_block: u64) -> PathBuf {
    get_cache_dir(chain_id).join(format!("{}-{}-stdin.bin", start_block, end_block))
}

/// Save SP1Stdin to cache using bincode.
///
/// SP1Stdin is DA-agnostic (same type regardless of witness generator), so this
/// works with generic host types unlike WitnessData caching.
pub fn save_stdin_to_cache(
    chain_id: u64,
    start_block: u64,
    end_block: u64,
    stdin: &SP1Stdin,
) -> Result<PathBuf> {
    let cache_dir = get_cache_dir(chain_id);
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }

    let cache_path = get_stdin_cache_path(chain_id, start_block, end_block);
    let bytes = bincode::serialize(stdin)?;
    fs::write(&cache_path, &bytes)?;

    Ok(cache_path)
}

/// Load SP1Stdin from cache if it exists.
///
/// Returns `Ok(Some(stdin))` if the cache file exists and was successfully deserialized,
/// `Ok(None)` if the cache file doesn't exist, or an error if deserialization failed.
pub fn load_stdin_from_cache(
    chain_id: u64,
    start_block: u64,
    end_block: u64,
) -> Result<Option<SP1Stdin>> {
    let cache_path = get_stdin_cache_path(chain_id, start_block, end_block);

    if !cache_path.exists() {
        return Ok(None);
    }

    let bytes = fs::read(&cache_path)?;
    let stdin = bincode::deserialize(&bytes)?;

    Ok(Some(stdin))
}
