pub mod fetcher;
pub mod helpers;

use alloy_primitives::B256;
use fetcher::NativeExecutionBlockData;
use sp1_core::runtime::ExecutionReport;
use sp1_sdk::{PlonkBn254Proof, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use zkvm_common::BootInfoWithoutRollupConfig;

use clap::Parser;
use std::str::FromStr;

use rkyv::{
    ser::{
        serializers::{AlignedSerializer, CompositeSerializer, HeapScratch, SharedSerializeMap},
        Serializer,
    },
    AlignedVec,
};

use crate::helpers::load_kv_store;

pub const KONA_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SP1KonaCliArgs {
    #[arg(long)]
    l1_head: String,

    #[arg(long)]
    l2_output_root: String,

    #[arg(long)]
    l2_claim: String,

    #[arg(long)]
    l2_claim_block: u64,

    #[arg(long)]
    chain_id: u64,
}

impl From<NativeExecutionBlockData> for BootInfoWithoutRollupConfig {
    fn from(block_data: NativeExecutionBlockData) -> Self {
        BootInfoWithoutRollupConfig {
            l1_head: block_data.l1_head,
            l2_output_root: block_data.l2_output_root,
            l2_claim: block_data.l2_claim,
            l2_claim_block: block_data.l2_block_number,
            chain_id: block_data.l2_chain_id,
        }
    }
}

impl From<SP1KonaCliArgs> for BootInfoWithoutRollupConfig {
    fn from(args: SP1KonaCliArgs) -> Self {
        BootInfoWithoutRollupConfig {
            l1_head: B256::from_str(&args.l1_head).unwrap(),
            l2_output_root: B256::from_str(&args.l2_output_root).unwrap(),
            l2_claim: B256::from_str(&args.l2_claim).unwrap(),
            l2_claim_block: args.l2_claim_block,
            chain_id: args.chain_id,
        }
    }
}

fn get_kona_program_input(boot_info: &BootInfoWithoutRollupConfig) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();

    stdin.write(&boot_info);

    // Read KV store into raw bytes and pass to stdin.
    let kv_store = load_kv_store(&format!("../data/{}", boot_info.l2_claim_block).into());

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

    let (mut _public_values, report) = client.execute(KONA_ELF, stdin).unwrap();
    report
}

/// Execute the Kona program and return the execution report.
pub fn prove_kona_program(
    boot_info: &BootInfoWithoutRollupConfig,
) -> SP1ProofWithPublicValues<PlonkBn254Proof> {
    // First instantiate a mock prover client to just execute the program and get the estimation of
    // cycle count.
    let client = ProverClient::new();
    let (pk, _) = client.setup(KONA_ELF);

    let stdin = get_kona_program_input(boot_info);

    let proof = client.prove_plonk(&pk, stdin).unwrap();
    proof
}
