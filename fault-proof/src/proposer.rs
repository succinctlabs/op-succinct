use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, TxHash, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_sol_types::{SolEvent, SolValue};
use anyhow::{Context, Result};
use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher, get_agg_proof_stdin, host::OPSuccinctHost,
    metrics::MetricsGauge, witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::get_range_elf_embedded;
use op_succinct_signer_utils::Signer;
use sp1_sdk::{
    NetworkProver, Prover, ProverClient, SP1ProofMode, SP1ProofWithPublicValues, SP1ProvingKey,
    SP1VerifyingKey, SP1_CIRCUIT_VERSION,
};
use tokio::{sync::Mutex, time};

use crate::{
    config::ProposerConfig,
    contract::{
        AnchorStateRegistry,
        DisputeGameFactory::{DisputeGameCreated, DisputeGameFactoryInstance},
        GameStatus, OPSuccinctFaultDisputeGame, ProposalStatus,
    },
    prometheus::ProposerGauge,
    Action, FactoryTrait, L1Provider, L2Provider, L2ProviderTrait, Mode,
};

/// Type alias for task ID
pub type TaskId = u64;

/// Type alias for task handles
pub type TaskHandle = tokio::task::JoinHandle<Result<()>>;

/// Type alias for a map of task IDs to their join handles and associated task info
pub type TaskMap = HashMap<TaskId, (TaskHandle, TaskInfo)>;

/// Information about a running task
#[derive(Clone, Debug)]
pub enum TaskInfo {
    GameCreation { block_number: U256 },
    GameProving { game_address: Address, is_defense: bool },
    GameResolution,
    BondClaim,
}

#[derive(Clone)]
struct SP1Prover {
    network_prover: Arc<NetworkProver>,
    range_pk: Arc<SP1ProvingKey>,
    range_vk: Arc<SP1VerifyingKey>,
    agg_pk: Arc<SP1ProvingKey>,
}

/// Represents a dispute game in the on-chain game DAG.
///
/// Games form a directed acyclic graph where each game (except the anchor)
/// builds upon a parent game, extending the chain with a new proposed output root.
/// The proposer tracks these games to determine when to propose new games,
/// defend existing ones, or resolve completed games.
#[derive(Clone)]
struct Game {
    index: U256,
    address: Address,
    parent_index: u32,
    l2_block: U256,
    status: GameStatus,
    proposal_status: ProposalStatus,
    deadline: u64,
    proposer_credit_available: bool,
    is_descendant_of_anchor: bool,
}

/// Central state management for tracking the game DAG and canonical chain.
///
/// The state maintains:
/// 1. **Anchor state**: The current finalized anchor game that all valid games must descend from
/// 2. **Canonical head**: The valid game with the highest L2 block number
/// 3. **Game cache**: All games that are descendants of the current anchor
/// 4. **Cursor**: Progress marker for loading games from the factory
///
/// The state ensures consistency by:
/// - Only keeping games that descend from the anchor
/// - Automatically recomputing the canonical head when games are added/removed
/// - Resetting completely when the anchor changes to a non-cached game
#[derive(Default)]
struct ProposerState {
    anchor_game: Option<Game>,
    canonical_head_index: Option<U256>,
    canonical_head_l2_block: Option<U256>,
    cursor: Option<U256>,
    games: HashMap<U256, Game>,
}

impl ProposerState {
    /// Returns the current cursor position for incremental game loading.
    fn cursor(&self) -> Option<U256> {
        self.cursor
    }

    /// Updates the cursor to track progress through the game list.
    fn set_cursor(&mut self, index: U256) {
        self.cursor = Some(index);
    }

    /// Mark `is_descendant_of_anchor` for all games reachable from the current anchor.
    ///
    /// Performs a depth-first search from the anchor game index, following child edges where
    /// `game.parent_index == current`. Every visited index is recorded in `reachable`,
    /// then each game’s `is_descendant_of_anchor` is updated accordingly.
    fn update_anchor_descendant_flags(&mut self) {
        let anchor_index = self
            .anchor_game
            .as_ref()
            .map(|game| game.index)
            .expect("anchor game must be set before updating descendant flags");

        let mut reachable: HashSet<U256> = HashSet::new();
        let mut stack = vec![anchor_index];

        while let Some(index) = stack.pop() {
            if reachable.insert(index) {
                stack.extend(
                    self.games
                        .values()
                        .filter(|game| U256::from(game.parent_index) == index)
                        .map(|game| game.index),
                );
            }
        }

        for game in self.games.values_mut() {
            game.is_descendant_of_anchor = reachable.contains(&game.index);
        }
    }

    /// Drop a game and every cached descendant below it.
    ///
    /// Performs a depth-first traversal to identify all games that descend from
    /// the root game, then removes them all from the cache. This is used when
    /// a game is found to be invalid (CHALLENGER_WINS), invalidating its entire
    /// subtree.
    ///
    /// After removal, the canonical head is recomputed to maintain consistency.
    fn remove_subtree(&mut self, root_index: U256) {
        let mut stack = vec![root_index];
        let mut to_remove = HashSet::new();

        while let Some(index) = stack.pop() {
            if !to_remove.insert(index) {
                continue;
            }

            let children: Vec<U256> = self
                .games
                .values()
                .filter_map(|game| {
                    if game.index == index {
                        return None;
                    }

                    if game.parent_index == u32::MAX {
                        None
                    } else if U256::from(game.parent_index) == index {
                        Some(game.index)
                    } else {
                        // FIXME(fakedev9999): fix.
                        None
                    }
                })
                .collect();

            stack.extend(children);
        }

        for index in to_remove {
            self.games.remove(&index);
        }
    }

