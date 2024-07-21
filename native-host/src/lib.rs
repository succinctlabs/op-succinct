use anyhow::Result;
pub use kona_host::init_tracing_subscriber;
use kona_host::{start_server_and_native_client, HostCli};

pub async fn run_native_host(cfg: &HostCli) -> Result<()> {
    println!("cfg.data_dir: {:?}", cfg.data_dir);
    start_server_and_native_client(cfg.clone()).await.unwrap();
    Ok(())
}
