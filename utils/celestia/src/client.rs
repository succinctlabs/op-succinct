use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use hana_oracle::{pipeline::OraclePipeline, provider::OracleCelestiaProvider};
use kona_preimage::{HintWriter, NativeChannel, OracleReader};
use kona_proof::{l1::OracleBlobProvider, CachingOracle};
use op_succinct_client_utils::witness::{
    executor::{CelestiaDAWitnessExecutor, WitnessExecutor},
    preimage_store::PreimageStore,
    BlobData, DefaultWitnessData,
};
use op_succinct_host_utils::witness_generation::{
    client::WitnessGenClient, online_blob_store::OnlineBlobStore,
    preimage_witness_collector::PreimageWitnessCollector,
};

#[derive(Clone)]
pub struct CelestiaDAWitnessGenClient;

#[async_trait]
impl WitnessGenClient for CelestiaDAWitnessGenClient {
    type WitnessData = DefaultWitnessData;

    async fn run(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<Self::WitnessData> {
        let executor = CelestiaDAWitnessExecutor;

        let preimage_witness_store = Arc::new(Mutex::new(PreimageStore::default()));
        let blob_data = Arc::new(Mutex::new(BlobData::default()));

        let preimage_oracle = Arc::new(CachingOracle::new(
            2048,
            OracleReader::new(preimage_chan),
            HintWriter::new(hint_chan),
        ));
        let blob_provider = OracleBlobProvider::new(preimage_oracle.clone());

        let oracle = Arc::new(PreimageWitnessCollector {
            preimage_oracle: preimage_oracle.clone(),
            preimage_witness_store: preimage_witness_store.clone(),
        });
        let beacon = OnlineBlobStore { provider: blob_provider.clone(), store: blob_data.clone() };

        let (boot_info, input) = executor.get_inputs_for_pipeline(oracle.clone()).await.unwrap();
        if let Some((cursor, l1_provider, l2_provider)) = input {
            let rollup_config = Arc::new(boot_info.rollup_config.clone());
            let pipeline = OraclePipeline::new(
                rollup_config.clone(),
                cursor.clone(),
                oracle.clone(),
                beacon,
                l1_provider.clone(),
                l2_provider.clone(),
                OracleCelestiaProvider::new(oracle.clone()),
            )
            .await
            .unwrap();
            executor.run(boot_info, pipeline, cursor, l2_provider).await.unwrap();
        }

        let witness = DefaultWitnessData {
            preimage_store: preimage_witness_store.lock().unwrap().clone(),
            blob_data: blob_data.lock().unwrap().clone(),
        };

        Ok(witness)
    }
}
