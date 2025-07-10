//! Process management utilities for running proposer and challenger tasks.
use std::sync::Arc;

use alloy_primitives::Address;
use alloy_provider::ProviderBuilder;
use anyhow::Result;
use fault_proof::{
    challenger::OPSuccinctChallenger, config::ChallengerConfig, contract::DisputeGameFactory,
    proposer::OPSuccinctProposer,
};
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCConfig};
use op_succinct_proof_utils::initialize_host;
use op_succinct_signer_utils::Signer;

/// Start a proposer using the native library implementation
pub async fn start_proposer_native(
    rpc_config: &RPCConfig,
    private_key: &str,
    factory_address: &Address,
    game_type: u32,
) -> Result<tokio::task::JoinHandle<Result<()>>> {
    // Create signer directly from private key
    let signer = Signer::new_local_signer(private_key)?;
    let prover_address = signer.address();

    // Create proposer config with test-specific settings
    let config = fault_proof::config::ProposerConfig {
        l1_rpc: rpc_config.l1_rpc.clone(),
        l2_rpc: rpc_config.l2_rpc.clone(),
        factory_address: factory_address.clone(),
        mock_mode: false,
        fast_finality_mode: false,
        proposal_interval_in_blocks: 10, // Much smaller interval for testing
        fetch_interval: 2,               // Check more frequently in tests
        game_type,
        max_games_to_check_for_defense: 100,
        enable_game_resolution: true,
        max_games_to_check_for_resolution: 100,
        max_games_to_check_for_bond_claiming: 100,
        safe_db_fallback: false,
        metrics_port: 9000,
    };

    let l1_provider = ProviderBuilder::default().connect_http(rpc_config.l1_rpc.clone());
    let factory = DisputeGameFactory::new(factory_address.clone(), l1_provider.clone());

    let init_bond = fault_proof::FactoryTrait::fetch_init_bond(&factory, config.game_type).await?;
    tracing::info!("outer init_bond: {}", init_bond);

    // For testing, we use mock mode, so we use a dummy network private key.
    let network_private_key =
        "0x0000000000000000000000000000000000000000000000000000000000000001".to_string();

    let fetcher = Arc::new(OPSuccinctDataFetcher::new_with_rollup_config().await?);
    let host = initialize_host(fetcher.clone());

    Ok(tokio::spawn(async move {
        let proposer = OPSuccinctProposer::new(
            config,
            network_private_key,
            prover_address,
            signer,
            factory,
            fetcher,
            host,
        )
        .await?;
        Arc::new(proposer).run().await
    }))
}

/// Start a challenger using the native library implementation  
pub async fn start_challenger_native(
    rpc_config: &RPCConfig,
    private_key: &str,
    factory_address: &Address,
    game_type: u32,
    _prover_network_rpc: Option<&str>,
    malicious_percentage: Option<f64>,
) -> Result<tokio::task::JoinHandle<Result<()>>> {
    // Create signer directly from private key
    let signer = Signer::new_local_signer(private_key)?;

    // Create challenger config with test-specific settings
    let config = ChallengerConfig {
        l1_rpc: rpc_config.l1_rpc.clone(),
        l2_rpc: rpc_config.l2_rpc.clone(),
        factory_address: factory_address.clone(),
        fetch_interval: 2, // Check more frequently in tests
        game_type,
        max_games_to_check_for_challenge: 10, // Check more games
        enable_game_resolution: true,
        max_games_to_check_for_resolution: 100,
        max_games_to_check_for_bond_claiming: 100,
        metrics_port: 9001,
        malicious_challenge_percentage: malicious_percentage.unwrap_or(0.0),
    };

    let l1_provider = ProviderBuilder::default().connect_http(rpc_config.l1_rpc.clone());
    let factory = DisputeGameFactory::new(factory_address.clone(), l1_provider.clone());

    Ok(tokio::spawn(async move {
        let mut challenger =
            OPSuccinctChallenger::test(config, l1_provider.clone(), factory, signer).await?;
        challenger.run().await
    }))
}
