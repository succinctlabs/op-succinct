pub mod block_range;
mod contract;
pub mod fetcher;
pub mod hosts;
mod proof;
pub mod stats;
pub use contract::*;
pub use proof::*;
pub mod metrics;

use clap::{Parser, ValueEnum};
use strum_macros::EnumString;
pub use op_succinct_elfs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Parser, ValueEnum, EnumString)]
/// The configuration for the DA provider.
pub enum DAConfig {
    /// The default DA configuration.
    Default,
    /// The Celestia DA configuration.
    Celestia,
    /// The EigenDA DA configuration.
    EigenDA,
}
