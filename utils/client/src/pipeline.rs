//! Contains an oracle-backed pipeline.

use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use core::fmt::Debug;
use kona_derive::{
    attributes::StatefulAttributesBuilder,
    errors::PipelineErrorKind,
    pipeline::{DerivationPipeline, PipelineBuilder, PipelineResult, Signal, StepResult},
    sources::EthereumDataSource,
    stages::{
        AttributesQueue, BatchProvider, BatchStream, ChannelProvider, ChannelReader, FrameQueue,
        L1Retrieval, L1Traversal,
    },
    traits::{BlobProvider, OriginProvider, Pipeline, SignalReceiver},
};
use kona_derive::traits::EigenDAProvider;
use kona_driver::{DriverPipeline, PipelineCursor};
use kona_preimage::CommsClient;
use kona_proof::{l1::OracleL1ChainProvider, FlushableCache};
use op_alloy_genesis::{RollupConfig, SystemConfig};
use op_alloy_protocol::{BlockInfo, L2BlockInfo};
use op_alloy_rpc_types_engine::OpAttributesWithParent;

use crate::l2_chain_provider::MultiblockOracleL2ChainProvider;

/// An oracle-backed derivation pipeline.
pub type OracleDerivationPipeline<O, B, E> = DerivationPipeline<
    OracleAttributesQueue<OracleDataProvider<O, B, E>, O>,
    MultiblockOracleL2ChainProvider<O>,
>;

/// An oracle-backed Ethereum data source.
pub type OracleDataProvider<O, B, E> = EthereumDataSource<OracleL1ChainProvider<O>, B, E>;

/// An oracle-backed payload attributes builder for the `AttributesQueue` stage of the derivation
/// pipeline.
pub type OracleAttributesBuilder<O> =
    StatefulAttributesBuilder<OracleL1ChainProvider<O>, MultiblockOracleL2ChainProvider<O>>;

/// An oracle-backed attributes queue for the derivation pipeline.
pub type OracleAttributesQueue<DAP, O> = AttributesQueue<
    BatchProvider<
        BatchStream<
            ChannelReader<
                ChannelProvider<
                    FrameQueue<L1Retrieval<DAP, L1Traversal<OracleL1ChainProvider<O>>>>,
                >,
            >,
        >,
    >,
    OracleAttributesBuilder<O>,
>;

/// An oracle-backed derivation pipeline.
#[derive(Debug)]
pub struct MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// The internal derivation pipeline.
    pub pipeline: OracleDerivationPipeline<O, B, E>,
    /// The caching oracle.
    pub caching_oracle: Arc<O>,
}

impl<O, B, E> MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// Constructs a new oracle-backed derivation pipeline.
    pub fn new(
        cfg: Arc<RollupConfig>,
        sync_start: PipelineCursor,
        caching_oracle: Arc<O>,
        blob_provider: B,
        eigen_da_provider: E,
        chain_provider: OracleL1ChainProvider<O>,
        l2_chain_provider: MultiblockOracleL2ChainProvider<O>,
    ) -> Self {
        let attributes = StatefulAttributesBuilder::new(
            cfg.clone(),
            l2_chain_provider.clone(),
            chain_provider.clone(),
        );
        let dap = EthereumDataSource::new(chain_provider.clone(), blob_provider, eigen_da_provider, &cfg);

        let pipeline = PipelineBuilder::new()
            .rollup_config(cfg)
            .dap_source(dap)
            .l2_chain_provider(l2_chain_provider)
            .chain_provider(chain_provider)
            .builder(attributes)
            .origin(sync_start.origin())
            .build();
        Self {
            pipeline,
            caching_oracle,
        }
    }
}

impl<O, B, E> DriverPipeline<OracleDerivationPipeline<O, B, E>> for MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// Flushes the cache on re-org.
    fn flush(&mut self) {
        self.caching_oracle.flush();
    }
}

#[async_trait]
impl<O, B, E> SignalReceiver for MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// Receives a signal from the driver.
    async fn signal(&mut self, signal: Signal) -> PipelineResult<()> {
        self.pipeline.signal(signal).await
    }
}

impl<O, B, E> OriginProvider for MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// Returns the optional L1 [BlockInfo] origin.
    fn origin(&self) -> Option<BlockInfo> {
        self.pipeline.origin()
    }
}

impl<O, B, E> Iterator for MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    type Item = OpAttributesWithParent;

    fn next(&mut self) -> Option<Self::Item> {
        self.pipeline.next()
    }
}

#[async_trait]
impl<O, B, E> Pipeline for MultiblockOraclePipeline<O, B, E>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: EigenDAProvider + Send + Sync + Debug + Clone,
{
    /// Peeks at the next [OpAttributesWithParent] from the pipeline.
    fn peek(&self) -> Option<&OpAttributesWithParent> {
        self.pipeline.peek()
    }

    /// Attempts to progress the pipeline.
    async fn step(&mut self, cursor: L2BlockInfo) -> StepResult {
        self.pipeline.step(cursor).await
    }

    /// Returns the rollup config.
    fn rollup_config(&self) -> &RollupConfig {
        self.pipeline.rollup_config()
    }

    /// Returns the [SystemConfig] by L2 number.
    async fn system_config_by_number(
        &mut self,
        number: u64,
    ) -> Result<SystemConfig, PipelineErrorKind> {
        self.pipeline.system_config_by_number(number).await
    }
}
