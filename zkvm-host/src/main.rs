// A host program to generate a proof of an Optimism L2 block STF in the zkVM.

mod helpers;
use helpers::load_kv_store;

mod cli;
use cli::SP1KonaCliArgs;

use zkvm_common::{BootInfoWithoutRollupConfig, BytesHasherBuilder};

use sp1_sdk::{utils, ProverClient, SP1Stdin, SP1Proof};
use clap::Parser;
use std::{collections::HashMap, thread, sync::Arc};
use rkyv::{
    ser::{serializers::*, Serializer},
    AlignedVec, Archive, Deserialize, Serialize,
};

const ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

#[derive(Debug, Clone, Archive, Serialize, Deserialize)]
#[archive_attr(derive(Debug))]
pub struct InMemoryOracle {
    cache: HashMap<[u8; 32], Vec<u8>, BytesHasherBuilder>,
}

fn main() {
    utils::setup_logger();
    let mut stdin = SP1Stdin::new();

    let cli_args = SP1KonaCliArgs::parse();
    let boot_info = BootInfoWithoutRollupConfig::from(cli_args);

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

    // Mock proof for testing and cycle counts
    // let client = ProverClient::mock();
    // let (mut _public_values, report) = client.execute(ELF, stdin).unwrap();
    // println!("Report: {}", report);

    // Real proof.
    // let client = ProverClient::new();
    // let (pk, vk) = client.setup(ELF);
    // let proof = client.prove(&pk, stdin).unwrap();

    // proof
    //     .save(format!("proofs/{}.bin", boot_info.l2_claim_block))
    //     .expect("saving proof failed");

    // let deserialized_proof = SP1Proof::load(format!("proofs/{}.bin", boot_info.l2_claim_block)).expect("loading proof failed");

    // println!("generated zk proof");
    // client.verify(&deserialized_proof, &vk).expect("verification failed");

    // println!("verified");
}
