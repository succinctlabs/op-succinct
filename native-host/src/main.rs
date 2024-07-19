use clap::Parser;
use kona_host::{init_tracing_subscriber, HostCli};
use native_host::run_native_host;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cfg = HostCli::parse();
    init_tracing_subscriber(cfg.v).unwrap();
    run_native_host(&cfg).await.unwrap();
}
