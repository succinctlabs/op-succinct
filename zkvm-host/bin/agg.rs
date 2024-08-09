use std::fs;

use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use client_utils::{RawBootInfo, BOOT_INFO_SIZE};
use host_utils::fetcher::{ChainMode, SP1KonaDataFetcher};
use sp1_sdk::{utils, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin};

pub const AGG_ELF: &[u8] = include_bytes!("../../elf/aggregation-client-elf");
pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/validity-client-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start L2 block number.
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    proofs: Vec<String>,

    #[arg(short, long)]
    multi: bool,

    /// Verbosity level.
    #[arg(short, long, default_value = "0")]
    verbosity: u8,
}

/// Load the aggregation proof data.
fn load_aggregation_proof_data(proof_names: Vec<String>) -> (Vec<SP1Proof>, Vec<RawBootInfo>) {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let proof_directory = format!("{}/data/proofs", workspace_root);

    let mut proofs = Vec::with_capacity(proof_names.len());
    let mut boot_infos = Vec::with_capacity(proof_names.len());

    for proof_name in proof_names.iter() {
        let proof_path = format!("{}/{}.bin", proof_directory, proof_name);
        if fs::metadata(&proof_path).is_err() {
            panic!("Proof file not found: {}", proof_path);
        }
        let mut deserialized_proof =
            SP1ProofWithPublicValues::load(proof_path).expect("loading proof failed");
        proofs.push(deserialized_proof.proof);

        // The public values are the ABI-encoded BootInfo.
        let mut boot_info_buf = [0u8; BOOT_INFO_SIZE];
        deserialized_proof
            .public_values
            .read_slice(&mut boot_info_buf);
        let boot_info = RawBootInfo::abi_decode(&boot_info_buf).unwrap();
        boot_infos.push(boot_info);
    }

    (proofs, boot_infos)
}

// Execute the Kona program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();

    let args = Args::parse();
    let fetcher = SP1KonaDataFetcher::new();
    let prover = ProverClient::new();

    let (proofs, boot_infos) = load_aggregation_proof_data(args.proofs);

    // Fetch the headers from the L1 head of the last block to the L1 head of the first block.
    let first_head = boot_infos.last().unwrap().l1_head;
    let last_head = boot_infos.first().unwrap().l1_head;

    // Confirm that the headers are in the correct order.
    let start_header = fetcher.get_header_by_hash(ChainMode::L1, last_head).await?;
    let end_header = fetcher
        .get_header_by_hash(ChainMode::L1, first_head)
        .await?;
    if start_header.number > end_header.number {
        panic!("Headers are not in the correct order");
    }
    let mut headers = Vec::with_capacity(
        (end_header.number.unwrap() - start_header.number.unwrap() + 1) as usize,
    );
    let mut curr_head = end_header;

    // Fetch the headers from the end header to the start header.
    while curr_head.number > start_header.number {
        headers.push(curr_head.clone());
        curr_head = fetcher
            .get_header_by_hash(ChainMode::L1, curr_head.parent_hash)
            .await?;
    }

    // Reverse the headers to put them in order from start to end.
    headers.reverse();

    let (_, vkey) = prover.setup(MULTI_BLOCK_ELF);

    let mut stdin = SP1Stdin::new();
    for proof in proofs {
        let SP1Proof::Compressed(compressed_proof) = proof else {
            panic!();
        };
        stdin.write_proof(compressed_proof, vkey.vk.clone());
    }
    stdin.write(&boot_infos);
    stdin.write(&headers);

    let (agg_pk, _) = prover.setup(AGG_ELF);

    // let (_, report) = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run().unwrap();
    prover
        .prove(&agg_pk, stdin)
        .plonk()
        .run()
        .expect("proving failed");

    Ok(())
}
