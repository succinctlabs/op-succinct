#![allow(clippy::all)]
//! This lib re-exports the contract bindings.

#[cfg_attr(rustfmt, rustfmt_skip)]
mod codegen;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub use codegen::*;
