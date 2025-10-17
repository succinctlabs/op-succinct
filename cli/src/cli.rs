use clap::{Parser, Subcommand};
use op_succinct_validity::RequestStatus;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to environment file
    #[arg(long, default_value = ".env")]
    pub env_file: String,

    /// Database URL
    #[arg(long, env)]
    pub database_url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List proof requests
    List { status: RequestStatus },
}
