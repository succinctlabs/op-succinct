//! Validium Blob Store
//!
//! Stores batch data keyed by keccak256 hash.
//! When the pipeline encounters an AltDA commitment on L1,
//! it looks up the actual data here and verifies the hash.

use std::collections::BTreeMap;

use alloy_primitives::{keccak256, B256};

/// AltDA version byte. Batcher txs starting with this byte contain a commitment, not data.
pub const ALTDA_TX_DATA_VERSION: u8 = 0x01;

/// Keccak256 commitment type byte (from op-alt-da).
pub const KECCAK256_COMMITMENT_TYPE: u8 = 0x00;

/// Off-chain batch data, serialized into the witness.
/// Each entry is a raw batch; keyed by `keccak256(batch)` at load time.
#[derive(Clone, Debug, Default, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ValidiumBlobData {
    pub batches: Vec<Vec<u8>>,
}

impl ValidiumBlobData {
    /// Creates new validium blob data from raw batches.
    pub fn new(batches: Vec<Vec<u8>>) -> Self {
        Self { batches }
    }
}

/// Validium blob store.
/// Keyed by keccak256 hash for commitment-based lookup.
#[derive(Clone, Debug, Default)]
pub struct ValidiumBlobStore {
    /// Map from keccak256(data) → data.
    store: BTreeMap<[u8; 32], Vec<u8>>,
}

impl From<ValidiumBlobData> for ValidiumBlobStore {
    fn from(value: ValidiumBlobData) -> Self {
        let mut store = BTreeMap::new();
        for batch in value.batches {
            let hash = keccak256(&batch);
            store.insert(hash.0, batch);
        }
        Self { store }
    }
}

impl ValidiumBlobStore {
    /// Looks up batch data by keccak256 commitment.
    ///
    /// Defense-in-depth: re-hashes the data even though the store is already
    /// keyed by hash, so a corrupted witness cannot bypass verification.
    pub fn get_by_commitment(&self, commitment: &B256) -> Option<Vec<u8>> {
        let data = self.store.get(&commitment.0)?;
        let computed = keccak256(data);
        if computed != *commitment {
            return None;
        }
        Some(data.clone())
    }

    /// Checks if a commitment exists in the store.
    pub fn contains(&self, commitment: &B256) -> bool {
        self.store.contains_key(&commitment.0)
    }

    /// Returns the number of batches in the store.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Returns true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}
