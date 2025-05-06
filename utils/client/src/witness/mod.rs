pub mod executor;
pub mod preimage_store;

use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use kzg_rs::{Blob, Bytes48};
use preimage_store::PreimageStore;
use rkyv::to_bytes;
use serde::{Deserialize, Serialize};
use sp1_sdk::SP1Stdin;

use crate::BlobStore;

#[async_trait]
pub trait WitnessData {
    fn preimage_store(&self) -> &PreimageStore;

    fn blob_data(&self) -> &BlobData;

    // Gets the oracle and blob provider from the witness data.
    async fn get_oracle_and_blob_provider(&self) -> Result<(Arc<PreimageStore>, BlobStore)> {
        println!("cycle-tracker-report-start: oracle-verify");
        // Check the preimages in the witness are valid.
        self.preimage_store().check_preimages().expect("Failed to validate preimages");
        println!("cycle-tracker-report-end: oracle-verify");

        // Create an Arc of the preimage store.
        let oracle = Arc::new(self.preimage_store().clone());

        // Create a BlobStore from the blobs in the witness and verifies them for correctness.
        println!("cycle-tracker-report-start: blob-verification");
        let beacon = BlobStore::from(self.blob_data().clone());
        println!("cycle-tracker-report-end: blob-verification");

        Ok((oracle, beacon))
    }

    async fn into_sp1_stdin(self) -> Result<SP1Stdin>;
}

#[derive(Clone, Debug, Default, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct DefaultWitnessData {
    pub preimage_store: preimage_store::PreimageStore,
    pub blob_data: BlobData,
}

#[async_trait]
impl WitnessData for DefaultWitnessData {
    fn preimage_store(&self) -> &PreimageStore {
        &self.preimage_store
    }

    fn blob_data(&self) -> &BlobData {
        &self.blob_data
    }

    async fn into_sp1_stdin(self) -> Result<SP1Stdin> {
        let mut stdin = SP1Stdin::new();
        let buffer = to_bytes::<rkyv::rancor::Error>(&self)?;
        stdin.write_slice(&buffer);
        Ok(stdin)
    }
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct BlobData {
    pub blobs: Vec<Blob>,
    pub commitments: Vec<Bytes48>,
    pub proofs: Vec<Bytes48>,
}
