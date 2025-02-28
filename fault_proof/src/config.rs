use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use alloy_primitives::Address;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ProposerConfig {
    /// The L1 RPC URL.
    pub l1_rpc: Url,

    /// The L2 RPC URL.
    pub l2_rpc: Url,

    /// The address of the factory contract.
    pub factory_address: Address,

    /// Whether to use fast finality mode.
    pub fast_finality_mode: bool,

    /// The interval in blocks between proposing new games.
    pub proposal_interval_in_blocks: u64,

    /// The interval in seconds between checking for new proposals and game resolution.
    /// During each interval, the proposer:
    /// 1. Checks the safe L2 head block number
    /// 2. Gets the latest valid proposal
    /// 3. Creates a new game if conditions are met
    /// 4. Optionally attempts to resolve unchallenged games
    pub fetch_interval: u64,

    /// The type of game to propose.
    pub game_type: u32,

    /// The number of games to check for defense.
    pub max_games_to_check_for_defense: u64,

    /// Whether to enable game resolution.
    /// When game resolution is not enabled, the proposer will only propose new games.
    pub enable_game_resolution: bool,

    /// The number of games to check for resolution.
    /// When game resolution is enabled, the proposer will attempt to resolve games that are
    /// unchallenged up to `max_games_to_check_for_resolution` games behind the latest game.
    pub max_games_to_check_for_resolution: u64,

    /// The address to expose Prometheus metrics on.
    /// Default is 0.0.0.0:7300
    pub metrics_addr: Option<SocketAddr>,
}

impl ProposerConfig {
    pub fn from_env() -> Result<Self> {
        dotenv::from_filename(".env.proposer").ok();

        let metrics_addr = env::var("METRICS_ADDR")
            .ok()
            .map(|addr| SocketAddr::from_str(&addr))
            .transpose()?;

        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse()?,
            l2_rpc: env::var("L2_RPC")?.parse()?,
            factory_address: env::var("FACTORY_ADDRESS")?.parse()?,
            fast_finality_mode: env::var("FAST_FINALITY_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()?,
            proposal_interval_in_blocks: env::var("PROPOSAL_INTERVAL_IN_BLOCKS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()?,
            fetch_interval: env::var("FETCH_INTERVAL")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            game_type: env::var("GAME_TYPE")
                .unwrap_or_else(|_| "0".to_string())
                .parse()?,
            max_games_to_check_for_defense: env::var("MAX_GAMES_TO_CHECK_FOR_DEFENSE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            metrics_addr,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChallengerConfig {
    pub l1_rpc: Url,
    pub l2_rpc: Url,
    pub factory_address: Address,

    /// The interval in seconds between checking for new challenges opportunities.
    pub fetch_interval: u64,

    /// The game type to challenge.
    pub game_type: u32,

    /// The number of games to check for challenges.
    /// The challenger will check for challenges up to `max_games_to_check_for_challenge` games behind the latest game.
    pub max_games_to_check_for_challenge: u64,

    /// Whether to enable game resolution.
    /// When game resolution is not enabled, the challenger will only challenge games.
    pub enable_game_resolution: bool,

    /// The number of games to check for resolution.
    /// When game resolution is enabled, the challenger will attempt to resolve games that are
    /// challenged up to `max_games_to_check_for_resolution` games behind the latest game.
    pub max_games_to_check_for_resolution: u64,

    /// The address to expose Prometheus metrics on.
    /// Default is 0.0.0.0:7301
    pub metrics_addr: Option<SocketAddr>,
}

impl ChallengerConfig {
    pub fn from_env() -> Result<Self> {
        dotenv::from_filename(".env.challenger").ok();

        let metrics_addr = env::var("METRICS_ADDR")
            .ok()
            .map(|addr| SocketAddr::from_str(&addr))
            .transpose()?;

        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse()?,
            l2_rpc: env::var("L2_RPC")?.parse()?,
            factory_address: env::var("FACTORY_ADDRESS")?.parse()?,
            game_type: env::var("GAME_TYPE")?.parse()?,
            fetch_interval: env::var("FETCH_INTERVAL")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            max_games_to_check_for_challenge: env::var("MAX_GAMES_TO_CHECK_FOR_CHALLENGE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
            metrics_addr,
        })
    }
}
