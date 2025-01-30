//! This module contains the single-chain mode for the host.
//!
//! This logic is replicating SingleChainHostCli in `kona`. https://github.com/op-rs/kona/tree/main/bin/host/src/single

mod cli;
pub use cli::SingleChainHostCli;

mod orchestrator;

mod eth;

mod local_kv;

mod fetcher;
pub use fetcher::SingleChainFetcher;

use alloy_provider::ReqwestProvider;
use alloy_rpc_client::RpcClient;
use alloy_transport_http::Http;
use reqwest::Client;

/// Returns an HTTP provider for the given URL.
pub fn http_provider(url: &str) -> ReqwestProvider {
    let url = url.parse().unwrap();
    let http = Http::<Client>::new(url);
    ReqwestProvider::new(RpcClient::new(http, true))
}
