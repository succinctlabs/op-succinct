use std::{env, fs};

use anyhow::Result;
use clap::Parser;
use host_utils::{fetcher::SP1KonaDataFetcher, get_sp1_stdin};
use kona_host::{init_tracing_subscriber, start_server_and_native_client};
use sp1_sdk::ProverClient;

pub const KONA_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start block number.
    #[arg(short, long)]
    l2_block_number: u64,

    /// Whether or not to do the cost estimation.
    #[arg(short, long)]
    cost_estimation: bool,

    /// Generate the execution data.
    #[arg(short, long)]
    generate_execution_data: bool,
}

/// Collect the execution reports across a number of blocks. Inclusive of start and end block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let data_fetcher = SP1KonaDataFetcher {
        l2_rpc: env::var("CLABBY_RPC_L2").expect("CLABBY_RPC_L2 is not set."),
        ..Default::default()
    };

    // TODO: Use `optimism_outputAtBlock` to fetch the L2 block at head
    // https://github.com/ethereum-optimism/kona/blob/d9dfff37e2c5aef473f84bf2f28277186040b79f/bin/client/justfile#L26-L32
    let l2_safe_head = args.l2_block_number - 1;

    let host_cli = data_fetcher
        .get_host_cli_args(l2_safe_head, args.l2_block_number)
        .await?;

    let data_dir = host_cli
        .data_dir
        .clone()
        .expect("Data directory is not set.");

    // If the user wants to generate the execution data, or the data directory doesn't exist,
    // we need to start the server and generate the execution data for the block.
    if args.generate_execution_data || !std::path::Path::new(&data_dir).exists() {
        // Overwrite existing data directory.
        fs::create_dir_all(&data_dir).unwrap();

        init_tracing_subscriber(host_cli.v).unwrap();
        println!("Starting native server");
        start_server_and_native_client(host_cli.clone())
            .await
            .unwrap();
    }

    println!("Ran native server");

    // Get the stdin for the block.
    let sp1_stdin = get_sp1_stdin(&host_cli)?;

    let prover = ProverClient::new();
    if args.cost_estimation {
        let (_, report) = prover.execute(KONA_ELF, sp1_stdin).run().unwrap();

        println!("Executed prover");

        println!(
            "Block {} cycle count: {}",
            args.l2_block_number,
            report.total_instruction_count()
        );
    }
    Ok(())
}
