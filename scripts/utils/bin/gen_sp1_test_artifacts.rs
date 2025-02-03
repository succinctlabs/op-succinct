use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use log::info;
use op_succinct_host_utils::{
    block_range::{get_validated_block_range, split_range_basic},
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_proof_stdin, start_server_and_native_client, ProgramType,
};
use op_succinct_scripts::HostExecutorArgs;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sp1_sdk::{utils, ProverClient};
use std::{
    fs::{self},
    path::PathBuf,
};

pub const RANGE_ELF: &[u8] = include_bytes!("../../../elf/range-elf");

#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file).ok();
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;
    let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

    let (l2_start_block, l2_end_block) =
        get_validated_block_range(&data_fetcher, args.start, args.end, args.default_range).await?;

    let split_ranges = split_range_basic(l2_start_block, l2_end_block, args.batch_size);

    info!(
        "The span batch ranges which will be executed: {:?}",
        split_ranges
    );

    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };

    // Get the host CLIs in order, in parallel.
    let host_clis = futures::stream::iter(split_ranges.iter())
        .map(|range| async {
            data_fetcher
                .get_host_cli_args(range.start, range.end, ProgramType::Multi, cache_mode)
                .await
                .expect("Failed to get host CLI args")
        })
        .buffered(15)
        .collect::<Vec<_>>()
        .await;

    // Use futures::future::join_all to run the server and client in parallel. Note: stream::iter did not work here, possibly
    // because the server and client are long-lived tasks.
    let handles = host_clis.iter().cloned().map(|host_cli| {
        tokio::spawn(async move {
            let oracle = start_server_and_native_client(&host_cli).await.unwrap();
            get_proof_stdin(oracle).unwrap()
        })
    });
    let stdins = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    let prover = ProverClient::builder().cpu().build();

    // Execute the program for each block range in parallel.
    stdins.par_iter().for_each(|sp1_stdin| {
        prover
            .execute(RANGE_ELF, sp1_stdin)
            .run()
            .expect("Failed to execute program");
    });

    // Now, write the successful ranges to /sp1-testing-suite-artifacts/op-succinct-chain-{l2_chain_id}-{start}-{end}
    // The folders should each have the RANGE_ELF as program.bin, and the serialized stdin should be
    // written to stdin.bin.
    let cargo_metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let root_dir = PathBuf::from(cargo_metadata.workspace_root).join("sp1-testing-suite-artifacts");

    let dir_name = root_dir.join(format!("op-succinct-chain-{}", l2_chain_id));
    info!("Writing artifacts to {:?}", dir_name);
    for (sp1_stdin, range) in stdins.iter().zip(split_ranges.iter()) {
        let program_dir = PathBuf::from(format!(
            "{}-{}-{}",
            dir_name.to_string_lossy(),
            range.start,
            range.end
        ));
        fs::create_dir_all(&program_dir)?;

        fs::write(program_dir.join("program.bin"), RANGE_ELF)?;
        fs::write(
            program_dir.join("stdin.bin"),
            bincode::serialize(&sp1_stdin).unwrap(),
        )?;
    }

    Ok(())
}
