use clap::Parser;
use sp1_sdk::network::FulfillmentStrategy;
use std::sync::Arc;

use alloy_provider::{network::EthereumWallet, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use op_succinct_proposer::{DriverDBClient, OPChainMetricer, Proposer, ProposerConfigArgs};
use sp1_sdk::utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    l1_rpc: String,
    #[clap(long)]
    private_key: String,
    #[clap(long)]
    db_url: String,
    #[clap(long)]
    l2_ws_rpc: String,
    // Proposer config args
    #[clap(long)]
    l2oo_address: String,
    #[clap(long)]
    dgf_address: String,
    #[clap(long, default_value = "10")]
    range_proof_interval: u64,
    #[clap(long, default_value = "10")]
    max_concurrent_witness_gen: u64,
    #[clap(long, default_value = "10")]
    max_concurrent_proof_requests: u64,
    #[clap(long, default_value = "reserved")]
    range_proof_strategy: String,
    #[clap(long, default_value = "reserved")]
    agg_proof_strategy: String,
    #[clap(long, default_value = "groth16")]
    agg_proof_mode: String,
    #[clap(long, default_value = "false")]
    op_succinct_mock: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments using Clap
    let args = Args::parse();

    // Set up logging using the provided format
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true);

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into())
            }),
        )
        .event_format(format)
        .init();

    // Set up the SP1 SDK logger.
    utils::setup_logger();

    // Use Clap args instead of dotenv/env vars
    let rpc_url = args.l1_rpc;
    let private_key: PrivateKeySigner = args
        .private_key
        .parse()
        .expect("Failed to parse PRIVATE_KEY");
    let signer = EthereumWallet::new(private_key);
    let l1_provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(rpc_url.parse().expect("Failed to parse RPC URL"));

    let db_url = args.db_url;
    let db_client = Arc::new(DriverDBClient::new(&db_url).await?);

    // Build ProposerConfigArgs from Clap args
    let range_proof_strategy = if args.range_proof_strategy.to_lowercase() == "hosted" {
        FulfillmentStrategy::Hosted
    } else {
        FulfillmentStrategy::Reserved
    };
    let agg_proof_strategy = if args.agg_proof_strategy.to_lowercase() == "hosted" {
        FulfillmentStrategy::Hosted
    } else {
        FulfillmentStrategy::Reserved
    };
    let agg_proof_mode = if args.agg_proof_mode.to_lowercase() == "plonk" {
        sp1_sdk::SP1ProofMode::Plonk
    } else {
        sp1_sdk::SP1ProofMode::Groth16
    };

    let l2oo_address = args.l2oo_address.parse().expect("Invalid L2OO_ADDRESS");
    let dgf_address = args
        .dgf_address
        .parse()
        .expect("Invalid DISPUTE_GAME_FACTORY_ADDRESS");

    let proposer_config = ProposerConfigArgs {
        l2oo_address,
        dgf_address,
        range_proof_interval: args.range_proof_interval,
        max_concurrent_witness_gen: args.max_concurrent_witness_gen,
        max_concurrent_proof_requests: args.max_concurrent_proof_requests,
        range_proof_strategy,
        agg_proof_strategy,
        agg_proof_mode,
        op_succinct_mock: args.op_succinct_mock,
    };

    let proposer = Proposer::new(l1_provider, db_client.clone(), proposer_config).await?;

    let l2_ws_rpc = args.l2_ws_rpc;
    let l2_provider = alloy_provider::ProviderBuilder::default()
        .on_ws(WsConnect::new(l2_ws_rpc))
        .await?;

    // TODO: Set up proposer metrics.

    // Create the OP Metrics collector.
    let eth_listener = OPChainMetricer::new(db_client.clone(), Arc::new(l2_provider));

    // Spawn a thread for the ETH listener.
    let eth_handle = tokio::spawn(async move {
        if let Err(e) = eth_listener.listen().await {
            tracing::error!("ETH listener error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    // Spawn a thread for the proposer.
    let proposer_handle = tokio::spawn(async move {
        if let Err(e) = proposer.start().await {
            tracing::error!("Proposer error: {}", e);
            return Err(e);
        }
        Ok(())
    });

    // Wait for both tasks to complete.
    tokio::select! {
        res = eth_handle => {
            if let Err(e) = res {
                tracing::error!("ETH listener task failed: {}", e);
                return Err(anyhow::anyhow!("ETH listener task failed: {}", e));
            }
        }
        res = proposer_handle => {
            if let Err(e) = res {
                tracing::error!("Proposer task failed: {}", e);
                return Err(anyhow::anyhow!("Proposer task failed: {}", e));
            }
        }
    }

    Ok(())
}
