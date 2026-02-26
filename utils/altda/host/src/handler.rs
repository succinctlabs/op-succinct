//! [`HintHandler`] for the [`AltDAChainHost`].
//!
//! Routes standard kona hints to [`SingleChainHintHandler`] and handles `altda-commitment`
//! hints by fetching batch data from the DA server and storing it in the preimage oracle.

use alloy_primitives::hex;
use anyhow::{ensure, Result};
use async_trait::async_trait;
use kona_host::{
    single::SingleChainHintHandler, HintHandler, OnlineHostBackendCfg, SharedKeyValueStore,
};
use kona_preimage::PreimageKey;
use kona_proof::Hint;
use op_succinct_altda_client_utils::data_source::{
    GENERIC_COMMITMENT_TYPE, KECCAK256_COMMITMENT_TYPE,
};
use tracing::{info, warn};

use crate::cfg::{AltDAChainHost, AltDAChainProviders, AltDAExtendedHintType};

/// The [`HintHandler`] for the [`AltDAChainHost`].
///
/// Routes hints based on their type:
/// - Standard kona hints (`L1BlockHeader`, `L1Transactions`, etc.) are delegated to
///   [`SingleChainHintHandler`].
/// - `AltDACommitment` hints are handled by fetching batch data from the DA server.
#[derive(Debug, Clone, Copy)]
pub struct AltDAHintHandler;

#[async_trait]
impl HintHandler for AltDAHintHandler {
    type Cfg = AltDAChainHost;

    async fn fetch_hint(
        hint: Hint<<Self::Cfg as OnlineHostBackendCfg>::HintType>,
        cfg: &Self::Cfg,
        providers: &<Self::Cfg as OnlineHostBackendCfg>::Providers,
        kv: SharedKeyValueStore,
    ) -> Result<()> {
        match hint.ty {
            AltDAExtendedHintType::Standard(ty) => {
                let inner_hint = Hint { ty, data: hint.data };
                SingleChainHintHandler::fetch_hint(
                    inner_hint,
                    &cfg.single_host,
                    &providers.inner_providers,
                    kv,
                )
                .await
            }
            AltDAExtendedHintType::AltDACommitment => {
                fetch_altda_commitment(&hint, providers, kv).await
            }
        }
    }
}

/// Fetches batch data from the DA server for an AltDA commitment hint.
///
/// The hint data contains the encoded commitment: `[commitment_type_byte][commitment_data...]`.
/// This is the same encoding as Go's `CommitmentData.Encode()`, which is used directly in the
/// DA server's GET endpoint URL.
///
/// For Keccak256 commitments (type `0x00`), the commitment data is a 32-byte keccak256 hash.
/// The resolved batch data is stored in the KV store under `PreimageKey::new_keccak256(hash)`,
/// which matches what the client reads via `oracle.get(PreimageKey::new_keccak256(hash))`.
///
/// DA server endpoint: `GET {da_server_url}/get/0x{hex(encoded_commitment)}`
/// This matches Go's `DAClient.GetInput`: `fmt.Sprintf("%s/get/0x%x", c.url, comm.Encode())`
async fn fetch_altda_commitment(
    hint: &Hint<AltDAExtendedHintType>,
    providers: &AltDAChainProviders,
    kv: SharedKeyValueStore,
) -> Result<()> {
    let encoded_commitment = &hint.data;

    ensure!(!encoded_commitment.is_empty(), "AltDA commitment hint data is empty");

    let commitment_type = encoded_commitment[0];
    let commitment_data = &encoded_commitment[1..];

    match commitment_type {
        KECCAK256_COMMITMENT_TYPE => {
            ensure!(
                commitment_data.len() == 32,
                "Keccak256 commitment must be 32 bytes, got {}",
                commitment_data.len()
            );

            let commitment_hash: [u8; 32] =
                commitment_data.try_into().expect("length already validated as 32");

            info!(
                target: "altda_host",
                "Fetching AltDA Keccak256 commitment: 0x{}",
                hex::encode(commitment_hash)
            );

            // Fetch batch data from the DA server.
            // URL format matches Go's DAClient.GetInput:
            //   GET {url}/get/0x{hex(commitment.Encode())}
            // where Encode() = [type_byte][commitment_data] = our encoded_commitment
            let url = format!(
                "{}/get/0x{}",
                providers.da_server_url,
                hex::encode(encoded_commitment.as_ref())
            );

            let response = providers
                .http_client
                .get(&url)
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to fetch from DA server: {e}"))?;

            ensure!(
                response.status().is_success(),
                "DA server returned error status {} for commitment 0x{}",
                response.status(),
                hex::encode(commitment_hash)
            );

            let batch_data = response
                .bytes()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to read DA server response body: {e}"))?;

            info!(
                target: "altda_host",
                "Fetched {} bytes of batch data for AltDA commitment 0x{}",
                batch_data.len(),
                hex::encode(commitment_hash)
            );

            // Store the batch data in the KV store under the keccak256 preimage key.
            // The client reads this via: oracle.get(PreimageKey::new_keccak256(commitment_hash))
            let mut kv_lock = kv.write().await;
            kv_lock.set(PreimageKey::new_keccak256(commitment_hash).into(), batch_data.to_vec())?;
        }
        GENERIC_COMMITMENT_TYPE => {
            warn!(
                target: "altda_host",
                "Generic AltDA commitments are not supported, skipping"
            );
        }
        _ => {
            warn!(
                target: "altda_host",
                "Unknown AltDA commitment type: 0x{:02x}, skipping",
                commitment_type
            );
        }
    }

    Ok(())
}
