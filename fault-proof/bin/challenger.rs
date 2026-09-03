use std::{env, sync::Arc, time::Duration};

use alloy_primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::{Client, Url};
use anyhow::Result;
use clap::Parser;
use fault_proof::{
    challenger::OPSuccinctChallenger,
    config::{ChallengerConfig, OPStackGameValidatorConfig},
    contract::{AnchorStateRegistry, DisputeGameFactory},
    op_stack_game_validator::OPStackGameValidator,
    prometheus::ChallengerGauge,
    TIMEOUT_SECONDS,
};
use op_succinct_host_utils::{
    metrics::{init_metrics, MetricsGauge},
    setup_logger,
};
use op_succinct_signer_utils::SignerLock;
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

fn build_http_client(timeout: Duration) -> Result<Client> {
    Ok(Client::builder().timeout(timeout).build()?)
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = ".env.challenger")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    dotenv::from_filename(args.env_file).ok();

    setup_logger();

    let challenger_config = ChallengerConfig::from_env()?;
    challenger_config.log();
    let validator_config = OPStackGameValidatorConfig::from_env()?;
    validator_config.log();

    let challenger_signer = SignerLock::from_env().await?;

    // Use one finite-timeout HTTP client for all RPC providers so one stalled backend cannot
    // block the serial challenger sync loop indefinitely.
    let http_client = build_http_client(Duration::from_secs(TIMEOUT_SECONDS))?;
    let l1_provider = ProviderBuilder::default()
        .connect_reqwest(http_client.clone(), challenger_config.l1_rpc.clone());

    let anchor_state_registry = AnchorStateRegistry::new(
        env::var("ANCHOR_STATE_REGISTRY_ADDRESS")
            .expect("ANCHOR_STATE_REGISTRY_ADDRESS must be set")
            .parse::<Address>()
            .unwrap(),
        l1_provider.clone(),
    );

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")
            .expect("FACTORY_ADDRESS must be set")
            .parse::<Address>()
            .unwrap(),
        l1_provider.clone(),
    );

    let game_validator = Arc::new(OPStackGameValidator::new(
        l1_provider.clone(),
        ProviderBuilder::default().connect_reqwest(http_client.clone(), validator_config.l2_rpc),
        ProviderBuilder::default().connect_reqwest(http_client, validator_config.l2_node_rpc),
    ));

    let mut challenger = OPSuccinctChallenger::new_with_game_validator(
        challenger_config,
        l1_provider,
        anchor_state_registry,
        factory,
        challenger_signer,
        game_validator,
    );

    // Initialize challenger gauges.
    ChallengerGauge::register_all();

    // Initialize metrics exporter.
    init_metrics(&challenger.config.metrics_port);

    // Initialize the metrics gauges.
    ChallengerGauge::init_all();

    challenger.run().await.expect("Runs in an infinite loop");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_provider::{Provider, RootProvider};
    use tokio::{io::AsyncReadExt, net::TcpListener};

    #[tokio::test]
    async fn configured_http_client_times_out_stalled_rpc_request() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0; 1024];
            let _ = stream.read(&mut request).await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        });

        let provider: RootProvider = ProviderBuilder::default().connect_reqwest(
            build_http_client(Duration::from_millis(10)).unwrap(),
            Url::parse(&format!("http://{address}")).unwrap(),
        );
        let result =
            tokio::time::timeout(Duration::from_millis(50), provider.get_block_number()).await;

        assert!(matches!(result, Ok(Err(_))), "a stalled RPC request must time out");
        server.await.unwrap();
    }
}
