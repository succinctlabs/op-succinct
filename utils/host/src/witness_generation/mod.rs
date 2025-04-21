mod online_blob_store;
mod preimage_witness_collector;

use anyhow::Result;
use hokulea_eigenda::EigenDABlobProvider;
use hokulea_proof::eigenda_blob_witness::EigenDABlobWitnessData;
use hokulea_client_bin::witness::OracleEigenDAWitnessProvider;
use kona_derive::prelude::BlobProvider;
use kona_preimage::CommsClient;
use kona_proof::{BootInfo, FlushableCache};
use online_blob_store::OnlineBlobStore;
use op_succinct_client_utils::{
    client::run_opsuccinct_client,
    eigenda_client::run_opsuccinct_eigenda_client,
    witness::{preimage_store::PreimageStore, BlobData, WitnessData},
};
use preimage_witness_collector::PreimageWitnessCollector;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use std::ops::DerefMut;

/// Generate a witness with the given oracle and blob provider.
pub async fn generate_opsuccinct_witness<O, B>(
    preimage_oracle: Arc<O>,
    blob_provider: B,
) -> Result<(BootInfo, WitnessData)>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    let preimage_witness_store = Arc::new(Mutex::new(PreimageStore::default()));
    let blob_data = Arc::new(Mutex::new(BlobData::default()));

    let oracle = Arc::new(PreimageWitnessCollector {
        preimage_oracle: preimage_oracle.clone(),
        preimage_witness_store: preimage_witness_store.clone(),
    });
    let beacon = OnlineBlobStore { provider: blob_provider.clone(), store: blob_data.clone() };

    let boot = run_opsuccinct_client(oracle, beacon).await?;

    let witness = WitnessData {
        preimage_store: preimage_witness_store.lock().unwrap().clone(),
        blob_data: blob_data.lock().unwrap().clone(),
        eigenda_data: vec![],
    };

    Ok((boot, witness))
}


/// Generate a witness with the given oracle and blob provider.
pub async fn generate_opsuccinct_eigenda_witness<O, B, E>(
    preimage_oracle: Arc<O>,
    blob_provider: B,
    eigenda_blob_provider: E,
) -> Result<(BootInfo, WitnessData)>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,   
    E: EigenDABlobProvider + Send + Sync + Debug + Clone,
{
    let preimage_witness_store = Arc::new(Mutex::new(PreimageStore::default()));
    let blob_data = Arc::new(Mutex::new(BlobData::default()));

    let oracle = Arc::new(PreimageWitnessCollector {
        preimage_oracle: preimage_oracle.clone(),
        preimage_witness_store: preimage_witness_store.clone(),
    });
    let beacon = OnlineBlobStore { provider: blob_provider.clone(), store: blob_data.clone() };

    let eigenda_blobs_witness = Arc::new(Mutex::new(EigenDABlobWitnessData::default()));

    let eigenda_blob_and_witness_provider = OracleEigenDAWitnessProvider {
        provider: eigenda_blob_provider,
        witness: eigenda_blobs_witness.clone(),
    };

    let boot = run_opsuccinct_eigenda_client(oracle, beacon, eigenda_blob_and_witness_provider).await?;    

    let eigenda_witness = core::mem::take(eigenda_blobs_witness.lock().unwrap().deref_mut());       

    let eigenda_witness_byte = serde_cbor::to_vec(&eigenda_witness)?;

    let witness = WitnessData {
        preimage_store: preimage_witness_store.lock().unwrap().clone(),
        blob_data: blob_data.lock().unwrap().clone(),
        eigenda_data: eigenda_witness_byte,
    };

    Ok((boot, witness))
}
