use std::fs;

use anyhow::Result;
use clap::Parser;
use client_utils::precompiles::PRECOMPILE_HOOK_FD;
use host_utils::{fetcher::SP1KonaDataFetcher, get_sp1_stdin, ProgramType};
use kona_host::start_server_and_native_client;
use num_format::{Locale, ToFormattedString};
use sp1_sdk::{utils, ProverClient};
use zkvm_host::{precompile_hook, ExecutionStats};

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

    /// Whether to print out the statistics.
    #[arg(short, long, default_value = "true")]
    stats: bool,
}

/// Execute the Kona program for a single block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    utils::setup_logger();
    let args = Args::parse();

    let data_fetcher = SP1KonaDataFetcher {
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
        .with_hook(PRECOMPILE_HOOK_FD, precompile_hook)
        .run()
        .unwrap();

    let total_instruction_count = report.total_instruction_count();

    if args.stats {
        let nb_blocks = args.end - args.start + 1;

        // Fetch the number of transactions in the blocks from the L2 RPC.
        let nb_transactions = SP1KonaDataFetcher::get_block_transaction_count_range(
            &data_fetcher.l2_rpc,
            args.start,
            args.end,
        )
        .await?;

        println!(
            "{}",
            ExecutionStats {
                total_instruction_count,
                nb_blocks,
                nb_transactions,
            }
        );
    } else {
        println!(
            "Total cycle count: {}",
            total_instruction_count.to_formatted_string(&Locale::en)
        );
    }

    Ok(())
}
