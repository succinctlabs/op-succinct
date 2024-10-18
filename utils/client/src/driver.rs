//! Contains the [MultiBlockDerivationDriver] struct, which handles the [L2PayloadAttributes]
//! derivation process.
//!
//! [L2PayloadAttributes]: kona_derive::types::L2PayloadAttributes

use crate::l2_chain_provider::MultiblockOracleL2ChainProvider;
use alloc::sync::Arc;
use alloy_consensus::{Header, Sealed};
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use kona_client::{
    l1::{OracleBlobProvider, OracleL1ChainProvider},
    BootInfo, HintType,
};
use kona_derive::{
    attributes::StatefulAttributesBuilder,
    errors::{PipelineErrorKind, ResetError},
    pipeline::{DerivationPipeline, Pipeline, PipelineBuilder, StepResult},
    prelude::{ChainProvider, L2ChainProvider},
    sources::EthereumDataSource,
    stages::{
        AttributesQueue, BatchQueue, BatchStream, ChannelProvider, ChannelReader, FrameQueue,
        L1Retrieval, L1Traversal,
    },
    traits::{ActivationSignal, OriginProvider, ResetSignal, SignalReceiver},
};
use kona_executor::{KonaHandleRegister, StatelessL2BlockExecutor};
use kona_mpt::{TrieHinter, TrieProvider};
use kona_preimage::{CommsClient, PreimageKey, PreimageKeyType};
use op_alloy_genesis::RollupConfig;
use op_alloy_protocol::{BlockInfo, L2BlockInfo};
use op_alloy_rpc_types_engine::OpAttributesWithParent;

use log::{info, warn};

/// An oracle-backed derivation pipeline.
pub type OraclePipeline<O> = DerivationPipeline<
    MultiblockOracleAttributesQueue<OracleDataProvider<O>, O>,
    MultiblockOracleL2ChainProvider<O>,
>;

/// An oracle-backed Ethereum data source.
pub type OracleDataProvider<O> =
    EthereumDataSource<OracleL1ChainProvider<O>, OracleBlobProvider<O>>;

/// An oracle-backed payload attributes builder for the `AttributesQueue` stage of the derivation
/// pipeline.
pub type OracleAttributesBuilder<O> =
    StatefulAttributesBuilder<OracleL1ChainProvider<O>, MultiblockOracleL2ChainProvider<O>>;

/// An oracle-backed attributes queue for the derivation pipeline.
pub type MultiblockOracleAttributesQueue<DAP, O> = AttributesQueue<
    BatchQueue<
        BatchStream<
            ChannelReader<
                ChannelProvider<
                    FrameQueue<L1Retrieval<DAP, L1Traversal<OracleL1ChainProvider<O>>>>,
                >,
            >,
            MultiblockOracleL2ChainProvider<O>,
        >,
        MultiblockOracleL2ChainProvider<O>,
    >,
    OracleAttributesBuilder<O>,
>;

/// The [MultiBlockDerivationDriver] struct is responsible for handling the [L2PayloadAttributes]
/// derivation process.
///
/// It contains an inner [OraclePipeline] that is used to derive the attributes, backed by
/// oracle-based data sources.
///
/// [L2PayloadAttributes]: kona_derive::types::L2PayloadAttributes
///
/// Note: Closely mirrors the implementation of:
/// https://github.com/anton-rs/kona/blob/main/bin/client/src/l1/driver.rs. The only difference is
/// the addition of the `l2_claim_block` field, which is used to determine the final L2 block that
/// the driver should produce payloads for.
#[derive(Debug)]
pub struct MultiBlockDerivationDriver<O: CommsClient + Send + Sync + Debug> {
    /// The current L2 safe head.
    pub l2_safe_head: L2BlockInfo,
    /// The header of the L2 safe head.
    pub l2_safe_head_header: Sealed<Header>,
    /// The inner pipeline.
    pub pipeline: OraclePipeline<O>,
    /// The block number of the final L2 block being claimed.
    pub l2_claim_block: u64,
}

