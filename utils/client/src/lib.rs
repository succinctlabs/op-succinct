#![feature(trivial_bounds)]

mod hasher;
pub use hasher::BytesHasherBuilder;

pub mod boot;
pub use boot::{BootInfoWithBytesConfig, AGGREGATION_OUTPUTS_SIZE};

mod oracle;
pub use oracle::{InMemoryOracle, StoreOracle};

pub mod precompiles;

pub mod types;

extern crate alloc;

pub mod client;
