pub mod fetcher;
pub mod helpers;

use client_utils::RawBootInfo;
use kona_host::HostCli;
use sp1_sdk::SP1Stdin;

use alloy_sol_types::sol;

use rkyv::{
    ser::{
        serializers::{AlignedSerializer, CompositeSerializer, HeapScratch, SharedSerializeMap},
        Serializer,
    },
    AlignedVec,
};

use crate::helpers::load_kv_store;

sol! {
    struct L2Claim {
        uint64 num;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}

/// Execute the Kona program and return the execution report.
pub fn get_sp1_stdin(host_cli: &HostCli) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();

    let boot_info = RawBootInfo {
        l1_head: host_cli.l1_head,
        l2_output_root: host_cli.l2_output_root,
        l2_claim: host_cli.l2_claim,
        l2_claim_block: host_cli.l2_block_number,
        chain_id: host_cli.l2_chain_id,
    };
    stdin.write(&boot_info);

    // Read KV store into raw bytes and pass to stdin.
    let kv_store = load_kv_store(&format!("../data/{}", boot_info.l2_claim_block));

    let mut serializer = CompositeSerializer::new(
        AlignedSerializer::new(AlignedVec::new()),
        // TODO: This value is hardcoded to minimum for this block.
        // Figure out how to compute it so it works on all blocks.
        HeapScratch::<8388608>::new(),
        SharedSerializeMap::new(),
    );
    serializer.serialize_value(&kv_store).unwrap();

    let buffer = serializer.into_serializer().into_inner();
    let kv_store_bytes = buffer.into_vec();
    stdin.write_slice(&kv_store_bytes);

    stdin
}
