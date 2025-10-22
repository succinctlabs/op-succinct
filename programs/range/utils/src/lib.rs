use std::sync::Arc;

use celo_genesis::CeloRollupConfig;
use celo_proof::CeloOracleL2ChainProvider;
use celo_protocol::CeloToOpProviderAdapter;
use kona_proof::l1::OracleL1ChainProvider;
use op_succinct_client_utils::{
    boot::BootInfoStruct,
    witness::{
        executor::{get_inputs_for_pipeline, WitnessExecutor},
        preimage_store::PreimageStore,
    },
    BlobStore,
};

/// Sets up tracing for the range program
#[cfg(feature = "tracing-subscriber")]
pub fn setup_tracing() {
    use anyhow::anyhow;
    use tracing::Level;

    let subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow!(e)).unwrap();
}

pub async fn run_range_program<E>(executor: E, oracle: Arc<PreimageStore>, beacon: BlobStore)
where
    E: WitnessExecutor<
            O = PreimageStore,
            B = BlobStore,
            L1 = OracleL1ChainProvider<PreimageStore>,
            L2 = CeloToOpProviderAdapter<CeloOracleL2ChainProvider<PreimageStore>>,
        > + Send
        + Sync,
{
    ////////////////////////////////////////////////////////////////
    //                          PROLOGUE                          //
    ////////////////////////////////////////////////////////////////
    let (boot_info, input) = get_inputs_for_pipeline(oracle.clone()).await.unwrap();
    let boot_info = match input {
        Some((cursor, l1_provider, l2_provider)) => {
            // Wrap RollupConfig with CeloRollupConfig
            let celo_rollup_config = CeloRollupConfig(boot_info.rollup_config.clone());
            let l1_config = Arc::new(boot_info.l1_config.clone());

            let pipeline = executor
                .create_pipeline(
<<<<<<< HEAD
                    Arc::new(celo_rollup_config),
||||||| ae1b78c
                    rollup_config,
=======
                    rollup_config,
                    l1_config,
>>>>>>> upstream/main
                    cursor.clone(),
                    oracle,
                    beacon,
                    l1_provider,
                    CeloToOpProviderAdapter(l2_provider.clone()),
                )
                .await
                .unwrap();

            executor
                .run(boot_info, pipeline, cursor, l2_provider.to_oracle_l2_chain_provider())
                .await
                .unwrap()
        }
        None => boot_info,
    };

    sp1_zkvm::io::commit(&BootInfoStruct::from(boot_info));
}
