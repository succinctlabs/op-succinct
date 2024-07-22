use std::env;

use anyhow::Result;
use clap::Parser;
use host_utils::SP1KonaDataFetcher;
use kona_host::HostCli;

pub const KONA_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start block number.
    #[arg(short, long)]
    l2_block_number: u64,

    /// Whether or not to do the cost estimation.
    /// TODO: default to false
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

    let host_cli = data_fetcher
        .get_native_execution_data(args.l2_block_number)
        .await?;

    // TODO: check that the data exists. If it does not, then generate it using native-host
    if !args.skip_datagen {
        use kona_host::{init_tracing_subscriber, start_server_and_native_client, HostCli};
        init_tracing_subscriber(cfg.v).unwrap();
        start_server_and_native_client(cfg.clone()).await.unwrap();
    }

    // This will panic if the data directory does not exist.
    let sp1_stdin = stdin_from_host_cli(&host_cli)?;

    if args.cost_estimation {
        // Instantiate mock prover withstdin & run it
        // Just get the execution report
        // Nicely print out the total instruction count for each block.
        for (i, report) in reports.iter().enumerate() {
            println!(
                "Block {} cycle count: {}",
                i,
                report
                    .total_instruction_count()
                    .to_formatted_string(&Locale::en)
            );
        }
    } else {
        // Actually generate the proof
    }

    Ok(())
}
