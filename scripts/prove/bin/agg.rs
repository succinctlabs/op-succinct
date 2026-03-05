use alloy_primitives::{Address, B256};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use clap::Parser;
use op_succinct_client_utils::{boot::BootInfoStruct, types::u32_to_u8};
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher,
    get_agg_proof_stdin,
    network::{build_network_prover_from_env, parse_fulfillment_strategy},
};
use op_succinct_proof_utils::get_range_elf_embedded;
use sp1_sdk::{
    utils, Elf, HashableKey, ProveRequest, Prover, ProvingKey, SP1Proof, SP1ProofMode,
    SP1ProofWithPublicValues, SP1VerifyingKey,
};
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Proof file names to aggregate.
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    proofs: Vec<String>,

    /// Prove flag.
    #[arg(short, long)]
    prove: bool,

    /// Prover address.
    #[arg(short, long)]
    prover: Address,

    /// Env file path.
    #[arg(default_value = ".env", short, long)]
    env_file: String,
}

/// Load the aggregation proof data.
fn load_aggregation_proof_data(
    proof_names: &[String],
    range_vkey: &SP1VerifyingKey,
    prover: &impl Prover,
) -> (Vec<SP1Proof>, Vec<BootInfoStruct>) {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let proof_directory = format!("{workspace_root}/data/fetched_proofs");

    let mut proofs = Vec::with_capacity(proof_names.len());
    let mut boot_infos = Vec::with_capacity(proof_names.len());

    for proof_name in proof_names.iter() {
        let proof_path = format!("{proof_directory}/{proof_name}.bin");
        if fs::metadata(&proof_path).is_err() {
            panic!("Proof file not found: {proof_path}");
        }
        let mut deserialized_proof =
            SP1ProofWithPublicValues::load(proof_path).expect("loading proof failed");
        // None = infer proof mode from the proof itself.
        prover.verify(&deserialized_proof, range_vkey, None).expect("proof verification failed");
        proofs.push(deserialized_proof.proof);

        // The public values are the BootInfoStruct.
        let boot_info = deserialized_proof.public_values.read();
        boot_infos.push(boot_info);
    }

    (proofs, boot_infos)
}

/// Aggregates multiple compressed range proofs into a single proof.
#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();

    let args = Args::parse();

    dotenv::from_filename(args.env_file).ok();

    let agg_proof_strategy = parse_fulfillment_strategy(
        env::var("AGG_PROOF_STRATEGY").unwrap_or_else(|_| "reserved".to_string()),
    )?;
    let prover = build_network_prover_from_env(agg_proof_strategy).await?;
    let fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let range_pk = prover.setup(Elf::Static(get_range_elf_embedded())).await?;
    let vkey = range_pk.verifying_key().clone();

    let proof_names = args.proofs;
    let (proofs, boot_infos) = load_aggregation_proof_data(&proof_names, &vkey, &prover);

    let header = fetcher.get_latest_l1_head_in_batch(&boot_infos).await?;
    let headers = fetcher.get_header_preimages(&boot_infos, header.hash_slow()).await?;
    let multi_block_vkey_u8 = u32_to_u8(vkey.hash_u32());
    let multi_block_vkey_b256 = B256::from(multi_block_vkey_u8);
    println!("Range ELF Verification Key Commitment: {multi_block_vkey_b256}");
    let stdin =
        get_agg_proof_stdin(proofs, boot_infos, headers, &vkey, header.hash_slow(), args.prover)
            .expect("Failed to get agg proof stdin");

    let agg_pk = prover.setup(Elf::Static(AGGREGATION_ELF)).await?;
    let agg_vk = agg_pk.verifying_key();
    println!("Aggregate ELF Verification Key: {:?}", agg_vk.bytes32());

    if args.prove {
        let agg_proof_mode = match env::var("AGG_PROOF_MODE")
            .unwrap_or_else(|_| "plonk".to_string())
            .to_lowercase()
            .as_str()
        {
            "groth16" => SP1ProofMode::Groth16,
            _ => SP1ProofMode::Plonk,
        };
        let proof = prover
            .prove(&agg_pk, stdin)
            .mode(agg_proof_mode)
            .strategy(agg_proof_strategy)
            .await
            .expect("proving failed");

        // Save the aggregation proof to disk.
        let chain_id = fetcher.get_l2_chain_id().await?;
        let proof_dir = format!("data/{chain_id}/proofs/agg");
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).context("failed to create proof directory")?;
        }
        let proof_name = proof_names.join("_");
        let proof_path = format!("{proof_dir}/{proof_name}.bin");
        proof.save(&proof_path).expect("saving proof failed");
        println!("Aggregation proof saved to {proof_path}");
    } else {
        let (_, report) = prover
            .execute(Elf::Static(AGGREGATION_ELF), stdin)
            .calculate_gas(true)
            .deferred_proof_verification(false)
            .await
            .unwrap();
        println!("report: {report:?}");
    }

    Ok(())
}
