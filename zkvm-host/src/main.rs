// A host program to generate a proof of an Optimism L2 block STF in the zkVM.

use clap::Parser;
use zkvm_host::ZKVMHostCliArgs;

use sp1_sdk::utils;
use zkvm_common::{BootInfoWithoutRollupConfig, SP1KonaDataFetcher};
use zkvm_host::execute_kona_program;

fn main() {
    utils::setup_logger();

    let cli_args = ZKVMHostCliArgs::parse();
    let data_fetcher = SP1KonaDataFetcher::default();
    data_fetcher.pull_block_data(cli_args.block).await.unwrap();

    if cli_args.run_native {
        // Run the native host to generate the merkle proofs.
        let native_execution_data = data_fetcher.into()?;
        run_native_host(&native_execution_data).await?;
    }

    let boot_info: BootInfoWithoutRollupConfig = data_fetcher.into();
    let report = execute_kona_program(&boot_info);

    println!("Report: {}", report);
}
