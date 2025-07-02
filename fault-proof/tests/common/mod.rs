//! Common test utilities for fault-proof E2E tests.

pub mod anvil;
pub mod contracts;
pub mod monitor;
pub mod process;

pub use anvil::*;
pub use contracts::{
    configure_contracts, create_test_game, deploy_test_contracts, DeployedContracts,
    CHALLENGER_BOND, DELAY_PERIOD, FALLBACK_TIMEOUT, INIT_BOND, MAX_CHALLENGE_DURATION,
    MAX_PROVE_DURATION, TEST_GAME_TYPE,
};
pub use monitor::{
    extract_game_address_from_factory_logs, verify_all_bonds_claimed,
    verify_all_resolved_correctly, wait_and_track_games, wait_for_bond_claims, wait_for_challenges,
    wait_for_resolutions, wait_for_single_game, TrackedGame,
};
pub use process::{
    find_binary_path, generate_challenger_env, generate_proposer_env, start_challenger_binary,
    start_proposer_binary, ManagedProcess,
};
