use std::{env, ffi::OsString, fs, path::PathBuf, time::Duration};

use anyhow::{anyhow, bail, Context, Result};
use sp1_sdk::{
    network::{signer::NetworkSigner, FulfillmentStrategy, NetworkMode},
    NetworkProver, ProverClient,
};

const NETWORK_MTLS_CERT_PATH: &str = "NETWORK_MTLS_CERT_PATH";
const NETWORK_MTLS_KEY_PATH: &str = "NETWORK_MTLS_KEY_PATH";
const NETWORK_MTLS_PREFLIGHT_TIMEOUT: Duration = Duration::from_secs(20);

/// Parse a fulfillment strategy from a string.
pub fn parse_fulfillment_strategy(value: String) -> Result<FulfillmentStrategy> {
    match value.to_ascii_lowercase().as_str() {
        "reserved" => Ok(FulfillmentStrategy::Reserved),
        "hosted" => Ok(FulfillmentStrategy::Hosted),
        "auction" => Ok(FulfillmentStrategy::Auction),
        _ => bail!(
            "Invalid fulfillment strategy '{value}': must be 'reserved', 'hosted', or 'auction'"
        ),
    }
}

/// Try to determine the network mode from the provided fulfillment strategies.
pub fn determine_network_mode(
    range_proof_strategy: FulfillmentStrategy,
    agg_proof_strategy: FulfillmentStrategy,
) -> Result<NetworkMode> {
    match (range_proof_strategy, agg_proof_strategy) {
            (FulfillmentStrategy::Auction, FulfillmentStrategy::Auction) => {
                Ok(NetworkMode::Mainnet)
            }
            (
                FulfillmentStrategy::Hosted | FulfillmentStrategy::Reserved,
                FulfillmentStrategy::Hosted | FulfillmentStrategy::Reserved,
            ) => Ok(NetworkMode::Reserved),
            (FulfillmentStrategy::UnspecifiedFulfillmentStrategy, _) |
            (_, FulfillmentStrategy::UnspecifiedFulfillmentStrategy) => Err(anyhow!(
                "The range and agg fulfillment Strategies must be specified"
            )),
            _ => Err(anyhow!(
                "The range fulfillment Strategy '{}' and agg fulfillment Strategy '{}' are incompatible",
                range_proof_strategy.as_str_name().to_ascii_lowercase(),
                agg_proof_strategy.as_str_name().to_ascii_lowercase()
            )),
        }
}

