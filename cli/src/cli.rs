use clap::{Parser, Subcommand};
use op_succinct_validity::RequestStatus;

#[derive(Parser)]
pub struct EnvFileArg {
    /// Path to environment file
    #[arg(long, default_value = ".env")]
    pub env_file: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
    /// Adds files to myapp
    List { status: RequestStatus },
}
