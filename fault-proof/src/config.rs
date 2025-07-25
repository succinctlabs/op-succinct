use std::env;

use alloy_primitives::Address;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ProposerConfig {
    /// The L1 RPC URL.
    pub l1_rpc: Url,

    /// The L2 RPC URL.
    pub l2_rpc: Url,

    /// The address of the factory contract.
    pub factory_address: Address,

    /// Whether to use mock mode.
    pub mock_mode: bool,

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

    /// The maximum number of games to check for bond claiming.
    pub max_games_to_check_for_bond_claiming: u64,

    /// Whether to fallback to timestamp-based L1 head estimation even though SafeDB is not
    /// activated for op-node.
    pub safe_db_fallback: bool,

    /// The metrics port.
    pub metrics_port: u16,
}

impl ProposerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            l2_rpc: env::var("L2_RPC")?.parse().expect("L2_RPC not set"),
            factory_address: env::var("FACTORY_ADDRESS")?.parse().expect("FACTORY_ADDRESS not set"),
            mock_mode: env::var("MOCK_MODE").unwrap_or("false".to_string()).parse()?,
            fast_finality_mode: env::var("FAST_FINALITY_MODE")
                .unwrap_or("false".to_string())
                .parse()?,
            proposal_interval_in_blocks: env::var("PROPOSAL_INTERVAL_IN_BLOCKS")
                .unwrap_or("1800".to_string())
                .parse()?,
            fetch_interval: env::var("FETCH_INTERVAL").unwrap_or("30".to_string()).parse()?,
            game_type: env::var("GAME_TYPE").expect("GAME_TYPE not set").parse()?,
            max_games_to_check_for_defense: env::var("MAX_GAMES_TO_CHECK_FOR_DEFENSE")
                .unwrap_or("100".to_string())
                .parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or("true".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or("100".to_string())
                .parse()?,
            max_games_to_check_for_bond_claiming: env::var("MAX_GAMES_TO_CHECK_FOR_BOND_CLAIMING")
                .unwrap_or("100".to_string())
                .parse()?,
            safe_db_fallback: env::var("SAFE_DB_FALLBACK")
                .unwrap_or("false".to_string())
                .parse()?,
            metrics_port: env::var("PROPOSER_METRICS_PORT")
                .unwrap_or("9000".to_string())
                .parse()?,
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
    /// The challenger will check for challenges up to `max_games_to_check_for_challenge` games
    /// behind the latest game.
    pub max_games_to_check_for_challenge: u64,

    /// Whether to enable game resolution.
    /// When game resolution is not enabled, the challenger will only challenge games.
    pub enable_game_resolution: bool,

    /// The number of games to check for resolution.
    /// When game resolution is enabled, the challenger will attempt to resolve games that are
    /// challenged up to `max_games_to_check_for_resolution` games behind the latest game.
    pub max_games_to_check_for_resolution: u64,

    /// The maximum number of games to check for bond claiming.
    pub max_games_to_check_for_bond_claiming: u64,

    /// The metrics port.
    pub metrics_port: u16,

    /// Percentage (0.0-100.0) of valid games to challenge maliciously for testing.
    /// Set to 0.0 (default) for production use (honest challenging only).
    /// Set to >0.0 for testing defense mechanisms.
    pub malicious_challenge_percentage: f64,
}

impl ChallengerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            l2_rpc: env::var("L2_RPC")?.parse().expect("L2_RPC not set"),
            factory_address: env::var("FACTORY_ADDRESS")?.parse().expect("FACTORY_ADDRESS not set"),
            game_type: env::var("GAME_TYPE").expect("GAME_TYPE not set").parse()?,
            fetch_interval: env::var("FETCH_INTERVAL").unwrap_or("30".to_string()).parse()?,
            max_games_to_check_for_challenge: env::var("MAX_GAMES_TO_CHECK_FOR_CHALLENGE")
                .unwrap_or("100".to_string())
                .parse()?,
            enable_game_resolution: env::var("ENABLE_GAME_RESOLUTION")
                .unwrap_or("true".to_string())
                .parse()?,
            max_games_to_check_for_resolution: env::var("MAX_GAMES_TO_CHECK_FOR_RESOLUTION")
                .unwrap_or("100".to_string())
                .parse()?,
            max_games_to_check_for_bond_claiming: env::var("MAX_GAMES_TO_CHECK_FOR_BOND_CLAIMING")
                .unwrap_or("100".to_string())
                .parse()?,
            metrics_port: env::var("CHALLENGER_METRICS_PORT")
                .unwrap_or("9001".to_string())
                .parse()?,
            malicious_challenge_percentage: env::var("MALICIOUS_CHALLENGE_PERCENTAGE")
                .unwrap_or("0.0".to_string())
                .parse()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// The config for deploying the OPSuccinctFaultDisputeGame.
/// Note: The fields should be in alphabetical order for Solidity to parse it correctly.
pub struct FaultDisputeGameConfig {
    pub aggregation_vkey: String,
    pub challenger_addresses: Vec<String>,
    pub challenger_bond_wei: u64,
    pub dispute_game_finality_delay_seconds: u64,
    pub fallback_timeout_fp_secs: u64,
    pub game_type: u32,
    pub initial_bond_wei: u64,
    pub max_challenge_duration: u64,
    pub max_prove_duration: u64,
    pub optimism_portal2_address: String,
    pub permissionless_mode: bool,
    pub proposer_addresses: Vec<String>,
    pub range_vkey_commitment: String,
    pub rollup_config_hash: String,
    pub starting_l2_block_number: u64,
    pub starting_root: String,
    pub use_sp1_mock_verifier: bool,
    pub verifier_address: String,
}
