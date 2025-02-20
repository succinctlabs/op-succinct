use std::{env, sync::Arc};
use tracing_subscriber::{fmt, EnvFilter};

use alloy_provider::{network::EthereumWallet, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use op_succinct_proposer::{DriverDBClient, EthListener, Proposer};
use sp1_sdk::utils;

#[tokio::main]
async fn main() -> Result<()> {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true);

    // Initialize logging using RUST_LOG environment variable, defaulting to INFO level
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
            EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into())
        }))
        .event_format(format)
        .init();

    // Set up the SP1 SDK logger.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // TODO: Read L1_RPC from the fetcher.
    let rpc_url = env::var("L1_RPC").expect("L1_RPC not set");

    // TODO: Set up KMS.
    let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY not set")
        .parse()
        .expect("Failed to parse PRIVATE_KEY");
    let signer = EthereumWallet::new(private_key);
    let l1_provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(rpc_url.parse().expect("Failed to parse RPC URL"));

    let db_url = env::var("DB_URL").expect("DB_URL not set");
    let db_client = Arc::new(DriverDBClient::new(&db_url).await?);
    let proposer = Proposer::new(l1_provider, db_client.clone()).await?;

    let l2_rpc = env::var("L2_RPC").expect("L2_RPC not set");
    let l2_provider = ProviderBuilder::default()
        .on_ws(WsConnect::new(l2_rpc))
        .await?;

    // TODO: Set up proposer metrics.

    // Create the ETH listener.
    let eth_listener = EthListener::new(db_client.clone(), Arc::new(l2_provider));

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
