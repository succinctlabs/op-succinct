use anyhow::Result;
use clap::Parser;
use op_succinct_validity::DriverDBClient;

use crate::cli::{Args, Commands, EnvFileArg};

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(arg) = EnvFileArg::try_parse() {
        dotenv::from_filename(arg.env_file).ok();
    } else {
        dotenv::dotenv().ok();
    }

    let args = Args::parse();

    match args.command {
        Commands::List { status } => {
            let client = DriverDBClient::new(&args.database_url).await?;
            let table = commands::list(status, client).await?;

            println!("{table}");
        }
    }

    Ok(())
}
