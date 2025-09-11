use alloy_consensus::SignableTransaction;
use alloy_primitives::{hex, Address, ChainId, Signature, B256};
use alloy_signer::{sign_transaction_with_chain_id, Result, Signer};
use async_trait::async_trait;
use gcloud_sdk::{
    google::cloud::kms::{
        self,
        v1::{key_management_service_client::KeyManagementServiceClient, AsymmetricSignRequest},
    },
    tonic::{self, Request},
    GoogleApi, GoogleAuthMiddleware, TokenSourceType,
    GCP_DEFAULT_SCOPES,
};
use k256::ecdsa;
use std::fmt::{self, Debug};
use thiserror::Error;

type Client = GoogleApi<KeyManagementServiceClient<GoogleAuthMiddleware>>;

pub async fn init_client(creds_json_hex: String) -> Result<Client, GcpSignerError> {
    let creds_json_bytes = hex::decode(&creds_json_hex)
        .map_err(|e| GcpSignerError::HexDecodeError(e.to_string()))?;
    let creds_json = String::from_utf8(creds_json_bytes)
        .map_err(|e| GcpSignerError::Utf8DecodeError(e.to_string()))?;
    
    let client = Client::from_function_with_token_source(
        KeyManagementServiceClient::new,
        "https://cloudkms.googleapis.com",
        None,
        GCP_DEFAULT_SCOPES.clone(),
        TokenSourceType::Json(creds_json),
    )
    .await
    .expect("Failed to create GCP KMS Client");
    Ok(client)
}

#[derive(Clone)]
pub struct GcpSigner {
    client: Client,
    key_name: String,
    chain_id: Option<ChainId>,
    address: Address,
}

impl fmt::Debug for GcpSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GcpSigner")
            .field("key_name", &self.key_name)
            .field("chain_id", &self.chain_id)
            .field("address", &self.address)
            .finish()
    }
}

/// Errors thrown by [`GcpSigner`].
#[derive(Debug, Error)]
pub enum GcpSignerError {
    /// Thrown when the GCP KMS API returns a signing error.
    #[error(transparent)]
    GoogleKmsError(#[from] gcloud_sdk::error::Error),

    /// Thrown on a request error.
    #[error(transparent)]
    RequestError(#[from] tonic::Status),

    /// [`ecdsa`] error.
    #[error(transparent)]
    K256(#[from] ecdsa::Error),

    /// Hex decoding error.
    #[error("Failed to decode hex string: {0}")]
    HexDecodeError(String),

    /// UTF-8 decoding error.
    #[error("Failed to decode UTF-8 string: {0}")]
    Utf8DecodeError(String),
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl alloy_network::TxSigner<Signature> for GcpSigner {
    fn address(&self) -> Address {
        self.address
    }

    #[inline]
    #[doc(alias = "sign_tx")]
    async fn sign_transaction(
        &self,
        tx: &mut dyn SignableTransaction<Signature>,
    ) -> Result<Signature> {
        sign_transaction_with_chain_id!(self, tx, self.sign_hash(&tx.signature_hash()).await)
    }
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl Signer for GcpSigner {
    #[instrument(err)]
    #[allow(clippy::blocks_in_conditions)]
    async fn sign_hash(&self, hash: &B256) -> Result<Signature> {
        self.sign_digest_inner(hash).await.map_err(alloy_signer::Error::other)
    }

    #[inline]
    fn address(&self) -> Address {
        self.address
    }

    #[inline]
    fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    #[inline]
    fn set_chain_id(&mut self, chain_id: Option<ChainId>) {
        self.chain_id = chain_id;
    }
}

alloy_network::impl_into_wallet!(GcpSigner);

impl GcpSigner {
    /// Instantiate a new signer from an existing `Client`, keyring reference, key ID, and version.
    ///
    /// Takes the Ethereum address directly instead of retrieving the public key from GCP.
    pub fn new(
        client: Client,
        key_specifier: String,
        chain_id: Option<ChainId>,
        address: Address,
    ) -> Result<Self, GcpSignerError> {
        debug!(%address, "instantiated GCP signer");
        Ok(Self { client, key_name: key_specifier, chain_id, address })
    }

    /// Sign a digest with this signer's key
    pub async fn sign_digest(&self, digest: &B256) -> Result<ecdsa::Signature, GcpSignerError> {
        request_sign_digest(&self.client, &self.key_name, digest).await.and_then(decode_signature)
    }

    /// Sign a digest with this signer's key and add the eip155 `v` value
    /// corresponding to the input chain_id
    #[instrument(err, skip(digest), fields(digest = %hex::encode(digest)))]
    async fn sign_digest_inner(&self, digest: &B256) -> Result<Signature, GcpSignerError> {
        let sig = self.sign_digest(digest).await?;
        Ok(sig_from_digest_bytes_trial_recovery_with_address(sig, digest, self.address))
    }
}

#[instrument(skip(client, digest), fields(digest = %hex::encode(digest)), err)]
async fn request_sign_digest(
    client: &Client,
    kms_key_name: &str,
    digest: &B256,
) -> Result<Vec<u8>, GcpSignerError> {
    let mut request = Request::new(AsymmetricSignRequest {
        name: kms_key_name.to_string(),
        digest: Some(kms::v1::Digest {
            digest: Some(kms::v1::digest::Digest::Sha256(digest.to_vec())),
        }),
        ..Default::default()
    });

    // Add metadata for request routing: https://cloud.google.com/kms/docs/grpc
    request
        .metadata_mut()
        .insert("x-goog-request-params", format!("name={kms_key_name}").parse().unwrap());

    let response = client.get().asymmetric_sign(request).await?;
    let signature = response.into_inner().signature;
    Ok(signature)
}

/// Decode a raw GCP KMS Signature response.
fn decode_signature(raw: Vec<u8>) -> Result<ecdsa::Signature, GcpSignerError> {
    let sig = ecdsa::Signature::from_der(raw.as_ref())?;
    Ok(sig.normalize_s().unwrap_or(sig))
}

/// Recover an RSig from a signature by trial/error using address verification.
fn sig_from_digest_bytes_trial_recovery_with_address(
    sig: ecdsa::Signature,
    hash: &B256,
    expected_address: Address,
) -> Signature {
    let signature = Signature::from_signature_and_parity(sig, false);
    if check_candidate_with_address(&signature, hash, expected_address) {
        return signature;
    }

    let signature = signature.with_parity(true);
    if check_candidate_with_address(&signature, hash, expected_address) {
        return signature;
    }

    panic!("bad sig");
}

/// Makes a trial recovery to check whether an RSig corresponds to the expected address.
fn check_candidate_with_address(
    signature: &Signature,
    hash: &B256,
    expected_address: Address,
) -> bool {
    signature
        .recover_from_prehash(hash)
        .map(|key| alloy_signer::utils::public_key_to_address(&key) == expected_address)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sign_message() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install default crypto provider");
        let specifier = std::env::var("HSM_API_NAME").expect("HSM_API_NAME");
        let address_str = std::env::var("HSM_ADDRESS").expect("HSM_ADDRESS");
        let creds_json_hex = std::env::var("HSM_CREDENTIALS").expect("HSM_CREDENTIALS");
        let address: Address = address_str.parse().expect("Invalid address format");
        let client = init_client(creds_json_hex).await.unwrap();
        let signer = GcpSigner::new(client, specifier, None, address).expect("get key");

        let message = vec![0, 1, 2, 3];
        let sig = signer.sign_message(&message).await.unwrap();
        assert_eq!(sig.recover_address_from_msg(message).unwrap(), signer.address());
    }
}
