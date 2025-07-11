use std::{env, str::FromStr, sync::Arc};

use alloy_primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_transport_http::reqwest::Url;
use anyhow::{Context, Result};
use op_succinct_host_utils::{fetcher::OPSuccinctDataFetcher, setup_logger};
use op_succinct_proof_utils::initialize_host;
use op_succinct_signer_utils::Signer;
use tokio::time::Duration;

use fault_proof::{
    contract::{DisputeGameFactory, OPSuccinctFaultDisputeGame, ProposalStatus},
    proposer::OPSuccinctProposer,
    FactoryTrait,
};

#[tokio::test(flavor = "multi_thread")]
async fn test_proposer_defends_successfully() -> Result<()> {
    setup_logger();
    let _span = tracing::info_span!("[[TEST]]").entered();

    dotenv::from_filename(".env.proposer").ok();

    let signer = if let (Some(signer_url), Some(signer_address)) =
        (env::var("SIGNER_URL").ok(), env::var("SIGNER_ADDRESS").ok())
    {
        let signer_url = Url::parse(&signer_url).expect("Failed to parse SIGNER_URL");
        let signer_address =
            Address::from_str(&signer_address).expect("Failed to parse SIGNER_ADDRESS");
        Signer::Web3Signer(signer_url, signer_address)
    } else if let Ok(private_key) = env::var("PRIVATE_KEY") {
        let private_key =
            PrivateKeySigner::from_str(&private_key).expect("Failed to parse PRIVATE_KEY");
        Signer::LocalSigner(private_key)
    } else {
        anyhow::bail!("Neither PRIVATE_KEY nor Web3Signer is set");
    };

    let l1_provider =
        ProviderBuilder::new().connect_http(env::var("L1_RPC").unwrap().parse::<Url>().unwrap());

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")
            .expect("FACTORY_ADDRESS must be set")
            .parse::<Address>()
            .unwrap(),
        l1_provider.clone(),
    );

    let prover_address = env::var("PROVER_ADDRESS")
        .ok()
        .and_then(|addr| addr.parse::<Address>().ok())
        .unwrap_or_else(|| signer.address());

    let fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
    let host = initialize_host(Arc::new(fetcher.clone()));
    let proposer =
        OPSuccinctProposer::new(prover_address, signer, factory.clone(), Arc::new(fetcher), host)
            .await
            .unwrap();
    let game_address = proposer.handle_game_creation().await?.unwrap();

    // Malicious challenger challenging a valid game
    tracing::info!("Malicious challenger challenging a valid game");
    let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider.clone());
    let challenger_bond = factory.fetch_challenger_bond(proposer.config.game_type).await?;
    let challenge_receipt = game
        .challenge()
        .value(challenger_bond)
        .send()
        .await
        .context("Failed to send challenge transaction")?
        .with_required_confirmations(1)
        .with_timeout(Some(Duration::from_secs(60)))
        .get_receipt()
        .await
        .context("Failed to get transaction receipt for challenge")?;
    tracing::info!(
        "\x1b[1mSuccessfully challenged game {:?} with tx {:?}\x1b[0m",
        game_address,
        challenge_receipt.transaction_hash
    );

    // Proposer defending the game with a valid proof
    tracing::info!("Proposer defending the game with a valid proof");
    let tx_hash = proposer.prove_game(game_address).await?;
    tracing::info!(
        "\x1b[1mSuccessfully defended game {:?} with tx {:?}\x1b[0m",
        game_address,
        tx_hash
    );

    let status = game.claimData().call().await?.status;
    assert_eq!(
        status,
        ProposalStatus::ChallengedAndValidProofProvided,
        "Game was not successfully defended"
    );

    Ok(())
}
