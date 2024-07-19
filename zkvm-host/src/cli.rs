use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ZKVMHostCliArgs {
    /// L2 block number.
    #[arg(short, long)]
    pub block: u64,

    /// Run native data generation if data directory doesn't yet exist.
    #[arg(short, long)]
    pub run_native: bool,
}
