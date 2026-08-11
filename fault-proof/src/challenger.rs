use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock,
    },
    time::Duration,
};

use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::{Address, B256, U256};
use alloy_provider::Provider;
use anyhow::{bail, Context, Result};
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::{sync::Mutex, time};

pub use crate::game_validator::{GameValidation, InvalidReason, UnavailableReason};

use crate::{
    checked_l2_block_number,
    config::ChallengerConfig,
    contract::{
        AnchorStateRegistry::AnchorStateRegistryInstance,
        DisputeGameFactory::DisputeGameFactoryInstance, GameStatus, OPSuccinctFaultDisputeGame,
        ProposalStatus,
    },
    game_validator::{initial_game_validation, GameValidationRequest, GameValidator},
    is_parent_challenger_wins, is_parent_resolved,
    prometheus::ChallengerGauge,
    FactoryTrait, L1Provider, TxErrorExt, TX_REVERTED_PREFIX,
};
use op_succinct_host_utils::metrics::MetricsGauge;
use op_succinct_signer_utils::SignerLock;

pub struct OPSuccinctChallenger<P>
where
    P: Provider + Clone,
{
    pub config: ChallengerConfig,
    signer: SignerLock,
    l1_provider: L1Provider,
    game_validator: Arc<dyn GameValidator>,
    anchor_state_registry: AnchorStateRegistryInstance<P>,
    factory: DisputeGameFactoryInstance<P>,
    challenger_bond: OnceLock<U256>,
    highest_accepted_l1_block: AtomicU64,
    sync_lock: Mutex<()>,
    state: Arc<Mutex<ChallengerState>>,
}

