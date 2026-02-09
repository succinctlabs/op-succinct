//! Validium Client Utils for OP Succinct
//!
//! Implements the AltDA protocol for validium:
//! - Batcher posts keccak256(batch_data) to L1 as calldata (34 bytes)
//! - Actual batch data stored off-chain
//! - zkVM verifies: keccak256(data) == on-chain commitment
//!
//! L1 calldata format:
//!   [0x01] [0x00] [32 bytes keccak256]
//!    │      │      └── commitment
//!    │      └── type (keccak256)
//!    └── AltDA version byte

pub mod blob_store;
pub mod da_source;
pub mod executor;
pub mod witness;

pub use blob_store::{ValidiumBlobData, ValidiumBlobStore};
pub use da_source::ValidiumDADataSource;
pub use executor::ValidiumWitnessExecutor;
pub use witness::ValidiumWitnessData;
