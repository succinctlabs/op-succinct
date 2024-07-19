mod file_helpers;
use file_helpers::load_kv_store;

mod cli;
pub use cli::{ZkVmHostCliArgs, CostEstimatorCliArgs};

use sp1_core::runtime::ExecutionReport;
use sp1_sdk::{ProverClient, SP1Stdin};
use zkvm_common::BootInfoWithoutRollupConfig;

use rkyv::{
    ser::{
        serializers::{AlignedSerializer, CompositeSerializer, HeapScratch, SharedSerializeMap},
        Serializer,
    },
    AlignedVec
};

pub const CLIENT_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

/// Execute the Kona program and return the execution report.
pub fn execute_kona_program(boot_info: &BootInfoWithoutRollupConfig) -> ExecutionReport {
    let mut stdin = SP1Stdin::new();
    load_inputs(boot_info, &mut stdin);

    // Use mock prover client to just execute the program and get the estimation of the cost.
    let client = ProverClient::mock();
    let (mut _public_values, report) = client.execute(CLIENT_ELF, stdin).unwrap();

    report
}

pub fn prove_kona_program(boot_info: &BootInfoWithoutRollupConfig) {
    let mut stdin = SP1Stdin::new();
    load_inputs(boot_info, &mut stdin);

    let client = ProverClient::new();
    let (pk, _vk) = client.setup(CLIENT_ELF);

    let mut _proof = client.prove(&pk, stdin).unwrap();
    println!("generated zk proof");

    // save proof, verify, etc.
}

fn load_inputs(boot_info: &BootInfoWithoutRollupConfig, stdin: &mut SP1Stdin) {
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
}
