use std::{collections::HashMap, sync::Arc, time::Duration};

use alloy_primitives::U256;
use alloy_provider::{Provider, ProviderBuilder};
use anyhow::Result;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::{sync::Mutex, time};

use crate::{
    config::ChallengerConfig,
    contract::{
        AnchorStateRegistry, DisputeGameFactory::DisputeGameFactoryInstance, GameStatus,
        OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    is_parent_resolved,
    prometheus::ChallengerGauge,
    ChallengerState, FactoryTrait, Game, L1Provider, L2Provider, L2ProviderTrait,
};
use op_succinct_host_utils::metrics::MetricsGauge;
use op_succinct_signer_utils::Signer;

pub struct OPSuccinctChallenger<P>
where
    P: Provider + Clone,
{
    pub config: ChallengerConfig,
    signer: Signer,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    factory: DisputeGameFactoryInstance<P>,
    challenger_bond: U256,
    state: Arc<Mutex<ChallengerState>>,
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
        let challenger_bond = factory.fetch_challenger_bond(config.game_type).await?;
        let l2_rpc = config.l2_rpc.clone();

        Ok(OPSuccinctChallenger {
            config,
            signer,
            l1_provider: l1_provider.clone(),
            l2_provider: ProviderBuilder::default().connect_http(l2_rpc),
            factory,
            challenger_bond,
            state: Arc::new(Mutex::new(ChallengerState { cursor: None, games: HashMap::new() })),
        })
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
        // TODO(fakedev9999): update comment.
        loop {
            interval.tick().await;

            // 1. Synchronize cached dispute state before scheduling work.
            if let Err(e) = self.sync_state().await {
                tracing::warn!("Failed to sync challenger state: {:?}", e);
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

    /// Synchronizes the game cache.
    ///
    /// 1. Load new games.
    ///    - Incrementally load new games from the factory starting from the cursor.
    /// 2. Synchronize the status of all cached games.
    ///    - Games are marked for resolution if the parent is resolved and the game is over.
    ///    - Games are marked for bond claim if they are finalized and there is credit to claim.
    async fn sync_state(&self) -> Result<()> {
        // 1. Load new games.
        let mut next_index = {
            let state = self.state.lock().await;
            match state.cursor {
                Some(cursor) => cursor + U256::from(1),
                None => U256::ZERO,
            }
        };

        let Some(latest_index) = self.factory.fetch_latest_game_index().await? else {
            return Ok(());
        };

        while next_index <= latest_index {
            self.fetch_game(next_index).await?;
            next_index += U256::from(1);
        }

        // 2. Synchronize the status of all cached games.
        let indices = {
            let state = self.state.lock().await;
            state.games.keys().cloned().collect::<Vec<_>>()
        };

        for index in indices {
            let game_address = {
                let state = self.state.lock().await;
                match state.games.get(&index) {
                    Some(g) => g.address,
                    None => continue,
                }
            };

            let contract = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
            // FIXME(fakedev9999): game might be not opsuccinct fault dispute game. Check
            // compatibilty with the IDisputeGame interface. claimData does not exist in the
            // IDisputeGame interface.
            let claim_data = contract.claimData().call().await?;
            let status = contract.status().call().await?;
            let registry_address = contract.anchorStateRegistry().call().await?;
            let registry = AnchorStateRegistry::new(registry_address, self.l1_provider.clone());
            let is_finalized = registry.isGameFinalized(game_address).call().await?;
            let credit = contract.credit(self.signer.address()).call().await?;

            {
                let mut state = self.state.lock().await;
                let Some(game) = state.games.get_mut(&index) else { continue };

                match status {
                    GameStatus::IN_PROGRESS => {
                        game.proposal_status = claim_data.status;

                        if claim_data.status == ProposalStatus::Unchallenged {
                            if contract.gameType().call().await? == self.config.game_type &&
                                !contract.gameOver().call().await?
                            {
                                game.should_attempt_to_challenge = true;
                            }
                        } else if claim_data.status == ProposalStatus::Challenged &&
                            is_parent_resolved(&self.state, index, self.l1_provider.clone())
                                .await? &&
                            contract.gameOver().call().await? &&
                            claim_data.counteredBy == self.signer.address()
                        {
                            game.should_attempt_to_resolve = true;
                        }
                    }
                    GameStatus::CHALLENGER_WINS => {
                        game.status = status;

                        if is_finalized && credit > U256::ZERO {
                            game.should_attempt_to_claim_bond = true;
                        }
                    }
                    // TODO(fakedev9999): is this correct? Won't this affect is_parent_resolved
                    // check for resolution?
                    GameStatus::DEFENDER_WINS => {
                        state.games.remove(&index);
                    }
                    _ => unreachable!("Unexpected game status: {:?}", status),
                }
            }
        }

        Ok(())
    }

    /// Fetch game from the factory.
    ///
    /// Drop game if the game type is invalid.
    async fn fetch_game(&self, index: U256) -> Result<()> {
        // FIXME(fakedev9999): game might be not opsuccinct fault dispute game.
        let game = self.factory.gameAtIndex(index).call().await?;
        let game_address = game.proxy;
        let contract = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let l2_block_number = contract.l2BlockNumber().call().await?;
        let output_root = contract.rootClaim().call().await?;
        let computed_output_root =
            self.l2_provider.compute_output_root_at_block(l2_block_number).await?;

        // TODO(fakedev9999): fetch all games no matter of the validity of the output root.
        // This is because all descendants of the invalid game are also invalid. However, if
        // the descendants of the invalid game are not challenged prior to resolution, the
        // proposer bond is burnt. To maximize the profit of the challenger, we should fetch all
        // games no matter of the validity of the output root.
        //
        // The current implementation fetches only invalid games for simplicity, but handles
        // the worst case scenario where invalid output root gets finalized.
        //
        // When updating the implementation, we should clevererly decide when to evict games,
        // since essentially, the challenger should track all games to maximize the profit.
        if contract.wasRespectedGameTypeWhenCreated().call().await? &&
            output_root != computed_output_root
        {
            let mut state = self.state.lock().await;
            state.games.insert(
                index,
                Game {
                    index,
                    address: game_address,
                    game_type: contract.gameType().call().await?,
                    l2_block_number,
                    status: contract.status().call().await?,
                    proposal_status: contract.claimData().call().await?.status,
                    should_attempt_to_challenge: false,
                    should_attempt_to_resolve: false,
                    should_attempt_to_claim_bond: false,
                },
            );
        } else {
            tracing::debug!(
                game_index = %index,
                ?game_address,
                "Dropping game due to invalid game type or valid output root"
            );
        }

        self.state.lock().await.cursor = Some(index);

        Ok(())
    }

    /// Handles challenging of invalid games by scanning recent games for potential challenges.
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
            if let Err(error) = self.submit_challenge_transaction(&game).await {
                tracing::warn!(
                    game_index = %game.index,
                    game_address = ?game.address,
                    ?error,
                    "Failed to challenge game"
                );
                ChallengerGauge::GameChallengingError.increment(1.0);
                continue;
            }

            ChallengerGauge::GamesChallenged.increment(1.0);
        }

        // Maliciously challenge valid games (if configured for testing defense mechanisms)
        if self.config.malicious_challenge_percentage > 0.0 {
            let mut rng = StdRng::from_os_rng();
            let should_challenge: f64 = rng.random_range(0.0..100.0);

            if should_challenge <= self.config.malicious_challenge_percentage {
                let candidate = {
                    let state = self.state.lock().await;
                    state
                        .games
                        .values()
                        .filter(|game| {
                            !game.should_attempt_to_challenge &&
                                game.game_type == self.config.game_type
                        })
                        .max_by_key(|game| game.index)
                        .cloned()
                };

                if let Some(game) = candidate {
                    tracing::warn!(
                        "\x1b[31m[MALICIOUS CHALLENGE]\x1b[0m Attempting to challenge valid game {:?} at index {} for testing ({}% chance)",
                        game.address,
                        game.index,
                        self.config.malicious_challenge_percentage
                    );

                    if let Err(error) = self.submit_challenge_transaction(&game).await {
                        tracing::warn!(
                            game_index = %game.index,
                            game_address = ?game.address,
                            ?error,
                            "Failed to maliciously challenge game"
                        );
                        ChallengerGauge::GameChallengingError.increment(1.0);
                    } else {
                        ChallengerGauge::GamesChallenged.increment(1.0);
                    }
                }
            }
        }

        Ok(())
    }

    async fn submit_challenge_transaction(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let transaction_request =
            contract.challenge().value(self.challenger_bond).into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Game challenged successfully"
        );

        Ok(())
    }

    /// Handles resolution of challenged games that are ready to be resolved.
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
            if let Err(error) = self.submit_resolution_transaction(&game).await {
                tracing::warn!(
                    game_index = %game.index,
                    game_address = ?game.address,
                    ?error,
                    "Failed to resolve game"
                );
                ChallengerGauge::GameResolutionError.increment(1.0);
                continue;
            }

            ChallengerGauge::GamesResolved.increment(1.0);
        }

        Ok(())
    }

    // TODO(fakedev9999): Reduce code dup with proposer.
    async fn submit_resolution_transaction(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let transaction_request = contract.resolve().into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block_end = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Game resolved successfully"
        );

        Ok(())
    }

    /// Handles claiming bonds from resolved games.
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

        for game in candidates {
            if let Err(error) = self.submit_bond_claim_transaction(&game).await {
                tracing::warn!(
                    game_index = %game.index,
                    game_address = ?game.address,
                    ?error,
                    "Failed to claim bond for game"
                );
                ChallengerGauge::BondClaimingError.increment(1.0);
                continue;
            }

            ChallengerGauge::GamesBondsClaimed.increment(1.0);
        }

        Ok(())
    }

    // TODO(fakedev9999): Reduce code dup with proposer.
    /// Submit the on-chain transaction to claim the proposer's bond for a given game.
    #[tracing::instrument(name = "[[Claiming Proposer Bonds]]", skip(self, game))]
    async fn submit_bond_claim_transaction(&self, game: &Game) -> Result<()> {
        let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
        let transaction_request =
            contract.claimCredit(self.signer.address()).gas(200_000).into_transaction_request();
        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        tracing::info!(
            game_index = %game.index,
            game_address = ?game.address,
            l2_block_end = %game.l2_block_number,
            tx_hash = ?receipt.transaction_hash,
            "Bond claimed successfully"
        );

        Ok(())
    }
}
