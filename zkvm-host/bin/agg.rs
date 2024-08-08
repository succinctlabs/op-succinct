use std::fs;

use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use client_utils::RawBootInfo;
use sp1_sdk::{utils, HashableKey, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin};

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

/// Execute the Kona program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();

    let args = Args::parse();
    let prover = ProverClient::new();

    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let proof_directory = format!("{}/data/proofs", workspace_root);

    let mut proofs = Vec::with_capacity(args.proofs.len());
    let mut boot_infos = Vec::with_capacity(args.proofs.len());

    for proof_name in args.proofs.iter() {
        let proof_path = format!("{}/{}.bin", proof_directory, proof_name);
        if fs::metadata(&proof_path).is_err() {
            panic!("Proof file not found: {}", proof_path);
        }
        let mut deserialized_proof =
            SP1ProofWithPublicValues::load(proof_path).expect("loading proof failed");

        proofs.push(deserialized_proof.proof);
        boot_infos.push(deserialized_proof.public_values.read::<RawBootInfo>());
    }

    let (_, vkey) = prover.setup(MULTI_BLOCK_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&vkey.hash_u32());
    for proof in proofs {
        let SP1Proof::Compressed(compressed_proof) = proof else {
            panic!();
        };
        stdin.write_proof(compressed_proof, vkey.vk.clone());
    }
    stdin.write(&boot_infos);

    let (agg_pk, _) = prover.setup(AGG_ELF);

    // let (_, report) = prover.execute(MULTI_BLOCK_ELF, sp1_stdin).run().unwrap();
    prover
        .prove(&agg_pk, stdin)
        .plonk()
        .run()
        .expect("proving failed");

    Ok(())
}
