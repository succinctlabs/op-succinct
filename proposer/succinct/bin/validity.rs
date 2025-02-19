use std::env;

use anyhow::Result;
use op_succinct_proposer::Proposer;
use sp1_sdk::utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Enable logging.
    env::set_var("RUST_LOG", "info");

    // Set up the SP1 SDK logger.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // TODO: Read this from the fetcher.
    let rpc_url = env::var("L1_RPC").expect("L1_RPC not set");
    let signer = EthereumWallet::new(private_key);
    let provider = ProviderBuilder::new()
        .wallet(signer.clone())
        .on_http(rpc_url.parse().expect("Failed to parse RPC URL"));
    let proposer = Proposer::new(provider).await?;

    proposer.start().await?;

    Ok(())
}
