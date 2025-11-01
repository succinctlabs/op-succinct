use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use op_succinct_validity::init_db;

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
    let pool = init_db(&args.database_url).await?;

    match args.command {
        Commands::List { status, from, to } => {
            let table = commands::list(status, from, to, &pool).await?;

            println!("{table}");
        }
        Commands::Split { id, at } => {
            let fetcher = OPSuccinctDataFetcher::new();

            commands::split(id, at, &pool, Arc::new(fetcher)).await?;
        }
        Commands::Join { a, b } => {
            let fetcher = OPSuccinctDataFetcher::new();

            commands::join(a, b, &pool, Arc::new(fetcher)).await?;
        }
        Commands::Kill { id } => {
            commands::kill(id, &pool).await?;
        }
    }

    Ok(())
}
