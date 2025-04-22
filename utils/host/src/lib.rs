pub mod block_range;
mod contract;
pub mod fetcher;
pub mod hosts;
mod proof;
pub mod stats;
pub use contract::*;
pub use proof::*;
pub mod metrics;
pub mod witness_generation;

pub use op_succinct_elfs::{
    AGGREGATION_ELF, CELESTIA_RANGE_ELF_EMBEDDED, EIGENDA_RANGE_ELF_EMBEDDED, RANGE_ELF_BUMP,
    RANGE_ELF_EMBEDDED,
};

/// Get the range ELF depending on the feature flag.
pub fn get_range_elf_embedded() -> &'static [u8] {
    cfg_if::cfg_if! {
        if #[cfg(feature = "celestia")] {
            CELESTIA_RANGE_ELF_EMBEDDED
        } else {
            RANGE_ELF_EMBEDDED
        }
    }
}
