use hokulea_proof::eigenda_witness::EigenDAWitness;
use op_succinct_client_utils::witness::{
    preimage_store::PreimageStore, BlobData, EigenDAWitnessData,
};
use op_succinct_eigenda_host_utils::witness_generator::EigenDAWitnessGenerator;
use op_succinct_host_utils::witness_generation::WitnessGenerator;

fn default_witness() -> EigenDAWitnessData {
    EigenDAWitnessData {
        preimage_store: PreimageStore::default(),
        blob_data: BlobData::default(),
        eigenda_data: None,
    }
}

#[test]
fn test_get_sp1_stdin_with_no_eigenda_data() {
    let generator = EigenDAWitnessGenerator {};
    assert!(generator.get_sp1_stdin(default_witness()).is_ok());
}

#[test]
fn test_get_sp1_stdin_rejects_malformed_eigenda_data() {
    let generator = EigenDAWitnessGenerator {};
    let witness = EigenDAWitnessData {
        preimage_store: PreimageStore::default(),
        blob_data: BlobData::default(),
        eigenda_data: Some(vec![0xFF, 0xFF, 0xFF, 0xFF]), // Malformed data
    };

    let err = generator.get_sp1_stdin(witness).unwrap_err();
    assert!(err.to_string().contains("Failed to deserialize EigenDA blob witness data"));
}

/// Valid EigenDAWitness with no canoe proof (canoe_proof_bytes: None).
/// This is a realistic scenario for blocks without EigenDA certs requiring validity proofs.
#[test]
fn test_get_sp1_stdin_with_eigenda_data_but_no_canoe_proof() {
    let generator = EigenDAWitnessGenerator {};

    let eigenda_witness =
        EigenDAWitness { validities: vec![], encoded_payloads: vec![], canoe_proof_bytes: None };

    let eigenda_data = serde_cbor::to_vec(&eigenda_witness).expect("serialization should work");

    let witness = EigenDAWitnessData {
        preimage_store: PreimageStore::default(),
        blob_data: BlobData::default(),
        eigenda_data: Some(eigenda_data),
    };

    assert!(generator.get_sp1_stdin(witness).is_ok());
}

/// Adversarial case: Valid EigenDAWitness structure with invalid canoe_proof_bytes.
/// This bypasses the first deserialization but should fail at nested proof deserialization.
#[test]
fn test_get_sp1_stdin_rejects_invalid_canoe_proof_bytes() {
    let generator = EigenDAWitnessGenerator {};

    // Create a valid EigenDAWitness with garbage in canoe_proof_bytes
    let eigenda_witness = EigenDAWitness {
        validities: vec![],
        encoded_payloads: vec![],
        canoe_proof_bytes: Some(vec![0xFF, 0xFF, 0xFF, 0xFF]), // Invalid proof bytes
    };

    let eigenda_data = serde_cbor::to_vec(&eigenda_witness).expect("serialization should work");

    let witness = EigenDAWitnessData {
        preimage_store: PreimageStore::default(),
        blob_data: BlobData::default(),
        eigenda_data: Some(eigenda_data),
    };

    let err = generator.get_sp1_stdin(witness).unwrap_err();
    assert!(err.to_string().contains("Failed to deserialize canoe proof"));
}

/// Requires: L1_RPC, L1_BEACON_RPC, L2_RPC, L2_NODE_RPC, EIGENDA_PROXY_ADDRESS
#[cfg(feature = "integration")]
mod integration {
    use alloy_eips::BlockId;
    use alloy_primitives::{keccak256, Bytes, B256};
    use alloy_rpc_client::RpcClient;
    use anyhow::{Context, Result};
    use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_failing_raw_header_probe() -> Result<()> {
        dotenv::dotenv().ok();

        let rpc_url = std::env::var("L2_RPC").context("L2_RPC is not set")?;
        let client = RpcClient::new_http(rpc_url.parse().context("L2_RPC is not a valid URL")?);
        let block_number = "0x81aa7c";
        let block_hash: B256 =
            "0x01ec1aa2801be61f3e62032e6990cdffe6ca0df3e63ea5e736d85ce8a9da2113".parse()?;

        let fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
        match fetcher.get_l2_header(BlockId::number(8_497_788)).await {
            Ok(header) => println!(
                "eth_getBlockByNumber: hash={}, number={}, parentHash={}",
                header.hash_slow(),
                header.number,
                header.parent_hash
            ),
            Err(err) => println!("eth_getBlockByNumber: {err}"),
        }

        match fetcher.get_l2_header(BlockId::hash(block_hash)).await {
            Ok(header) => println!(
                "eth_getBlockByHash: hash={}, number={}, parentHash={}",
                header.hash_slow(),
                header.number,
                header.parent_hash
            ),
            Err(err) => println!("eth_getBlockByHash: {err}"),
        }

        match client.request::<_, Bytes>("debug_getRawHeader", [block_number]).await {
            Ok(raw) => {
                println!(
                    "debug_getRawHeader(number): bytes={}, keccak={}",
                    raw.len(),
                    keccak256(&raw)
                )
            }
            Err(err) => println!("debug_getRawHeader(number): {err}"),
        }

        match client.request::<_, Bytes>("debug_getRawHeader", [block_hash]).await {
            Ok(raw) => {
                println!(
                    "debug_getRawHeader(hash): bytes={}, keccak={}",
                    raw.len(),
                    keccak256(&raw)
                )
            }
            Err(err) => println!("debug_getRawHeader(hash): {err}"),
        }

        Ok(())
    }
}
