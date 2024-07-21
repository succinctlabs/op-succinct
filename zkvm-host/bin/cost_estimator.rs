use anyhow::Result;
use clap::Parser;
use kona_host::init_tracing_subscriber;
use native_host::run_native_host;
use num_format::{Locale, ToFormattedString};
use sp1_sdk::ExecutionReport;
use std::io;
use std::io::{stderr, stdin, stdout, Read, Write};
use zkvm_host::execute_kona_program;
use zkvm_host::fetcher::SP1KonaDataFetcher;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start block number.
    #[arg(short, long)]
    start_block: u64,

    /// End block number.
    #[arg(short, long)]
    end_block: u64,

    /// RPC URL for the OP Stack Chain to do cost estimation for.
    #[arg(short, long)]
    rpc_url: String,

    /// Skip native data generation if data directory already exists.
    #[arg(
        long,
        help = "Skip native data generation if the Merkle tree data is already stored in data."
    )]
    skip_datagen: bool,

    /// Verbosity level.
    #[arg(short, long, default_value = "0")]
    verbosity_level: u8,
}

fn clear_stdin() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut buffer = [0; 1];

    while let Ok(n) = stdin.read(&mut buffer) {
        if n == 0 || buffer[0] == b'\n' {
            break;
        }
    }
}
/// Collect the execution reports across a number of blocks. Inclusive of start and end block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    // Initialize tracing subscriber.
    init_tracing_subscriber(args.verbosity_level).unwrap();

    let mut reports: Vec<ExecutionReport> = Vec::new();

    let data_fetcher = SP1KonaDataFetcher {
        l2_rpc: args.rpc_url,
        ..Default::default()
    };

    for block_num in args.start_block..=args.end_block {
        // Get the relevant data for native and zkvm execution.
        let block_data = data_fetcher.pull_block_data(block_num).await?;

        println!("Pulled block data for block {}", block_num);

        if !args.skip_datagen {
            // Get native execution data.
            let native_execution_data = data_fetcher.get_native_execution_data(&block_data)?;
            println!(
                "Got native execution data for block {}. {:?}",
                block_num, native_execution_data
            );
            run_native_host(&native_execution_data).await?;
        }

        println!("Ran native host for block {}", block_num);

        stdout().flush()?;
        println!("Flushed stdout for block {}", block_num);
        stderr().flush()?;
        println!("Flushed stderr for block {}", block_num);
        // let empty_stdin = io::empty();
        // let old_stdin = io::set_stdin(empty_stdin);
        // let result = f();
        // io::set_stdin(old_stdin);
        // clear_stdin();

        // println!("Cleared stdin for block {}", block_num);

        // // Execute the Kona program.
        // let report = execute_kona_program(&block_data.into());

        // reports.push(report);

        // println!("Executed block {}", block_num);
    }

    // Nicely print out the total instruction count for each block.
    for (i, report) in reports.iter().enumerate() {
        println!(
            "Block {} cycle count: {}",
            args.start_block + i as u64,
            report
                .total_instruction_count()
                .to_formatted_string(&Locale::en)
        );
    }

    Ok(())
}
