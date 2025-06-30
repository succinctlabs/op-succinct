//! Common test utilities for fault-proof E2E tests.

pub mod anvil;
pub mod contracts;

pub use anvil::*;
pub use contracts::{
    configure_contracts, create_test_game, deploy_test_contracts, DeployedContracts,
    CHALLENGER_BOND, DELAY_PERIOD, FALLBACK_TIMEOUT, INIT_BOND, MAX_CHALLENGE_DURATION,
    MAX_PROVE_DURATION, TEST_GAME_TYPE,
};
