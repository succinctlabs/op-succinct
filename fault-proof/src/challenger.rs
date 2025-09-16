use std::time::Duration;

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, U256};
use alloy_provider::{Provider, ProviderBuilder};
use anyhow::Result;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::time;

use crate::{
    config::ChallengerConfig,
    contract::{
        DisputeGameFactory::DisputeGameFactoryInstance, OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    prometheus::ChallengerGauge,
    Action, FactoryTrait, L1Provider, L2Provider, L2ProviderTrait, Mode,
};
use op_succinct_host_utils::metrics::MetricsGauge;
use op_succinct_signer_utils::Signer;

pub struct OPSuccinctChallenger<P>
where
    P: Provider + Clone,
{
    pub config: ChallengerConfig,
    challenger_address: Address,
    signer: Signer,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    factory: DisputeGameFactoryInstance<P>,
    challenger_bond: U256,
    // In-memory scan cursor: last latest index we scanned up to.
    scan_cursor: Option<U256>,
}

impl<P> OPSuccinctChallenger<P>
where
    P: Provider + Clone,
{
    /// Creates a new challenger instance with the provided L1 provider with wallet and factory
    /// contract instance.
    pub async fn from_env(
        l1_provider: L1Provider,
        factory: DisputeGameFactoryInstance<P>,
        signer: Signer,
    ) -> Result<Self> {
        let config = ChallengerConfig::from_env()?;

        Self::new_with_config(config, l1_provider, factory, signer).await
    }

    /// Creates a new challenger instance for testing with provided configuration.
    pub async fn new_with_config(
        config: ChallengerConfig,
        l1_provider: L1Provider,
        factory: DisputeGameFactoryInstance<P>,
        signer: Signer,
    ) -> Result<Self> {
        let challenger_address = signer.address();
        let challenger_bond = factory.fetch_challenger_bond(config.game_type).await?;
        let l2_rpc = config.l2_rpc.clone();

        Ok(OPSuccinctChallenger {
            config,
            challenger_address,
            signer,
            l1_provider: l1_provider.clone(),
            l2_provider: ProviderBuilder::default().connect_http(l2_rpc),
            factory,
            challenger_bond,
            scan_cursor: None,
        })
    }

    /// Challenges a specific game at the given address.
    async fn challenge_game(&self, game_address: Address) -> Result<()> {
        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

        let transaction_request =
            game.challenge().value(self.challenger_bond).into_transaction_request();

        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        tracing::info!(
            "Successfully challenged game {:?} with tx {:?}",
            game_address,
            receipt.transaction_hash
        );

        // Increment metrics on successful challenge
        ChallengerGauge::GamesChallenged.increment(1.0);

        Ok(())
    }

    /// Get the current L1 timestamp (latest block).
    async fn l1_now(&self) -> Result<u64> {
        let now = self
            .l1_provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .unwrap()
            .header
            .timestamp;
        Ok(now)
    }

    /// Scan newly appended games and challenge invalid ones.
    ///
    /// - Filters to OP Succinct game type.
    /// - Considers only in-progress and unexpired (deadline > now) unchallenged games.
    /// - Computes local output root and challenges on mismatch.
    async fn scan_and_challenge(&mut self) -> Result<Action> {
        // Fetch latest index and current L1 time.
        let Some(latest_index) = self.factory.fetch_latest_game_index().await? else {
            tracing::debug!("No games exist yet, skipping challenging");
            return Ok(Action::Skipped);
        };
        let now = self.l1_now().await?;

        ChallengerGauge::LatestIndex.set(latest_index.to::<u64>() as f64);

        // Determine lower bound for scanning on initial run by finding the first OP Succinct
        // fault dispute game whose deadline has passed and is still in progress (not resolved).
        let mut boundary_index: Option<U256> = None;
        if self.scan_cursor.is_none() {
            let mut i = latest_index;
            loop {
                let game = self.factory.gameAtIndex(i).call().await?;
                if game.gameType == self.config.game_type {
                    let game_addr = game.proxy;
                    let game = OPSuccinctFaultDisputeGame::new(game_addr, self.l1_provider.clone());
                    let claim = game.claimData().call().await?;
                    let deadline = U256::from(claim.deadline).to::<u64>();
                    if claim.status != ProposalStatus::Resolved && deadline < now {
                        boundary_index = Some(i);
                        break;
                    }
                }
                if i == U256::ZERO {
                    break;
                }
                i -= U256::from(1);
            }
        }

        // Define scan range lower bound (exclusive). None => scan all the way to index 0.
        let lower_exclusive_opt = match (self.scan_cursor, boundary_index) {
            (Some(c), _) => Some(c),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        let mut performed = false;
        let mut oldest_scanned: Option<U256> = None;

        let mut i = latest_index;
        loop {
            if let Some(lower_exclusive) = lower_exclusive_opt {
                if i <= lower_exclusive {
                    break;
                }
            }
            let game = self.factory.gameAtIndex(i).call().await?;
            // Filter only OP Succinct fault dispute games
            if game.gameType != self.config.game_type {
                if i == U256::ZERO {
                    break;
                }
                i -= U256::from(1);
                continue;
            }

            let game_addr = game.proxy;
            let game = OPSuccinctFaultDisputeGame::new(game_addr, self.l1_provider.clone());
            let claim = game.claimData().call().await?;

            // Only consider in-progress unchallenged games
            if claim.status != ProposalStatus::Unchallenged {
                tracing::debug!(
                    game_index = %i,
                    status = ?claim.status,
                    "Skipping game since not in-progress unchallenged"
                );
                if i == U256::ZERO {
                    break;
                }
                i -= U256::from(1);
                continue;
            }

            // Skip expired challenges
            let deadline = U256::from(claim.deadline).to::<u64>();
            if deadline <= now {
                tracing::debug!(
                    game_index = %i,
                    deadline = %claim.deadline,
                    now = %now,
                    "Skipping game due to expired challenge window"
                );
                if i == U256::ZERO {
                    break;
                }
                i -= U256::from(1);
                continue;
            }

            // Compute expected output root and compare
            let l2_block_number = game.l2BlockNumber().call().await?;
            let expected = self.l2_provider.compute_output_root_at_block(l2_block_number).await?;
            let proposed = game.rootClaim().call().await?;

            oldest_scanned = Some(i);

            if expected != proposed {
                tracing::info!(
                    "\x1b[32m[CHALLENGE]\x1b[0m Attempting to challenge invalid game {:?} at index {}",
                    game_addr,
                    i
                );
                self.challenge_game(game_addr).await?;
                performed = true;
            } else {
                tracing::debug!(
                    game_index = %i,
                    l2_block = %l2_block_number,
                    "Valid game detected (no challenge)"
                );
            }

            if i == U256::ZERO {
                break;
            }
            i -= U256::from(1);
        }

        // Update cursor to the latest index scanned up to.
        self.scan_cursor = Some(latest_index);
        ChallengerGauge::CursorIndex.set(latest_index.to::<u64>() as f64);
        if let Some(oldest) = oldest_scanned {
            ChallengerGauge::OldestIndexScanned.set(oldest.to::<u64>() as f64);
        }

        Ok(if performed { Action::Performed } else { Action::Skipped })
    }

    /// Handles challenging of invalid games by scanning recent games for potential challenges.
    /// Also supports malicious challenging of valid games for testing defense mechanisms when
    /// configured.
    #[tracing::instrument(skip(self), level = "info", name = "[[Challenging]]")]
    async fn handle_game_challenging(&mut self) -> Result<Action> {
        // Challenge invalid games (honest challenger behavior)
        let action = self.scan_and_challenge().await?;
        if matches!(action, Action::Performed) {
            return Ok(action);
        }

        // Maliciously challenge valid games (if configured for testing defense mechanisms)
        if self.config.malicious_challenge_percentage > 0.0 {
            if let Some(index) = self.scan_cursor {
                tracing::debug!("Checking scan_cursor index for malicious challenge...");
                let game = self.factory.gameAtIndex(index).call().await?;
                if game.gameType == self.config.game_type {
                    let now = self.l1_now().await?;
                    let game_addr = game.proxy;
                    let game = OPSuccinctFaultDisputeGame::new(game_addr, self.l1_provider.clone());
                    let claim = game.claimData().call().await?;
                    if claim.status == ProposalStatus::Unchallenged {
                        let deadline = U256::from(claim.deadline).to::<u64>();
                        if deadline > now {
                            let l2_block_number = game.l2BlockNumber().call().await?;
                            let expected = self
                                .l2_provider
                                .compute_output_root_at_block(l2_block_number)
                                .await?;
                            let proposed = game.rootClaim().call().await?;
                            if expected == proposed {
                                let mut rng = StdRng::from_os_rng();
                                let should_challenge: f64 = rng.random_range(0.0..100.0);
                                if should_challenge <= self.config.malicious_challenge_percentage {
                                    tracing::warn!(
                                        "\x1b[31m[MALICIOUS CHALLENGE]\x1b[0m Attempting to challenge valid game {:?} for testing ({}% chance)",
                                        game_addr,
                                        self.config.malicious_challenge_percentage
                                    );
                                    self.challenge_game(game_addr).await?;
                                    return Ok(Action::Performed);
                                } else {
                                    tracing::debug!(
                                        "Cursor game {:?} valid but skipping malicious challenge ({}% chance)",
                                        game_addr,
                                        self.config.malicious_challenge_percentage
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                tracing::debug!("Skipping malicious challenge: scan_cursor not initialized yet");
            }
        }

        Ok(Action::Skipped)
    }

    /// Handles resolution of challenged games that are ready to be resolved.
    #[tracing::instrument(skip(self), level = "info", name = "[[Resolving]]")]
    async fn handle_game_resolution(&self) -> Result<()> {
        self.factory
            .resolve_games(
                Mode::Challenger,
                self.config.max_games_to_check_for_resolution,
                self.signer.clone(),
                self.config.l1_rpc.clone(),
                self.l1_provider.clone(),
            )
            .await
    }

    /// Handles claiming bonds from resolved games.
    #[tracing::instrument(skip(self), level = "info", name = "[[Claiming Challenger Bonds]]")]
    pub async fn handle_bond_claiming(&self) -> Result<Action> {
        if let Some(game_address) = self
            .factory
            .get_oldest_claimable_bond_game_address(
                self.config.game_type,
                self.config.max_games_to_check_for_bond_claiming,
                self.challenger_address,
                Mode::Challenger,
            )
            .await?
        {
            tracing::info!(
                "Attempting to claim bond from game {:?} where challenger won",
                game_address
            );

            // Create a contract instance for the game
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

            // Create a transaction to claim credit
            let transaction_request =
                game.claimCredit(self.challenger_address).gas(200_000).into_transaction_request();

            match self
                .signer
                .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
                .await
            {
                Ok(receipt) => {
                    tracing::info!(
                        "\x1b[1mSuccessfully claimed challenger bond from game {:?} with tx {:?}\x1b[0m",
                        game_address,
                        receipt.transaction_hash
                    );
                    ChallengerGauge::GamesBondsClaimed.increment(1.0);

                    Ok(Action::Performed)
                }
                Err(e) => Err(anyhow::anyhow!(
                    "Failed to claim challenger bond from game {:?}: {:?}",
                    game_address,
                    e
                )),
            }
        } else {
            tracing::info!("No games found where challenger won to claim bonds from");

            Ok(Action::Skipped)
        }
    }

    /// Runs the challenger in an infinite loop, periodically checking for games to challenge and
    /// resolve.
    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("OP Succinct Challenger running...");
        if self.config.malicious_challenge_percentage > 0.0 {
            tracing::warn!(
                "\x1b[33mMalicious challenging enabled: {}% of valid games will be challenged for testing\x1b[0m",
                self.config.malicious_challenge_percentage
            );
        } else {
            tracing::info!("Honest challenger mode (malicious challenging disabled)");
        }
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        // Each loop, check the oldest challengeable game and challenge it if it exists.
        // Eventually, all games will be challenged (as long as the rate at which games are being
        // created is slower than the fetch interval).
        loop {
            interval.tick().await;

            match self.handle_game_challenging().await {
                Ok(Action::Performed) => {}
                Ok(Action::Skipped) => {}
                Err(e) => {
                    tracing::warn!("Failed to handle game challenging: {:?}", e);
                    ChallengerGauge::GameChallengingError.increment(1.0);
                }
            }

            if let Err(e) = self.handle_game_resolution().await {
                tracing::warn!("Failed to handle game resolution: {:?}", e);
                ChallengerGauge::GameResolutionError.increment(1.0);
            }

            match self.handle_bond_claiming().await {
                Ok(Action::Performed) => {}
                Ok(Action::Skipped) => {}
                Err(e) => {
                    tracing::warn!("Failed to handle bond claiming: {:?}", e);
                    ChallengerGauge::BondClaimingError.increment(1.0);
                }
            }
        }
    }
}
