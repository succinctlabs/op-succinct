//! Validium Host Utils for OP Succinct
//!
//! Uses the AltDA protocol:
//! - L1 preimages collected same as Ethereum DA
//! - Batch data fetched from op-alt-da server
//! - Packages both into ValidiumWitnessData for the zkVM

pub mod da_client;
pub mod host;
pub mod witness_generator;

pub use da_client::AltDAClient;
pub use host::ValidiumOPSuccinctHost;
pub use witness_generator::{create_validium_stdin, ValidiumWitnessGenerator};

// Re-export client types.
pub use op_succinct_validium_client_utils::{
    ValidiumBlobData, ValidiumBlobStore, ValidiumDADataSource, ValidiumWitnessData,
    ValidiumWitnessExecutor,
};
