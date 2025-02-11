use clap::Parser;
use eyre::Result;
use tracing::info;

mod config;
mod driver;
mod metrics;
mod proposer;
mod rpc;
mod types;

use crate::config::Config;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(flatten)]
    config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file before doing anything else
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    info!("Starting OP Succinct Proposer");

    // Create and start the proposer service
    let service = proposer::ProposerService::new(cli.config).await?;
    service.run().await?;

    Ok(())
}
