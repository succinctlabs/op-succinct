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
    List {
        status: RequestStatus,

        /// The starting block number to filter from
        #[arg(long)]
        from: Option<u64>,

        /// The ending block number to filter to
        #[arg(long)]
        to: Option<u64>,
    },

    /// Split a proof request
    Split {
        /// The id of the proof request to set as failed and split
        #[arg(long)]
        id: u64,

        /// The block number where the proof request will be splitted
        #[arg(long)]
        at: u64,
    },

    /// Join 2 proof requests consecutives ranges into a new proof reques, marking the 2 inputs as
    /// failed
    Join {
        /// The id of the first proof request
        #[arg(long)]
        a: u64,

        /// The id of the second proof request
        #[arg(long)]
        b: u64,
    },

    /// Set a proof request to failed
    Kill {
        /// The id of the proof request to set as failed
        #[arg(long)]
        id: u64,
    },
}
