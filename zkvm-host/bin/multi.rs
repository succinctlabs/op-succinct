use std::{env, fs};

use anyhow::Result;
use clap::Parser;
use client_utils::precompiles::PRECOMPILE_HOOK_FD;
use host_utils::{fetcher::SP1KonaDataFetcher, get_sp1_stdin, ProgramType};
use kona_host::start_server_and_native_client;
use num_format::{Locale, ToFormattedString};
use revm::{precompile::Precompiles, primitives::{Address, Bytes, Precompile}};
use sp1_sdk::{utils, ProverClient};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/validity-client-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start L2 block number.
    #[arg(short, long)]
    start: u64,

    /// End L2 block number.
    #[arg(short, long)]
    end: u64,

    /// Verbosity level.
    #[arg(short, long, default_value = "0")]
    verbosity: u8,

    /// Skip running native execution.
    #[arg(short, long)]
    use_cache: bool,
}

/// Execute the Kona program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::setup_logger();
    let args = Args::parse();

    let data_fetcher = SP1KonaDataFetcher {
        l2_rpc: env::var("CLABBY_RPC_L2").expect("CLABBY_RPC_L2 is not set."),
        ..Default::default()
    };

    let host_cli = data_fetcher
        .get_host_cli_args(args.start, args.end, args.verbosity, ProgramType::Multi)
        .await?;

    let data_dir = host_cli
        .data_dir
        .clone()
        .expect("Data directory is not set.");

    // By default, re-run the native execution unless the user passes `--use-cache`.
    if !args.use_cache {
        // Overwrite existing data directory.
        fs::create_dir_all(&data_dir).unwrap();

        // Start the server and native client.
        start_server_and_native_client(host_cli.clone())
            .await
            .unwrap();
    }

    // Get the stdin for the block.
    let sp1_stdin = get_sp1_stdin(&host_cli)?;

    let prover = ProverClient::new();
    let (_, report) = prover
        .execute(MULTI_BLOCK_ELF, sp1_stdin)
        .with_hook(PRECOMPILE_HOOK_FD, |env, buf| {
            let addr: Address = buf[0..20].try_into().unwrap();
            let gas_limit = u64::from_le_bytes(buf[20..28].try_into().unwrap());
            let input: Bytes = buf[28..].to_vec().into();
            println!("[HOOK] Precompile addr {} called.", addr);

            let precompiles = Precompiles::byzantium();
            let precompile = precompiles.inner().get(&addr).unwrap();
            let result = match precompile {
                Precompile::Standard(precompile) => precompile(&input, gas_limit),
                _ => panic!("Annotated precompile must be a standard precompile."),
            };

            let mut serialized_vec = vec![];
            match result {
                Ok(result) => {
                    serialized_vec.push(0);
                    serialized_vec.extend_from_slice(&result.gas_used.to_le_bytes());
                    serialized_vec.extend_from_slice(&result.bytes.to_vec());
                }
                Err(err) => {
                    serialized_vec.push(1);
                    match err {
                        revm::precompile::PrecompileErrors::Error(err) => {
                            serialized_vec.push(0);
                        }
                        revm::precompile::PrecompileErrors::Fatal { msg } => {
                            serialized_vec.push(1);
                        }
                    }
                }
            }
            vec![serialized_vec]
        })
        .run()
        .unwrap();

    println!(
        "Cycle count: {}",
        report
            .total_instruction_count()
            .to_formatted_string(&Locale::en)
    );

    Ok(())
}
