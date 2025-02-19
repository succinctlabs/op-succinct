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

    let proposer = Proposer::new().await?;

    proposer.start().await?;

    Ok(())
}
