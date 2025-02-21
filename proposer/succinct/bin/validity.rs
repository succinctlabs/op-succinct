use alloy_primitives::Address;
use sp1_sdk::network::FulfillmentStrategy;
use std::{env, sync::Arc, time::Duration};
use tracing::info;

use alloy_provider::{network::EthereumWallet, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use op_succinct_proposer::{read_env, DriverDBClient, OPChainMetricer, Proposer, RequesterConfig};

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
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
                .add_directive("maili_protocol=error".parse().unwrap())
                .add_directive("sp1_core_executor=off".parse().unwrap()),
        )
        .event_format(format)
        .init();

    info!("Initializing DB client");

    // Read the environment variables.
    let (db_client, proposer_config) = read_env().await?;

    // Read all config from env vars
    let rpc_url = env::var("L1_RPC").expect("L1_RPC is not set");
    let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY is not set")
        .parse()
        .expect("Failed to parse PRIVATE_KEY");
    let signer = EthereumWallet::new(private_key);
    let l1_provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(rpc_url.parse().expect("Failed to parse L1_RPC"));

    let proposer = Proposer::new(l1_provider, db_client.clone(), proposer_config).await?;

    info!("Initializing L2 provider");

    let l2_ws_rpc = env::var("L2_WS_RPC").expect("L2_WS_RPC is not set");
    let l2_provider = alloy_provider::ProviderBuilder::default()
        .on_ws(WsConnect::new(l2_ws_rpc))
        .await?;

    info!("Initializing ETH listener");

    // Create the OP chain metrics collector.
    let eth_listener = OPChainMetricer::new(db_client.clone(), Arc::new(l2_provider));

    // Spawn a thread for the ETH listener.
    let eth_handle = tokio::spawn(async move {
        if let Err(e) = eth_listener.listen().await {
            tracing::error!("ETH listener error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    info!("Initializing proposer");

    // Spawn a thread for the proposer.
    let proposer_handle = tokio::spawn(async move {
        if let Err(e) = proposer.run().await {
            tracing::error!("Proposer error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    // // Spawn a thread for the metrics exporter.
    // info!("Initializing metrics exporter");
    // const METRICS_PORT: u16 = 7000;
    // op_succinct_proposer::init_metrics(&METRICS_PORT);

    // info!("Starting metrics update loop");

    // const METRICS_UPDATE_INTERVAL: u64 = 1;
    // let metrics_handle = tokio::spawn(async move {
    //     loop {
    //         op_succinct_proposer::update_cpu_and_memory();
    //         tokio::time::sleep(Duration::from_secs(METRICS_UPDATE_INTERVAL)).await;
    //     }
    // });

    // Wait for all tasks to complete.
    let (eth_res, proposer_res) = tokio::try_join!(eth_handle, proposer_handle)?;
    if let Err(e) = eth_res.or(proposer_res) {
        tracing::error!("Proposer task failed: {}", e);
        return Err(e);
    }

    Ok(())
}
