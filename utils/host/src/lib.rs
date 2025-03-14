pub mod block_range;
mod contract;
pub mod fetcher;
pub mod hosts;
mod proof;
pub mod stats;
pub use contract::*;
pub use proof::*;

pub const RANGE_ELF_BUMP: &[u8] = include_bytes!("../../../elf/range-elf-bump");
pub const RANGE_ELF_EMBEDDED: &[u8] = include_bytes!("../../../elf/range-elf-embedded");
pub const AGGREGATION_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");
