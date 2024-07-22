// A host program to generate a proof of an Optimism L2 block STF in the zkVM.

use clap::Parser;
use anyhow::Result;
use sp1_sdk::utils;
use zkvm_host::{execute_kona_program, MultiblockCliArgs, fetcher::SP1KonaDataFetcher};
use native_host::run_native_host;

pub const ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-multiblock-elf");

#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_logger();
    let args = MultiblockCliArgs::parse();

    let data_fetcher = SP1KonaDataFetcher::default();
    let block_data = data_fetcher.pull_block_data(Some(args.start_block), args.end_block).await?;

    if args.run_native {
        let native_execution_data = data_fetcher.get_native_host_cli_args(&block_data, args.verbosity_level)?;
        run_native_host(&native_execution_data).await?;
    }

    let report = execute_kona_program(&block_data.into(), ELF);
    println!("Report: {}", report);

    // let (pk, vk) = client.setup(ELF);
    // let mut proof = client.prove(&pk, stdin).unwrap();
    // println!("generated valid zk proof");

    Ok(())
}
