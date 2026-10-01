//! [`HintHandler`] for the [`AltDAChainHost`].
//!
//! Routes standard kona hints to [`SingleChainHintHandler`] and handles `altda-commitment`
//! hints by fetching batch data from the DA server and storing it in the preimage oracle.

use alloy_primitives::hex;
use anyhow::{bail, ensure, Result};
use async_trait::async_trait;
use kona_host::{
    single::SingleChainHintHandler, HintHandler, NonRetryableHintError, OnlineHostBackendCfg,
    SharedKeyValueStore,
};
use kona_preimage::PreimageKey;
use kona_proof::Hint;
use op_succinct_altda_client_utils::data_source::{
    GENERIC_COMMITMENT_TYPE, KECCAK256_COMMITMENT_TYPE,
};
use tracing::info;

use crate::cfg::{AltDAChainHost, AltDAExtendedHintType};

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
                fetch_altda_commitment(
                    &hint,
                    &providers.da_server_url,
                    &providers.http_client,
                    providers.max_input_size,
                    kv,
                )
                .await
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
    da_server_url: &str,
    http_client: &reqwest::Client,
    max_input_size: u64,
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
            let url =
                format!("{}/get/0x{}", da_server_url, hex::encode(encoded_commitment.as_ref()));

            let response = http_client
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

            let batch_data = read_batch_data(response, max_input_size).await?;

            info!(
                target: "altda_host",
                "Fetched {} bytes of batch data for AltDA commitment 0x{}",
                batch_data.len(),
                hex::encode(commitment_hash)
            );

            // Store the batch data in the KV store under the keccak256 preimage key.
            // The client reads this via: oracle.get(PreimageKey::new_keccak256(commitment_hash))
            let mut kv_lock = kv.write().await;
            kv_lock.set(PreimageKey::new_keccak256(commitment_hash).into(), batch_data)?;
        }
        GENERIC_COMMITMENT_TYPE => {
            bail!("Generic AltDA commitments are not supported (type 0x{:02x})", commitment_type);
        }
        _ => {
            bail!("Unknown AltDA commitment type: 0x{:02x}", commitment_type);
        }
    }

    Ok(())
}

async fn read_batch_data(mut response: reqwest::Response, max_input_size: u64) -> Result<Vec<u8>> {
    if response.content_length().is_some_and(|length| length > max_input_size) {
        return Err(oversized_response(max_input_size));
    }

    let mut batch_data = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read DA server response body: {e}"))?
    {
        if chunk.len() as u64 > max_input_size - batch_data.len() as u64 {
            return Err(oversized_response(max_input_size));
        }
        batch_data.extend_from_slice(&chunk);
    }
    Ok(batch_data)
}

fn oversized_response(max_input_size: u64) -> anyhow::Error {
    NonRetryableHintError(anyhow::anyhow!(
        "DA server response exceeds AltDA max input size of {max_input_size} bytes"
    ))
    .into()
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::Arc,
    };

    use alloy_primitives::keccak256;
    use kona_host::{MemoryKeyValueStore, OnlineHostBackend, PreimageServer};
    use kona_preimage::{
        BidirectionalChannel, HintReader, HintWriter, HintWriterClient, OracleReader, OracleServer,
        PreimageOracleClient,
    };
    use tokio::sync::RwLock;

    use super::*;

    fn serve_body(body: &'static [u8], chunked: bool) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 1024];
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                let read = stream.read(&mut buffer).unwrap();
                assert!(read > 0);
                request.extend_from_slice(&buffer[..read]);
            }
            if chunked {
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(body).unwrap();
                stream.write_all(b"\r\n0\r\n\r\n").unwrap();
            } else {
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len())
                    .unwrap();
                stream.write_all(body).unwrap();
            }
        });
        format!("http://{address}")
    }

    async fn response_body(body: &'static [u8], chunked: bool) -> reqwest::Response {
        reqwest::get(serve_body(body, chunked)).await.unwrap()
    }

    struct TestCfg {
        da_server_url: String,
        http_client: reqwest::Client,
        max_input_size: u64,
    }

    impl OnlineHostBackendCfg for TestCfg {
        type HintType = AltDAExtendedHintType;
        type Providers = ();
    }

    struct TestHandler;

    #[async_trait]
    impl HintHandler for TestHandler {
        type Cfg = TestCfg;

        async fn fetch_hint(
            hint: Hint<AltDAExtendedHintType>,
            cfg: &TestCfg,
            _providers: &(),
            kv: SharedKeyValueStore,
        ) -> Result<()> {
            fetch_altda_commitment(
                &hint,
                &cfg.da_server_url,
                &cfg.http_client,
                cfg.max_input_size,
                kv,
            )
            .await
        }
    }

    #[tokio::test]
    async fn rejects_oversized_response_before_buffering() {
        assert_eq!(
            read_batch_data(response_body(b"12345678", false).await, 8).await.unwrap(),
            b"12345678"
        );
        for chunked in [false, true] {
            let error =
                read_batch_data(response_body(b"123456789", chunked).await, 8).await.unwrap_err();
            assert!(error.downcast_ref::<NonRetryableHintError>().is_some());
        }
    }

    #[tokio::test]
    async fn oversized_commitment_is_not_stored() {
        let body = b"123456789";
        let hash = keccak256(body);
        let encoded = [vec![KECCAK256_COMMITMENT_TYPE], hash.to_vec()].concat();
        let kv: SharedKeyValueStore = Arc::new(RwLock::new(MemoryKeyValueStore::new()));
        let backend = OnlineHostBackend::new(
            TestCfg {
                da_server_url: serve_body(body, true),
                http_client: reqwest::Client::new(),
                max_input_size: 8,
            },
            kv.clone(),
            (),
            TestHandler,
        );
        let hint_channel = BidirectionalChannel::new().unwrap();
        let preimage_channel = BidirectionalChannel::new().unwrap();
        let server = tokio::spawn(
            PreimageServer::new(
                OracleServer::new(preimage_channel.host),
                HintReader::new(hint_channel.host),
                Arc::new(backend),
            )
            .start(),
        );
        let hint_writer = HintWriter::new(hint_channel.client);
        hint_writer.write(&format!("altda-commitment {}", hex::encode(encoded))).await.unwrap();
        let key = PreimageKey::new_keccak256(*hash);
        tokio::time::timeout(
            std::time::Duration::from_secs(1),
            OracleReader::new(preimage_channel.client).get(key),
        )
        .await
        .expect("oversized response must not leave the client waiting")
        .unwrap_err();
        let error = tokio::time::timeout(std::time::Duration::from_secs(1), server)
            .await
            .expect("oversized response must stop the preimage server")
            .unwrap()
            .unwrap_err();
        drop(hint_writer);
        assert!(error.to_string().contains("exceeds AltDA max input size"));
        assert!(kv.read().await.get(key.into()).is_none());
    }
}
