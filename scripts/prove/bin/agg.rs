use alloy_primitives::{hex, B256};
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;
use op_succinct_client_utils::{boot::BootInfoStruct, types::u32_to_u8};
use op_succinct_host_utils::{
    fetcher::{OPSuccinctDataFetcher, RunContext},
    get_agg_proof_stdin,
};
use op_succinct_prove::{AGG_ELF, RANGE_ELF};
use sp1_sdk::{
    utils, HashableKey, Prover, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1VerifyingKey,
};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start L2 block number.
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    proofs: Vec<String>,

    /// Prove flag.
    #[arg(short, long)]
    prove: bool,

    /// Env file path.
    #[arg(default_value = ".env", short, long)]
    env_file: String,
}

/// Load the aggregation proof data.
fn load_aggregation_proof_data(
    proof_names: Vec<String>,
    range_vkey: &SP1VerifyingKey,
) -> (Vec<SP1Proof>, Vec<BootInfoStruct>) {
    // let metadata = MetadataCommand::new().exec().unwrap();
    // let workspace_root = metadata.workspace_root;
    // let proof_directory = format!("{}/data/fetched_proofs", workspace_root);

    // let mut proofs = Vec::with_capacity(proof_names.len());
    // let mut boot_infos = Vec::with_capacity(proof_names.len());

    // let prover = ProverClient::builder().cpu().build();

    // for proof_name in proof_names.iter() {
    //     let proof_path = format!("{}/{}.bin", proof_directory, proof_name);
    //     if fs::metadata(&proof_path).is_err() {
    //         panic!("Proof file not found: {}", proof_path);
    //     }
    //     let mut deserialized_proof =
    //         SP1ProofWithPublicValues::load(proof_path).expect("loading proof failed");
    //     prover
    //         .verify(&deserialized_proof, range_vkey)
    //         .expect("proof verification failed");
    //     proofs.push(deserialized_proof.proof);

    //     // The public values are the BootInfoStruct.
    //     let boot_info = deserialized_proof.public_values.read();
    //     boot_infos.push(boot_info);
    // }

    let serialized_proof = hex!("01000000000000000000000000000000000000000000000000000000000000000000000000000000134243013037620580485009248768018069090267698507765387011d32cc34e4a25e43977ed82150242403a09a480ebbb9611a759fa36d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a80000000000000020000000000000008a32956e2f67c7e68893b6e2b1538442d9f048e5e889858c79a959fa6419917c2000000000000000672155f4db0fbbbf23d718dbf939344bd39cc059e7703081cfb0b08af49599af200000000000000056f948f1896d0a6af455a6efd1f2b2d9b4bf2f49bef68a218a65d40f9cff4b86c43d060000000000200000000000000071241d0f92749d7365aaaf6a015de550816632a4e4e84e273f865f582e8190aa0b0000000000000076342e302e302d72632e33");
    let mut proof: SP1ProofWithPublicValues = bincode::deserialize(&serialized_proof).unwrap();
    let boot_info = BootInfoStruct::from(proof.public_values.read::<BootInfoStruct>());

    (vec![proof.proof], vec![boot_info])
}

// Execute the OP Succinct program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();

    let args = Args::parse();

    dotenv::from_filename(args.env_file).ok();

    let prover = ProverClient::from_env();
    let fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;

    let (_, vkey) = prover.setup(RANGE_ELF);

    let (proofs, boot_infos) = load_aggregation_proof_data(args.proofs, &vkey);

    let header = fetcher.get_latest_l1_head_in_batch(&boot_infos).await?;
    let headers = fetcher
        .get_header_preimages(&boot_infos, header.hash_slow())
        .await?;
    let multi_block_vkey_u8 = u32_to_u8(vkey.vk.hash_u32());
    let multi_block_vkey_b256 = B256::from(multi_block_vkey_u8);
    println!(
        "Range ELF Verification Key Commitment: {}",
        multi_block_vkey_b256
    );
    let stdin =
        get_agg_proof_stdin(proofs, boot_infos, headers, &vkey, header.hash_slow()).unwrap();

    let (agg_pk, agg_vk) = prover.setup(AGG_ELF);
    println!("Aggregate ELF Verification Key: {:?}", agg_vk.vk.bytes32());

    if args.prove {
        prover
            .prove(&agg_pk, &stdin)
            .groth16()
            .run()
            .expect("proving failed");
    } else {
        let (_, report) = prover.execute(AGG_ELF, &stdin).run().unwrap();
        println!("report: {:?}", report);
    }

    Ok(())
}