    /// Checks if a game's parent is in a resolved state, allowing this game to be resolved.
    ///
    /// A game can only be resolved after its parent is no longer IN_PROGRESS.
    /// Root games (parent_index = u32::MAX) are always considered ready.
    fn is_parent_ready(&self, game: &Game) -> bool {
        if game.parent_index == u32::MAX {
            return true;
        }

        let parent_index = U256::from(game.parent_index);
        match self.games.get(&parent_index) {
            Some(parent) => parent.status != GameStatus::IN_PROGRESS,
            None => false,
        }
    }

    /// Determines if a game is ready for the proposer to resolve it.
    ///
    /// A game is ready for proposer resolution when:
    /// - It's unchallenged and past the deadline (automatic win)
    /// - A valid proof has been provided (immediate resolution allowed)
    fn is_proposer_ready(game: &Game, now_ts: u64) -> bool {
        match game.proposal_status {
            ProposalStatus::Unchallenged => now_ts >= game.deadline,
            ProposalStatus::UnchallengedAndValidProofProvided |
            ProposalStatus::ChallengedAndValidProofProvided => true,
            _ => false,
        }
    }

    /// Determines if a game is ready for the challenger to resolve it.
    ///
    /// A game is ready for challenger resolution when it's challenged
    /// but no proof was provided before the deadline.
    fn is_challenger_ready(game: &Game, now_ts: u64) -> bool {
        matches!(game.proposal_status, ProposalStatus::Challenged) && now_ts >= game.deadline
    }

    /// Returns games that are ready to be resolved based on mode and timing.
    ///
    /// Games must meet all conditions:
    /// 1. Still IN_PROGRESS (not yet resolved)
    /// 2. Not already marked as resolved in proposal status
    /// 3. Parent must be resolved (maintains DAG ordering)
    /// 4. Meet mode-specific timing requirements
    ///
    /// Returns game indices sorted in ascending order for deterministic processing.
    fn resolvable_candidates(&self, mode: Mode, now_ts: u64) -> Vec<U256> {
        let mut games: Vec<&Game> = self.games.values().collect();
        games.sort_by_key(|game| game.index);

        games
            .into_iter()
            .filter(|game| game.status == GameStatus::IN_PROGRESS)
            .filter(|game| game.proposal_status != ProposalStatus::Resolved)
            .filter(|game| self.is_parent_ready(game))
            .filter(|game| match mode {
                Mode::Proposer => Self::is_proposer_ready(game, now_ts),
                Mode::Challenger => Self::is_challenger_ready(game, now_ts),
            })
            .map(|game| game.index)
            .collect()
    }
}

#[derive(Clone)]
pub struct OPSuccinctProposer<P, H: OPSuccinctHost>
where
    P: Provider + Clone + Send + Sync + 'static,
    H: OPSuccinctHost + Clone + Send + Sync + 'static,
{
    pub config: ProposerConfig,
    // The address being committed to when generating the aggregation proof to prevent
    // front-running attacks. This should be the same address that is being used to send
    // `prove` transactions.
    pub prover_address: Address,
    pub signer: Signer,
    pub l1_provider: L1Provider,
    pub l2_provider: L2Provider,
    pub factory: Arc<DisputeGameFactoryInstance<P>>,
    pub init_bond: U256,
    pub safe_db_fallback: bool,
    prover: SP1Prover,
    fetcher: Arc<OPSuccinctDataFetcher>,
    host: Arc<H>,
    tasks: Arc<Mutex<TaskMap>>,
    next_task_id: Arc<AtomicU64>,
    state: Arc<Mutex<ProposerState>>,
}