/// Compute the network signer using the `NETWORK_PRIVATE_KEY` env var.
/// If the `use_kms_requester` parameter is set to `true`, the `NETWORK_PRIVATE_KEY` env var
/// must be set with a key ARN.
pub async fn get_network_signer(use_kms_requester: bool) -> Result<NetworkSigner> {
    let network_signer = if use_kms_requester {
        // If using KMS, NETWORK_PRIVATE_KEY should be a KMS key ARN.
        let kms_key_arn = env::var("NETWORK_PRIVATE_KEY")
            .context("NETWORK_PRIVATE_KEY must be set when USE_KMS_REQUESTER is true")?;
        let signer = NetworkSigner::aws_kms(&kms_key_arn).await?;
        tracing::info!("Using KMS requester with address: {:?}", signer.address());

        signer
    } else {
        // Otherwise, use a private key with a default value to avoid errors in mock mode.
        let private_key = env::var("NETWORK_PRIVATE_KEY").unwrap_or_else(|_| {
            tracing::warn!(
                "Using default NETWORK_PRIVATE_KEY of 0x01. This is only valid in mock mode."
            );
            "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        });
        let signer = NetworkSigner::local(&private_key)?;
        tracing::info!("Using local requester with address: {:?}", signer.address());

        signer
    };

    Ok(network_signer)
}

/// Build a network prover from `USE_KMS_REQUESTER` env var, using the provided fulfillment
/// strategy.
pub async fn build_network_prover_from_env(strategy: FulfillmentStrategy) -> Result<NetworkProver> {
    let use_kms_requester = env::var("USE_KMS_REQUESTER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .context("USE_KMS_REQUESTER must be true or false")?;
    let network_signer = get_network_signer(use_kms_requester).await?;

    let network_mode = match strategy {
        FulfillmentStrategy::Auction => NetworkMode::Mainnet,
        FulfillmentStrategy::Hosted | FulfillmentStrategy::Reserved => NetworkMode::Reserved,
        _ => bail!("Fulfillment strategy must be 'reserved', 'hosted', or 'auction'"),
    };

    build_network_prover(network_mode, network_signer).await
}

/// Build a network prover with optional mutual TLS client credentials from the environment.
pub async fn build_network_prover(
    network_mode: NetworkMode,
    network_signer: NetworkSigner,
) -> Result<NetworkProver> {
    let client_identity = load_network_client_identity()?;
    let uses_client_identity = client_identity.is_some();
    let mut builder = ProverClient::builder().network_for(network_mode).signer(network_signer);

    if let Some((certificate, private_key)) = client_identity {
        builder = builder.client_identity(certificate, private_key);
    }

    let prover = builder.build().await;
    if uses_client_identity {
        tokio::time::timeout(NETWORK_MTLS_PREFLIGHT_TIMEOUT, prover.get_balance())
            .await
            .context("Network mTLS preflight timed out after 20 seconds")?
            .context("Network mTLS preflight failed")?;
    }

    Ok(prover)
}

fn load_network_client_identity() -> Result<Option<(Vec<u8>, Vec<u8>)>> {
    load_network_client_identity_from_paths(
        env::var_os(NETWORK_MTLS_CERT_PATH),
        env::var_os(NETWORK_MTLS_KEY_PATH),
    )
}

fn load_network_client_identity_from_paths(
    certificate_path: Option<OsString>,
    private_key_path: Option<OsString>,
) -> Result<Option<(Vec<u8>, Vec<u8>)>> {
    let (certificate_path, private_key_path) = match (certificate_path, private_key_path) {
        (None, None) => return Ok(None),
        (Some(certificate_path), Some(private_key_path)) => {
            (PathBuf::from(certificate_path), PathBuf::from(private_key_path))
        }
        _ => bail!("{NETWORK_MTLS_CERT_PATH} and {NETWORK_MTLS_KEY_PATH} must be set together"),
    };

    if certificate_path.as_os_str().is_empty() {
        bail!("{NETWORK_MTLS_CERT_PATH} must not be empty");
    }
    if private_key_path.as_os_str().is_empty() {
        bail!("{NETWORK_MTLS_KEY_PATH} must not be empty");
    }

    let certificate = fs::read(&certificate_path).with_context(|| {
        format!("Failed to read {NETWORK_MTLS_CERT_PATH} file at {}", certificate_path.display())
    })?;
    let private_key = fs::read(&private_key_path).with_context(|| {
        format!("Failed to read {NETWORK_MTLS_KEY_PATH} file at {}", private_key_path.display())
    })?;

    Ok(Some((certificate, private_key)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_identity_is_optional() {
        let identity = load_network_client_identity_from_paths(None, None).unwrap();

        assert!(identity.is_none());
    }

    #[test]
    fn client_identity_paths_must_be_set_together() {
        let error =
            load_network_client_identity_from_paths(Some(OsString::from("certificate.pem")), None)
                .unwrap_err();

        assert!(error.to_string().contains("must be set together"));
    }

    #[test]
    fn client_identity_paths_must_not_be_empty() {
        let error = load_network_client_identity_from_paths(
            Some(OsString::new()),
            Some(OsString::from("private-key.pem")),
        )
        .unwrap_err();

        assert!(error.to_string().contains("NETWORK_MTLS_CERT_PATH must not be empty"));
    }

    #[test]
    fn loads_client_identity_files() {
        let directory = tempfile::tempdir().unwrap();
        let certificate_path = directory.path().join("certificate.pem");
        let private_key_path = directory.path().join("private-key.pem");
        fs::write(&certificate_path, b"certificate").unwrap();
        fs::write(&private_key_path, b"private key").unwrap();

        let identity = load_network_client_identity_from_paths(
            Some(certificate_path.into_os_string()),
            Some(private_key_path.into_os_string()),
        )
        .unwrap()
        .unwrap();

        assert_eq!(identity.0, b"certificate");
        assert_eq!(identity.1, b"private key");
    }

    #[test]
    fn reports_unreadable_client_identity_file() {
        let directory = tempfile::tempdir().unwrap();
        let certificate_path = directory.path().join("missing-certificate.pem");
        let private_key_path = directory.path().join("private-key.pem");
        fs::write(&private_key_path, b"private key").unwrap();

        let error = load_network_client_identity_from_paths(
            Some(certificate_path.into_os_string()),
            Some(private_key_path.into_os_string()),
        )
        .unwrap_err();

        assert!(error.to_string().contains("Failed to read NETWORK_MTLS_CERT_PATH file"));
    }
}
