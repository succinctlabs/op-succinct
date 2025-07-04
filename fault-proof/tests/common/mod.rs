//! Common test utilities for fault-proof E2E tests.

pub mod anvil;
pub mod contracts;
pub mod env;
pub mod monitor;
pub mod process;

pub use anvil::*;
pub use contracts::{AIRGAP, MAX_CHALLENGE_DURATION, MAX_PROVE_DURATION, TEST_GAME_TYPE};
pub use env::TestEnvironment;
pub use process::{
    find_binary_path, generate_challenger_env, generate_proposer_env, start_challenger_binary,
    start_proposer_binary,
};
