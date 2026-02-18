//! Validium DA Data Source
//!
//! Wraps `EthereumDataSource` and intercepts AltDA commitments.
//!
//! L1 calldata format for validium:
//!   `[0x01] [0x00] [32 bytes keccak256 hash]`
//!
//! When the pipeline reads a batcher tx with this format,
//! `ValidiumDADataSource` extracts the commitment, looks up the actual data
//! in `ValidiumBlobStore`, verifies `keccak256(data) == commitment`,
//! and returns the data to the pipeline.

use std::fmt::Debug;

use alloy_primitives::{Address, Bytes, B256};
use async_trait::async_trait;
use kona_derive::{
    BlobProvider, ChainProvider, DataAvailabilityProvider, EthereumDataSource, PipelineError,
    PipelineResult,
};
use kona_protocol::BlockInfo;

use crate::blob_store::{ValidiumBlobStore, ALTDA_TX_DATA_VERSION, KECCAK256_COMMITMENT_TYPE};

/// Validium DA Data Source.
///
/// Wraps `EthereumDataSource` and intercepts AltDA commitments (version byte 0x01).
/// When a commitment is found, looks up the actual data in `ValidiumBlobStore`
/// and verifies the keccak256 hash matches.
#[derive(Debug, Clone)]
pub struct ValidiumDADataSource<C, B>
where
    C: ChainProvider + Send + Clone,
    B: BlobProvider + Send + Clone,
{
    /// The underlying Ethereum data source (for L1 calldata/blobs).
    pub ethereum_source: EthereumDataSource<C, B>,
    /// The validium blob store (off-chain batch data keyed by hash).
    pub blob_store: ValidiumBlobStore,
}

impl<C, B> ValidiumDADataSource<C, B>
where
    C: ChainProvider + Send + Clone + Debug,
    B: BlobProvider + Send + Clone + Debug,
{
    /// Creates a new `ValidiumDADataSource`.
    pub const fn new(
        ethereum_source: EthereumDataSource<C, B>,
        blob_store: ValidiumBlobStore,
    ) -> Self {
        Self { ethereum_source, blob_store }
    }
}

#[async_trait]
impl<C, B> DataAvailabilityProvider for ValidiumDADataSource<C, B>
where
    C: ChainProvider + Send + Sync + Clone + Debug,
    B: BlobProvider + Send + Sync + Clone + Debug,
{
    type Item = Bytes;

    async fn next(
        &mut self,
        block_ref: &BlockInfo,
        batcher_address: Address,
    ) -> PipelineResult<Self::Item> {
        // Get data from the Ethereum source (reads L1 batcher txs).
        let data = self.ethereum_source.next(block_ref, batcher_address).await?;

        // Check if this is an AltDA commitment: [0x01] [0x00] [32 bytes keccak256]
        if data.len() == 34
            && data[0] == ALTDA_TX_DATA_VERSION
            && data[1] == KECCAK256_COMMITMENT_TYPE
        {
            let commitment = B256::from_slice(&data[2..34]);

            // Look up + verify: keccak256(data) == commitment.
            let batch_data =
                self.blob_store.get_by_commitment(&commitment).ok_or_else(|| {
                    PipelineError::Provider(format!(
                        "Validium: batch data not found for commitment {}. \
                         Ensure off-chain data is included in the witness.",
                        commitment
                    ))
                    .crit()
                })?;

            return Ok(Bytes::from(batch_data));
        }

        // Not an AltDA commitment — pass through as normal data.
        Ok(data)
    }

    fn clear(&mut self) {
        self.ethereum_source.clear();
    }
}
