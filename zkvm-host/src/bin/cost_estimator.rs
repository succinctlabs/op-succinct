use zkvm_host::{
    execute_kona_program,
    cli::CostEstimatorCliArgs
};
use zkvm_common::SP1KonaDataFetcher;
use native_host::run_native_host;

use clap::Parser;
use anyhow::Result;
use num_format::{Locale, ToFormattedString};

/// Collect the execution reports across a number of blocks. Inclusive of start and end block.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = CostEstimatorCliArgs::parse();

    let mut reports = Vec::new();

    // TODO: Make sure this works and still loads chain id, since new isn't called directly.
    let mut data_fetcher = SP1KonaDataFetcher {
        l2_node_address: args.l2_rpc_url,
        ..Default::default()
    };

    for block_num in args.start_block..=args.end_block {
        // Get native execution data.
        data_fetcher.pull_block_data(block_num).await?;

        if !args.skip_datagen {
            // Run the native host to generate the merkle proofs.
            let native_execution_data = data_fetcher.get_host_cli();
            run_native_host(&native_execution_data).await?;
        }

        // Execute Kona program and collect execution reports.
        let boot_info = data_fetcher.get_boot_info();
        let report = execute_kona_program(&boot_info);

        println!("Block {}: {}", block_num, report);

        reports.push(report);
    }

    // Nicely print out the total instruction count for each block.
    // TODO: Add $ cost estimation?
    for (i, report) in reports.iter().enumerate() {
        println!(
            "Block {} cycle count: {}",
            i,
            report
                .total_instruction_count()
                .to_formatted_string(&Locale::en)
        );
    }

    Ok(())
}
