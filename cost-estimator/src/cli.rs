use clap::Parser;

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
    pub l2_rpc_url: String,

    /// Skip native data generation if data directory already exists.
    #[arg(
        short,
        long,
        help = "Skip native data generation if the Merkle tree data is already stored in data."
    )]
    pub skip_datagen: bool,
}
