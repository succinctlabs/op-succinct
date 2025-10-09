#[macro_use]
extern crate tracing;

mod signer;
pub use signer::{init_client, GcpSigner, GcpSignerError};

pub use gcloud_sdk;
