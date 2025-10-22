use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
<<<<<<< HEAD
use celo_genesis::CeloRollupConfig;
use celo_proof::CeloOracleL2ChainProvider;
use celo_protocol::CeloToOpProviderAdapter;
use kona_derive::{sources::EthereumDataSource, traits::BlobProvider};
||||||| ae1b78c
use kona_derive::{sources::EthereumDataSource, traits::BlobProvider};
=======
use kona_derive::{BlobProvider, EthereumDataSource};
>>>>>>> upstream/main
use kona_driver::PipelineCursor;
<<<<<<< HEAD
||||||| ae1b78c
use kona_genesis::RollupConfig;
=======
use kona_genesis::{L1ChainConfig, RollupConfig};
>>>>>>> upstream/main
use kona_preimage::CommsClient;
use kona_proof::{
    l1::{OracleL1ChainProvider, OraclePipeline},
    FlushableCache,
};
use op_succinct_client_utils::witness::executor::WitnessExecutor;
use spin::RwLock;

pub struct ETHDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    _marker: std::marker::PhantomData<(O, B)>,
}

#[allow(clippy::new_without_default)]
impl<O, B> ETHDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

#[async_trait]
impl<O, B> WitnessExecutor for ETHDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    type O = O;
    type B = B;
    type L1 = OracleL1ChainProvider<Self::O>;
    type L2 = CeloToOpProviderAdapter<CeloOracleL2ChainProvider<Self::O>>;
    type DA = EthereumDataSource<Self::L1, Self::B>;

    async fn create_pipeline(
        &self,
<<<<<<< HEAD
        rollup_config: Arc<CeloRollupConfig>,
||||||| ae1b78c
        rollup_config: Arc<RollupConfig>,
=======
        rollup_config: Arc<RollupConfig>,
        l1_config: Arc<L1ChainConfig>,
>>>>>>> upstream/main
        cursor: Arc<RwLock<PipelineCursor>>,
        oracle: Arc<Self::O>,
        beacon: Self::B,
        l1_provider: Self::L1,
        l2_provider: Self::L2,
    ) -> Result<OraclePipeline<Self::O, Self::L1, Self::L2, Self::DA>> {
        let da_provider =
            EthereumDataSource::new_from_parts(l1_provider.clone(), beacon, &rollup_config);
        Ok(OraclePipeline::new(
<<<<<<< HEAD
            Arc::new(rollup_config.0.clone()),
||||||| ae1b78c
            rollup_config,
=======
            rollup_config,
            l1_config,
>>>>>>> upstream/main
            cursor,
            oracle,
            da_provider,
            l1_provider,
            l2_provider,
        )
        .await?)
    }
}
