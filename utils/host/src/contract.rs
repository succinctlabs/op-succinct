//! Contract bindings for the host utilities.
//!
//! Contract types are imported from forge-generated bindings in `op-succinct-bindings`,
//! which are the source of truth for ABI correctness.
//!
//! Only types that are not generated (external contracts like SP1Blobstream) are kept as
//! hand-written `sol!` definitions.

use alloy_sol_types::sol;

// Re-export contract modules from forge-generated bindings.
pub use op_succinct_bindings::{
    dispute_game_factory::DisputeGameFactory,
    op_succinct_l2_output_oracle::OPSuccinctL2OutputOracle, L2Output,
};

sol! {
    #[sol(rpc)]
    contract SP1Blobstream {
        uint64 public latestBlock;
    }
}
