use std::env;

use anyhow::Result;
use clap::Parser;
use host_utils::{fetcher::SP1KonaDataFetcher, get_sp1_stdin};
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
}

/// Collect the execution reports across a number of blocks. Inclusive of start and end block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let data_fetcher = SP1KonaDataFetcher {
        l2_rpc: env::var("CLABBY_RPC_L2").unwrap(),
        ..Default::default()
    };

    let l2_safe_head = data_fetcher
        .get_l2_safe_head_block(args.l2_block_number)
        .await?;
    let host_cli = data_fetcher
        .get_native_execution_data(l2_safe_head, args.l2_block_number)
        .await?;

    // Always generate data using native-host
    use kona_host::{init_tracing_subscriber, start_server_and_native_client};
    init_tracing_subscriber(host_cli.v).unwrap();
    start_server_and_native_client(host_cli.clone())
        .await
        .unwrap();

    // Fetch stdin from the host cli
    let sp1_stdin = get_sp1_stdin(&host_cli);

    let prover = ProverClient::new();
    if args.cost_estimation {
        env::set_var("SP1_PROVER", "mock");
        let (_, report) = prover.execute(KONA_ELF, sp1_stdin).run().unwrap();

        println!(
            "Block {} cycle count: {}",
            args.l2_block_number,
            report.total_instruction_count()
        );
    }
    Ok(())
}
