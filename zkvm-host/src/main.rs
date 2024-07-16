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

const CLIENT_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-client-elf");
const AGG_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-aggregator-elf");

// TODO: Can I just remove this?
#[derive(Debug, Clone, Archive, Serialize, Deserialize)]
#[archive_attr(derive(Debug))]
pub struct InMemoryOracle {
    cache: HashMap<[u8; 32], Vec<u8>, BytesHasherBuilder>,
}

fn main() {
    utils::setup_logger();

    // Initialize client & get proving and verifying keys.
    let client = ProverClient::new();
    let (client_pk, client_vk) = client.setup(CLIENT_ELF);
    let (agg_pk, agg_vk) = client.setup(AGG_ELF);

    // TODO: Get starting block and ending block from the CLI.
    let start_block = 12345678;
    let end_block = 12345679;

    let mut proofs = Vec::with_capacity(end_block - start_block + 1);
    let mut boot_infos: Vec<BootInfoWithoutRollupConfig> = Vec::with_capacity(end_block - start_block + 1);

    // TODO: Parallelize.
    for block in start_block..end_block {
        let mut stdin = SP1Stdin::new();

        // TODO: Generate BootInfoWithoutRollupConfig from block number (Ratan working on this).
        // l2_claim_block should equal block
        let boot_info = BootInfoWithoutRollupConfig {
            l2_claim_block: block,
            chain_id: 10,
            l2_claim: [0; 32],
            l2_output_root: [0; 32],
            l1_head: [0; 32],
        };
        stdin.write(&boot_info);
        boot_infos[block - start_block] = boot_info;

        // Read KV store into raw bytes and pass to stdin.
        let kv_store = load_kv_store(&format!("../data/{}", block));

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

        proofs[block - start_block] = client
            .prove_compressed(&client_pk, stdin)
            .expect("proving failed");
    }

    let mut stdin = SP1Stdin::new();
    stdin.write(&client_vk);
    stdin.write(&boot_infos);

    for proof in proofs {
        stdin.write_proof(proof, client_vk);
    }

    // Generate the plonk bn254 proof.
    let agg_proof = client
        .prove_plonk(&agg_pk, stdin)
        .expect("proving failed");
    println!("aggregated proof generated");

    agg_proof
        .save("proofs/agg_proof.bin")
        .expect("saving proof failed");

    let deserialized_proof = SP1Proof::load(format!("proofs/agg_proof.bin").expect("loading proof failed");
    client.verify(&deserialized_proof, &agg_vk).expect("verification failed");
    println!("aggregated proof verified");
}
