use alloy_primitives::Address;
use alloy_provider::{network::EthereumWallet, Provider, ProviderBuilder};
use anyhow::Result;
use metrics_process::Collector;
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RunContext};
use op_succinct_proposer::{read_env, DriverDBClient, Proposer, RequesterConfig};
use std::{sync::Arc, thread, time::Duration};
use tracing::info;

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to environment file
    #[arg(long, default_value = ".env")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    dotenv::from_filename(args.env_file).ok();

    // Set up logging using the provided format
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true);

    // Turn off all logging from kona.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
                .add_directive("single_hint_handler=error".parse().unwrap())
                .add_directive("execute=error".parse().unwrap())
                .add_directive("sp1_prover=error".parse().unwrap())
                .add_directive("boot-loader=error".parse().unwrap())
                .add_directive("client-executor=error".parse().unwrap())
                .add_directive("client=error".parse().unwrap())
                .add_directive("channel-assembler=error".parse().unwrap())
                .add_directive("attributes-queue=error".parse().unwrap())
                .add_directive("batch-validator=error".parse().unwrap())
                .add_directive("client-derivation-driver=error".parse().unwrap())
                .add_directive("host-server=error".parse().unwrap())
                .add_directive("kona_protocol=error".parse().unwrap())
                .add_directive("sp1_core_executor=off".parse().unwrap()),
        )
        .event_format(format)
        .init();

    let fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;

    // Read the environment variables.
    let env_config = read_env()?;

    let db_client = Arc::new(DriverDBClient::new(&env_config.db_url).await?);
    let proposer_config = RequesterConfig {
        l1_chain_id: fetcher.l1_provider.get_chain_id().await? as i64,
        l2_chain_id: fetcher.l2_provider.get_chain_id().await? as i64,
        l2oo_address: env_config.l2oo_address,
        dgf_address: Address::ZERO,
        range_proof_interval: env_config.range_proof_interval,
        max_concurrent_witness_gen: env_config.max_concurrent_witness_gen,
        max_concurrent_proof_requests: env_config.max_concurrent_proof_requests,
        range_proof_strategy: env_config.range_proof_strategy,
        agg_proof_strategy: env_config.agg_proof_strategy,
        agg_proof_mode: env_config.agg_proof_mode,
        submission_interval: env_config.submission_interval,
        mock: env_config.mock,
    };

    // Read all config from env vars
    let signer = EthereumWallet::new(env_config.private_key);
    let l1_provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(env_config.l1_rpc.parse().expect("Failed to parse L1_RPC"));

    let proposer = Proposer::new(
        l1_provider,
        db_client.clone(),
        Arc::new(fetcher),
        proposer_config,
        env_config.loop_interval,
    )
    .await?;

    // Spawn a thread for the proposer.
    info!("Starting proposer.");
    let proposer_handle = tokio::spawn(async move {
        if let Err(e) = proposer.run().await {
            tracing::error!("Proposer error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    // Initialize metrics exporter.
    op_succinct_proposer::init_metrics(&env_config.metrics_port);

    // Wait for all tasks to complete.
    let proposer_res = proposer_handle.await?;
    if let Err(e) = proposer_res {
        tracing::error!("Proposer task failed: {}", e);
        return Err(e);
    }

    Ok(())
}
