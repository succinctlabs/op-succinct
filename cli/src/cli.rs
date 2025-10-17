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
#[command(flatten_help = true)]
pub enum Commands {
    /// List proof requests
    List { status: RequestStatus },

    /// Split a proof request
    Split {
        /// The id of the proof request to set as failed and split
        #[arg(long)]
        id: u64,

        /// The block number where the proof request will be splitted
        #[arg(long)]
        at: u64,
    },
}
