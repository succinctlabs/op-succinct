mod cli;
pub mod fetcher;
pub mod helpers;
pub use cli::{CostEstimatorCliArgs, SP1KonaCliArgs};

use cargo_metadata::MetadataCommand;
use sp1_core::runtime::ExecutionReport;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use zkvm_common::BootInfoWithoutRollupConfig;

use rkyv::{
    ser::{
        serializers::{AlignedSerializer, CompositeSerializer, HeapScratch, SharedSerializeMap},
        Serializer,
    },
    AlignedVec,
};

use crate::helpers::load_kv_store;

pub const KONA_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

fn get_kona_program_input(boot_info: &BootInfoWithoutRollupConfig) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();

    stdin.write(&boot_info);

    // Read KV store into raw bytes and pass to stdin.
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let data_directory = format!("{}/data/{}", workspace_root, boot_info.l2_claim_block);

    let kv_store = load_kv_store(&data_directory.into());

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

/// Execute the Kona program and return the execution report.
pub fn execute_kona_program(boot_info: &BootInfoWithoutRollupConfig) -> ExecutionReport {
    // First instantiate a mock prover client to just execute the program and get the estimation of
    // cycle count.
    let client = ProverClient::mock();

    let stdin = get_kona_program_input(boot_info);

    let (mut _public_values, report) = client.execute(KONA_ELF, stdin).run().unwrap();
    report
}

/// Execute the Kona program and return the execution report.
pub fn prove_kona_program(boot_info: &BootInfoWithoutRollupConfig) -> SP1ProofWithPublicValues {
    // First instantiate a mock prover client to just execute the program and get the estimation of
    // cycle count.
    let client = ProverClient::new();
    let (pk, _) = client.setup(KONA_ELF);

    let stdin = get_kona_program_input(boot_info);

    let proof = client.prove(&pk, stdin).plonk().run().unwrap();
    proof
}
