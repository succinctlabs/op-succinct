use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use hokulea_proof::{eigenda_provider::OracleEigenDAProvider, eigenda_blob_witness::EigenDABlobWitnessData};
use hokulea_witgen::witness_provider::OracleEigenDAWitnessProvider;
use kona_preimage::{NativeChannel, OracleReader, HintWriter};
use kona_proof::l1::OracleBlobProvider;
use op_succinct_client_utils::witness::{EigenDAWitnessData, preimage_store::PreimageStore, BlobData, executor::{WitnessExecutor as WitnessExecutorTrait, get_inputs_for_pipeline}};
use op_succinct_eigenda_client_utils::executor::EigenDAWitnessExecutor;
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, witness_generation::{
    online_blob_store::OnlineBlobStore, preimage_witness_collector::PreimageWitnessCollector,
    DefaultOracleBase, WitnessGenerator,
}};
use rkyv::to_bytes;
use sp1_sdk::SP1Stdin;

type WitnessExecutor = EigenDAWitnessExecutor<
    PreimageWitnessCollector<DefaultOracleBase>,
    OnlineBlobStore<OracleBlobProvider<DefaultOracleBase>>,
    OracleEigenDAProvider<DefaultOracleBase>,
>;

pub struct EigenDAWitnessGenerator {
    pub executor: (),  // Placeholder - executor will be created dynamically
    pub fetcher: Arc<OPSuccinctDataFetcher>,
}

#[async_trait]
impl WitnessGenerator for EigenDAWitnessGenerator {
    type WitnessData = EigenDAWitnessData;
    type WitnessExecutor = WitnessExecutor;

    fn get_executor(&self) -> &Self::WitnessExecutor {
        panic!("get_executor should not be called directly for EigenDAWitnessGenerator")
    }

    fn get_sp1_stdin(&self, witness: Self::WitnessData) -> Result<SP1Stdin> {
        let mut stdin = SP1Stdin::new();
        let buffer = to_bytes::<rkyv::rancor::Error>(&witness)?;
        stdin.write_slice(&buffer);
        Ok(stdin)
    }

    async fn run(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<Self::WitnessData> {
        let preimage_witness_store = Arc::new(std::sync::Mutex::new(PreimageStore::default()));
        let blob_data = Arc::new(std::sync::Mutex::new(BlobData::default()));

        let preimage_oracle = Arc::new(kona_proof::CachingOracle::new(
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

        // Create EigenDA blob provider that collects witness data
        let eigenda_blob_provider = OracleEigenDAProvider::new(oracle.clone());
        let eigenda_blobs_witness = Arc::new(Mutex::new(EigenDABlobWitnessData::default()));
        
        let eigenda_blob_and_witness_provider = OracleEigenDAWitnessProvider {
            provider: eigenda_blob_provider,
            witness: eigenda_blobs_witness.clone(),
        };
        
        let executor = EigenDAWitnessExecutor::new(eigenda_blob_and_witness_provider);

        let (boot_info, input) = get_inputs_for_pipeline(oracle.clone()).await.unwrap();
        if let Some((cursor, l1_provider, l2_provider)) = input {
            let rollup_config = Arc::new(boot_info.rollup_config.clone());
            let pipeline = WitnessExecutorTrait::create_pipeline(
                    &executor,
                    rollup_config,
                    cursor.clone(),
                    oracle.clone(),
                    beacon,
                    l1_provider.clone(),
                    l2_provider.clone(),
                )
                .await
                .unwrap();
            WitnessExecutorTrait::run(&executor, boot_info, pipeline, cursor, l2_provider).await.unwrap();
        }

        // Extract the EigenDA witness data
        let eigenda_witness_data = std::mem::take(&mut *eigenda_blobs_witness.lock().unwrap());
        let eigenda_witness_bytes = serde_cbor::to_vec(&eigenda_witness_data)
            .expect("Failed to serialize EigenDA witness data");
        
        let witness = EigenDAWitnessData {
            preimage_store: preimage_witness_store.lock().unwrap().clone(),
            blob_data: blob_data.lock().unwrap().clone(),
            eigenda_data: Some(eigenda_witness_bytes),
        };

        Ok(witness)
    }
}
