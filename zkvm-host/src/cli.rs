use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SP1KonaCliArgs {
    #[arg(long)]
    pub l2_claim_block: u64,

    #[arg(long)]
    pub run_native: bool,

    #[arg(short, long, default_value = "4")]
    pub verbosity_level: u8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MultiblockCliArgs {
    #[arg(long)]
    pub start_block: u64,

    #[arg(long)]
    pub end_block: u64,

    #[arg(long)]
    pub run_native: bool,

    #[arg(short, long, default_value = "4")]
    pub verbosity_level: u8,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CostEstimatorCliArgs {
    /// Start block number.
    #[arg(short, long)]
    pub start_block: u64,

    /// End block number.
    #[arg(short, long)]
    pub end_block: u64,

    /// RPC URL for the OP Stack Chain to do cost estimation for.
    #[arg(short, long)]
    pub rpc_url: String,

    /// Skip native data generation if data directory already exists.
    #[arg(
        long,
        help = "Skip native data generation if the Merkle tree data is already stored in data."
    )]
    pub skip_datagen: bool,

    /// Verbosity level.
    #[arg(short, long, default_value = "0")]
    pub verbosity_level: u8,
}
