pub mod block_range;
pub mod fetcher;
pub mod helpers;
pub mod rollup_config;
pub mod stats;

use alloy_consensus::Header;
use alloy_primitives::{map::HashMap, B256};
use alloy_sol_types::sol;
use anyhow::anyhow;
use anyhow::Result;
use kona_host::HostOrchestrator;
use kona_host::PreimageServer;
use kona_host::{single::SingleChainHostCli, DiskKeyValueStore, MemoryKeyValueStore};
use kona_preimage::BidirectionalChannel;
use kona_preimage::HintReader;
use kona_preimage::HintWriter;
use kona_preimage::OracleReader;
use kona_preimage::OracleServer;
use log::info;
use maili_genesis::RollupConfig;
use op_succinct_client_utils::InMemoryOracle;
use op_succinct_client_utils::{
    boot::BootInfoStruct, types::AggregationInputs, BootInfoWithBytesConfig, BytesHasherBuilder,
};
use rkyv::to_bytes;
use sp1_sdk::{HashableKey, SP1Proof, SP1Stdin};
use std::{fs::File, io::Read};
use tokio::task;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract L2OutputOracle {
        bytes32 public aggregationVkey;
        bytes32 public rangeVkeyCommitment;
        bytes32 public rollupConfigHash;

        function updateAggregationVKey(bytes32 _aggregationVKey) external onlyOwner;

        function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) external onlyOwner;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProgramType {
    Single,
    Multi,
}

sol! {
    struct L2Output {
        uint64 zero;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}

/// Get the stdin to generate a proof for the given L2 claim.
pub fn get_proof_stdin(
    host_cli: &SingleChainHostCli,
    mem_kv_store: MemoryKeyValueStore,
) -> Result<SP1Stdin> {
    let mut stdin = SP1Stdin::new();

    // Read the rollup config.
    let mut rollup_config_file = File::open(host_cli.rollup_config_path.as_ref().unwrap())?;
    let mut rollup_config_bytes = Vec::new();
    rollup_config_file.read_to_end(&mut rollup_config_bytes)?;

    let ser_config = std::fs::read_to_string(host_cli.rollup_config_path.as_ref().unwrap())?;
    let rollup_config: RollupConfig = serde_json::from_str(&ser_config)?;

    let boot_info = BootInfoWithBytesConfig {
        l1_head: host_cli.l1_head,
        l2_output_root: host_cli.agreed_l2_output_root,
        l2_claim: host_cli.claimed_l2_output_root,
        l2_claim_block: host_cli.claimed_l2_block_number,
        chain_id: rollup_config.l2_chain_id,
        rollup_config_bytes,
    };
    stdin.write(&boot_info);

    // Convert the memory KV store to a HashMap<[u8;32], Vec<u8>>.
    let mut kv_store_map: HashMap<[u8; 32], Vec<u8>, BytesHasherBuilder> =
        HashMap::with_hasher(BytesHasherBuilder);
    for (k, v) in mem_kv_store.store {
        kv_store_map.insert(k.0, v);
    }

    let in_memory_oracle = InMemoryOracle {
        cache: kv_store_map,
    };

    // Serialize the underlying KV store.
    let buffer = to_bytes::<rkyv::rancor::Error>(&in_memory_oracle)?;

    let kv_store_bytes = buffer.into_vec();
    stdin.write_slice(&kv_store_bytes);

    Ok(stdin)
}

/// Get the stdin for the aggregation proof.
pub fn get_agg_proof_stdin(
    proofs: Vec<SP1Proof>,
    boot_infos: Vec<BootInfoStruct>,
    headers: Vec<Header>,
    multi_block_vkey: &sp1_sdk::SP1VerifyingKey,
    latest_checkpoint_head: B256,
) -> Result<SP1Stdin> {
    let mut stdin = SP1Stdin::new();
    for proof in proofs {
        let SP1Proof::Compressed(compressed_proof) = proof else {
            panic!();
        };
        stdin.write_proof(*compressed_proof, multi_block_vkey.vk.clone());
    }

    // Write the aggregation inputs to the stdin.
    stdin.write(&AggregationInputs {
        boot_infos,
        latest_l1_checkpoint_head: latest_checkpoint_head,
        multi_block_vkey: multi_block_vkey.hash_u32(),
    });
    // The headers have issues serializing with bincode, so use serde_json instead.
    let headers_bytes = serde_cbor::to_vec(&headers).unwrap();
    stdin.write_vec(headers_bytes);

    Ok(stdin)
}

/// TODO: Can we run many program tasks in parallel?
pub async fn start_server_and_native_client(
    cfg: &SingleChainHostCli,
) -> Result<MemoryKeyValueStore, anyhow::Error> {
    let hint_chan = BidirectionalChannel::new().map_err(|e| anyhow!(e))?;
    let preimage_chan = BidirectionalChannel::new().map_err(|e| anyhow!(e))?;

    let kv_store = cfg.create_key_value_store()?;
    let providers = cfg.create_providers().await?;
    let fetcher = cfg.create_fetcher(providers, kv_store.clone());

    let server_task = task::spawn(
        PreimageServer::new(
            OracleServer::new(preimage_chan.host),
            HintReader::new(hint_chan.host),
            kv_store.clone(),
            fetcher,
        )
        .start(),
    );

    let program_task = tokio::spawn(SingleChainHostCli::run_client_native(
        HintWriter::new(hint_chan.client),
        OracleReader::new(preimage_chan.client),
    ));

    info!("Starting preimage server and client program.");
    let (_, client_result) =
        tokio::try_join!(server_task, program_task,).map_err(|e| anyhow!(e))?;
    if let Err(e) = client_result {
        return Err(e);
    }
    info!(target: "kona_host", "Preimage server and client program have joined.");

    // Drop the KV store after the server is complete, to avoid conflicting locks.
    drop(kv_store);

    // Loop over all of the keys in the KV store and convert to MemoryKeyValueStore.
    let disk_kv_store = DiskKeyValueStore::new(cfg.data_dir.clone().unwrap());

    let mem_kv_store: MemoryKeyValueStore = disk_kv_store.try_into()?;

    // Convert to MemoryKeyValueStore and return it
    Ok(mem_kv_store)
}
