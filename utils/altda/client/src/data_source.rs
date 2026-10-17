//! Contains the [`AltDADataSource`], a concrete implementation of the
//! [`DataAvailabilityProvider`] trait for the OP Stack AltDA (Keccak256 commitment) protocol.
//!
//! The AltDA data source wraps an [`EthereumDataSource`] and intercepts batcher transactions
//! that contain AltDA commitments (identified by the `DerivationVersion1` byte prefix `0x01`).
//! When a commitment is detected, it sends a hint to the host to fetch the actual batch data
//! from the DA server, then reads the resolved data from the preimage oracle.
//!
//! This mirrors the Go implementation at `op-node/rollup/derive/altda_data_source.go`.

use alloy_primitives::{keccak256, Address, Bytes};
use anyhow::{ensure, Result};
use async_trait::async_trait;
use kona_derive::{
    BlobProvider, ChainProvider, DataAvailabilityProvider, EthereumDataSource, PipelineError,
    PipelineErrorKind, PipelineResult,
};
use kona_genesis::RollupConfig;
use kona_preimage::{CommsClient, PreimageKey};
use kona_proof::Hint;
use kona_protocol::BlockInfo;
use std::{fmt::Debug, sync::Arc};
use tracing::{info, warn};

use crate::hint::AltDAHintType;

/// AltDA derivation version byte (`0x01`).
///
/// When the first byte of batcher transaction data is `0x01`, it indicates the data contains
/// an AltDA commitment rather than normal batch data (which uses `0x00` for calldata/blobs).
///
/// Go: `op-node/rollup/derive/params/versions.go:DerivationVersion1 = 1`
/// Spec: `specs/protocol/derivation.md` (batch submission wire format)
pub const ALTDA_DERIVATION_VERSION: u8 = 0x01;

/// Keccak256 commitment type byte (`0x00`).
///
/// The commitment data is a 32-byte keccak256 hash of the batch data.
///
/// Go: `op-alt-da/commitment.go:Keccak256CommitmentType = 0`
/// Spec: `specs/experimental/alt-da.md` (commitment type definitions)
pub const KECCAK256_COMMITMENT_TYPE: u8 = 0x00;

/// Generic commitment type byte (`0x01`).
///
/// The commitment data is an opaque bytestring. The DA server validates on store/retrieve.
///
/// Go: `op-alt-da/commitment.go:GenericCommitmentType = 1`
/// Spec: `specs/experimental/alt-da.md` (commitment type definitions)
pub const GENERIC_COMMITMENT_TYPE: u8 = 0x01;

/// The op-alt-da default for Keccak commitments when the rollup omits its limit.
const DEFAULT_MAX_INPUT_SIZE: u64 = 130_672;

/// A data source that wraps [`EthereumDataSource`] and resolves AltDA commitments.
///
/// When the batcher posts AltDA commitments (version byte `0x01`) instead of raw batch data,
/// this source:
///
/// 1. Reads raw L1 calldata from the inner [`EthereumDataSource`]
/// 2. Detects the `DerivationVersion1` prefix (`0x01`)
/// 3. Decodes the commitment type and data
/// 4. Sends an [`AltDAHintType::AltDACommitment`] hint to the host
/// 5. Reads the resolved batch data from the preimage oracle keyed by the commitment hash
/// 6. Returns the resolved batch data to the pipeline
///
/// For Keccak256 commitments, this source verifies the complete commitment hash before returning
/// the resolved batch data.
///
/// Follows the same pattern as hokulea's `EigenDADataSource` and the Go `AltDADataSource`.
#[derive(Debug, Clone)]
pub struct AltDADataSource<C, B, O>
where
    C: ChainProvider + Send + Clone,
    B: BlobProvider + Send + Clone,
    O: CommsClient + Send + Clone,
{
    /// The inner Ethereum data source that reads raw L1 transaction data.
    pub ethereum_source: EthereumDataSource<C, B>,
    /// The oracle client for sending hints and reading preimages.
    pub oracle: Arc<O>,
    max_input_size: u64,
    /// Pending commitment from a previous iteration that encountered a temporary error
    /// during resolution. Stored so we can retry without re-reading from L1.
    pending_commitment: Option<PendingCommitment>,
}

/// A parsed AltDA commitment waiting to be resolved.
#[derive(Debug, Clone)]
struct PendingCommitment {
    /// The full commitment data: `[type_byte][commitment_bytes...]`
    encoded: Vec<u8>,
    /// The parsed commitment type.
    commitment_type: u8,
    /// The raw commitment bytes (without the type prefix).
    commitment_data: Vec<u8>,
}

