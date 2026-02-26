//! Contains the [`AltDAWitnessExecutor`], which implements the [`WitnessExecutor`] trait
//! for AltDA-backed OP Stack chains.
//!
//! This executor constructs the derivation pipeline with an [`AltDADataSource`] as the
//! data availability provider, enabling the proof program to resolve AltDA commitments
//! from L1 into actual batch data via the host's preimage oracle.

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

use crate::data_source::AltDADataSource;

/// A [`WitnessExecutor`] implementation for AltDA-backed OP Stack chains.
///
/// Constructs the derivation pipeline with an [`AltDADataSource`] that wraps the standard
/// [`EthereumDataSource`] and adds AltDA commitment resolution. When the inner source
/// returns data prefixed with `DerivationVersion1` (`0x01`), the AltDA source sends a
/// hint to the host and reads the resolved batch data from the preimage oracle.
///
/// Follows the same structural pattern as `CelestiaDAWitnessExecutor` and
/// `EigenDAWitnessExecutor`.
pub struct AltDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    _marker: std::marker::PhantomData<(O, B)>,
}

#[allow(clippy::new_without_default)]
impl<O, B> AltDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<O, B> WitnessExecutor for AltDAWitnessExecutor<O, B>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
{
    type O = O;
    type B = B;
    type L1 = OracleL1ChainProvider<Self::O>;
    type L2 = OracleL2ChainProvider<Self::O>;
    type DA = AltDADataSource<Self::L1, Self::B, Self::O>;

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
        let ethereum_data_source =
            EthereumDataSource::new_from_parts(l1_provider.clone(), beacon, &rollup_config);
        let da_provider = AltDADataSource::new(ethereum_data_source, oracle.clone());

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
