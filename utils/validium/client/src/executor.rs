//! Validium Witness Executor
//!
//! Uses ValidiumDADataSource which wraps EthereumDataSource and intercepts
//! AltDA commitments (version byte 0x01) to fetch data from off-chain storage.

use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use kona_derive::{BlobProvider, EthereumDataSource};
use kona_driver::PipelineCursor;
use kona_genesis::{L1ChainConfig, RollupConfig};
use kona_preimage::CommsClient;
use kona_proof::{
    l1::{OracleL1ChainProvider, OraclePipeline},
    l2::OracleL2ChainProvider,
    FlushableCache,
};
use op_succinct_client_utils::witness::executor::WitnessExecutor;
use spin::RwLock;

use crate::blob_store::ValidiumBlobStore;
use crate::da_source::ValidiumDADataSource;

/// Validium Witness Executor
///
/// Uses `ValidiumDADataSource` which:
/// 1. Reads batcher txs from L1 via EthereumDataSource
/// 2. Intercepts AltDA commitments (0x01 prefix)
/// 3. Fetches actual data from ValidiumBlobStore
/// 4. Verifies keccak256(data) == commitment
pub struct ValidiumWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    /// The validium blob store (off-chain batch data).
    blob_store: ValidiumBlobStore,
    _marker: std::marker::PhantomData<(O, B)>,
}

impl<O, B> ValidiumWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    pub fn new(blob_store: ValidiumBlobStore) -> Self {
        Self { blob_store, _marker: std::marker::PhantomData }
    }
}

#[async_trait]
impl<O, B> WitnessExecutor for ValidiumWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    type O = O;
    type B = B;
    type L1 = OracleL1ChainProvider<Self::O>;
    type L2 = OracleL2ChainProvider<Self::O>;
    type DA = ValidiumDADataSource<Self::L1, Self::B>;

    async fn create_pipeline(
        &self,
        rollup_config: Arc<RollupConfig>,
        l1_config: Arc<L1ChainConfig>,
        cursor: Arc<RwLock<PipelineCursor>>,
        oracle: Arc<Self::O>,
        beacon: Self::B,
        l1_provider: Self::L1,
        l2_provider: Self::L2,
    ) -> Result<OraclePipeline<Self::O, Self::L1, Self::L2, Self::DA>> {
        // Create the underlying Ethereum data source (reads L1 calldata/blobs).
        let ethereum_source =
            EthereumDataSource::new_from_parts(l1_provider.clone(), beacon, &rollup_config);

        // Wrap with ValidiumDADataSource to intercept AltDA commitments.
        let da_provider = ValidiumDADataSource::new(ethereum_source, self.blob_store.clone());

        Ok(OraclePipeline::new(
            rollup_config,
            l1_config,
            cursor,
            oracle,
            da_provider,
            l1_provider,
            l2_provider,
        )
        .await?)
    }
}
