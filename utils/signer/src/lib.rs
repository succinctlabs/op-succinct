use std::str::FromStr;

use alloy_consensus::TxEnvelope;
use alloy_eips::Decodable2718;
use alloy_network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy_primitives::{Address, Bytes};
use alloy_provider::{Provider, ProviderBuilder, Web3Signer};
use alloy_rpc_types_eth::{TransactionReceipt, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_transport_http::reqwest::Url;
use anyhow::{Context, Result};
use tokio::time::Duration;

// AWS imports for KMS signing
use alloy_signer_aws::AwsSigner;
use aws_sdk_kms::Client as KmsClient;

pub const NUM_CONFIRMATIONS: u64 = 3;
pub const TIMEOUT_SECONDS: u64 = 60;

#[derive(Clone, Debug)]
/// The type of signer to use for signing transactions.
pub enum Signer {
    /// The signer URL and address.
    Web3Signer(Url, Address),
    /// The local signer.
    LocalSigner(PrivateKeySigner),
    /// AWS KMS signer with the KMS key ID and corresponding address.
    AWSSigner { kms_key_id: String, address: Address },
}

impl Signer {
    pub fn address(&self) -> Address {
        match self {
            Signer::Web3Signer(_, address) => *address,
            Signer::LocalSigner(signer) => signer.address(),
            Signer::AWSSigner { address, .. } => *address,
        }
    }

    pub fn from_env() -> Result<Self> {
        // Check for all possible signing mechanisms
        let has_aws_kms = std::env::var("AWS_KMS_KEY_ID").is_ok();
        let has_web3signer = std::env::var("WEB3SIGNER_URL").is_ok();
        let has_private_key = std::env::var("PRIVATE_KEY").is_ok();

        // Count how many mechanisms are configured
        let mechanism_count = has_aws_kms as u8 + has_web3signer as u8 + has_private_key as u8;

        // Warn if multiple mechanisms are configured
        if mechanism_count > 1 {
            let mut mechanisms = Vec::new();
            if has_aws_kms {
                mechanisms.push("AWS_KMS_KEY_ID");
            }
            if has_web3signer {
                mechanisms.push("WEB3SIGNER_URL");
            }
            if has_private_key {
                mechanisms.push("PRIVATE_KEY");
            }

            tracing::warn!(
                "Multiple signing mechanisms detected: {}. Using the first one found in order: AWS KMS, Web3Signer, Private Key",
                mechanisms.join(", ")
            );
        }

        // Check for AWS KMS signer configuration first
        // AWS_KMS_KEY_ID: The ARN or ID of the KMS key to use for signing
        if has_aws_kms {
            // For AWS signer, we need to fetch the address from KMS asynchronously
            // Since this is a sync function, we'll require the address to be provided
            let signer_address_str = std::env::var("SIGNER_ADDRESS")
                .context("SIGNER_ADDRESS must be set when using AWS_KMS_KEY_ID")?;
            let signer_address =
                Address::from_str(&signer_address_str).context("Failed to parse SIGNER_ADDRESS")?;
            Ok(Signer::AWSSigner {
                kms_key_id: std::env::var("AWS_KMS_KEY_ID").unwrap(),
                address: signer_address,
            })
        } else if has_web3signer {
            let signer_url_str = std::env::var("WEB3SIGNER_URL").unwrap();
            let signer_address_str = std::env::var("SIGNER_ADDRESS")
                .context("SIGNER_ADDRESS must be set when using WEB3SIGNER_URL")?;

            let signer_url =
                Url::parse(&signer_url_str).context("Failed to parse WEB3SIGNER_URL")?;
            let signer_address =
                Address::from_str(&signer_address_str).context("Failed to parse SIGNER_ADDRESS")?;
            Ok(Signer::Web3Signer(signer_url, signer_address))
        } else if has_private_key {
            let private_key_str = std::env::var("PRIVATE_KEY").unwrap();
            let private_key = PrivateKeySigner::from_str(&private_key_str)
                .context("Failed to parse PRIVATE_KEY")?;
            Ok(Signer::LocalSigner(private_key))
        } else {
            anyhow::bail!("Set exactly one signing method.")
        }
    }

    /// Sends a transaction request, signed by the configured `signer`.
    pub async fn send_transaction_request(
        &self,
        l1_rpc: Url,
        mut transaction_request: TransactionRequest,
    ) -> Result<TransactionReceipt> {
        match self {
            Signer::Web3Signer(signer_url, signer_address) => {
                // Set the from address to the signer address.
                transaction_request.set_from(*signer_address);

                // Fill the transaction request with all of the relevant gas and nonce information.
                let provider = ProviderBuilder::new().network::<Ethereum>().connect_http(l1_rpc);
                let filled_tx = provider.fill(transaction_request).await?;

                // Sign the transaction request using the Web3Signer.
                let web3_provider =
                    ProviderBuilder::new().network::<Ethereum>().connect_http(signer_url.clone());
                let signer = Web3Signer::new(web3_provider.clone(), *signer_address);

                let mut tx = filled_tx.as_builder().unwrap().clone();
                tx.normalize_data();

                let raw: Bytes =
                    signer.provider().client().request("eth_signTransaction", (tx,)).await?;

                let tx_envelope = TxEnvelope::decode_2718(&mut raw.as_ref()).unwrap();

                let receipt = provider
                    .send_tx_envelope(tx_envelope)
                    .await
                    .context("Failed to send transaction")?
                    .with_required_confirmations(NUM_CONFIRMATIONS)
                    .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
                    .get_receipt()
                    .await?;

                Ok(receipt)
            }
            Signer::LocalSigner(private_key) => {
                let provider = ProviderBuilder::new()
                    .network::<Ethereum>()
                    .wallet(EthereumWallet::new(private_key.clone()))
                    .connect_http(l1_rpc);

                // Set the from address to the Ethereum wallet address.
                transaction_request.set_from(private_key.address());

                // Fill the transaction request with all of the relevant gas and nonce information.
                let filled_tx = provider.fill(transaction_request).await?;

                let receipt = provider
                    .send_tx_envelope(filled_tx.as_envelope().unwrap().clone())
                    .await
                    .context("Failed to send transaction")?
                    .with_required_confirmations(NUM_CONFIRMATIONS)
                    .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
                    .get_receipt()
                    .await?;

                Ok(receipt)
            }
            Signer::AWSSigner { kms_key_id, address } => {
                // Initialize AWS configuration and KMS client
                // This will automatically load credentials from environment variables, IAM roles,
                // or AWS config files
                tracing::info!("Loading AWS configuration for KMS key: {}", kms_key_id);

                // Check if AWS credentials are available in environment
                if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
                    tracing::warn!("AWS_ACCESS_KEY_ID not found in environment variables");
                }
                if std::env::var("AWS_SECRET_ACCESS_KEY").is_err() {
                    tracing::warn!("AWS_SECRET_ACCESS_KEY not found in environment variables");
                }
                if std::env::var("AWS_DEFAULT_REGION").is_err() {
                    tracing::warn!("AWS_DEFAULT_REGION not found in environment variables");
                }

                let aws_config = aws_config::load_from_env().await;
                tracing::info!("AWS config loaded, region: {:?}", aws_config.region());
                let kms_client = KmsClient::new(&aws_config);

                // Create the AWS signer with the KMS key
                let aws_signer = AwsSigner::new(kms_client, kms_key_id.clone(), None)
                    .await
                    .context("Failed to create AWS signer")?;

                // Verify the address matches
                let aws_address = alloy_signer::Signer::address(&aws_signer);
                if aws_address != *address {
                    anyhow::bail!(
                        "AWS KMS key address {} does not match expected address {}",
                        aws_address,
                        address
                    );
                }

                // Create provider with AWS signer
                let provider = ProviderBuilder::new()
                    .network::<Ethereum>()
                    .wallet(EthereumWallet::new(aws_signer))
                    .connect_http(l1_rpc);

                // Set the from address to the AWS signer address
                transaction_request.set_from(*address);

                // Fill the transaction request with all of the relevant gas and nonce information
                let filled_tx = provider.fill(transaction_request).await?;

                let receipt = provider
                    .send_tx_envelope(filled_tx.as_envelope().unwrap().clone())
                    .await
                    .context("Failed to send transaction")?
                    .with_required_confirmations(NUM_CONFIRMATIONS)
                    .with_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS)))
                    .get_receipt()
                    .await?;

                Ok(receipt)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy_eips::BlockId;
    use alloy_primitives::{address, U256};
    use op_succinct_host_utils::OPSuccinctL2OutputOracle::OPSuccinctL2OutputOracleInstance as OPSuccinctL2OOContract;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_send_transaction_request() {
        let proposer_signer = Signer::Web3Signer(
            "http://localhost:9000".parse().unwrap(),
            "0x9b3F173823E944d183D532ed236Ee3B83Ef15E1d".parse().unwrap(),
        );

        let provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http("http://localhost:8545".parse().unwrap());

        let l2oo_contract = OPSuccinctL2OOContract::new(
            address!("0xDafA1019F21AB8B27b319B1085f93673F02A69B7"),
            provider.clone(),
        );

        let latest_header = provider.get_block(BlockId::latest()).await.unwrap().unwrap();

        let transaction_request = l2oo_contract
            .checkpointBlockHash(U256::from(latest_header.header.number))
            .into_transaction_request();

        let receipt = proposer_signer
            .send_transaction_request("http://localhost:8545".parse().unwrap(), transaction_request)
            .await
            .unwrap();

        println!("Signed transaction receipt: {receipt:?}");
    }
}
