use std::sync::{Arc, Mutex};

use alloy_rpc_client::RpcClient;
use anyhow::Result;
use async_trait::async_trait;
use canoe_sp1_cc_host::CanoeSp1CCProvider;
use canoe_verifier_address_fetcher::CanoeVerifierAddressFetcherDeployedByEigenLabs;
use hokulea_compute_proof::create_kzg_proofs_for_eigenda_preimage;
use hokulea_proof::{
    eigenda_provider::OracleEigenDAPreimageProvider,
    eigenda_witness::{EigenDAPreimage, EigenDAWitness},
};
use hokulea_witgen::witness_provider::OracleEigenDAPreimageProviderWithPreimage;
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
use sp1_sdk::SP1Stdin;
use url::Url;

type WitnessExecutor = EigenDAWitnessExecutor<
    PreimageWitnessCollector<DefaultOracleBase>,
    OnlineBlobStore<OracleBlobProvider<DefaultOracleBase>>,
    OracleEigenDAPreimageProvider<DefaultOracleBase>,
>;

/// EigenDA witness generator with optional canoe proof generation.
///
/// When `l1_rpc_url` is provided, the generator will create canoe proofs for EigenDA
/// certificate validation. When `None`, canoe proof generation is skipped.
pub struct EigenDAWitnessGenerator {
    /// L1 RPC URL for canoe proof generation. If None, canoe proofs are not generated.
    pub l1_rpc_url: Option<Url>,
    /// If true, generate mock proofs instead of real network proofs.
    pub mock_mode: bool,
}

impl EigenDAWitnessGenerator {
    /// Create a new witness generator without canoe proof generation.
    pub fn new() -> Self {
        Self {
            l1_rpc_url: None,
            mock_mode: false,
        }
    }

    /// Create a new witness generator with canoe proof generation enabled.
    ///
    /// # Arguments
    /// * `l1_rpc_url` - The L1 RPC endpoint URL for the canoe proof provider
    /// * `mock_mode` - If true, generate mock proofs; if false, use network proving
    pub fn with_canoe_proofs(l1_rpc_url: Url, mock_mode: bool) -> Self {
        Self {
            l1_rpc_url: Some(l1_rpc_url),
            mock_mode,
        }
    }
}

impl Default for EigenDAWitnessGenerator {
    fn default() -> Self {
        Self::new()
    }
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

        // Write the witness data (including canoe proof bytes if present)
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
        let eigenda_preimage_provider = OracleEigenDAPreimageProvider::new(oracle.clone());
        let eigenda_preimage = Arc::new(Mutex::new(EigenDAPreimage::default()));

        let eigenda_preimage_provider = OracleEigenDAPreimageProviderWithPreimage {
            provider: eigenda_preimage_provider,
            preimage: eigenda_preimage.clone(),
        };

        let executor = EigenDAWitnessExecutor::new(eigenda_preimage_provider);

        let (boot_info, input) = get_inputs_for_pipeline(oracle.clone()).await?;
        if let Some((cursor, l1_provider, l2_provider)) = input {
            let rollup_config = Arc::new(boot_info.rollup_config.clone());
            let l1_config = Arc::new(boot_info.l1_config.clone());
            let pipeline = WitnessExecutorTrait::create_pipeline(
                &executor,
                rollup_config,
                l1_config,
                cursor.clone(),
                oracle.clone(),
                beacon,
                l1_provider.clone(),
                l2_provider.clone(),
            )
            .await?;
            WitnessExecutorTrait::run(&executor, boot_info.clone(), pipeline, cursor, l2_provider)
                .await?;
        }

        // Extract the EigenDA preimage data
        let eigenda_preimage_data = std::mem::take(&mut *eigenda_preimage.lock().unwrap());

        let kzg_proofs = create_kzg_proofs_for_eigenda_preimage(&eigenda_preimage_data);

        // Generate canoe proof if L1 RPC URL is configured
        let maybe_canoe_proof_bytes: Option<Vec<u8>> = match &self.l1_rpc_url {
            Some(l1_rpc_url) => {
                // Create the canoe provider with the L1 RPC client
                let canoe_provider = CanoeSp1CCProvider {
                    eth_rpc_client: RpcClient::new_http(l1_rpc_url.clone()),
                    mock_mode: self.mock_mode,
                };
                let canoe_address_fetcher = CanoeVerifierAddressFetcherDeployedByEigenLabs {};

                // Generate the canoe proof using hokulea's proof generation function
                let optional_canoe_proof = hokulea_witgen::from_boot_info_to_canoe_proof(
                    &boot_info,
                    &eigenda_preimage_data,
                    oracle.as_ref(),
                    canoe_provider,
                    canoe_address_fetcher,
                )
                .await?;

                // Serialize the proof using serde_json (consistent with hokulea examples)
                optional_canoe_proof
                    .map(|proof| serde_json::to_vec(&proof).expect("Failed to serialize canoe proof"))
            }
            None => {
                if !eigenda_preimage_data.validities.is_empty() {
                    tracing::warn!(
                        "EigenDA preimage contains {} certificates requiring validation, but L1 RPC URL \
                         is not configured. Canoe proof generation is skipped.",
                        eigenda_preimage_data.validities.len()
                    );
                }
                None
            }
        };

        let eigenda_witness = EigenDAWitness::from_preimage(
            eigenda_preimage_data,
            kzg_proofs,
            maybe_canoe_proof_bytes,
        )?;

        let eigenda_witness_bytes =
            serde_cbor::to_vec(&eigenda_witness).expect("Failed to serialize EigenDA witness data");

        let witness = EigenDAWitnessData {
            preimage_store: preimage_witness_store.lock().unwrap().clone(),
            blob_data: blob_data.lock().unwrap().clone(),
            eigenda_data: Some(eigenda_witness_bytes),
        };

        Ok(witness)
    }
}
