use alloy_primitives::Address;
use sp1_sdk::network::FulfillmentStrategy;
use std::{env, sync::Arc};

use alloy_provider::{network::EthereumWallet, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use op_succinct_proposer::{DriverDBClient, OPChainMetricer, Proposer, ProposerConfigArgs};

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

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::INFO.into())
            }),
        )
        .event_format(format)
        .init();

    // Read all config from env vars
    let rpc_url = env::var("L1_RPC").expect("L1_RPC is not set");
    let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY is not set")
        .parse()
        .expect("Failed to parse PRIVATE_KEY");
    let signer = EthereumWallet::new(private_key);
    let l1_provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(rpc_url.parse().expect("Failed to parse RPC URL"));

    let db_url = env::var("DB_URL").expect("DB_URL is not set");
    let db_client = Arc::new(DriverDBClient::new(&db_url).await?);

    let range_proof_strategy = if env::var("RANGE_PROOF_STRATEGY")
        .unwrap_or_else(|_| "reserved".to_string())
        .to_lowercase()
        == "hosted"
    {
        FulfillmentStrategy::Hosted
    } else {
        FulfillmentStrategy::Reserved
    };

    let agg_proof_strategy = if env::var("AGG_PROOF_STRATEGY")
        .unwrap_or_else(|_| "reserved".to_string())
        .to_lowercase()
        == "hosted"
    {
        FulfillmentStrategy::Hosted
    } else {
        FulfillmentStrategy::Reserved
    };

    let agg_proof_mode = if env::var("AGG_PROOF_MODE")
        .unwrap_or_else(|_| "groth16".to_string())
        .to_lowercase()
        == "plonk"
    {
        sp1_sdk::SP1ProofMode::Plonk
    } else {
        sp1_sdk::SP1ProofMode::Groth16
    };

    let l2oo_address = env::var("L2OO_ADDRESS")
        .expect("L2OO_ADDRESS is not set")
        .parse()
        .expect("Invalid L2OO_ADDRESS");

    let proposer_config = ProposerConfigArgs {
        l2oo_address,
        dgf_address: Address::ZERO,
        range_proof_interval: env::var("RANGE_PROOF_INTERVAL")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("Invalid RANGE_PROOF_INTERVAL"),
        max_concurrent_witness_gen: env::var("MAX_CONCURRENT_WITNESS_GEN")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("Invalid MAX_CONCURRENT_WITNESS_GEN"),
        max_concurrent_proof_requests: env::var("MAX_CONCURRENT_PROOF_REQUESTS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("Invalid MAX_CONCURRENT_PROOF_REQUESTS"),
        submission_interval: env::var("SUBMISSION_INTERVAL")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("Invalid SUBMISSION_INTERVAL"),
        range_proof_strategy,
        agg_proof_strategy,
        agg_proof_mode,
        op_succinct_mock: env::var("OP_SUCCINCT_MOCK")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("Invalid OP_SUCCINCT_MOCK"),
    };

    let proposer = Proposer::new(l1_provider, db_client.clone(), proposer_config).await?;

    let l2_ws_rpc = env::var("L2_WS_RPC").expect("L2_WS_RPC is not set");
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
