use std::{fmt::Debug, sync::Arc};

use kona_proof::{l1::OracleL1ChainProvider, l2::OracleL2ChainProvider};
use op_succinct_client_utils::{
    boot::BootInfoStruct,
    witness::{
        executor::{get_inputs_for_pipeline, WitnessExecutor},
        preimage_store::PreimageStore,
        DefaultWitnessData, WitnessData,
    },
    BlobStore,
};
use rkyv::rancor::Error;

pub async fn run_range_program<E>(executor: E)
where
    E: WitnessExecutor<
            O = PreimageStore,
            B = BlobStore,
            L1 = OracleL1ChainProvider<PreimageStore>,
            L2 = OracleL2ChainProvider<PreimageStore>,
        > + Send
        + Sync
        + Debug,
{
    ////////////////////////////////////////////////////////////////
    //                          PROLOGUE                          //
    ////////////////////////////////////////////////////////////////
    let witness_rkyv_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
    let witness_data = rkyv::from_bytes::<DefaultWitnessData, Error>(&witness_rkyv_bytes)
        .expect("Failed to deserialize witness data.");
    let (oracle, beacon) = witness_data.get_oracle_and_blob_provider().await.unwrap();

    let (boot_info, input) = get_inputs_for_pipeline(oracle.clone()).await.unwrap();
    let boot_info = match input {
        Some((cursor, l1_provider, l2_provider)) => {
            let rollup_config = Arc::new(boot_info.rollup_config.clone());

            let pipeline = executor
                .create_pipeline(
                    rollup_config,
                    cursor.clone(),
                    oracle,
                    beacon,
                    l1_provider,
                    l2_provider.clone(),
                )
                .await
                .unwrap();

            executor.run(boot_info, pipeline, cursor, l2_provider).await.unwrap()
        }
        None => boot_info,
    };

    sp1_zkvm::io::commit(&BootInfoStruct::from(boot_info));
}
