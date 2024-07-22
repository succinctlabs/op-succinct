pub mod helpers;

use alloy_primitives::{keccak256, B256};
use client_utils::{BytesHasherBuilder, RawBootInfo};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, H160, U256},
};
use kona_host::HostCli;
use sp1_core::runtime::ExecutionReport;
use sp1_sdk::{ProverClient, SP1Stdin};

use clap::Parser;
use std::{collections::HashMap, str::FromStr};

use alloy_sol_types::{sol, SolValue};
use anyhow::Result;
use std::{env, fs};

use rkyv::{
    ser::{
        serializers::{AlignedSerializer, CompositeSerializer, HeapScratch, SharedSerializeMap},
        Serializer,
    },
    AlignedVec, Archive, Deserialize, Serialize,
};

use crate::helpers::load_kv_store;

sol! {
    struct L2Claim {
        uint64 num;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }

    struct L2Output {
        uint64 num;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_head;
    }
}

/// Execute the Kona program and return the execution report.
pub fn get_sp1_std(host_cli: &HostCli) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();

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

    // First instantiate a mock prover client to just execute the program and get the estimation of
    // cycle count.
    let client = ProverClient::mock();

    let (mut _public_values, report) = client.execute(KONA_ELF, stdin).unwrap();
    report
}