impl<C, B, O> AltDADataSource<C, B, O>
where
    C: ChainProvider + Send + Clone + Debug,
    B: BlobProvider + Send + Clone + Debug,
    O: CommsClient + Send + Clone + Debug,
{
    /// Creates a new [`AltDADataSource`].
    pub fn new(
        ethereum_source: EthereumDataSource<C, B>,
        oracle: Arc<O>,
        rollup_config: &RollupConfig,
    ) -> Result<Self> {
        let max_input_size = rollup_config
            .alt_da_config
            .as_ref()
            .and_then(|config| config.da_max_input_size)
            .unwrap_or(DEFAULT_MAX_INPUT_SIZE);
        ensure!(max_input_size > 0, "AltDA da_max_input_size must be greater than zero");
        Ok(Self { ethereum_source, oracle, max_input_size, pending_commitment: None })
    }
}

#[async_trait]
impl<C, B, O> DataAvailabilityProvider for AltDADataSource<C, B, O>
where
    C: ChainProvider + Send + Sync + Clone + Debug,
    B: BlobProvider + Send + Sync + Clone + Debug,
    O: CommsClient + Send + Sync + Clone + Debug,
{
    type Item = Bytes;

    async fn next(
        &mut self,
        block_ref: &BlockInfo,
        batcher_addr: Address,
    ) -> PipelineResult<Self::Item> {
        // If we have a pending commitment from a previous failed resolution attempt,
        // retry resolving it before fetching new data from L1.
        if self.pending_commitment.is_none() {
            let data = match self.ethereum_source.next(block_ref, batcher_addr).await {
                Ok(d) => d,
                Err(e) => {
                    // On EOF, clear state before propagating (matches EigenDA pattern).
                    if let PipelineErrorKind::Temporary(PipelineError::Eof) = e {
                        self.clear();
                    }
                    return Err(e);
                }
            };

            // Empty data — signal the pipeline to skip.
            // Matches Go: `if len(data) == 0 { return nil, NotEnoughData }`
            if data.is_empty() {
                return Err(PipelineError::NotEnoughData.temp());
            }

            // If the tx data type is not AltDA (version byte != 0x01), pass it through
            // unchanged to the downstream pipeline stages.
            if data[0] != ALTDA_DERIVATION_VERSION {
                return Ok(data);
            }

            // Parse the AltDA commitment.
            // Wire format: [0x01(version)] [type_byte] [commitment_data...]
            // After stripping the version byte: [type_byte] [commitment_data...]
            let commitment_bytes = &data[1..];
            if commitment_bytes.is_empty() {
                warn!(target: "altda", "AltDA commitment is empty after version byte, skipping");
                return Err(PipelineError::NotEnoughData.temp());
            }

            let commitment_type = commitment_bytes[0];
            let commitment_data = &commitment_bytes[1..];

            // Validate commitment based on type.
            match commitment_type {
                KECCAK256_COMMITMENT_TYPE => {
                    if commitment_data.len() != 32 {
                        warn!(
                            target: "altda",
                            "Keccak256 commitment must be 32 bytes, got {}, skipping",
                            commitment_data.len()
                        );
                        return Err(PipelineError::NotEnoughData.temp());
                    }
                }
                GENERIC_COMMITMENT_TYPE => {
                    if commitment_data.is_empty() {
                        warn!(target: "altda", "Generic commitment is empty, skipping");
                        return Err(PipelineError::NotEnoughData.temp());
                    }
                }
                _ => {
                    warn!(
                        target: "altda",
                        "Unknown AltDA commitment type: 0x{:02x}, skipping",
                        commitment_type
                    );
                    return Err(PipelineError::NotEnoughData.temp());
                }
            }

            self.pending_commitment = Some(PendingCommitment {
                encoded: commitment_bytes.to_vec(),
                commitment_type,
                commitment_data: commitment_data.to_vec(),
            });
        }

        // Resolve the pending commitment by sending a hint to the host and reading
        // the resolved batch data from the preimage oracle.
        let commitment = self.pending_commitment.as_ref().expect("pending commitment must be set");

        match commitment.commitment_type {
            KECCAK256_COMMITMENT_TYPE => {
                let result = self.resolve_keccak256_commitment(commitment).await;
                match &result {
                    Ok(data) => {
                        // Consume oversized inputs too, so the next call can make progress.
                        self.pending_commitment = None;
                        if data.len() as u64 > self.max_input_size {
                            warn!(target: "altda", size = data.len(), max = self.max_input_size,
                                "AltDA input exceeds maximum size, skipping");
                            return Err(PipelineError::NotEnoughData.temp());
                        }
                    }
                    Err(PipelineErrorKind::Temporary(_)) => {
                        // Temporary error — keep the pending commitment for retry.
                    }
                    Err(_) => {
                        // Critical error — clear the pending commitment.
                        self.pending_commitment = None;
                    }
                }
                result
            }
            GENERIC_COMMITMENT_TYPE => {
                // Generic commitments are not supported in this implementation.
                // Clear the pending commitment and skip.
                warn!(target: "altda", "Generic AltDA commitments are not supported, skipping");
                self.pending_commitment = None;
                Err(PipelineError::NotEnoughData.temp())
            }
            _ => {
                // Unknown commitment type — clear and skip (should not reach here).
                self.pending_commitment = None;
                Err(PipelineError::NotEnoughData.temp())
            }
        }
    }

    fn clear(&mut self) {
        self.ethereum_source.clear();
        self.pending_commitment = None;
    }
}

