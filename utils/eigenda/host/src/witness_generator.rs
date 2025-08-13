use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use hokulea_proof::{
    eigenda_blob_witness::EigenDABlobWitnessData, eigenda_provider::OracleEigenDAProvider,
};
use hokulea_witgen::witness_provider::OracleEigenDAWitnessProvider;
use kona_preimage::{HintWriter, NativeChannel, OracleReader};
use kona_proof::l1::OracleBlobProvider;
use op_succinct_client_utils::witness::{
    executor::{get_inputs_for_pipeline, WitnessExecutor as WitnessExecutorTrait},
    preimage_store::PreimageStore,
    BlobData, EigenDAWitnessData,
};
use op_succinct_eigenda_client_utils::executor::EigenDAWitnessExecutor;
use op_succinct_host_utils::witness_generation::{
    online_blob_store::OnlineBlobStore, preimage_witness_collector::PreimageWitnessCollector,
    DefaultOracleBase, WitnessGenerator,
};
use rkyv::to_bytes;
use sp1_core_executor::SP1ReduceProof;
use sp1_prover::InnerSC;
use sp1_sdk::{ProverClient, SP1Stdin};

type WitnessExecutor = EigenDAWitnessExecutor<
    PreimageWitnessCollector<DefaultOracleBase>,
    OnlineBlobStore<OracleBlobProvider<DefaultOracleBase>>,
    OracleEigenDAProvider<DefaultOracleBase>,
>;

pub struct EigenDAWitnessGenerator {
    pub executor: (), // Placeholder - executor will be created dynamically
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

        // If eigenda blob witness data is present, write the canoe proofs to stdin
        if let Some(eigenda_data) = &witness.eigenda_data {
            let eigenda_blob_witness_data: EigenDABlobWitnessData =
                serde_cbor::from_slice(eigenda_data)
                    .expect("Failed to deserialize EigenDA blob witness data");

            // Get the canoe SP1 CC client ELF and setup verification key
            // The ELF is included in the canoe-sp1-cc-host crate
            const CANOE_ELF: &[u8] = canoe_sp1_cc_host::ELF;
            let client = ProverClient::from_env();
            let (_pk, canoe_vk) = client.setup(CANOE_ELF);

            // Deserialize and write canoe proofs, skipping None values
            for (_, cert_validity) in eigenda_blob_witness_data.validity.iter() {
                if let Some(proof_bytes) = &cert_validity.canoe_proof {
                    let reduced_proof: SP1ReduceProof<InnerSC> =
                        serde_cbor::from_slice(proof_bytes)
                            .expect("Failed to deserialize canoe proof");
                    stdin.write_proof(reduced_proof, canoe_vk.vk.clone());
                }
            }
        }

        // Write the witness data after the proofs
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
            WitnessExecutorTrait::run(&executor, boot_info.clone(), pipeline, cursor, l2_provider)
                .await
                .unwrap();
        }

        // Extract the EigenDA witness data
        let mut eigenda_witness_data = std::mem::take(&mut *eigenda_blobs_witness.lock().unwrap());

        // Generate canoe proofs using the reduced proof provider for proof aggregation
        use canoe_sp1_cc_host::CanoeSp1CCReducedProofProvider;
        let canoe_provider = CanoeSp1CCReducedProofProvider {
            eth_rpc_url: std::env::var("L1_RPC").ok().unwrap_or_default(),
        };
        let canoe_proofs = hokulea_witgen::from_boot_info_to_canoe_proof(
            &boot_info,
            &eigenda_witness_data,
            oracle.clone(),
            canoe_provider,
        )
        .await?;

        // Populate canoe proof for witness data
        for ((_, cert_validity), canoe_proof) in
            eigenda_witness_data.validity.iter_mut().zip(canoe_proofs.iter())
        {
            let canoe_proof_bytes =
                serde_cbor::to_vec(&canoe_proof).expect("Failed to serialize canoe proof");
            cert_validity.canoe_proof = Some(canoe_proof_bytes);
        }

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
