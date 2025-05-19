use alloy_consensus::{TxEnvelope, TypedTransaction};
use alloy_eips::Decodable2718;
use alloy_network::{EthereumWallet, TransactionBuilder, TransactionBuilder4844};
use alloy_primitives::{Address, Bytes};
use alloy_provider::{Network, PendingTransactionBuilder, Provider, ProviderBuilder, Web3Signer};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug)]
/// The type of signer to use for the proposer.
pub enum ProposerSigner {
    /// The signer URL and address.
    Web3Signer(Url, Address),
    /// The local signer.
    LocalSigner(PrivateKeySigner),
}

impl ProposerSigner {
    pub fn address(&self) -> Address {
        match self {
            ProposerSigner::Web3Signer(_, address) => *address,
            ProposerSigner::LocalSigner(signer) => signer.address(),
        }
    }
}

/// Sign a transaction request using the configured `proposer_signer`.
pub async fn sign_transaction_request_inner<N>(
    proposer_signer: ProposerSigner,
    l1_rpc: Url,
    mut transaction_request: N::TransactionRequest,
) -> Result<PendingTransactionBuilder<N>>
where
    N: Network<UnsignedTx = TypedTransaction, TxEnvelope = TxEnvelope>,
    N::TransactionRequest: TransactionBuilder4844,
{
    match proposer_signer {
        ProposerSigner::Web3Signer(signer_url, signer_address) => {
            // Set the from address to the signer address.
            transaction_request.set_from(signer_address);

            // Fill the transaction request with all of the relevant gas and nonce information.
            let provider = ProviderBuilder::new().network::<N>().connect_http(l1_rpc);
            let filled_tx = provider.fill(transaction_request).await?;

            // Sign the transaction request using the Web3Signer.
            let web3_provider = ProviderBuilder::new().network::<N>().connect_http(signer_url);
            let signer = Web3Signer::new(web3_provider.clone(), signer_address);

            let tx = filled_tx.as_builder().unwrap().clone();

            // NOTE: This is a hack because there is not a "data" field on the TransactionRequest.
            // `eth_signTransaction` expects a "data" field with the calldata.
            // TODO: Once alloy fixes this, we can remove this wrapper.
            let wrapper = TransactionRequestWrapper::<N>::new(tx);

            let raw: Bytes =
                signer.provider().client().request("eth_signTransaction", (wrapper,)).await?;

            let tx_envelope = N::TxEnvelope::decode_2718(&mut raw.as_ref()).unwrap();

            Ok(provider.send_tx_envelope(tx_envelope).await?)
        }
        ProposerSigner::LocalSigner(private_key) => {
            let provider = ProviderBuilder::new()
                .network::<N>()
                .wallet(EthereumWallet::new(private_key.clone()))
                .connect_http(l1_rpc);

            // Set the from address to the Ethereum wallet address.
            transaction_request.set_from(private_key.address());

            // Fill the transaction request with all of the relevant gas and nonce information.
            let filled_tx = provider.fill(transaction_request).await?;

            Ok(provider.send_tx_envelope(filled_tx.as_envelope().unwrap().clone()).await?)
        }
    }
}

/// A wrapper around `TransactionRequest` that adds a data field as bytes.
///
/// This is needed because:
/// 1. The `TransactionRequest` trait and `TransactionBuilder` trait don't include methods to set
///    the "data" field directly (only `input()` and `set_input()` are available).
/// 2. Web3Signer's `eth_signTransaction` method specifically expects a "data" field in the JSON-RPC
///    request, not the "input" field.
/// 3. While the alloy-rpc-types-eth crate has methods like `normalize_data()` and `set_both()` to
///    work with both fields, these are only implemented on the specific Ethereum transaction type
///    struct and aren't accessible through the generic `N::TransactionRequest` interface we're
///    using here.
///
/// TODO(fakedev9999): Once alloy fixes this, we can remove this wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequestWrapper<N: Network> {
    /// The underlying transaction request
    #[serde(flatten)]
    pub tx: N::TransactionRequest,
    /// The transaction data as bytes
    pub data: Option<Bytes>,
}

impl<N: Network> TransactionRequestWrapper<N> {
    /// Create a new wrapper around a transaction request
    pub fn new(tx: N::TransactionRequest) -> Self {
        let data = tx.input().cloned();
        Self { tx, data }
    }
}

#[cfg(test)]
mod tests {
    use alloy_eips::BlockId;
    use alloy_network::Ethereum;
    use alloy_primitives::address;
    use op_succinct_host_utils::OPSuccinctL2OutputOracle::OPSuccinctL2OOContractInstance as OPSuccinctL2OOContract;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_sign_transaction_request() {
        let proposer_signer = ProposerSigner::Web3Signer(
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

        let signed_tx = sign_transaction_request_inner::<Ethereum>(
            proposer_signer,
            "http://localhost:8545".parse().unwrap(),
            transaction_request,
        )
        .await
        .unwrap();

        println!("Signed transaction receipt: {:?}", signed_tx.get_receipt().await.unwrap());
    }
}