impl<O: CommsClient + Send + Sync + Debug> MultiBlockDerivationDriver<O> {
    /// Returns the current L2 safe head [L2BlockInfo].
    pub fn l2_safe_head(&self) -> &L2BlockInfo {
        &self.l2_safe_head
    }

    /// Returns the [Header] of the current L2 safe head.
    pub fn l2_safe_head_header(&self) -> &Sealed<Header> {
        &self.l2_safe_head_header
    }

    /// Consumes self and returns the owned [Header] of the current L2 safe head.
    pub fn take_l2_safe_head_header(self) -> Sealed<Header> {
        self.l2_safe_head_header
    }

    /// Creates a new [MultiBlockDerivationDriver] with the given configuration, blob provider, and
    /// chain providers.
    ///
    /// ## Takes
    /// - `cfg`: The rollup configuration.
    /// - `blob_provider`: The blob provider.
    /// - `chain_provider`: The L1 chain provider.
    /// - `l2_chain_provider`: The L2 chain provider.
    ///
    /// ## Returns
    /// - A new [MultiBlockDerivationDriver] instance.
    pub async fn new(
        boot_info: &BootInfo,
        caching_oracle: &O,
        blob_provider: OracleBlobProvider<O>,
        mut chain_provider: OracleL1ChainProvider<O>,
        mut l2_chain_provider: MultiblockOracleL2ChainProvider<O>,
    ) -> Result<Self> {
        let cfg = Arc::new(boot_info.rollup_config.clone());

        // Fetch the startup information.
        let (l1_origin, l2_safe_head, l2_safe_head_header) = Self::find_startup_info(
            caching_oracle,
            boot_info,
            &mut chain_provider,
            &mut l2_chain_provider,
        )
        .await?;

        // Construct the pipeline.
        let attributes = StatefulAttributesBuilder::new(
            cfg.clone(),
            l2_chain_provider.clone(),
            chain_provider.clone(),
        );
        let dap = EthereumDataSource::new(chain_provider.clone(), blob_provider, &cfg);

        // Walk back the starting L1 block by `channel_timeout` to ensure that the full channel is
        // captured.
        let channel_timeout = boot_info
            .rollup_config
            .channel_timeout(l2_safe_head.block_info.timestamp);
        let mut l1_origin_number = l1_origin.number.saturating_sub(channel_timeout);
        if l1_origin_number < boot_info.rollup_config.genesis.l1.number {
            l1_origin_number = boot_info.rollup_config.genesis.l1.number;
        }
        let l1_origin = chain_provider
            .block_info_by_number(l1_origin_number)
            .await?;

        let pipeline = PipelineBuilder::new()
            .rollup_config(cfg)
            .dap_source(dap)
            .l2_chain_provider(l2_chain_provider)
            .chain_provider(chain_provider)
            .builder(attributes)
            .origin(l1_origin)
            .build();

        let l2_claim_block = boot_info.claimed_l2_block_number;
        Ok(Self {
            l2_safe_head,
            l2_safe_head_header,
            pipeline,
            l2_claim_block,
        })
    }

    pub fn update_safe_head(
        &mut self,
        new_safe_head: L2BlockInfo,
        new_safe_head_header: Sealed<Header>,
    ) {
        self.l2_safe_head = new_safe_head;
        self.l2_safe_head_header = new_safe_head_header;
    }

