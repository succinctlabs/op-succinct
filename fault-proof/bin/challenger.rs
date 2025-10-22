use std::env;

use alloy_primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::reqwest::Url;
use anyhow::Result;
use clap::Parser;
use fault_proof::{
    challenger::OPSuccinctChallenger, challenger::OPSuccinctChallenger, contract::DisputeGameFactory, prometheus::ChallengerGauge,
};
use op_succinct_host_utils::{
    metrics::{init_metrics, MetricsGauge},
    setup_logger,
};
use op_succinct_signer_utils::Signer;
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

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

    setup_logger();

    let challenger_config = ChallengerConfig::from_env()?;
    let challenger_signer = Signer::from_env().await?;

    let l1_provider = ProviderBuilder::default()
        .connect_http(env::var("L1_RPC").unwrap().parse::<Url>().unwrap());

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")
            .expect("FACTORY_ADDRESS must be set")
            .parse::<Address>()
            .unwrap(),
        l1_provider.clone(),
    );

    let mut challenger =
<<<<<<< HEAD
        OPSuccinctChallenger::from_env(l1_provider, factory, challenger_signer).await.unwrap();
||||||| ae1b78c
        OPSuccinctChallenger::from_env(
            challenger_signer.address(),
            challenger_signer,
            l1_provider,
            factory,
        ).await.unwrap();
=======
        OPSuccinctChallenger::from_env(challenger_config, l1_provider, factory, challenger_signer).await.unwrap();
>>>>>>> upstream/main

    // Initialize challenger gauges.
    ChallengerGauge::register_all();

    // Initialize metrics exporter.
    init_metrics(&challenger.config.metrics_port);

    // Initialize the metrics gauges.
    ChallengerGauge::init_all();

    challenger.run().await.expect("Runs in an infinite loop");

    Ok(())
}
