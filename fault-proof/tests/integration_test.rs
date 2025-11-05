pub mod common;

use std::{sync::Arc, time::Duration};

use alloy_primitives::U256;
use anyhow::Result;
use common::{
    constants::{
        DISPUTE_GAME_FINALITY_DELAY_SECONDS, MAX_CHALLENGE_DURATION, PROPOSER_ADDRESS,
        PROPOSER_PRIVATE_KEY, TEST_GAME_TYPE,
    },
    monitor::{
        verify_all_resolved_correctly, wait_and_track_games, wait_for_bond_claims,
        wait_for_resolutions,
    },
    process::init_proposer,
    warp_time, TestEnvironment,
};
use op_succinct_bindings::dispute_game_factory::DisputeGameFactory;
use tokio::time::sleep;

#[tokio::test(flavor = "multi_thread")]
async fn proposer_retains_anchor_after_bond_claim() -> Result<()> {
    TestEnvironment::init_logging();
    let env = TestEnvironment::setup().await?;

    let proposer = Arc::new(
        init_proposer(&env.rpc_config, PROPOSER_PRIVATE_KEY, &env.deployed.factory, TEST_GAME_TYPE)
            .await?,
    );

    let proposer_handle = {
        let proposer_clone = proposer.clone();
        tokio::spawn(async move { proposer_clone.run().await })
    };

    let factory = DisputeGameFactory::new(env.deployed.factory, env.anvil.provider.clone());

    let tracked_games =
        wait_and_track_games(&factory, TEST_GAME_TYPE, 3, Duration::from_secs(120)).await?;

    warp_time(&env.anvil.provider, Duration::from_secs(MAX_CHALLENGE_DURATION)).await?;

    let resolutions =
        wait_for_resolutions(&env.anvil.provider, &tracked_games, Duration::from_secs(120)).await?;
    verify_all_resolved_correctly(&resolutions)?;

    warp_time(&env.anvil.provider, Duration::from_secs(DISPUTE_GAME_FINALITY_DELAY_SECONDS))
        .await?;

    wait_for_bond_claims(
        &env.anvil.provider,
        &tracked_games,
        PROPOSER_ADDRESS,
        Duration::from_secs(120),
    )
    .await?;

    // Allow the proposer loop to observe the finalized games and update its cache.
    let settle_delay = Duration::from_secs(proposer.config.fetch_interval + 5);
    sleep(settle_delay).await;

    let snapshot = proposer.state_snapshot().await;

    // The third game with index 2 should be the anchor game.
    let expected_anchor_index = U256::from(2);
    assert_eq!(snapshot.anchor_index, Some(expected_anchor_index));

    // Verify the anchor game is present in the games cache.
    let _snapshot_anchor_game = snapshot
        .games
        .iter()
        .find(|(index, _)| *index == expected_anchor_index)
        .expect("anchor game not found in snapshot");

    proposer_handle.abort();
    let _ = proposer_handle.await;

    Ok(())
}
