use alloy_primitives::B256;
use clap::Parser;
use op_succinct_client_utils::boot::hash_rollup_config;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use op_succinct_proof_utils::get_range_elf_embedded;
use sp1_sdk::{utils, HashableKey, Prover, ProverClient};

use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct ConfigArgs {
    /// The environment file to use.
    #[arg(long)]
    pub env_file: Option<PathBuf>,
}

// Get the verification keys for the ELFs and check them against the contract.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = ConfigArgs::parse();
    if let Some(path) = args.env_file {
        dotenv::from_path(path)?;
    }
    utils::setup_logger();

    let prover = ProverClient::builder().cpu().build();
    let (_, range_vk) = prover.setup(get_range_elf_embedded());
    println!("Range Verification Key Hash: {}", B256::from(range_vk.hash_bytes()));
    let (_, agg_vk) = prover.setup(AGGREGATION_ELF);
    println!("Aggregation Verification Key Hash: {}", agg_vk.bytes32());

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
    let rollup_config = data_fetcher.rollup_config.as_ref().unwrap();
    println!("Rollup Config Hash: {}", hash_rollup_config(rollup_config));

    Ok(())
}
