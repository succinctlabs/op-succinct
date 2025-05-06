//! A program to verify a Optimism L2 block STF with Ethereum DA in the zkVM.
//!
//! This binary contains the client program for executing the Optimism rollup state transition
//! across a range of blocks, which can be used to generate an on chain validity proof. Depending on
//! the compilation pipeline, it will compile to be run either in native mode or in zkVM mode. In
//! native mode, the data for verifying the batch validity is fetched from RPC, while in zkVM mode,
//! the data is supplied by the host binary to the verifiable program.

#![no_main]
sp1_zkvm::entrypoint!(main);

use std::sync::Arc;

use kona_proof::l1::OraclePipeline;
use op_succinct_client_utils::{
    boot::BootInfoStruct,
    witness::{executor::WitnessExecutor, DefaultWitnessData, WitnessData},
};
use op_succinct_ethereum_utils::executor::ETHDAWitnessExecutor;
use rkyv::rancor::Error;

fn main() {
    #[cfg(feature = "tracing-subscriber")]
    {
        use anyhow::anyhow;
        use tracing::Level;

        let subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
        tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow!(e)).unwrap();
    }

    kona_proof::block_on(async move {
        ////////////////////////////////////////////////////////////////
        //                          PROLOGUE                          //
        ////////////////////////////////////////////////////////////////
        let witness_rkyv_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
        let witness_data = rkyv::from_bytes::<DefaultWitnessData, Error>(&witness_rkyv_bytes)
            .expect("Failed to deserialize witness data.");
        let (oracle, beacon) = witness_data.get_oracle_and_blob_provider().await.unwrap();

        let executor = ETHDAWitnessExecutor;
        let (boot_info, input) = executor.get_inputs_for_pipeline(oracle.clone()).await.unwrap();
        let boot_info = match input {
            Some((cursor, l1_provider, l2_provider)) => {
                let rollup_config = Arc::new(boot_info.rollup_config.clone());
                let pipeline = OraclePipeline::new(
                    rollup_config.clone(),
                    cursor.clone(),
                    oracle.clone(),
                    beacon,
                    l1_provider.clone(),
                    l2_provider.clone(),
                )
                .await
                .unwrap();

                executor.run(boot_info, pipeline, cursor, l2_provider).await.unwrap()
            }
            None => boot_info,
        };

        sp1_zkvm::io::commit(&BootInfoStruct::from(boot_info));
    });
}
