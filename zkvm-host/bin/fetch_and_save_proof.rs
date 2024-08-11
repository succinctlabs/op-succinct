use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use sp1_sdk::{NetworkProver, ProverClient, SP1ProofWithPublicValues};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Request ID string
    #[arg(short, long)]
    request_id: String,

    /// Start L2 block number
    #[arg(short, long)]
    start: u64,

    /// End L2 block number
    #[arg(short, long)]
    end: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let prover = NetworkProver::new();

    // Fetch the proof
    let proof: SP1ProofWithPublicValues = prover.wait_proof(&args.request_id, None).await?;

    // Create the proofs directory if it doesn't exist
    let proof_dir = Path::new("data/proofs");
    fs::create_dir_all(proof_dir)?;

    // Generate the filename
    let filename = format!("{}-{}.bin", args.start, args.end);
    let file_path = proof_dir.join(filename);

    // Save the proof
    proof.save(file_path).expect("Failed to save proof");

    println!(
        "Proof saved successfully for blocks {} to {}",
        args.start, args.end
    );

    Ok(())
}