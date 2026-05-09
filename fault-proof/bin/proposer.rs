use std::sync::Arc;

use alloy_provider::ProviderBuilder;
use anyhow::Result;
use clap::Parser;
use fault_proof::{
    config::ProposerConfig,
    contract::{AnchorStateRegistry, DisputeGameFactory},
    prometheus::ProposerGauge,
    proposer::OPSuccinctProposer,
};
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher,
    host::enforce_l1_selection_supported,
    l1_selection::L1BlockSelectionConfig,
    metrics::{init_metrics, MetricsGauge},
    setup_logger,
};
use op_succinct_proof_utils::initialize_host;
use op_succinct_signer_utils::SignerLock;
use tikv_jemallocator::Jemalloc;
use tracing::info;

#[global_allocator]
static ALLOCATOR: Jemalloc = Jemalloc;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = ".env.proposer")]
    env_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    dotenv::from_filename(args.env_file).ok();

    setup_logger();

    let proposer_config = ProposerConfig::from_env()?;
    proposer_config.log();

    // Parse the L1 block selection config first so any user-facing error happens before
    // we touch RPCs or the dispute game factory.
    let l1_selection = L1BlockSelectionConfig::from_env()?;
    info!(
        tag = ?l1_selection.tag,
        confirmations = l1_selection.confirmations,
        is_default = l1_selection.is_default(),
        "L1 block selection configured"
    );

    let proposer_signer = SignerLock::from_env().await?;

    let l1_provider = ProviderBuilder::new().connect_http(proposer_config.l1_rpc.clone());

    let anchor_state_registry = AnchorStateRegistry::new(
        proposer_config.anchor_state_registry_address,
        l1_provider.clone(),
    );

    let factory = DisputeGameFactory::new(proposer_config.factory_address, l1_provider.clone());

    let fetcher =
        OPSuccinctDataFetcher::new_with_rollup_config_and_l1_selection(l1_selection).await?;
    let host = initialize_host(Arc::new(fetcher.clone()));

    enforce_l1_selection_supported(host.as_ref(), &fetcher, l1_selection).await?;

    let proposer = Arc::new(
        OPSuccinctProposer::new(
            proposer_config,
            proposer_signer,
            anchor_state_registry,
            factory,
            Arc::new(fetcher),
            host,
        )
        .await
        .unwrap(),
    );

    // Initialize proposer gauges.
    ProposerGauge::register_all();

    // Initialize metrics exporter.
    init_metrics(&proposer.config.metrics_port);

    // Initialize the metrics gauges.
    ProposerGauge::init_all();

    proposer.run().await.expect("Runs in an infinite loop");

    Ok(())
}
