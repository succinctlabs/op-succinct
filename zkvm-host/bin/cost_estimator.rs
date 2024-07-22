use anyhow::Result;
use clap::Parser;
use kona_host::init_tracing_subscriber;
use native_host::run_native_host;
use num_format::{Locale, ToFormattedString};
use zkvm_host::execute_kona_program;
use zkvm_host::{CostEstimatorCliArgs, fetcher::SP1KonaDataFetcher};



/// Collect the execution reports across a number of blocks. Inclusive of start and end block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = CostEstimatorCliArgs::parse();

    // Initialize tracing subscriber.
    init_tracing_subscriber(args.verbosity_level).unwrap();

    let mut reports = Vec::new();

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
            let native_execution_data = data_fetcher.get_native_host_cli_args(&block_data, args.verbosity_level)?;
            println!(
                "Got native execution data for block {}. {:?}",
                block_num, native_execution_data
            );
            run_native_host(&native_execution_data).await?;
        }

        println!("Ran native host for block {}", block_num);

        // Execute the Kona program.
        let report = execute_kona_program(&block_data.into());

        reports.push(report);

        println!("Executed block {}", block_num);
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
