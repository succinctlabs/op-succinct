use std::{fs, time::Duration};

use anyhow::Result;
use clap::Parser;
use client_utils::{precompiles::PRECOMPILE_HOOK_FD, RawBootInfo};
use host_utils::{
    fetcher::{ChainMode, SP1KonaDataFetcher},
    ProgramType,
};
use sp1_sdk::{utils, ExecutionReport, ProverClient, SP1Stdin};
use zkvm_host::{precompile_hook, run_native_host_runner, BnStats, ExecutionStats};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/validity-client-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the stdin file.
    #[arg(short, long)]
    stdin_file: String,
    // /// Start L2 block number.
    // #[arg(short, long)]
    // start: u64,

    // /// End L2 block number.
    // #[arg(short, long)]
    // end: u64,

    // /// Verbosity level.
    // #[arg(short, long, default_value = "0")]
    // verbosity: u8,

    // /// Generate proof.
    // #[arg(short, long)]
    // prove: bool,
}

/// Execute the Kona program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::setup_logger();
    let args = Args::parse();

    let data_fetcher = SP1KonaDataFetcher::new();

    // Read stdin from file
    let sp1_stdin_bincode = fs::read(&args.stdin_file)?;
    let sp1_stdin: SP1Stdin = bincode::deserialize::<SP1Stdin>(&sp1_stdin_bincode).unwrap();
    // Read the first BootInfo from stdin.
    let mut cloned_stdin = sp1_stdin.clone();
    let boot_info = cloned_stdin.read::<RawBootInfo>();
    println!("BootInfo: {:?}", boot_info);

    let prover = ProverClient::new();

    // TODO: Remove this precompile hook once we merge the BN and BLS precompiles.
    let (_, report) = prover
        .execute(MULTI_BLOCK_ELF, sp1_stdin.clone())
        .with_hook(PRECOMPILE_HOOK_FD, precompile_hook)
        .run()
        .unwrap();

    Ok(())
}