    /// Produces the disputed [OpAttributesWithParent] payload, directly from the pipeline.
    pub async fn produce_payload(&mut self) -> Result<OpAttributesWithParent> {
        // As we start the safe head at the disputed block's parent, we step the pipeline until the
        // first attributes are produced. All batches at and before the safe head will be
        // dropped, so the first payload will always be the disputed one.
        loop {
            match self.pipeline.step(self.l2_safe_head).await {
                StepResult::PreparedAttributes => {
                    info!(target: "client_derivation_driver", "Stepped derivation pipeline")
                }
                StepResult::AdvancedOrigin => {
                    info!(target: "client_derivation_driver", "Advanced origin")
                }
                StepResult::OriginAdvanceErr(e) | StepResult::StepFailed(e) => {
                    warn!(target: "client_derivation_driver", "Failed to step derivation pipeline: {:?}", e);

                    // Break the loop unless the error signifies that there is not enough data to
                    // complete the current step. In this case, we retry the step to see if other
                    // stages can make progress.
                    match e {
                        PipelineErrorKind::Temporary(_) => { /* continue */ }
                        PipelineErrorKind::Reset(e) => {
                            if matches!(e, ResetError::HoloceneActivation) {
                                self.pipeline
                                    .signal(
                                        ActivationSignal {
                                            l2_safe_head: self.l2_safe_head,
                                            l1_origin: self
                                                .pipeline
                                                .origin()
                                                .ok_or_else(|| anyhow!("Missing L1 origin"))?,
                                            system_config: None,
                                        }
                                        .signal(),
                                    )
                                    .await?;
                            } else {
                                // Reset the pipeline to the initial L2 safe head and L1 origin,
                                // and try again.
                                self.pipeline
                                    .signal(
                                        ResetSignal {
                                            l2_safe_head: self.l2_safe_head,
                                            l1_origin: self
                                                .pipeline
                                                .origin()
                                                .ok_or_else(|| anyhow!("Missing L1 origin"))?,
                                            system_config: None,
                                        }
                                        .signal(),
                                    )
                                    .await?;
                            }
                        }
                        PipelineErrorKind::Critical(_) => return Err(e.into()),
                    }
                }
            }

            if let Some(attrs) = self.pipeline.next() {
                return Ok(attrs);
            }
        }
    }

    /// Finds the startup information for the derivation pipeline.
    ///
    /// ## Takes
    /// - `caching_oracle`: The caching oracle.
    /// - `boot_info`: The boot information.
    /// - `chain_provider`: The L1 chain provider.
    /// - `l2_chain_provider`: The L2 chain provider.
    ///
    /// ## Returns
    /// - A tuple containing the L1 origin block information and the L2 safe head information.
    async fn find_startup_info(
        caching_oracle: &O,
        boot_info: &BootInfo,
        chain_provider: &mut OracleL1ChainProvider<O>,
        l2_chain_provider: &mut MultiblockOracleL2ChainProvider<O>,
    ) -> Result<(BlockInfo, L2BlockInfo, Sealed<Header>)> {
        // Find the initial safe head, based off of the starting L2 block number in the boot info.
        caching_oracle
            .write(
                &HintType::StartingL2Output
                    .encode_with(&[boot_info.agreed_l2_output_root.as_ref()]),
            )
            .await?;
        let mut output_preimage = [0u8; 128];
        caching_oracle
            .get_exact(
                PreimageKey::new(*boot_info.agreed_l2_output_root, PreimageKeyType::Keccak256),
                &mut output_preimage,
            )
            .await?;

        let safe_hash = output_preimage[96..128]
            .try_into()
            .map_err(|_| anyhow!("Invalid L2 output root"))?;
        let safe_header = l2_chain_provider.header_by_hash(safe_hash)?;
        let safe_head_info = l2_chain_provider
            .l2_block_info_by_number(safe_header.number)
            .await?;

        let l1_origin = chain_provider
            .block_info_by_number(safe_head_info.l1_origin.number)
            .await?;

        Ok((
            l1_origin,
            safe_head_info,
            Sealed::new_unchecked(safe_header, safe_hash),
        ))
    }

    /// Returns a new [StatelessL2BlockExecutor] instance.
    ///
    /// ## Takes
    /// - `cfg`: The rollup configuration.
    /// - `provider`: The trie provider.
    /// - `hinter`: The trie hinter.
    /// - `handle_register`: The handle register for the EVM.
    pub fn new_executor<'a, P, H>(
        &mut self,
        cfg: &'a RollupConfig,
        provider: &P,
        hinter: &H,
        handle_register: KonaHandleRegister<P, H>,
    ) -> StatelessL2BlockExecutor<'a, P, H>
    where
        P: TrieProvider + Send + Sync + Clone,
        H: TrieHinter + Send + Sync + Clone,
    {
        StatelessL2BlockExecutor::builder(cfg, provider.clone(), hinter.clone())
            .with_parent_header(self.l2_safe_head_header().clone())
            .with_handle_register(handle_register)
            .build()
    }
}
