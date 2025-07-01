//! Event monitoring and state tracking utilities for E2E tests.

use alloy_primitives::{Address, FixedBytes, U256};
use alloy_provider::Provider;
use anyhow::Result;
use bindings::{
    dispute_game_factory::DisputeGameFactory,
    op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

/// Represents a tracked game for monitoring
#[derive(Debug, Clone)]
pub struct TrackedGame {
    pub address: Address,
    pub l2_block_number: U256,
    pub output_root: FixedBytes<32>,
    pub created_at_block: u64,
}

/// Wait for N games to be created and return their info
pub async fn wait_and_track_games<P: Provider>(
    factory: &DisputeGameFactory::DisputeGameFactoryInstance<P>,
    game_type: u32,
    count: usize,
    timeout_duration: Duration,
) -> Result<Vec<TrackedGame>> {
    info!("Waiting for {} games to be created...", count);

    let start_time = tokio::time::Instant::now();
    let mut tracked_games = Vec::new();
    let mut last_game_count = U256::ZERO;

    // Get initial game count
    let initial_count = factory.gameCount().call().await?;
    info!("Initial game count: {}", initial_count);

    loop {
        // Check timeout
        if start_time.elapsed() > timeout_duration {
            anyhow::bail!(
                "Timeout waiting for games. Got {} out of {} games",
                tracked_games.len(),
                count
            );
        }

        // Get current game count
        let current_count = factory.gameCount().call().await?;

        // Check for new games
        if current_count > last_game_count {
            for i in last_game_count.to::<u64>()..current_count.to::<u64>() {
                let game_info = factory.gameAtIndex(U256::from(i)).call().await?;

                // Check if it's our game type
                if game_info.gameType_ == game_type {
                    let game =
                        OPSuccinctFaultDisputeGame::new(game_info.proxy_, factory.provider());

                    // Get game details
                    let l2_block_number = game.l2BlockNumber().call().await?;
                    let output_root = game.rootClaim().call().await?;

                    let tracked = TrackedGame {
                        address: game_info.proxy_,
                        l2_block_number,
                        output_root,
                        created_at_block: factory.provider().get_block_number().await?,
                    };

                    info!(
                        "Tracked game {}/{}: {} at L2 block {}",
                        tracked_games.len() + 1,
                        count,
                        tracked.address,
                        tracked.l2_block_number
                    );

                    tracked_games.push(tracked);

                    if tracked_games.len() >= count {
                        return Ok(tracked_games);
                    }
                }
            }

            last_game_count = current_count;
        }

        // Wait before checking again
        sleep(Duration::from_secs(1)).await;
    }
}

/// Wait for a single game to be created
pub async fn wait_for_single_game<P: Provider>(
    factory: &DisputeGameFactory::DisputeGameFactoryInstance<P>,
    game_type: u32,
    timeout_duration: Duration,
) -> Result<Address> {
    let games = wait_and_track_games(factory, game_type, 1, timeout_duration).await?;
    Ok(games[0].address)
}

/// Wait for games to be resolved
pub async fn wait_for_resolutions<P: Provider>(
    provider: &P,
    tracked_games: &[TrackedGame],
    timeout_duration: Duration,
) -> Result<Vec<u8>> {
    info!("Waiting for {} games to be resolved...", tracked_games.len());

    let deadline = tokio::time::Instant::now() + timeout_duration;
    let mut statuses = vec![0u8; tracked_games.len()]; // 0 = IN_PROGRESS

    loop {
        if tokio::time::Instant::now() > deadline {
            anyhow::bail!("Timeout waiting for game resolutions");
        }

        let mut all_resolved = true;

        for (i, game) in tracked_games.iter().enumerate() {
            let game_contract = OPSuccinctFaultDisputeGame::new(game.address, provider);
            let status = game_contract.status().call().await?;

            statuses[i] = status;

            if status == 0 {
                // 0 = IN_PROGRESS
                all_resolved = false;
            } else {
                info!("Game {} resolved with status: {}", game.address, status);
            }
        }

        if all_resolved {
            return Ok(statuses);
        }

        sleep(Duration::from_secs(2)).await;
    }
}

/// Wait for challenges to be submitted to games
pub async fn wait_for_challenges<P: Provider>(
    provider: &P,
    game_addresses: &[Address],
    timeout_duration: Duration,
) -> Result<Vec<u8>> {
    info!("Waiting for challenges on {} games...", game_addresses.len());

    let deadline = tokio::time::Instant::now() + timeout_duration;
    let mut statuses = vec![0u8; game_addresses.len()]; // 0 = Unchallenged

    loop {
        if tokio::time::Instant::now() > deadline {
            anyhow::bail!("Timeout waiting for challenges");
        }

        let mut all_challenged = true;

        for (i, &game_address) in game_addresses.iter().enumerate() {
            let game = OPSuccinctFaultDisputeGame::new(game_address, provider);
            let claim_data = game.claimData().call().await?;

            statuses[i] = claim_data.status;

            if claim_data.status == 0 {
                // 0 = Unchallenged
                all_challenged = false;
            } else {
                info!("Game {} status: {}", game_address, claim_data.status);
            }
        }

        if all_challenged {
            return Ok(statuses);
        }

        sleep(Duration::from_secs(2)).await;
    }
}

/// Wait for a specific proposal status on a game
pub async fn wait_for_challenge_status<P: Provider>(
    game: &OPSuccinctFaultDisputeGame::OPSuccinctFaultDisputeGameInstance<P>,
    expected_status: u8,
    timeout_duration: Duration,
) -> Result<()> {
    info!("Waiting for game to reach status: {}", expected_status);

    let deadline = tokio::time::Instant::now() + timeout_duration;

    loop {
        if tokio::time::Instant::now() > deadline {
            anyhow::bail!("Timeout waiting for status {}", expected_status);
        }

        let claim_data = game.claimData().call().await?;

        if claim_data.status == expected_status {
            info!("Game reached expected status: {}", expected_status);
            return Ok(());
        }

        sleep(Duration::from_secs(2)).await;
    }
}

/// Wait for bond claims to be made
pub async fn wait_for_bond_claims<P: Provider>(
    provider: &P,
    tracked_games: &[TrackedGame],
    timeout_duration: Duration,
) -> Result<Vec<bool>> {
    info!("Waiting for bond claims on {} games...", tracked_games.len());

    let deadline = tokio::time::Instant::now() + timeout_duration;
    let mut claims = vec![false; tracked_games.len()];

    loop {
        if tokio::time::Instant::now() > deadline {
            anyhow::bail!("Timeout waiting for bond claims");
        }

        let mut all_claimed = true;

        for (i, game) in tracked_games.iter().enumerate() {
            // Check if bonds have been claimed by looking at contract balance
            // In production, you'd check specific claim events or state
            let balance = provider.get_balance(game.address).await?;

            // If balance is 0, bonds have likely been claimed
            if balance == U256::ZERO {
                claims[i] = true;
                info!("Bonds claimed for game {}", game.address);
            } else {
                all_claimed = false;
            }
        }

        if all_claimed {
            return Ok(claims);
        }

        sleep(Duration::from_secs(2)).await;
    }
}

/// Extract game address from factory creation receipt
pub fn extract_game_address_from_receipt(
    _receipt: &alloy_rpc_types_eth::Receipt,
) -> Result<Address> {
    // In a real implementation, you'd parse the DisputeGameCreated event
    // For now, return a placeholder
    anyhow::bail!("Game address extraction from receipt not implemented yet")
}

/// Verify all games resolved correctly (proposer wins)
pub fn verify_all_resolved_correctly(statuses: &[u8]) -> Result<()> {
    for (i, status) in statuses.iter().enumerate() {
        if *status != 2 {
            // 2 = DEFENDER_WINS
            anyhow::bail!("Game {} did not resolve to ProposerWins: {}", i, status);
        }
    }
    info!("All {} games resolved correctly (ProposerWins)", statuses.len());
    Ok(())
}

/// Verify all bonds were claimed
pub fn verify_all_bonds_claimed(claims: &[bool]) -> Result<()> {
    for (i, claimed) in claims.iter().enumerate() {
        if !claimed {
            anyhow::bail!("Game {} bonds were not claimed", i);
        }
    }
    info!("All {} games had bonds claimed", claims.len());
    Ok(())
}