impl<C, B, O> AltDADataSource<C, B, O>
where
    C: ChainProvider + Send + Sync + Clone + Debug,
    B: BlobProvider + Send + Sync + Clone + Debug,
    O: CommsClient + Send + Sync + Clone + Debug,
{
    /// Resolves a Keccak256 AltDA commitment by:
    ///
    /// 1. Sending an `altda-commitment` hint to the host with the encoded commitment bytes
    /// 2. Reading the resolved batch data from the preimage oracle using the commitment hash as the
    ///    keccak256 preimage key
    ///
    /// The commitment data is exactly 32 bytes: the keccak256 hash of the original batch data.
    /// The preimage key identifies the data in the oracle, and the explicit hash comparison below
    /// binds all 32 commitment bytes within the ZK proof.
    async fn resolve_keccak256_commitment(
        &self,
        commitment: &PendingCommitment,
    ) -> PipelineResult<Bytes> {
        let commitment_hash: [u8; 32] =
            commitment.commitment_data.as_slice().try_into().map_err(|_| {
                PipelineError::Provider("keccak256 commitment must be 32 bytes".to_string()).crit()
            })?;

        info!(
            target: "altda",
            "Resolving AltDA Keccak256 commitment: 0x{}",
            alloy_primitives::hex::encode(commitment_hash)
        );

        // Send hint to the host to fetch the batch data from the DA server.
        // Hint data is the full encoded commitment: [type_byte][commitment_data...]
        // The host will parse this to determine the commitment type and fetch accordingly.
        Hint::new(AltDAHintType::AltDACommitment, commitment.encoded.clone())
            .send(&*self.oracle)
            .await
            .map_err(|e| PipelineError::Provider(e.to_string()).temp())?;

        // Read the resolved batch data from the preimage oracle.
        // For Keccak256 commitments, the preimage key IS the commitment hash, since by
        // definition keccak256(batch_data) == commitment_hash. The host stores the fetched
        // batch data under this key.
        let resolved_data = self
            .oracle
            .get(PreimageKey::new_keccak256(commitment_hash))
            .await
            .map_err(|e| PipelineError::Provider(e.to_string()).temp())?;

        if keccak256(&resolved_data).0 != commitment_hash {
            return Err(PipelineError::Provider(
                "AltDA input does not match Keccak256 commitment".to_string(),
            )
            .crit())
        }

        info!(
            target: "altda",
            "Resolved AltDA Keccak256 commitment: {} bytes of batch data",
            resolved_data.len()
        );

        Ok(resolved_data.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::keccak256;
    use kona_derive::test_utils::{TestBlobProvider, TestChainProvider};
    use kona_genesis::AltDAConfig;
    use op_succinct_client_utils::witness::preimage_store::PreimageStore;

    fn source_with_batches(
        batches: &[Vec<u8>],
        limit: Option<u64>,
    ) -> Result<AltDADataSource<TestChainProvider, TestBlobProvider, PreimageStore>> {
        let config = RollupConfig {
            alt_da_config: Some(AltDAConfig { da_max_input_size: limit, ..Default::default() }),
            ..Default::default()
        };
        let mut ethereum_source = EthereumDataSource::new_from_parts(
            TestChainProvider::default(),
            TestBlobProvider::default(),
            &config,
        );
        let mut oracle = PreimageStore::default();
        // Start at the same buffered-calldata boundary as the upstream source tests.
        ethereum_source.calldata_source.open = true;
        for batch in batches {
            let hash = keccak256(batch).0;
            oracle.save_preimage(PreimageKey::new_keccak256(hash), batch.clone()).unwrap();
            let mut commitment = vec![ALTDA_DERIVATION_VERSION, KECCAK256_COMMITMENT_TYPE];
            commitment.extend_from_slice(&hash);
            ethereum_source.calldata_source.calldata.push_back(commitment.into());
        }
        AltDADataSource::new(ethereum_source, Arc::new(oracle), &config)
    }

    #[tokio::test]
    async fn oversized_commitment_is_skipped_without_blocking_next_batch() {
        let valid = vec![42; 100];
        let mut source = source_with_batches(&[vec![1; 130_673], valid.clone()], None).unwrap();

        assert!(matches!(
            source.next(&BlockInfo::default(), Address::ZERO).await,
            Err(PipelineErrorKind::Temporary(PipelineError::NotEnoughData))
        ));
        assert!(source.pending_commitment.is_none());
        assert_eq!(
            source.next(&BlockInfo::default(), Address::ZERO).await.unwrap(),
            Bytes::from(valid)
        );
        assert!(matches!(
            source.next(&BlockInfo::default(), Address::ZERO).await,
            Err(PipelineErrorKind::Temporary(PipelineError::Eof))
        ));
        assert!(!source.ethereum_source.calldata_source.open);
    }

    #[tokio::test]
    async fn configured_and_default_limits_accept_boundary_and_skip_one_byte_over() {
        for configured in [None, Some(1), Some(200_000)] {
            let limit = configured.unwrap_or(DEFAULT_MAX_INPUT_SIZE) as usize;
            let boundary = vec![42; limit];
            let mut source =
                source_with_batches(&[vec![1; limit + 1], boundary.clone()], configured).unwrap();
            assert!(matches!(
                source.next(&BlockInfo::default(), Address::ZERO).await,
                Err(PipelineErrorKind::Temporary(PipelineError::NotEnoughData))
            ));
            assert_eq!(
                source.next(&BlockInfo::default(), Address::ZERO).await.unwrap(),
                Bytes::from(boundary)
            );
        }
    }

    #[test]
    fn zero_limit_is_rejected_before_derivation() {
        assert_eq!(
            source_with_batches(&[], Some(0)).unwrap_err().to_string(),
            "AltDA da_max_input_size must be greater than zero"
        );
    }

    #[tokio::test]
    async fn missing_preimage_retries_pending_commitment_then_advances() {
        let first = vec![42; 8];
        let second = vec![43; 8];
        let mut source = source_with_batches(&[first.clone(), second.clone()], Some(8)).unwrap();
        let key = PreimageKey::new_keccak256(keccak256(&first).0);
        Arc::get_mut(&mut source.oracle).unwrap().preimage_map.remove(&key);
        for _ in 0..2 {
            assert!(matches!(
                source.next(&BlockInfo::default(), Address::ZERO).await,
                Err(PipelineErrorKind::Temporary(PipelineError::Provider(_)))
            ));
            assert!(source.pending_commitment.is_some());
            assert_eq!(source.ethereum_source.calldata_source.calldata.len(), 1);
        }
        Arc::get_mut(&mut source.oracle).unwrap().save_preimage(key, first.clone()).unwrap();
        assert_eq!(
            source.next(&BlockInfo::default(), Address::ZERO).await.unwrap(),
            Bytes::from(first)
        );
        assert!(source.pending_commitment.is_none());
        assert_eq!(
            source.next(&BlockInfo::default(), Address::ZERO).await.unwrap(),
            Bytes::from(second)
        );
    }

    #[tokio::test]
    async fn first_commitment_byte_mismatch_is_rejected() {
        let mut source = source_with_batches(&[vec![42; 8]], Some(8)).unwrap();
        let mut commitment =
            source.ethereum_source.calldata_source.calldata.pop_front().unwrap().to_vec();
        commitment[2] ^= 0xff;
        source.ethereum_source.calldata_source.calldata.push_back(commitment.into());

        assert!(matches!(
            source.next(&BlockInfo::default(), Address::ZERO).await,
            Err(PipelineErrorKind::Critical(PipelineError::Provider(message)))
                if message == "AltDA input does not match Keccak256 commitment"
        ));
        assert!(source.pending_commitment.is_none());
    }

    #[tokio::test]
    async fn non_altda_data_passes_through_and_generic_commitments_stay_unsupported() {
        let mut source = source_with_batches(&[], Some(1)).unwrap();
        let plain = Bytes::from(vec![0; 10]);
        source.ethereum_source.calldata_source.calldata.extend([
            vec![ALTDA_DERIVATION_VERSION, GENERIC_COMMITMENT_TYPE, 42].into(),
            plain.clone(),
        ]);
        assert!(matches!(
            source.next(&BlockInfo::default(), Address::ZERO).await,
            Err(PipelineErrorKind::Temporary(PipelineError::NotEnoughData))
        ));
        assert!(source.pending_commitment.is_none());
        assert_eq!(source.next(&BlockInfo::default(), Address::ZERO).await.unwrap(), plain);
    }

    #[tokio::test]
    async fn clear_discards_pending_commitment_after_fetch_failure() {
        let mut source = source_with_batches(&[vec![42; 8]], None).unwrap();
        Arc::get_mut(&mut source.oracle).unwrap().preimage_map.clear();
        assert!(source.next(&BlockInfo::default(), Address::ZERO).await.is_err());
        assert!(source.pending_commitment.is_some());
        source.clear();
        assert!(source.pending_commitment.is_none());
        assert!(!source.ethereum_source.calldata_source.open);
    }
}
