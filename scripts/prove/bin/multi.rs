use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::{
    fetcher::{CacheMode, OPSuccinctDataFetcher},
    get_proof_stdin,
    stats::ExecutionStats,
    witnessgen::WitnessGenExecutor,
    ProgramType,
};
use sp1_sdk::{utils, ProverClient};
use std::{fs, time::Instant};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../../elf/range-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start L2 block number.
    #[arg(short, long)]
    start: u64,

    /// End L2 block number.
    #[arg(short, long)]
    end: u64,

    /// Verbosity level.
    #[arg(short, long, default_value = "0")]
    verbosity: u8,

    /// Skip running native execution.
    #[arg(short, long)]
    use_cache: bool,

    /// Generate proof.
    #[arg(short, long)]
    prove: bool,

    /// Env file.
    #[arg(short, long, default_value = ".env")]
    env_file: Option<String>,
}

/// Execute the OP Succinct program for multiple blocks.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(env_file) = args.env_file {
        dotenv::from_filename(env_file).ok();
    }
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::default();

    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };

    let host_cli = data_fetcher
        .get_host_cli_args(args.start, args.end, ProgramType::Multi, cache_mode)
        .await?;

    // By default, re-run the native execution unless the user passes `--use-cache`.
    let start_time = Instant::now();
    if !args.use_cache {
        // Start the server and native client.
        let mut witnessgen_executor = WitnessGenExecutor::default();
        witnessgen_executor.spawn_witnessgen(&host_cli).await?;
        witnessgen_executor.flush().await?;
    }
    let witness_generation_time_sec = start_time.elapsed();
    println!(
        "Witness Generation Duration: {:?}",
        witness_generation_time_sec.as_secs()
    );

    // Get the stdin for the block.
    let sp1_stdin = get_proof_stdin(&host_cli)?;

    let prover = ProverClient::new();

    if args.prove {
        // If the prove flag is set, generate a proof.
        let (pk, _) = prover.setup(MULTI_BLOCK_ELF);

        // Generate proofs in compressed mode for aggregation verification.
        let proof = prover.prove(&pk, sp1_stdin).compressed().run().unwrap();

        // Create a proof directory for the chain ID if it doesn't exist.
        let proof_dir = format!(
            "data/{}/proofs",
            data_fetcher.get_l2_chain_id().await.unwrap()
        );
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).unwrap();
        }
        // Save the proof to the proof directory corresponding to the chain ID.
        proof
            .save(format!("{}/{}-{}.bin", proof_dir, args.start, args.end))
            .expect("saving proof failed");
    } else {
        let start_time = Instant::now();
        let (_, report) = prover
            .execute(MULTI_BLOCK_ELF, sp1_stdin.clone())
            .run()
            .unwrap();
        let execution_duration = start_time.elapsed();

        let l2_chain_id = data_fetcher.get_l2_chain_id().await.unwrap();
        let report_path = format!(
            "execution-reports/multi/{}/{}-{}.csv",
            l2_chain_id, args.start, args.end
        );

        // Create the report directory if it doesn't exist.
        let report_dir = format!("execution-reports/multi/{}", l2_chain_id);
        if !std::path::Path::new(&report_dir).exists() {
            fs::create_dir_all(&report_dir).unwrap();
        }

        let mut stats = ExecutionStats::default();
        stats
            .add_block_data(&data_fetcher, args.start, args.end)
            .await;
        stats.add_report_data(&report);
        stats.add_aggregate_data();
        stats.add_timing_data(
            execution_duration.as_secs(),
            witness_generation_time_sec.as_secs(),
        );
        println!("Execution Stats: \n{:?}", stats);

        // Write to CSV.
        let mut csv_writer = csv::Writer::from_path(report_path)?;
        csv_writer.serialize(&stats)?;
        csv_writer.flush()?;
    }

    Ok(())
}
