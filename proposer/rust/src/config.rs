use clap::Parser;
use ethers::types::Address;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// L1 Ethereum RPC URL
    #[arg(long, env = "L1_RPC")]
    pub l1_eth_rpc: String,

    /// L1 Beacon RPC URL
    #[arg(long, env = "L1_BEACON_RPC")]
    pub beacon_rpc: String,

    /// L2 Node RPC URL
    #[arg(long, env = "L2_RPC")]
    pub rollup_rpc: String,

    /// L2OutputOracle contract address
    #[arg(long, env = "L2OO_ADDRESS")]
    pub l2oo_address: Option<Address>,

    /// DisputeGameFactory contract address
    #[arg(long, env = "DGF_ADDRESS")]
    pub dgf_address: Option<Address>,

    /// Poll interval for checking new blocks
    #[arg(long, env = "POLL_INTERVAL", default_value = "12")]
    pub poll_interval: u64,

    /// Database path
    #[arg(long, env = "DB_PATH", default_value = "./op-proposer")]
    pub db_path: String,

    /// Maximum block range per span proof
    #[arg(long, env = "MAX_BLOCK_RANGE_PER_SPAN_PROOF", default_value = "300")]
    pub max_block_range_per_span_proof: u64,

    /// Maximum concurrent witness generation
    #[arg(long, env = "MAX_CONCURRENT_WITNESS_GEN", default_value = "5")]
    pub max_concurrent_witness_gen: u64,

    /// OP Succinct server URL
    #[arg(
        long,
        env = "OP_SUCCINCT_SERVER_URL",
        default_value = "http://127.0.0.1:3000"
    )]
    pub op_succinct_server_url: String,

    /// Private key for transactions
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
}

impl Config {
    pub fn validate(&self) -> eyre::Result<()> {
        if self.l2oo_address.is_none() && self.dgf_address.is_none() {
            return Err(eyre::eyre!(
                "Either L2OutputOracle or DisputeGameFactory address must be provided"
            ));
        }
        Ok(())
    }
}