impl<P> OPSuccinctChallenger<P>
where
    P: Provider + Clone,
{
    /// Creates a challenger with an injected chain-specific game validator.
    pub fn new_with_game_validator(
        config: ChallengerConfig,
        l1_provider: L1Provider,
        anchor_state_registry: AnchorStateRegistryInstance<P>,
        factory: DisputeGameFactoryInstance<P>,
        signer: SignerLock,
        game_validator: Arc<dyn GameValidator>,
    ) -> Self {
        Self {
            config,
            signer,
            l1_provider,
            game_validator,
            anchor_state_registry,
            factory,
            challenger_bond: OnceLock::new(),
            highest_accepted_l1_block: AtomicU64::new(0),
            sync_lock: Mutex::new(()),
            state: Arc::new(Mutex::new(ChallengerState::new())),
        }
    }

    /// Runs the main challenger loop. On each tick it waits for the configured interval, refreshes
    /// cached state, and then handles challenging, resolution, and bond-claiming tasks.
    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("OP Succinct Lite Challenger running...");

        self.try_init().await?;

        if self.config.malicious_challenge_percentage > 0.0 {
            tracing::warn!(
                "\x1b[33mMalicious challenging enabled: {}% of valid games will be challenged for testing\x1b[0m",
                self.config.malicious_challenge_percentage
            );
        } else {
            tracing::info!("Honest challenger mode (malicious challenging disabled)");
        }

        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        // Each loop iteration waits for the configured interval, synchronizes the cached state,
        // and then attempts to challenge, resolve, and claim bonds for any eligible games.
        loop {
            interval.tick().await;

            // Synchronize cached dispute state before scheduling work.
            if let Err(e) = self.sync_state().await {
                tracing::warn!("Failed to sync challenger state: {:?}", e);
                ChallengerGauge::SyncFailures.increment(1.0);
                continue
            }

            if let Err(e) = self.handle_game_challenging().await {
                tracing::warn!("Failed to handle game challenging: {:?}", e);
            }

            if let Err(e) = self.handle_game_resolution().await {
                tracing::warn!("Failed to handle game resolution: {:?}", e);
            }

            if let Err(e) = self.handle_bond_claiming().await {
                tracing::warn!("Failed to handle bond claiming: {:?}", e);
            }
        }
    }

    /// Runs startup validations with retries before entering main loop.
    pub async fn try_init(&self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));
        let mut retry_count = 0u32;

        loop {
            match self.validate_and_init().await {
                Ok(()) => break,
                Err(e) => {
                    retry_count += 1;
                    if retry_count == 1 {
                        tracing::error!(attempt = retry_count, error = ?e, "Startup validations failed");
                    } else {
                        tracing::warn!(
                            attempt = retry_count,
                            error = ?e,
                            "Startup validations still pending, retrying..."
                        );
                    }
                    interval.tick().await;
                }
            }
        }

        Ok(())
    }

    /// Validates startup and initializes state.
    async fn validate_and_init(&self) -> Result<()> {
        let bond = self.startup_validations().await?;
        self.init_state(bond);
        Ok(())
    }

    /// Runs one-time startup validations before the challenger begins normal operations.
    /// Returns the challenger bond on success.
    async fn startup_validations(&self) -> Result<U256> {
        // Validate game type is registered and get game implementation.
        let game_impl = self.factory.game_impl(self.config.game_type).await?;

        // Validate anchor state registry matches factory's game implementation.
        let expected_registry = game_impl.anchorStateRegistry().call().await?;
        if *self.anchor_state_registry.address() != expected_registry {
            anyhow::bail!(
                "Anchor state registry address mismatch: config has {}, but factory's game implementation uses {}",
                self.anchor_state_registry.address(),
                expected_registry
            );
        }

        // Fetch challenger bond.
        let bond = game_impl.challengerBond().call().await?;

        self.game_validator.validate_startup().await?;

        Ok(bond)
    }

    /// Initialize challenger state with the validated challenger bond.
    fn init_state(&self, bond: U256) {
        self.challenger_bond.set(bond).expect("challenger_bond must not already be set");
    }

    /// Synchronizes the game cache.
    ///
    /// 1. Load new games.
    ///    - Incrementally load new games from the factory starting from the cursor.
    /// 2. Synchronize the status of all cached games.
    ///    - Games are marked for challenging if output root is invalid or the parent is challenger
    ///      wins.
    ///    - Games are marked for resolution if the parent is resolved, the game is over, and it's
    ///      own game.
    ///    - Games are marked for bond claim if they are finalized and there is credit to claim.
    ///    - Games are evicted once finalized with no remaining credit or whenever resolves as
    ///      defender wins.
    pub async fn sync_state(&self) -> Result<()> {
        let _sync_guard = self.sync_lock.lock().await;
        let snapshot = self.l1_sync_snapshot().await?;
        self.highest_accepted_l1_block.store(snapshot.confirmed_number, Ordering::Relaxed);
        let pinned_block = snapshot.pinned_block;
        let now_ts = snapshot.latest_timestamp;

        // Retry indices whose factory/game metadata could not be read in an earlier cycle.
        let pending_discoveries = {
            let state = self.state.lock().await;
            state.pending_discoveries.iter().copied().collect::<Vec<_>>()
        };

        for index in pending_discoveries {
            match self.fetch_game(index, pinned_block).await {
                Ok(()) => {
                    self.state.lock().await.pending_discoveries.remove(&index);
                }
                Err(error) => {
                    tracing::warn!(
                        game_index = %index,
                        ?error,
                        "Failed to retry game discovery; will retry next cycle"
                    );
                    ChallengerGauge::SyncFailures.increment(1.0);
                }
            }
        }

        // Discover every new factory index independently. A failed index is retained for retry,
        // while the discovery cursor continues so later games cannot be starved.
        if let Some(latest_index) = self.factory.fetch_latest_game_index(pinned_block).await? {
            let mut next_index = {
                let state = self.state.lock().await;
                state.cursor.map_or(U256::ZERO, |cursor| cursor + U256::from(1))
            };

            while next_index <= latest_index {
                let failed = match self.fetch_game(next_index, pinned_block).await {
                    Ok(()) => false,
                    Err(error) => {
                        tracing::warn!(
                            game_index = %next_index,
                            ?error,
                            "Failed to discover game; retaining index for retry"
                        );
                        ChallengerGauge::SyncFailures.increment(1.0);
                        true
                    }
                };

                self.state.lock().await.record_discovery_attempt(next_index, failed);
                next_index += U256::from(1);
            }
        }

        // Synchronize each cached game independently so one broken game cannot starve unrelated
        // challenge, resolution, or claim work.
        let games = {
            let state = self.state.lock().await;
            state.games.values().cloned().collect::<Vec<_>>()
        };

        if !games.is_empty() {
            let signer_address = self.signer.address();
            let mut actions = Vec::with_capacity(games.len());

            for game in games {
                let game_index = game.index;
                let game_address = game.address;
                match self.sync_game(game, pinned_block, now_ts, signer_address).await {
                    Ok(action) => actions.push(action),
                    Err(error) => {
                        tracing::warn!(
                            game_index = %game_index,
                            game_address = ?game_address,
                            ?error,
                            "Failed to synchronize game; leaving it queued for retry"
                        );
                        ChallengerGauge::SyncFailures.increment(1.0);
                        self.state.lock().await.record_game_sync_failure(game_index);
                    }
                }
            }

            let mut state = self.state.lock().await;
            for action in actions {
                match action {
                    GameSyncAction::Update(game) => {
                        state.games.insert(game.index, game);
                    }
                    GameSyncAction::Remove(index) => {
                        state.games.remove(&index);
                    }
                }
            }

            let unavailable_games =
                state.games.values().filter(|game| game.requires_validation()).collect::<Vec<_>>();
            ChallengerGauge::UnverifiableGames.set(unavailable_games.len() as f64);
            let nearest_deadline =
                unavailable_games.iter().map(|game| game.deadline).collect::<Vec<_>>();
            let nearest_deadline = nearest_deadline_seconds(&nearest_deadline, now_ts);
            ChallengerGauge::NearestUnverifiableDeadlineSeconds.set(nearest_deadline);
        } else {
            ChallengerGauge::UnverifiableGames.set(0.0);
            ChallengerGauge::NearestUnverifiableDeadlineSeconds.set(-1.0);
        }

        Ok(())
    }

    async fn l1_sync_snapshot(&self) -> Result<L1SyncSnapshot> {
        let latest_block = self
            .l1_provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .context("Failed to fetch latest L1 block")?;
        let latest_number = latest_block.header.number;
        let latest_timestamp = latest_block.header.timestamp;
        let confirmed_number =
            confirmed_l1_number(latest_number, self.config.sync_l1_confirmations);
        let highest_accepted = self.highest_accepted_l1_block.load(Ordering::Relaxed);
        if confirmed_l1_head_regressed(confirmed_number, highest_accepted) {
            tracing::warn!(
                confirmed_number,
                highest_accepted,
                "L1 confirmed head moved backwards; skipping challenger sync"
            );
            bail!(
                "L1 confirmed head moved backwards from {highest_accepted} to {confirmed_number}"
            );
        }
        let confirmed_hash = if confirmed_number == latest_number {
            latest_block.header.hash
        } else {
            self.l1_provider
                .get_block_by_number(BlockNumberOrTag::Number(confirmed_number))
                .await?
                .with_context(|| {
                    format!("Confirmed L1 block {confirmed_number} is unavailable on this backend")
                })?
                .header
                .hash
        };

        ChallengerGauge::ConfirmedL1Head.set(confirmed_number as f64);
        ChallengerGauge::L1ConfirmationLagBlocks
            .set(latest_number.saturating_sub(confirmed_number) as f64);

        Ok(L1SyncSnapshot {
            confirmed_number,
            pinned_block: BlockId::hash_canonical(confirmed_hash),
            latest_timestamp,
        })
    }

    async fn sync_game(
        &self,
        mut game: Game,
        pinned_block: BlockId,
        now_ts: u64,
        signer_address: Address,
    ) -> Result<GameSyncAction> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let status = contract.status().block(pinned_block).call().await?;
        let claim_data = contract.claimData().block(pinned_block).call().await?;
        game.status = status;
        game.proposal_status = claim_data.status;
        game.deadline = U256::from(claim_data.deadline).to::<u64>();
        game.clear_action_flags();

        match status {
            GameStatus::IN_PROGRESS => {
                let is_game_over = now_ts >= game.deadline;
                match game.proposal_status {
                    ProposalStatus::Unchallenged => {
                        if is_game_over {
                            if let GameValidation::Unavailable(reason) = &game.validation {
                                tracing::error!(
                                    game_index = %game.index,
                                    game_address = ?game.address,
                                    l2_block_number = %game.l2_block_number,
                                    deadline = game.deadline,
                                    ?reason,
                                    "Game expired before validation became available"
                                );
                            }
                        } else {
                            if game.validation.is_unavailable() {
                                game.validation = self.validate_game(&game, now_ts).await;
                            }
                            let invalid = game.validation.is_invalid();
                            let parent_lost = if invalid {
                                false
                            } else {
                                is_parent_challenger_wins(
                                    game.parent_index,
                                    &self.factory,
                                    pinned_block,
                                )
                                .await?
                            };
                            game.should_attempt_to_challenge =
                                game.validation.is_invalid() || parent_lost;
                        }
                    }
                    ProposalStatus::Challenged => {
                        let is_own_game = claim_data.counteredBy == signer_address;
                        game.should_attempt_to_resolve = is_game_over &&
                            is_own_game &&
                            is_parent_resolved(game.parent_index, &self.factory, pinned_block)
                                .await?;
                    }
                    _ => {}
                }

                Ok(GameSyncAction::Update(game))
            }
            GameStatus::CHALLENGER_WINS => {
                let is_finalized = self
                    .anchor_state_registry
                    .isGameFinalized(game.address)
                    .block(pinned_block)
                    .call()
                    .await?;
                let credit = contract.credit(signer_address).block(pinned_block).call().await?;

                if is_finalized && credit == U256::ZERO {
                    Ok(GameSyncAction::Remove(game.index))
                } else {
                    game.should_attempt_to_claim_bond = is_finalized && credit > U256::ZERO;
                    Ok(GameSyncAction::Update(game))
                }
            }
            GameStatus::DEFENDER_WINS => Ok(GameSyncAction::Remove(game.index)),
            _ => unreachable!("Unexpected game status: {:?}", status),
        }
    }

    async fn validate_game(&self, game: &Game, now_timestamp: u64) -> GameValidation {
        if checked_l2_block_number(game.l2_block_number).is_err() {
            return GameValidation::Invalid(InvalidReason::L2BlockNumberOverflow)
        }
        self.game_validator.validate(&game.validation_request(now_timestamp)).await
    }

    /// Fetch game from the factory.
    ///
    /// Drop game if the game type is invalid or the game was not respected at the time of creation.
    async fn fetch_game(&self, index: U256, pinned_block: BlockId) -> Result<()> {
        let game = self.factory.gameAtIndex(index).block(pinned_block).call().await?;
        let game_address = game.proxy;
        let contract = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

        let game_type = contract.gameType().block(pinned_block).call().await?;
        if game_type != self.config.game_type {
            tracing::debug!(game_index = %index, ?game_address, game_type,
                expected_game_type = self.config.game_type,
                "Dropping game due to invalid game type"
            );
            return Ok(());
        }

        let was_respected =
            contract.wasRespectedGameTypeWhenCreated().block(pinned_block).call().await?;
        if !was_respected {
            tracing::debug!(
                game_index = %index,
                ?game_address,
                game_type,
                expected_game_type = self.config.game_type,
                "Dropping game because its type was not respected at the time of creation"
            );
            return Ok(())
        }

        let l2_block_number = contract.l2BlockNumber().block(pinned_block).call().await?;
        let output_root = contract.rootClaim().block(pinned_block).call().await?;
        let l1_head = contract.l1Head().block(pinned_block).call().await?.0;
        let claim_data = contract.claimData().block(pinned_block).call().await?;
        let status = contract.status().block(pinned_block).call().await?;

        self.state.lock().await.games.insert(
            index,
            Game {
                index,
                address: game_address,
                parent_index: claim_data.parentIndex,
                l2_block_number,
                output_root,
                l1_head: l1_head.into(),
                deadline: U256::from(claim_data.deadline).to::<u64>(),
                validation: initial_game_validation(l2_block_number),
                status,
                proposal_status: claim_data.status,
                should_attempt_to_challenge: false,
                should_attempt_to_resolve: false,
                should_attempt_to_claim_bond: false,
            },
        );

        Ok(())
    }

    /// Challenges games flagged for challenging.
    /// Also supports malicious challenging of valid games for testing defense mechanisms when
    /// configured.
    #[tracing::instrument(skip(self), level = "info", name = "[[Challenging]]")]
    async fn handle_game_challenging(&mut self) -> Result<()> {
        let candidates = {
            let state = self.state.lock().await;
            state
                .games
                .values()
                .filter(|game| game.should_attempt_to_challenge)
                .cloned()
                .collect::<Vec<_>>()
        };

        for game in candidates {
            match self.challenge_preflight(&game).await {
                Ok(true) => {}
                Ok(false) => {
                    tracing::info!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        "Skipping challenge because latest on-chain state is no longer eligible"
                    );
                    ChallengerGauge::PreflightSkips.increment(1.0);
                    continue
                }
                Err(error) => {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Skipping challenge because latest-state preflight failed"
                    );
                    ChallengerGauge::PreflightErrors.increment(1.0);
                    continue
                }
            }

            if let Err(error) = self.submit_challenge_transaction_unchecked(&game).await {
                if error.is_revert() {
                    tracing::error!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Challenge tx included but reverted on-chain"
                    );
                } else {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Challenge tx unconfirmed (may be on-chain), will verify next cycle"
                    );
                }
                ChallengerGauge::GameChallengingError.increment(1.0);
                continue;
            }

            // Clear the challenge flag after successful challenge
            {
                let mut state = self.state.lock().await;
                if let Some(game_state) = state.games.get_mut(&game.index) {
                    game_state.should_attempt_to_challenge = false;
                }
            }

            ChallengerGauge::GamesChallenged.increment(1.0);
        }

        // Maliciously challenge valid games (if configured for testing defense mechanisms)
        if self.config.malicious_challenge_percentage > 0.0 {
            let mut rng = StdRng::from_os_rng();
            let should_challenge: f64 = rng.random_range(0.0..100.0);

            if should_challenge <= self.config.malicious_challenge_percentage {
                let now_ts =
                    match self.l1_provider.get_block_by_number(BlockNumberOrTag::Latest).await {
                        Ok(Some(block)) => block.header.timestamp,
                        Ok(None) => {
                            tracing::warn!(
                                "Skipping malicious challenge: latest L1 block unavailable"
                            );
                            ChallengerGauge::PreflightErrors.increment(1.0);
                            return Ok(())
                        }
                        Err(error) => {
                            tracing::warn!(
                                ?error,
                                "Skipping malicious challenge: latest L1 timestamp query failed"
                            );
                            ChallengerGauge::PreflightErrors.increment(1.0);
                            return Ok(())
                        }
                    };
                let candidate = {
                    let state = self.state.lock().await;
                    state
                        .games
                        .values()
                        .filter(|game| {
                            matches!(game.validation, GameValidation::Valid) &&
                                game.status == GameStatus::IN_PROGRESS &&
                                game.proposal_status == ProposalStatus::Unchallenged &&
                                game.deadline > now_ts
                        })
                        .min_by_key(|game| game.index)
                        .cloned()
                };

                if let Some(game) = candidate {
                    tracing::warn!(
                        "\x1b[31m[MALICIOUS CHALLENGE]\x1b[0m Attempting to challenge valid game {:?} at index {} for testing ({}% chance)",
                        game.address,
                        game.index,
                        self.config.malicious_challenge_percentage
                    );

                    let preflight_ready = match self.challenge_preflight(&game).await {
                        Ok(ready) => ready,
                        Err(error) => {
                            tracing::warn!(
                                game_index = %game.index,
                                game_address = ?game.address,
                                ?error,
                                "Skipping malicious challenge because latest-state preflight failed"
                            );
                            ChallengerGauge::PreflightErrors.increment(1.0);
                            return Ok(())
                        }
                    };
                    if !preflight_ready {
                        ChallengerGauge::PreflightSkips.increment(1.0);
                    } else if let Err(error) =
                        self.submit_challenge_transaction_unchecked(&game).await
                    {
                        tracing::warn!(
                            game_index = %game.index,
                            game_address = ?game.address,
                            ?error,
                            "Failed to maliciously challenge game"
                        );
                        ChallengerGauge::GameChallengingError.increment(1.0);
                    } else {
                        // Clear the challenge flag after successful malicious challenge
                        {
                            let mut state = self.state.lock().await;
                            if let Some(game_state) = state.games.get_mut(&game.index) {
                                game_state.should_attempt_to_challenge = false;
                            }
                        }
                        ChallengerGauge::GamesChallenged.increment(1.0);
                    }
                }
            }
        }

        Ok(())
    }

    async fn latest_l1_view(&self) -> Result<(BlockId, u64)> {
        let latest_block = self
            .l1_provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .context("Failed to fetch latest L1 block for action preflight")?;
        Ok((BlockId::hash_canonical(latest_block.header.hash), latest_block.header.timestamp))
    }

    async fn challenge_preflight(&self, game: &Game) -> Result<bool> {
        let (latest_block, latest_timestamp) = self.latest_l1_view().await?;
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let status = contract.status().block(latest_block).call().await?;
        let claim_data = contract.claimData().block(latest_block).call().await?;
        let deadline = U256::from(claim_data.deadline).to::<u64>();

        Ok(challenge_preflight_ready(status, claim_data.status, latest_timestamp, deadline))
    }

    async fn resolution_preflight(&self, game: &Game) -> Result<bool> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        Ok(resolution_preflight_ready(contract.status().call().await?))
    }

    async fn claim_preflight(&self, game: &Game, recipient: Address) -> Result<bool> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        Ok(claim_preflight_ready(contract.credit(recipient).call().await?))
    }

    /// Submits a challenge after verifying eligibility against a fresh canonical L1 head.
    pub async fn submit_challenge_transaction(&self, game: &Game) -> Result<()> {
        if !self.challenge_preflight(game).await? {
            bail!("Challenge is no longer eligible at the latest L1 head");
        }
        self.submit_challenge_transaction_unchecked(game).await
    }

    async fn submit_challenge_transaction_unchecked(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let challenger_bond = *self
            .challenger_bond
            .get()
            .context("challenger_bond must be set via startup_validations")?;
        let transaction_request =
            contract.challenge().value(challenger_bond).into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request_with_timeout(
                self.config.l1_rpc.clone(),
                transaction_request,
                self.config.tx_confirmation_timeout,
            )
            .await?;

        if !receipt.status() {
            bail!("{TX_REVERTED_PREFIX} {receipt:?}");
        }

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Game challenged successfully"
        );

        Ok(())
    }

    /// Resolves games flagged for resolution.
    #[tracing::instrument(skip(self), level = "info", name = "[[Resolving]]")]
    async fn handle_game_resolution(&self) -> Result<()> {
        let candidates = {
            let state = self.state.lock().await;
            state
                .games
                .values()
                .filter(|game| game.should_attempt_to_resolve)
                .cloned()
                .collect::<Vec<_>>()
        };

        for game in candidates {
            match self.resolution_preflight(&game).await {
                Ok(true) => {}
                Ok(false) => {
                    tracing::info!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        "Skipping resolve: game already resolved on chain"
                    );
                    ChallengerGauge::PreflightSkips.increment(1.0);
                    continue
                }
                Err(error) => {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Skipping resolve because latest-state preflight failed"
                    );
                    ChallengerGauge::PreflightErrors.increment(1.0);
                    continue
                }
            }

            if let Err(error) = self.submit_resolution_transaction_unchecked(&game).await {
                if error.is_revert() {
                    tracing::error!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Resolution tx included but reverted on-chain"
                    );
                } else {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Resolution tx unconfirmed (may be on-chain), will verify next cycle"
                    );
                }
                ChallengerGauge::GameResolutionError.increment(1.0);
                continue;
            }

            ChallengerGauge::GamesResolved.increment(1.0);
        }

        Ok(())
    }

    /// Submits a resolution after verifying eligibility against a fresh canonical L1 head.
    pub async fn submit_resolution_transaction(&self, game: &Game) -> Result<()> {
        if !self.resolution_preflight(game).await? {
            bail!("Resolution is no longer eligible at the latest L1 head");
        }
        self.submit_resolution_transaction_unchecked(game).await
    }

    async fn submit_resolution_transaction_unchecked(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let transaction_request = contract.resolve().into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request_with_timeout(
                self.config.l1_rpc.clone(),
                transaction_request,
                self.config.tx_confirmation_timeout,
            )
            .await?;

        if !receipt.status() {
            bail!("{TX_REVERTED_PREFIX} {receipt:?}");
        }

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block_end = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Game resolved successfully"
        );

        Ok(())
    }

    /// Claims bonds from games flagged for claiming.
    #[tracing::instrument(skip(self), level = "info", name = "[[Claiming Challenger Bonds]]")]
    pub async fn handle_bond_claiming(&self) -> Result<()> {
        let candidates = {
            let state = self.state.lock().await;
            state
                .games
                .values()
                .filter(|game| game.should_attempt_to_claim_bond)
                .cloned()
                .collect::<Vec<_>>()
        };

        let signer_address = self.signer.address();
        for game in candidates {
            match self.claim_preflight(&game, signer_address).await {
                Ok(true) => {}
                Ok(false) => {
                    tracing::info!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        "Skipping claim: bond already claimed on chain"
                    );
                    ChallengerGauge::PreflightSkips.increment(1.0);
                    continue
                }
                Err(error) => {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Skipping claim because latest-state preflight failed"
                    );
                    ChallengerGauge::PreflightErrors.increment(1.0);
                    continue
                }
            }

            if let Err(error) = self.submit_bond_claim_transaction(&game).await {
                if error.is_revert() {
                    tracing::error!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Bond claim tx included but reverted on-chain"
                    );
                } else {
                    tracing::warn!(
                        game_index = %game.index,
                        game_address = ?game.address,
                        ?error,
                        "Bond claim tx unconfirmed (may be on-chain), will verify next cycle"
                    );
                }
                ChallengerGauge::BondClaimingError.increment(1.0);
                continue;
            }

            ChallengerGauge::GamesBondsClaimed.increment(1.0);
        }

        Ok(())
    }

    #[tracing::instrument(name = "[[Claiming Proposer Bonds]]", skip(self, game))]
    async fn submit_bond_claim_transaction(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let transaction_request =
            contract.claimCredit(self.signer.address()).gas(200_000).into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request_with_timeout(
                self.config.l1_rpc.clone(),
                transaction_request,
                self.config.tx_confirmation_timeout,
            )
            .await?;

        if !receipt.status() {
            bail!("{TX_REVERTED_PREFIX} {receipt:?}");
        }

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block_end = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Bond claimed successfully"
        );

        Ok(())
    }

    // ==================== Integration Test Helpers ====================

    /// Returns a copy of a game's full internal state for testing.
    #[cfg(feature = "integration")]
    pub async fn get_game(&self, index: U256) -> Option<Game> {
        let state = self.state.lock().await;
        state.games.get(&index).cloned()
    }

    /// Returns the number of cached games for testing.
    #[cfg(feature = "integration")]
    pub async fn cached_game_count(&self) -> usize {
        let state = self.state.lock().await;
        state.games.len()
    }

    /// Returns a snapshot of all cached game indices for testing.
    #[cfg(feature = "integration")]
    pub async fn cached_game_indices(&self) -> Vec<U256> {
        let state = self.state.lock().await;
        state.games.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Game {
    pub index: U256,
    pub address: Address,
    pub parent_index: u32,
    pub l2_block_number: U256,
    pub output_root: B256,
    pub l1_head: B256,
    pub deadline: u64,
    pub validation: GameValidation,
    pub status: GameStatus,
    pub proposal_status: ProposalStatus,
    pub should_attempt_to_challenge: bool,
    pub should_attempt_to_resolve: bool,
    pub should_attempt_to_claim_bond: bool,
}

impl Game {
    fn validation_request(&self, now_timestamp: u64) -> GameValidationRequest {
        GameValidationRequest {
            game_index: self.index,
            game_address: self.address,
            l1_head: self.l1_head,
            l2_block_number: self.l2_block_number,
            output_root: self.output_root,
            deadline: self.deadline,
            now_timestamp,
        }
    }

    fn clear_action_flags(&mut self) {
        self.should_attempt_to_challenge = false;
        self.should_attempt_to_resolve = false;
        self.should_attempt_to_claim_bond = false;
    }

    fn requires_validation(&self) -> bool {
        self.status == GameStatus::IN_PROGRESS &&
            self.proposal_status == ProposalStatus::Unchallenged &&
            self.validation.is_unavailable()
    }
}

fn nearest_deadline_seconds(deadlines: &[u64], now_ts: u64) -> f64 {
    deadlines
        .iter()
        .map(|deadline| deadline.saturating_sub(now_ts))
        .min()
        .map_or(-1.0, |remaining| remaining as f64)
}

fn confirmed_l1_number(latest_number: u64, confirmations: u64) -> u64 {
    latest_number.saturating_sub(confirmations)
}

fn confirmed_l1_head_regressed(confirmed_number: u64, highest_accepted: u64) -> bool {
    confirmed_number < highest_accepted
}

fn challenge_preflight_ready(
    status: GameStatus,
    proposal_status: ProposalStatus,
    latest_timestamp: u64,
    deadline: u64,
) -> bool {
    status == GameStatus::IN_PROGRESS &&
        proposal_status == ProposalStatus::Unchallenged &&
        latest_timestamp < deadline
}

fn resolution_preflight_ready(status: GameStatus) -> bool {
    status == GameStatus::IN_PROGRESS
}

fn claim_preflight_ready(credit: U256) -> bool {
    credit > U256::ZERO
}

enum GameSyncAction {
    Update(Game),
    Remove(U256),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct L1SyncSnapshot {
    confirmed_number: u64,
    pinned_block: BlockId,
    latest_timestamp: u64,
}

pub struct ChallengerState {
    cursor: Option<U256>,
    games: HashMap<U256, Game>,
    pending_discoveries: HashSet<U256>,
}

impl ChallengerState {
    fn new() -> Self {
        Self { cursor: None, games: HashMap::new(), pending_discoveries: HashSet::new() }
    }

    fn record_discovery_attempt(&mut self, index: U256, failed: bool) {
        self.cursor = Some(index);
        if failed {
            self.pending_discoveries.insert(index);
        }
    }

    fn record_game_sync_failure(&mut self, index: U256) {
        if let Some(game) = self.games.get_mut(&index) {
            game.clear_action_flags();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::VecDeque,
        sync::{
            atomic::{AtomicUsize, Ordering as AtomicOrdering},
            Mutex as StdMutex,
        },
    };

    use super::*;
    use crate::contract::ClaimData;
    use alloy::{rpc::client::RpcClient, transports::mock::Asserter};
    use alloy_provider::RootProvider;
    use alloy_rpc_types_eth::Block;
    use alloy_sol_types::SolValue;
    use async_trait::async_trait;

    const TEST_PRIVATE_KEY: &str =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    struct RecordingValidator {
        startup_calls: AtomicUsize,
        startup_error: Option<&'static str>,
        requests: StdMutex<Vec<GameValidationRequest>>,
        outcomes: StdMutex<VecDeque<GameValidation>>,
    }

    impl RecordingValidator {
        fn new(outcomes: impl IntoIterator<Item = GameValidation>) -> Self {
            Self {
                startup_calls: AtomicUsize::new(0),
                startup_error: None,
                requests: StdMutex::new(Vec::new()),
                outcomes: StdMutex::new(outcomes.into_iter().collect()),
            }
        }
    }

    #[async_trait]
    impl GameValidator for RecordingValidator {
        async fn validate_startup(&self) -> Result<()> {
            self.startup_calls.fetch_add(1, AtomicOrdering::Relaxed);
            if let Some(error) = self.startup_error {
                bail!("{error}")
            }
            Ok(())
        }

        async fn validate(&self, request: &GameValidationRequest) -> GameValidation {
            self.requests.lock().unwrap().push(request.clone());
            self.outcomes.lock().unwrap().pop_front().unwrap_or(GameValidation::Valid)
        }
    }

    fn test_game(validation: GameValidation) -> Game {
        Game {
            index: U256::ZERO,
            address: Address::ZERO,
            parent_index: u32::MAX,
            l2_block_number: U256::from(7),
            output_root: B256::ZERO,
            l1_head: B256::repeat_byte(0x55),
            deadline: 100,
            validation,
            status: GameStatus::IN_PROGRESS,
            proposal_status: ProposalStatus::Unchallenged,
            should_attempt_to_challenge: false,
            should_attempt_to_resolve: false,
            should_attempt_to_claim_bond: false,
        }
    }

    fn push_active_unchallenged_game(asserter: &Asserter, deadline: u64) {
        asserter.push_success(&GameStatus::IN_PROGRESS.abi_encode());
        asserter.push_success(
            &ClaimData {
                parentIndex: u32::MAX,
                counteredBy: Address::ZERO,
                prover: Address::ZERO,
                claim: B256::ZERO.into(),
                status: ProposalStatus::Unchallenged,
                deadline,
            }
            .abi_encode(),
        );
    }

    fn test_challenger(l1_asserter: &Asserter) -> OPSuccinctChallenger<L1Provider> {
        let l1_provider = RootProvider::new(RpcClient::mocked(l1_asserter.clone()));
        let anchor_state_registry =
            crate::contract::AnchorStateRegistry::new(Address::ZERO, l1_provider.clone());
        let factory = crate::contract::DisputeGameFactory::new(Address::ZERO, l1_provider.clone());
        let signer = SignerLock::new(
            op_succinct_signer_utils::Signer::new_local_signer(TEST_PRIVATE_KEY).unwrap(),
        );
        let config = ChallengerConfig {
            l1_rpc: "http://127.0.0.1:1".parse().unwrap(),
            anchor_state_registry_address: Address::ZERO,
            factory_address: Address::ZERO,
            fetch_interval: 1,
            game_type: 0,
            metrics_port: 0,
            malicious_challenge_percentage: 0.0,
            sync_l1_confirmations: 0,
            tx_confirmation_timeout: 60,
        };
        let game_validator = Arc::new(RecordingValidator::new([]));
        OPSuccinctChallenger::new_with_game_validator(
            config,
            l1_provider,
            anchor_state_registry,
            factory,
            signer,
            game_validator,
        )
    }

    fn challenger_with_recording_validator(
        outcomes: impl IntoIterator<Item = GameValidation>,
    ) -> (OPSuccinctChallenger<L1Provider>, Arc<RecordingValidator>, Asserter) {
        let l1_asserter = Asserter::new();
        let validator = Arc::new(RecordingValidator::new(outcomes));
        let mut challenger = test_challenger(&l1_asserter);
        challenger.game_validator = validator.clone();
        (challenger, validator, l1_asserter)
    }

    async fn sync_active_game(
        challenger: &OPSuccinctChallenger<L1Provider>,
        l1_asserter: &Asserter,
        game: Game,
        now_timestamp: u64,
    ) -> Game {
        push_active_unchallenged_game(l1_asserter, game.deadline);
        match challenger
            .sync_game(
                game,
                BlockId::hash(B256::repeat_byte(0x11)),
                now_timestamp,
                challenger.signer.address(),
            )
            .await
            .unwrap()
        {
            GameSyncAction::Update(game) => game,
            GameSyncAction::Remove(_) => panic!("active game should remain cached"),
        }
    }

    #[tokio::test]
    async fn injected_validator_receives_complete_game_context() {
        let (challenger, validator, _) =
            challenger_with_recording_validator([GameValidation::Valid]);

        let mut game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));
        game.index = U256::from(9);
        game.address = Address::from([0x11; 20]);
        game.l1_head = B256::repeat_byte(0x22);
        game.l2_block_number = U256::from(123);
        game.output_root = B256::repeat_byte(0x33);
        game.deadline = 456;

        assert_eq!(challenger.validate_game(&game, 321).await, GameValidation::Valid);
        assert_eq!(validator.requests.lock().unwrap().as_slice(), &[game.validation_request(321)]);
    }

    #[tokio::test]
    async fn oversized_block_number_does_not_reach_injected_validator() {
        let (challenger, validator, _) =
            challenger_with_recording_validator([GameValidation::Valid]);
        let mut game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));
        game.l2_block_number = U256::from(u64::MAX) + U256::from(1);

        assert_eq!(
            challenger.validate_game(&game, 0).await,
            GameValidation::Invalid(InvalidReason::L2BlockNumberOverflow)
        );
        assert!(validator.requests.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn unavailable_validation_is_retried_on_the_next_sync() {
        let unavailable = GameValidation::Unavailable(UnavailableReason::RpcFailure(
            "backend temporarily unavailable".to_string(),
        ));
        let (challenger, validator, l1_asserter) =
            challenger_with_recording_validator([unavailable.clone(), GameValidation::Valid]);
        let game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));

        let game = sync_active_game(&challenger, &l1_asserter, game, 50).await;
        assert_eq!(game.validation, unavailable);
        assert!(!game.should_attempt_to_challenge);

        let game = sync_active_game(&challenger, &l1_asserter, game, 51).await;
        assert_eq!(game.validation, GameValidation::Valid);
        assert!(!game.should_attempt_to_challenge);
        assert_eq!(validator.requests.lock().unwrap().len(), 2);
        assert!(l1_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn invalid_validation_flags_challenge_while_valid_does_not() {
        for (validation, should_challenge) in [
            (GameValidation::Invalid(InvalidReason::OutputRootMismatch), true),
            (GameValidation::Valid, false),
        ] {
            let (challenger, validator, l1_asserter) =
                challenger_with_recording_validator([validation.clone()]);
            let game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));

            let game = sync_active_game(&challenger, &l1_asserter, game, 50).await;

            assert_eq!(game.validation, validation);
            assert_eq!(game.should_attempt_to_challenge, should_challenge);
            assert_eq!(validator.requests.lock().unwrap().len(), 1);
            assert!(l1_asserter.read_q().is_empty());
        }
    }

    #[tokio::test]
    async fn expired_game_does_not_invoke_injected_validator() {
        let (challenger, validator, l1_asserter) =
            challenger_with_recording_validator([GameValidation::Invalid(
                InvalidReason::OutputRootMismatch,
            )]);
        let game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));

        let game = sync_active_game(&challenger, &l1_asserter, game, 100).await;

        assert!(matches!(game.validation, GameValidation::Unavailable(_)));
        assert!(!game.should_attempt_to_challenge);
        assert!(validator.requests.lock().unwrap().is_empty());
        assert!(l1_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn shared_startup_invokes_injected_validator() {
        let (challenger, validator, l1_asserter) = challenger_with_recording_validator([]);

        l1_asserter.push_success(&Address::from([0x44; 20]).abi_encode());
        l1_asserter.push_success(&Address::ZERO.abi_encode());
        l1_asserter.push_success(&U256::from(5).abi_encode());

        assert_eq!(challenger.startup_validations().await.unwrap(), U256::from(5));
        assert_eq!(validator.startup_calls.load(AtomicOrdering::Relaxed), 1);
        assert!(l1_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn shared_startup_propagates_injected_validator_failure() {
        let l1_asserter = Asserter::new();
        let validator = Arc::new(RecordingValidator {
            startup_calls: AtomicUsize::new(0),
            startup_error: Some("backend unavailable"),
            requests: StdMutex::new(Vec::new()),
            outcomes: StdMutex::new(VecDeque::new()),
        });
        let mut challenger = test_challenger(&l1_asserter);
        challenger.game_validator = validator.clone();

        l1_asserter.push_success(&Address::from([0x44; 20]).abi_encode());
        l1_asserter.push_success(&Address::ZERO.abi_encode());
        l1_asserter.push_success(&U256::from(5).abi_encode());

        assert_eq!(
            challenger.startup_validations().await.unwrap_err().to_string(),
            "backend unavailable"
        );
        assert_eq!(validator.startup_calls.load(AtomicOrdering::Relaxed), 1);
        assert!(l1_asserter.read_q().is_empty());
    }

    #[test]
    fn discovery_cursor_advances_past_pending_retries() {
        let mut state = ChallengerState::new();

        state.record_discovery_attempt(U256::ZERO, true);
        state.record_discovery_attempt(U256::from(1), false);

        assert_eq!(state.cursor, Some(U256::from(1)));
        assert!(state.pending_discoveries.contains(&U256::ZERO));
        assert!(!state.pending_discoveries.contains(&U256::from(1)));
    }

    #[test]
    fn unavailable_deadline_metric_distinguishes_none_from_expired() {
        assert_eq!(nearest_deadline_seconds(&[], 100), -1.0);
        assert_eq!(nearest_deadline_seconds(&[90], 100), 0.0);
        assert_eq!(nearest_deadline_seconds(&[130, 110], 100), 10.0);
    }

    #[test]
    fn confirmed_l1_number_saturates_without_config_invariants() {
        assert_eq!(confirmed_l1_number(100, 0), 100);
        assert_eq!(confirmed_l1_number(100, 12), 88);
        assert_eq!(confirmed_l1_number(5, 12), 0);
    }

    #[test]
    fn confirmed_l1_head_only_rejects_regressions() {
        assert!(!confirmed_l1_head_regressed(100, 100));
        assert!(!confirmed_l1_head_regressed(101, 100));
        assert!(confirmed_l1_head_regressed(99, 100));
    }

    #[test]
    fn action_preflight_decisions_are_fail_closed_at_boundaries() {
        assert!(challenge_preflight_ready(
            GameStatus::IN_PROGRESS,
            ProposalStatus::Unchallenged,
            99,
            100
        ));
        assert!(!challenge_preflight_ready(
            GameStatus::IN_PROGRESS,
            ProposalStatus::Unchallenged,
            100,
            100
        ));
        assert!(!challenge_preflight_ready(
            GameStatus::DEFENDER_WINS,
            ProposalStatus::Unchallenged,
            99,
            100
        ));
        assert!(!challenge_preflight_ready(
            GameStatus::IN_PROGRESS,
            ProposalStatus::Challenged,
            99,
            100
        ));
        assert!(resolution_preflight_ready(GameStatus::IN_PROGRESS));
        assert!(!resolution_preflight_ready(GameStatus::DEFENDER_WINS));
        assert!(!claim_preflight_ready(U256::ZERO));
        assert!(claim_preflight_ready(U256::from(1)));
    }

    #[tokio::test]
    async fn challenge_preflight_error_does_not_block_next_candidate() {
        let l1_asserter = Asserter::new();
        let mut challenger = test_challenger(&l1_asserter);
        let mut first = test_game(GameValidation::Invalid(InvalidReason::OutputRootMismatch));
        first.should_attempt_to_challenge = true;
        let mut second = first.clone();
        second.index = U256::from(1);
        second.address = Address::from([1u8; 20]);
        {
            let mut state = challenger.state.lock().await;
            state.games.insert(first.index, first);
            state.games.insert(second.index, second);
        }

        let mut latest_block: Block = Block::default();
        latest_block.header.hash = B256::repeat_byte(0x11);
        latest_block.header.timestamp = 50;
        l1_asserter.push_failure_msg("first candidate latest block unavailable");
        l1_asserter.push_success(&Some(latest_block));
        l1_asserter.push_failure_msg("second candidate status unavailable");

        challenger.handle_game_challenging().await.unwrap();

        assert!(
            l1_asserter.read_q().is_empty(),
            "the second candidate should still reach its status preflight"
        );
        let state = challenger.state.lock().await;
        assert!(state.games.values().all(|game| game.should_attempt_to_challenge));
    }

    #[test]
    fn failed_refresh_can_clear_all_stale_action_flags() {
        let mut state = ChallengerState::new();
        let mut game = test_game(GameValidation::Valid);
        game.should_attempt_to_challenge = true;
        game.should_attempt_to_resolve = true;
        game.should_attempt_to_claim_bond = true;
        state.games.insert(game.index, game);

        state.record_game_sync_failure(U256::ZERO);

        let game = state.games.get(&U256::ZERO).unwrap();
        assert!(!game.should_attempt_to_challenge);
        assert!(!game.should_attempt_to_resolve);
        assert!(!game.should_attempt_to_claim_bond);
    }

    #[test]
    fn only_active_unchallenged_unavailable_games_require_validation() {
        let mut game = test_game(GameValidation::Unavailable(UnavailableReason::ValidationPending));

        assert!(game.requires_validation());

        game.proposal_status = ProposalStatus::Challenged;
        assert!(!game.requires_validation());

        game.proposal_status = ProposalStatus::Unchallenged;
        game.status = GameStatus::CHALLENGER_WINS;
        assert!(!game.requires_validation());

        game.status = GameStatus::IN_PROGRESS;
        game.validation = GameValidation::Valid;
        assert!(!game.requires_validation());
    }
}
