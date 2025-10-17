use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use op_succinct_validity::DriverDBClient;

use crate::cli::{Args, Commands};

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    let env_file = std::env::args_os().skip_while(|arg| arg != "--env-file").nth(1); // Get the next argument after "--env-file"

    if let Some(env_file) = env_file {
        println!("{env_file:?}");
        dotenv::from_filename(env_file).ok();
    } else {
        dotenv::dotenv().ok();
    }

    let args = Args::parse();

    match args.command {
        Commands::List { status } => {
            let db_client = DriverDBClient::new(&args.database_url).await?;
            let table = commands::list(status, db_client).await?;

            println!("{table}");
        }
        Commands::Split { id, at } => {
            let db_client = DriverDBClient::new(&args.database_url).await?;
            let fetcher = OPSuccinctDataFetcher::new();

            commands::split(id, at, db_client, Arc::new(fetcher)).await?;

            println!("Ok");
        }
    }

    Ok(())
}
