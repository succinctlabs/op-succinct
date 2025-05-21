use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use hokulea_eigenda::{EigenDABlobSource, EigenDADataSource};
use hokulea_proof::{
    canoe_verifier::sp1_cc::CanoeSp1CCVerifier, eigenda_blob_witness::EigenDABlobWitnessData,
    preloaded_eigenda_provider::PreloadedEigenDABlobProvider,
};
use hokulea_zkvm_verification::eigenda_witness_to_preloaded_provider;
use kona_derive::{sources::EthereumDataSource, traits::BlobProvider};
use kona_driver::PipelineCursor;
use kona_genesis::RollupConfig;
use kona_preimage::CommsClient;
use kona_proof::{
    l1::{OracleL1ChainProvider, OraclePipeline},
    l2::OracleL2ChainProvider,
    FlushableCache,
};
use op_succinct_client_utils::witness::{executor::WitnessExecutor, EigenDAWitnessData};
use spin::RwLock;

pub struct EigenDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    eigenda_witness: EigenDABlobWitnessData,
    canoe_verifier: CanoeSp1CCVerifier,
    _marker: std::marker::PhantomData<(O, B)>,
}

#[allow(clippy::new_without_default)]
impl<O, B> EigenDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    pub fn new(witness_data: EigenDAWitnessData) -> Self {
        let eigenda_witness: EigenDABlobWitnessData = serde_cbor::from_slice(
            &witness_data.eigenda_data.expect("eigenda witness data is not present"),
        )
        .expect("cannot deserialize eigenda witness");

        Self {
            eigenda_witness,
            canoe_verifier: CanoeSp1CCVerifier {},
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<O, B> WitnessExecutor for EigenDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    type O = O;
    type B = B;
    type L1 = OracleL1ChainProvider<Self::O>;
    type L2 = OracleL2ChainProvider<Self::O>;
    type DA = EigenDADataSource<Self::L1, Self::B, PreloadedEigenDABlobProvider>;

    async fn create_pipeline(
        &self,
        rollup_config: Arc<RollupConfig>,
        cursor: Arc<RwLock<PipelineCursor>>,
        oracle: Arc<Self::O>,
        beacon: Self::B,
        l1_provider: Self::L1,
        l2_provider: Self::L2,
    ) -> Result<OraclePipeline<Self::O, Self::L1, Self::L2, Self::DA>> {
        let ethereum_data_source =
            EthereumDataSource::new_from_parts(l1_provider.clone(), beacon, &rollup_config);
        let preloaded_blob_provider = eigenda_witness_to_preloaded_provider(
            oracle.clone(),
            self.canoe_verifier.clone(),
            self.eigenda_witness.clone(),
        )
        .await?;
        let eigenda_blob_source = EigenDABlobSource::new(preloaded_blob_provider);
        let da_provider = EigenDADataSource::new(ethereum_data_source, eigenda_blob_source);

        Ok(OraclePipeline::new(
            rollup_config,
            cursor,
            oracle,
            da_provider,
            l1_provider,
            l2_provider,
        )
        .await?)
    }
}