impl<P, H> OPSuccinctProposer<P, H>
where
    P: Provider + Clone + Send + Sync + 'static,
    H: OPSuccinctHost + Clone + Send + Sync + 'static,
{
    /// Creates a new proposer instance with the provided L1 provider with wallet and factory
    /// contract instance.
    pub async fn new(
        config: ProposerConfig,
        network_private_key: String,
        prover_address: Address,
        signer: Signer,
        factory: DisputeGameFactoryInstance<P>,
        fetcher: Arc<OPSuccinctDataFetcher>,
        host: Arc<H>,
    ) -> Result<Self> {
        let network_prover =
            Arc::new(ProverClient::builder().network().private_key(&network_private_key).build());
        let (range_pk, range_vk) = network_prover.setup(get_range_elf_embedded());
        let (agg_pk, _) = network_prover.setup(AGGREGATION_ELF);

        let l1_provider = ProviderBuilder::default().connect_http(config.l1_rpc.clone());
        let l2_provider = ProviderBuilder::default().connect_http(config.l2_rpc.clone());
        let init_bond = factory.fetch_init_bond(config.game_type).await?;

        // Initialize state with anchor L2 block number
        let anchor_l2_block = factory.get_anchor_l2_block_number(config.game_type).await?;
        let initial_state =
            ProposerState { canonical_head_l2_block: Some(anchor_l2_block), ..Default::default() };

        Ok(Self {
            config: config.clone(),
            prover_address,
            signer,
            l1_provider,
            l2_provider,
            factory: Arc::new(factory.clone()),
            init_bond,
            safe_db_fallback: config.safe_db_fallback,
            prover: SP1Prover {
                network_prover,
                range_pk: Arc::new(range_pk),
                range_vk: Arc::new(range_vk),
                agg_pk: Arc::new(agg_pk),
            },
            fetcher: fetcher.clone(),
            host,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            next_task_id: Arc::new(AtomicU64::new(1)),
            state: Arc::new(Mutex::new(initial_state)),
        })
    }

    /// Runs the proposer indefinitely.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        tracing::info!("OP Succinct Proposer running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        // Spawn a dedicated task for continuous metrics collection
        self.spawn_metrics_collector();

        loop {
            interval.tick().await;

            // 1. Refresh the state.
            if let Err(e) = self.refresh_state().await {
                tracing::warn!("Failed to refresh proposer state: {:?}", e);
            }

            // 2. Handle completed tasks.
            if let Err(e) = self.handle_completed_tasks().await {
                tracing::warn!("Failed to handle completed tasks: {:?}", e);
            }

            // 3. Spawn new work (non-blocking).
            if let Err(e) = self.spawn_pending_operations().await {
                tracing::warn!("Failed to spawn pending operations: {:?}", e);
            }

            // 4. Log task statistics.
            self.log_task_stats().await;
        }
    }

    /// Proves a dispute game at the given address.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `TxHash`: The transaction hash of the proof submission
    /// - `u64`: Total instruction cycles used in the proof generation
    /// - `u64`: Total SP1 gas consumed in the proof generation
    #[tracing::instrument(name = "[[Proving]]", skip(self), fields(game_address = ?game_address))]
    pub async fn prove_game(&self, game_address: Address) -> Result<(TxHash, u64, u64)> {
        tracing::info!("Attempting to prove game {:?}", game_address);

        let fetcher = match OPSuccinctDataFetcher::new_with_rollup_config().await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to create data fetcher: {}", e);
                return Err(anyhow::anyhow!("Failed to create data fetcher: {}", e));
            }
        };

        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let l1_head_hash = game.l1Head().call().await?.0;
        tracing::debug!("L1 head hash: {:?}", hex::encode(l1_head_hash));
        let l2_block_number = game.l2BlockNumber().call().await?;

        let host_args = self
            .host
            .fetch(
                l2_block_number.to::<u64>() - self.config.proposal_interval_in_blocks,
                l2_block_number.to::<u64>(),
                Some(l1_head_hash.into()),
                self.config.safe_db_fallback,
            )
            .await
            .context("Failed to get host CLI args")?;

        let witness_data = self.host.run(&host_args).await?;

        let sp1_stdin = match self.host.witness_generator().get_sp1_stdin(witness_data) {
            Ok(stdin) => stdin,
            Err(e) => {
                tracing::error!("Failed to get proof stdin: {}", e);
                return Err(anyhow::anyhow!("Failed to get proof stdin: {}", e));
            }
        };

        tracing::info!("Generating Range Proof");
        let (range_proof, total_instruction_cycles, total_sp1_gas) = if self.config.mock_mode {
            tracing::info!("Using mock mode for range proof generation");
            let (public_values, report) = self
                .prover
                .network_prover
                .execute(get_range_elf_embedded(), &sp1_stdin)
                .calculate_gas(true)
                .deferred_proof_verification(false)
                .run()?;

            // Record execution stats
            let total_instruction_cycles = report.total_instruction_count();
            let total_sp1_gas = report.gas.unwrap_or(0);

            // Update Prometheus metrics
            ProposerGauge::TotalInstructionCycles.set(total_instruction_cycles as f64);
            ProposerGauge::TotalSP1Gas.set(total_sp1_gas as f64);

            tracing::info!(
                total_instruction_cycles = total_instruction_cycles,
                total_sp1_gas = total_sp1_gas,
                "Captured execution stats for range proof"
            );

            // Create a mock range proof with the public values.
            let proof = SP1ProofWithPublicValues::create_mock_proof(
                &self.prover.range_pk,
                public_values,
                SP1ProofMode::Compressed,
                SP1_CIRCUIT_VERSION,
            );

            (proof, total_instruction_cycles, total_sp1_gas)
        } else {
            // In network mode, we don't have access to execution stats
            let proof = self
                .prover
                .network_prover
                .prove(&self.prover.range_pk, &sp1_stdin)
                .compressed()
                .strategy(self.config.range_proof_strategy)
                .skip_simulation(true)
                .cycle_limit(1_000_000_000_000)
                .gas_limit(1_000_000_000_000)
                .timeout(Duration::from_secs(4 * 60 * 60))
                .run_async()
                .await?;

            (proof, 0, 0)
        };

        tracing::info!("Preparing Stdin for Agg Proof");
        let proof = range_proof.proof.clone();
        let mut public_values = range_proof.public_values.clone();
        let boot_info: BootInfoStruct = public_values.read();

        let headers = match fetcher
            .get_header_preimages(&vec![boot_info.clone()], boot_info.clone().l1Head)
            .await
        {
            Ok(headers) => headers,
            Err(e) => {
                tracing::error!("Failed to get header preimages: {}", e);
                return Err(anyhow::anyhow!("Failed to get header preimages: {}", e));
            }
        };

        let sp1_stdin = match get_agg_proof_stdin(
            vec![proof],
            vec![boot_info.clone()],
            headers,
            &self.prover.range_vk,
            boot_info.l1Head,
            self.prover_address,
        ) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("Failed to get agg proof stdin: {}", e);
                return Err(anyhow::anyhow!("Failed to get agg proof stdin: {}", e));
            }
        };

        tracing::info!("Generating Agg Proof");
        let agg_proof = if self.config.mock_mode {
            tracing::info!("Using mock mode for aggregation proof generation");
            let (public_values, _) = self
                .prover
                .network_prover
                .execute(AGGREGATION_ELF, &sp1_stdin)
                .deferred_proof_verification(false)
                .run()?;

            // Create a mock aggregation proof with the public values.
            SP1ProofWithPublicValues::create_mock_proof(
                &self.prover.agg_pk,
                public_values,
                SP1ProofMode::Groth16,
                SP1_CIRCUIT_VERSION,
            )
        } else {
            self.prover
                .network_prover
                .prove(&self.prover.agg_pk, &sp1_stdin)
                .groth16()
                .strategy(self.config.agg_proof_strategy)
                .timeout(Duration::from_secs(4 * 60 * 60))
                .run_async()
                .await?
        };

        let transaction_request = game.prove(agg_proof.bytes().into()).into_transaction_request();

        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        Ok((receipt.transaction_hash, total_instruction_cycles, total_sp1_gas))
    }

    /// Creates a new game with the given parameters.
    ///
    /// `l2_block_number`: the L2 block number we are proposing the output root for.
    /// `parent_game_index`: the index of the parent game.
    pub async fn create_game(
        &self,
        l2_block_number: U256,
        parent_game_index: u32,
    ) -> Result<Address> {
        tracing::info!(
            "Creating game at L2 block number: {:?}, with parent game index: {:?}",
            l2_block_number,
            parent_game_index
        );

        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let transaction_request = self
            .factory
            .create(
                self.config.game_type,
                self.l2_provider.compute_output_root_at_block(l2_block_number).await?,
                extra_data.into(),
            )
            .value(self.init_bond)
            .into_transaction_request();

        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        let game_address = receipt
            .inner
            .logs()
            .iter()
            .find_map(|log| {
                DisputeGameCreated::decode_log(&log.inner).ok().map(|event| event.disputeProxy)
            })
            .context("Could not find DisputeGameCreated event in transaction receipt logs")?;

        // Fetch game index after creation
        let game_count = self.factory.gameCount().call().await?;
        let game_index = game_count - U256::from(1);

        tracing::info!(
            game_index = %game_index,
            game_address = ?game_address,
            l2_block_end = %l2_block_number,
            parent_index = parent_game_index,
            tx_hash = ?receipt.transaction_hash,
            "Game created successfully"
        );

        if self.config.fast_finality_mode {
            tracing::info!("Fast finality mode enabled: Spawning proof generation task");

            // Spawn a tracked proving task for the new game
            if let Err(e) = self.spawn_game_proving_task(game_address, false).await {
                tracing::warn!("Failed to spawn fast finality proof task: {:?}", e);
            }
        }

        Ok(game_address)
    }

    /// Synchronize the cached game graph with on-chain anchor and latest game data.
    ///
    /// This function performs three key operations in sequence:
    /// 1. **Anchor synchronization**: Checks if the on-chain anchor has changed. If it has, either
    ///    prunes the cache (if anchor is known) or resets entirely.
    /// 2. **Game loading**: Incrementally loads new games from the factory, validating each one and
    ///    adding valid games to the cache.
    /// 3. **Status refresh**: Updates the status of all cached games to reflect current on-chain
    ///    state, removing any that became invalid.
    ///
    /// This should be called periodically to keep state synchronized with on-chain data.
    async fn refresh_state(&self) -> Result<()> {
        // Synchronize the game cache.
        self.sync_games().await?;

        // Synchronize the anchor game.
        self.sync_anchor_game().await?;

        // Compute the canonical head.
        self.compute_canonical_head().await;

        Ok(())
    }

    /// Synchronizes the anchor game from the factory.
    async fn sync_anchor_game(&self) -> Result<()> {
        let anchor_registry_address =
            self.factory.get_anchor_state_registry_address(self.config.game_type).await?;
        let anchor_registry =
            AnchorStateRegistry::new(anchor_registry_address, self.l1_provider.clone());
        let anchor_address = anchor_registry.anchorGame().call().await?;

        if anchor_address != Address::ZERO {
            let mut state = self.state.lock().await;

            // Fetch the anchor game from the cache.
            let (_index, anchor_game) = state
                .games
                .iter()
                .find(|(_, game)| game.address == anchor_address)
                .expect("Anchor game must be in the cache");

            state.anchor_game = Some(anchor_game.clone());
            state.update_anchor_descendant_flags();
        }

        Ok(())
    }

    /// Synchronizes the game cache.
    ///
    /// 1. Load new games: Incrementally load new games from the factory starting from the cursor.
    ///    Games are validated (correct type, valid output root) before being added.
    /// 2. Synchronize the status of all cached games.
    async fn sync_games(&self) -> Result<()> {
        // 1. Load new games.
        let mut next_index = {
            let state = self.state.lock().await;
            match state.cursor() {
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
        let mut state = self.state.lock().await;

        let mut to_remove = Vec::new();

        for (index, game) in state.games.iter_mut() {
            // Fetch the game's status.
            let contract = OPSuccinctFaultDisputeGame::new(game.address, self.l1_provider.clone());
            let claim_data = contract.claimData().call().await?;
            let status = contract.status().call().await?;
            let deadline = U256::from(claim_data.deadline).to::<u64>();
            let registry_address = contract.anchorStateRegistry().call().await?;
            let registry = AnchorStateRegistry::new(registry_address, self.l1_provider.clone());
            let is_finalized = registry.isGameFinalized(game.address).call().await?;

            if status == GameStatus::CHALLENGER_WINS {
                to_remove.push(*index);
                continue;
            } else {
                game.status = status;
                game.proposal_status = claim_data.status;
                game.deadline = deadline;
                game.proposer_credit_available =
                    is_finalized && contract.credit(self.prover_address).call().await? > U256::ZERO;
            }
        }

        for index in to_remove {
            state.remove_subtree(index);
        }

        Ok(())
    }

    /// Computes the canonical head by scanning all cached games.
    ///
    /// Canonical head is the game with the highest L2 block number. When an anchor game is present,
    /// only its descendants are eligible for canonical head.
    async fn compute_canonical_head(&self) {
        let mut state = self.state.lock().await;

        let canonical_head = state
            .games
            .values()
            .filter(|g| state.anchor_game.as_ref().is_none_or(|_| g.is_descendant_of_anchor))
            .max_by_key(|g| g.l2_block)
            .cloned();

        if let Some(canonical_head) = canonical_head {
            state.canonical_head_index = Some(canonical_head.index);
            state.canonical_head_l2_block = Some(canonical_head.l2_block);
        }
    }

    async fn resolve_candidates(&self, candidate_indices: Vec<U256>) -> Result<()> {
        for index in candidate_indices {
            let maybe_game = {
                let state = self.state.lock().await;
                state.games.get(&index).cloned()
            };

            if let Some(game) = maybe_game {
                if let Err(error) = self.submit_resolution_transaction(&game).await {
                    tracing::warn!(
                        game_index = %index,
                        game_address = ?game.address,
                        l2_block_end = %game.l2_block,
                        ?error,
                        "Failed to resolve game"
                    );
                }
            }
        }

        Ok(())
    }

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
            l2_block_end = %game.l2_block,
            tx_hash = ?receipt.transaction_hash,
            "Game resolved successfully"
        );

        ProposerGauge::GamesResolved.increment(1.0);
        self.mark_game_proposal_resolved(game.index).await;
        Ok(())
    }

    async fn mark_game_proposal_resolved(&self, index: U256) {
        let mut state = self.state.lock().await;
        if let Some(game) = state.games.get_mut(&index) {
            game.proposal_status = ProposalStatus::Resolved;
        }
    }

    async fn update_cursor(&self, index: U256) {
        let mut state = self.state.lock().await;
        state.set_cursor(index);
    }

    async fn fetch_game(&self, index: U256) -> Result<()> {
        let game = self.factory.gameAtIndex(index).call().await?;

        // TODO(fakedev9999): use `wasRespectedGameTypeWhenCreated` instead and use IDisputeGame
        if game.gameType != self.config.game_type {
            self.update_cursor(index).await;
            return Ok(());
        }

        let game_address = game.proxy;
        let contract = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let l2_block = contract.l2BlockNumber().call().await?;
        let output_root = self.l2_provider.compute_output_root_at_block(l2_block).await?;
        let claim = contract.rootClaim().call().await?;

        if output_root != claim {
            tracing::debug!(
                game_index = %index,
                ?game_address,
                "Skipping game due to mismatched output root"
            );
            self.update_cursor(index).await;
            return Ok(());
        }

        let claim_data = contract.claimData().call().await?;
        let status = contract.status().call().await?;
        let deadline = U256::from(claim_data.deadline).to::<u64>();

        let mut state = self.state.lock().await;
        state.games.insert(
            index,
            Game {
                index,
                address: game_address,
                parent_index: claim_data.parentIndex,
                l2_block,
                status,
                proposal_status: claim_data.status,
                deadline,
                proposer_credit_available: false,
                is_descendant_of_anchor: false,
            },
        );

        Ok(())
    }

    /// Handles the creation of a new game if conditions are met.
    /// Returns the address of the created game, if one was created.
    #[tracing::instrument(name = "[[Proposing]]", skip(self))]
    pub async fn handle_game_creation(&self) -> Result<Option<Address>> {
        let (latest_proposed_block_number, parent_game_index) = {
            let state = self.state.lock().await;

            let Some(latest_proposed_block_number) = state.canonical_head_l2_block else {
                tracing::info!("No canonical head; skipping game creation");
            return Ok(None);
        };

            let parent_game_index =
                state.canonical_head_index.map(|index| index.to::<u32>()).unwrap_or(u32::MAX);

            (latest_proposed_block_number, parent_game_index)
        };

        let next_l2_block_number_for_proposal =
            latest_proposed_block_number + U256::from(self.config.proposal_interval_in_blocks);

        let finalized_l2_head_block_number = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, latest_proposed_block_number.to::<u64>())
            .await?;

        // There's always a new game to propose, as the chain is always moving forward from the
        // genesis block set for the game type. Only create a new game if the finalized L2
        // head block number is at least the next L2 block number for proposal.
        if let Some(finalized_block) = finalized_l2_head_block_number {
            if U256::from(finalized_block) >= next_l2_block_number_for_proposal {
                let game_address =
                    self.create_game(next_l2_block_number_for_proposal, parent_game_index).await?;

                Ok(Some(game_address))
            } else {
                tracing::info!("No new game to propose since proposal interval has not elapsed");

                Ok(None)
            }
        } else {
            tracing::info!("No new finalized block number found since last proposed block");
            Ok(None)
        }
    }

    /// Handles claiming bonds from resolved games.
    #[tracing::instrument(name = "[[Claiming Proposer Bonds]]", skip(self))]
    async fn claim_bond_for_game(&self, index: U256, game_address: Address) -> Result<Action> {
        tracing::info!("Attempting to claim bond from game {:?} where proposer won", game_address);

        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let credit = game.credit(self.prover_address).call().await?;

        if credit == U256::ZERO {
            tracing::info!(
                game_address = ?game_address,
                "No credit available for proposer; skipping claim"
            );

            return Ok(Action::Skipped);
        }

        let l2_block_number = game.l2BlockNumber().call().await?;
        let transaction_request =
            game.claimCredit(self.prover_address).gas(200_000).into_transaction_request();

        match self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await
        {
            Ok(receipt) => {
                tracing::info!(
                    game_address = ?game_address,
                    l2_block_end = %l2_block_number,
                    tx_hash = ?receipt.transaction_hash,
                    "Bond claimed successfully"
                );

                Ok(Action::Performed)
            }
            Err(e) => {
                tracing::error!(
                    game_address = ?game_address,
                    l2_block_end = %l2_block_number,
                    error = %e,
                    "Bond claiming failed"
                );
                Err(anyhow::anyhow!(
                    "Failed to claim proposer bond from game {:?}: {:?}",
                    game_address,
                    e
                ))
            }
        }
    }

    /// Fetch the proposer metrics.
    async fn fetch_proposer_metrics(&self) -> Result<()> {
        let (canonical_head_l2_block, anchor_game) = {
            let state = self.state.lock().await;
            (state.canonical_head_l2_block, state.anchor_game.clone())
        };

        if let Some(canonical_head_l2_block) = canonical_head_l2_block {
            ProposerGauge::LatestGameL2BlockNumber.set(canonical_head_l2_block.to::<u64>() as f64);

            if let Some(finalized_l2_block_number) = self
                .host
                .get_finalized_l2_block_number(&self.fetcher, canonical_head_l2_block.to::<u64>())
                .await?
            {
                ProposerGauge::FinalizedL2BlockNumber.set(finalized_l2_block_number as f64);
            }

            if let Some(anchor_game) = anchor_game {
                ProposerGauge::AnchorGameL2BlockNumber.set(anchor_game.l2_block.to::<u64>() as f64);
        } else {
                ProposerGauge::AnchorGameL2BlockNumber.set(0.0);
            }
        } else {
            tracing::warn!("canonical_head_l2_block is None; skipping metrics update");
        }

        // Update active proving tasks metric
        let active_proving = self.count_active_proving_tasks().await;
        ProposerGauge::ActiveProvingTasks.set(active_proving as f64);

        Ok(())
    }

    /// Count active proving tasks
    async fn count_active_proving_tasks(&self) -> u64 {
        let tasks = self.tasks.lock().await;
        tasks.iter().filter(|(_, (_, info))| matches!(info, TaskInfo::GameProving { .. })).count()
            as u64
    }

    /// Count active defense tasks
    async fn count_active_defense_tasks(&self) -> u64 {
        let tasks = self.tasks.lock().await;
        tasks
            .iter()
            .filter(|(_, (_, info))| matches!(info, TaskInfo::GameProving { is_defense: true, .. }))
            .count() as u64
    }

    /// Spawn a dedicated metrics collection task
    fn spawn_metrics_collector(&self) {
        let proposer_metrics = self.clone();
        tokio::spawn(async move {
            let mut metrics_timer = time::interval(Duration::from_secs(15));
            loop {
                metrics_timer.tick().await;
                if let Err(e) = proposer_metrics.fetch_proposer_metrics().await {
                    tracing::warn!("Failed to fetch metrics: {:?}", e);
                    ProposerGauge::MetricsError.increment(1.0);
                }
            }
        });
    }

    /// Handle completed tasks and clean them up
    async fn handle_completed_tasks(&self) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        let mut completed = Vec::new();

        // Find completed tasks
        for (id, (handle, _)) in tasks.iter() {
            if handle.is_finished() {
                completed.push(*id);
            }
        }

        // Process completed tasks
        for id in completed {
            if let Some((handle, info)) = tasks.remove(&id) {
                match handle.await {
                    Ok(Ok(())) => {
                        tracing::info!("Task {:?} completed successfully", info);
                    }
                    Ok(Err(e)) => {
                        tracing::warn!("Task {:?} failed: {:?}", info, e);
                        // Handle task failure based on type
                        self.handle_task_failure(&info, e).await?;
                    }
                    Err(panic) => {
                        tracing::error!("Task {:?} panicked: {:?}", info, panic);
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle task failure based on task type
    async fn handle_task_failure(&self, info: &TaskInfo, _error: anyhow::Error) -> Result<()> {
        match info {
            TaskInfo::GameCreation { .. } => {
                ProposerGauge::GameCreationError.increment(1.0);
            }
            TaskInfo::GameProving { .. } => {
                ProposerGauge::GameProvingError.increment(1.0);
            }
            TaskInfo::GameResolution => {
                ProposerGauge::GameResolutionError.increment(1.0);
            }
            TaskInfo::BondClaim => {
                ProposerGauge::BondClaimingError.increment(1.0);
            }
        }
        Ok(())
    }

    /// Spawn pending operations if not already running
    async fn spawn_pending_operations(&self) -> Result<()> {
        // Check if we should create a game and spawn task if needed
        if !self.has_active_task_of_type(&TaskInfo::GameCreation { block_number: U256::ZERO }).await
        {
            match self.spawn_game_creation_task().await {
                Ok(true) => tracing::info!("Successfully spawned game creation task"),
                Ok(false) => {
                    tracing::debug!("No game creation needed - proposal interval not elapsed")
                }
                Err(e) => tracing::warn!("Failed to spawn game creation task: {:?}", e),
            }
        } else {
            tracing::info!("Game creation task already active");
        }

        // Check if we should defend games
        match self.spawn_game_defense_tasks().await {
            Ok(true) => tracing::info!("Successfully spawned game defense tasks"),
            Ok(false) => tracing::debug!("No games need defense or task already active"),
            Err(e) => tracing::warn!("Failed to spawn game defense tasks: {:?}", e),
        }

        // Check if we should resolve games
        if !self.has_active_task_of_type(&TaskInfo::GameResolution).await {
            match self.spawn_game_resolution_task().await {
                Ok(true) => tracing::info!("Successfully spawned game resolution task"),
                Ok(false) => tracing::debug!("No games need resolution"),
                Err(e) => tracing::warn!("Failed to spawn game resolution task: {:?}", e),
            }
        }

        // Check if we should claim bonds
        if !self.has_active_task_of_type(&TaskInfo::BondClaim).await {
            match self.spawn_bond_claim_task().await {
                Ok(true) => tracing::info!("Successfully spawned bond claim task"),
                Ok(false) => tracing::debug!("No bonds available to claim"),
                Err(e) => tracing::warn!("Failed to spawn bond claim task: {:?}", e),
            }
        } else {
            tracing::info!("Bond claim task already active");
        }

        Ok(())
    }

    /// Check if there's an active task of the given type
    async fn has_active_task_of_type(&self, task_type: &TaskInfo) -> bool {
        let tasks = self.tasks.lock().await;
        tasks
            .values()
            .any(|(_, info)| std::mem::discriminant(info) == std::mem::discriminant(task_type))
    }

    /// Log current task statistics
    async fn log_task_stats(&self) {
        let tasks = self.tasks.lock().await;
        let active_count = tasks.len();
        if active_count > 0 {
            let mut task_counts: HashMap<&str, usize> = HashMap::new();
            let mut proving_games: Vec<String> = Vec::new();

            for (_, (_, info)) in tasks.iter() {
                let task_type = match info {
                    TaskInfo::GameCreation { .. } => "GameCreation",
                    TaskInfo::GameProving { game_address, .. } => {
                        proving_games.push(format!("{game_address:?}"));
                        "GameProving"
                    }
                    TaskInfo::GameResolution => "GameResolution",
                    TaskInfo::BondClaim => "BondClaim",
                };
                *task_counts.entry(task_type).or_insert(0) += 1;
            }

            let task_types: Vec<String> = task_counts
                .into_iter()
                .map(|(type_name, count)| format!("{type_name}: {count}"))
                .collect();

            tracing::info!("Active tasks: {} ({})", active_count, task_types.join(", "));

            // Log specific games being proven
            if !proving_games.is_empty() {
                tracing::info!("Games being proven: {}", proving_games.join(", "));
            }
        }
    }

    /// Spawn a game creation task if conditions are met
    ///
    /// Returns:
    /// - Ok(true): Task was successfully spawned
    /// - Ok(false): No work needed (proposal interval not elapsed or no finalized blocks)
    /// - Err: Actual error occurred during task spawning
    async fn spawn_game_creation_task(&self) -> Result<bool> {
        // First check if we should create a game
        let (should_create, next_l2_block_number_for_proposal) = self.should_create_game().await?;
        if !should_create {
            return Ok(false); // No work needed - normal case
        }

        let proposer = self.clone();
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);

        let handle = tokio::spawn(async move {
            match proposer.handle_game_creation().await {
                Ok(Some(_game_address)) => {
                    ProposerGauge::GamesCreated.increment(1.0);
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(e) => Err(anyhow::anyhow!("error in game creation: {:?}", e)),
            }
        });

        let task_info = TaskInfo::GameCreation { block_number: next_l2_block_number_for_proposal };

        self.tasks.lock().await.insert(task_id, (handle, task_info));
        tracing::info!(
            "Spawned game creation task {} for block {}",
            task_id,
            next_l2_block_number_for_proposal
        );
        Ok(true)
    }

    /// Check if we should create a game
    ///
    /// Compares the next L2 block number for proposal with the finalized L2 block number.
    /// If the finalized L2 block number is greater than or equal to the next L2 block number for
    /// proposal, we should create a game.
    async fn should_create_game(&self) -> Result<(bool, U256)> {
        // In fast finality mode, check if we're at proving capacity
        // TODO(fakedev9999): Consider unifying proving concurrency control for both fast finality
        // and defense proving with a priority system.
        if self.config.fast_finality_mode {
            let active_proving = self.count_active_proving_tasks().await;
            if active_proving >= self.config.fast_finality_proving_limit {
                tracing::info!(
                    "Skipping game creation in fast finality mode: proving at capacity ({}/{})",
                    active_proving,
                    self.config.fast_finality_proving_limit
                );
                return Ok((false, U256::ZERO));
            }
        }

        let Some(canonical_head_l2_block) = self.state.lock().await.canonical_head_l2_block else {
            tracing::info!("No canonical head; skipping game creation");
            return Ok((false, U256::ZERO));
        };

        let next_l2_block_number_for_proposal =
            canonical_head_l2_block + U256::from(self.config.proposal_interval_in_blocks);

        let finalized_l2_head_block_number = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, canonical_head_l2_block.to::<u64>())
            .await?;

        Ok((
            finalized_l2_head_block_number
                .map(|finalized_block| {
                    U256::from(finalized_block) >= next_l2_block_number_for_proposal
                })
                .unwrap_or(false),
            next_l2_block_number_for_proposal,
        ))
    }

    /// Spawn game defense tasks if needed
    ///
    /// Returns:
    /// - Ok(true): Defense task was successfully spawned
    /// - Ok(false): No work needed (no defensible games or task already exists)
    /// - Err: Actual error occurred during task spawning
    #[tracing::instrument(name = "[[Defending]]", skip(self))]
    async fn spawn_game_defense_tasks(&self) -> Result<bool> {
        // Check if there are games needing defense
        let candidates = {
            let state = self.state.lock().await;
            state
                .games
                .values()
                .filter(|game| game.status == GameStatus::IN_PROGRESS)
                .filter(|game| matches!(game.proposal_status, ProposalStatus::Challenged))
                .map(|game| (game.index, game.address))
                .collect::<Vec<_>>()
        };

        let mut active_defense_tasks_count = self.count_active_defense_tasks().await;
        let max_concurrent = self.config.max_concurrent_defense_tasks;

        let mut tasks_spawned = false;

        for (index, game_address) in candidates {
            if active_defense_tasks_count >= max_concurrent {
                tracing::debug!(
                    "The max concurrent defense tasks count ({}) has been reached",
                    max_concurrent
                );
                break;
            }

            if self.has_active_proving_for_game(game_address).await {
                continue;
            }

            tracing::info!(
                game_address = ?game_address,
                game_index = %index,
                "Spawning defense for challenged game"
            );
            self.spawn_game_proving_task(game_address, true).await?;
            active_defense_tasks_count += 1;
            tasks_spawned = true;
        }

        Ok(tasks_spawned)
    }

    /// Check if there's an active proving task for a specific game
    async fn has_active_proving_for_game(&self, game_address: Address) -> bool {
        let tasks = self.tasks.lock().await;
        tasks.values().any(|(_, info)| {
            matches!(info, TaskInfo::GameProving { game_address: addr, .. } if *addr == game_address)
        })
    }

    /// Spawn a game proving task for a specific game
    async fn spawn_game_proving_task(&self, game_address: Address, is_defense: bool) -> Result<()> {
        let proposer: OPSuccinctProposer<P, H> = self.clone();
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);

        // Get the game block number to include in logs
        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let l2_block_number = game.l2BlockNumber().call().await?;
        let start_block = l2_block_number.to::<u64>() - self.config.proposal_interval_in_blocks;
        let end_block = l2_block_number.to::<u64>();

        tracing::info!(
            "Spawning game proving task {} for game {:?} (blocks {}-{})",
            task_id,
            game_address,
            start_block,
            end_block
        );

        // In mock mode, use spawn_blocking for CPU-intensive mock proof generation
        // In network mode, use spawn for async network operations
        let handle = if proposer.config.mock_mode {
            tokio::task::spawn_blocking(move || {
                // Use a runtime for the blocking task to handle async operations
                let rt = tokio::runtime::Handle::current();
                rt.block_on(async move {
                    let start_time = std::time::Instant::now();
                    let (tx_hash, total_instruction_cycles, total_sp1_gas) =
                        proposer.prove_game(game_address).await?;

                    // Record successful proving
                    ProposerGauge::GamesProven.increment(1.0);
                    ProposerGauge::ProvingDurationSeconds.set(start_time.elapsed().as_secs_f64());

                    tracing::info!(
                        game_address = ?game_address,
                        l2_block_start = start_block,
                        l2_block_end = end_block,
                        tx_hash = ?tx_hash,
                        duration_s = start_time.elapsed().as_secs_f64(),
                        total_instruction_cycles = total_instruction_cycles,
                        total_sp1_gas = total_sp1_gas,
                        "Game proven successfully"
                    );
                    Ok(())
                })
            })
        } else {
            tokio::spawn(async move {
                let start_time = std::time::Instant::now();
                let (tx_hash, total_instruction_cycles, total_sp1_gas) =
                    proposer.prove_game(game_address).await?;

                // Record successful proving
                ProposerGauge::GamesProven.increment(1.0);
                ProposerGauge::ProvingDurationSeconds.set(start_time.elapsed().as_secs_f64());

                tracing::info!(
                    game_address = ?game_address,
                    l2_block_start = start_block,
                    l2_block_end = end_block,
                    tx_hash = ?tx_hash,
                    duration_s = start_time.elapsed().as_secs_f64(),
                    total_instruction_cycles = total_instruction_cycles,
                    total_sp1_gas = total_sp1_gas,
                    "Game proven successfully"
                );
                Ok(())
            })
        };

        let task_info = TaskInfo::GameProving { game_address, is_defense };
        self.tasks.lock().await.insert(task_id, (handle, task_info));
        Ok(())
    }

    /// Spawn a game resolution task if needed
    ///
    /// Returns:
    /// - Ok(true): Resolution task was successfully spawned
    /// - Ok(false): No work needed (no games to resolve)
    /// - Err: Actual error occurred during task spawning
    #[tracing::instrument(name = "[[Proposer Resolving]]", skip(self))]
    async fn spawn_game_resolution_task(&self) -> Result<bool> {
        let latest_block = self
            .l1_provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .context("Failed to fetch latest L1 block for resolution task")?;
        let now_ts = latest_block.header.timestamp;

        let candidate_indices = {
            let state = self.state.lock().await;
            state.resolvable_candidates(Mode::Proposer, now_ts)
        };

        if candidate_indices.is_empty() {
            return Ok(false);
        }

        let proposer = self.clone();
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);

        let handle =
            tokio::spawn(async move { proposer.resolve_candidates(candidate_indices).await });

        let task_info = TaskInfo::GameResolution;
        self.tasks.lock().await.insert(task_id, (handle, task_info));
        tracing::info!("Spawned game resolution task {}", task_id);
        Ok(true)
    }

    /// Spawn a bond claim task if needed
    ///
    /// Returns:
    /// - Ok(true): Bond claim task was successfully spawned
    /// - Ok(false): No work needed (no claimable bonds available)
    /// - Err: Actual error occurred during task spawning
    async fn spawn_bond_claim_task(&self) -> Result<bool> {
        let candidates: Vec<_> = {
            let state = self.state.lock().await;
            state
                .games
                .values()
                .filter_map(|game| {
                    if game.proposer_credit_available {
                        Some((game.index, game.address))
                    } else {
                        None
                    }
                })
                .collect()
        };

        let proposer = self.clone();
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);

        let handle = tokio::spawn(async move {
            for (index, game_address) in candidates {
                match proposer.claim_bond_for_game(index, game_address).await {
                    Ok(Action::Performed) => {
                        let mut state = proposer.state.lock().await;
                        state.games.remove(&index);
                        ProposerGauge::GamesBondsClaimed.increment(1.0);
                    }
                    Ok(Action::Skipped) => {}
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        });

        let task_info = TaskInfo::BondClaim;
        self.tasks.lock().await.insert(task_id, (handle, task_info));
        tracing::info!("Spawned bond claim task {}", task_id);
        Ok(true)
    }
}
