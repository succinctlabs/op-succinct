use std::env;

use alloy::{primitives::Address, transports::http::reqwest::Url};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProposerConfig {
    pub l1_rpc: Url,
    pub l2_rpc: Url,
    pub factory_address: Address,
    pub proposal_interval_in_blocks: u64,
    pub fetch_interval: u64,
    pub game_type: u32,
    pub enable_game_resolution: bool,
    pub max_games_to_check_for_resolution: u64,
}

impl ProposerConfig {
    pub fn from_env() -> Result<Self> {
        dotenv::from_filename(".env.proposer").ok();
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            l2_rpc: env::var("L2_RPC")?.parse().expect("L2_RPC not set"),
            factory_address: env::var("FACTORY_ADDRESS")?
                .parse()
                .expect("FACTORY_ADDRESS not set"),
            proposal_interval_in_blocks: env::var("PROPOSAL_INTERVAL_IN_BLOCKS")
                .unwrap_or("1000".to_string())
                .parse()?,
            fetch_interval: env::var("FETCH_INTERVAL")
                .unwrap_or("30".to_string())
                .parse()?,
            game_type: env::var("GAME_TYPE").expect("GAME_TYPE not set").parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or("false".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or("100".to_string())
                .parse()?,
        })
    }
}
