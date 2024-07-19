// A host program to generate a proof of an Optimism L2 block STF in the zkVM.

use clap::Parser;
use zkvm_host::ZkVmHostCliArgs;
use native_host::run_native_host;
use anyhow::Result;
use sp1_sdk::utils;
use zkvm_common::SP1KonaDataFetcher;
use zkvm_host::execute_kona_program;


#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = ZkVmHostCliArgs::parse();
    let mut data_fetcher = SP1KonaDataFetcher::default();
    data_fetcher.pull_block_data(cli_args.block).await.unwrap();

    if cli_args.run_native {
        // Run the native host to generate the merkle proofs.
        // TODO: build.rs builds for ZKVM, but in this case should we also build for native?
        let native_execution_data = data_fetcher.get_host_cli();
        run_native_host(&native_execution_data).await?;
    }

    utils::setup_logger();

    let boot_info = data_fetcher.get_boot_info();

    let report = execute_kona_program(&boot_info);
    println!("Report: {}", report);

    // prove_kona_program(&boot_info);

    Ok(())
}
