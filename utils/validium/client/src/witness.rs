//! Validium Witness Data
//!
//! Contains L1 preimages (same as Ethereum DA) + off-chain batch data
//! keyed by keccak256 hash.

use std::sync::Arc;

use anyhow::Result;
use op_succinct_client_utils::{witness::preimage_store::PreimageStore, BlobStore};

use crate::blob_store::{ValidiumBlobData, ValidiumBlobStore};

/// Validium witness data.
///
/// - `preimage_store`: L1 data (headers, receipts, state) — same as Ethereum DA
/// - `l1_blob_data`: L1 EIP-4844 blobs (for any non-validium data still on L1)
/// - `validium_data`: Off-chain batch data, keyed by keccak256 hash at load time
#[derive(Clone, Debug, Default, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ValidiumWitnessData {
    /// L1 preimages (headers, receipts, etc.) - same as Ethereum DA.
    pub preimage_store: PreimageStore,
    /// L1 blob data for the standard pipeline (may be empty if all data is off-chain).
    pub l1_blob_data: op_succinct_client_utils::witness::BlobData,
    /// Off-chain batch data. Each entry: raw batch bytes.
    /// Stored as Vec<Vec<u8>>, keyed by keccak256 hash at load time.
    pub validium_data: ValidiumBlobData,
}

impl ValidiumWitnessData {
    /// Creates witness data from parts.
    pub fn from_parts(
        preimage_store: PreimageStore,
        l1_blob_data: op_succinct_client_utils::witness::BlobData,
        validium_data: ValidiumBlobData,
    ) -> Self {
        Self { preimage_store, l1_blob_data, validium_data }
    }

    /// Gets the oracle, blob provider, and validium blob store.
    pub async fn get_providers(
        self,
    ) -> Result<(Arc<PreimageStore>, BlobStore, ValidiumBlobStore)> {
        // Verify L1 preimages (same as Ethereum DA).
        println!("cycle-tracker-report-start: oracle-verify");
        self.preimage_store.check_preimages().expect("Failed to validate preimages");
        println!("cycle-tracker-report-end: oracle-verify");

        let oracle = Arc::new(self.preimage_store);

        // Create BlobStore for any L1 blobs (may be empty).
        println!("cycle-tracker-report-start: blob-verification");
        let beacon = BlobStore::from(self.l1_blob_data);
        println!("cycle-tracker-report-end: blob-verification");

        // Create ValidiumBlobStore (keyed by keccak256 hash, no KZG).
        println!("cycle-tracker-report-start: validium-blob-store");
        let validium_store = ValidiumBlobStore::from(self.validium_data);
        println!("cycle-tracker-report-end: validium-blob-store");

        Ok((oracle, beacon, validium_store))
    }
}
