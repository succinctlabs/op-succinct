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
    finalized: bool,
    proposer_credit_available: bool,
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
    anchor_index: Option<U256>,
    anchor_address: Option<Address>,
    anchor_l2_block: Option<U256>,
    canonical_head_index: Option<U256>,
    canonical_head_l2_block: Option<U256>,
    cursor: Option<U256>,
    games: HashMap<U256, Game>,
}

/// Immutable snapshot of key state values for lock-free access.
///
/// Used to make decisions about game creation without holding the state lock,
/// preventing contention between concurrent operations.
#[derive(Clone, Copy)]
struct StateSnapshot {
    anchor_l2_block: U256,
    canonical_head_index: Option<U256>,
    canonical_head_l2_block: U256,
}

impl ProposerState {
    /// Creates an immutable snapshot of the current state for lock-free access.
    ///
    /// Returns None if the state is not initialized (no anchor set).
    fn snapshot(&self) -> Option<StateSnapshot> {
        let anchor_l2_block = self.anchor_l2_block?;
        let canonical_head_l2_block = self.canonical_head_l2_block.unwrap_or(anchor_l2_block);

        Some(StateSnapshot {
            anchor_l2_block,
            canonical_head_index: self.canonical_head_index,
            canonical_head_l2_block,
        })
    }

    /// Resets all state when the anchor changes to an unknown game.
    ///
    /// This is called when the anchor changes to a game not in our cache,
    /// requiring a full reload of the game DAG from scratch.
    fn reset(&mut self) {
        self.anchor_index = None;
        self.anchor_address = None;
        self.anchor_l2_block = None;
        self.canonical_head_index = None;
        self.canonical_head_l2_block = None;
        self.cursor = None;
        self.games.clear();
    }

    /// Sets a new anchor game and initializes associated state.
    ///
    /// This function:
    /// 1. Sets the anchor indices and address
    /// 2. Adds the anchor game to the cache
    /// 3. Sets the anchor as the canonical head (since no descendants exist yet)
    /// 4. Updates the cursor to start loading from this point
    fn set_anchor(&mut self, game: &Game) {
        self.anchor_index = Some(game.index);
        self.anchor_address = Some(game.address);
        self.anchor_l2_block = Some(game.l2_block);
        self.games.insert(game.index, game.clone());
        self.set_canonical_head(game);
        self.cursor = Some(game.index);
    }

    /// Updates the canonical head to point to a specific game.
    ///
    /// The canonical head represents the valid game with the highest L2 block number,
    /// which determines where new games should build from.
    fn set_canonical_head(&mut self, game: &Game) {
        self.canonical_head_index = Some(game.index);
        self.canonical_head_l2_block = Some(game.l2_block);
    }

    /// Returns the current cursor position for incremental game loading.
    fn cursor(&self) -> Option<U256> {
        self.cursor
    }

    /// Updates the cursor to track progress through the game list.
    fn set_cursor(&mut self, index: U256) {
        self.cursor = Some(index);
    }

    /// Updates the canonical head if the candidate has a higher L2 block.
    ///
    /// This maintains the invariant that the canonical head always points to
    /// the valid game with the highest L2 block number.
    fn update_canonical_head_if_better(&mut self, candidate: &Game) {
        match self.canonical_head_l2_block {
            Some(current_block) if candidate.l2_block <= current_block => {}
            _ => self.set_canonical_head(candidate),
        }
    }

    /// Recomputes the canonical head by scanning all cached games.
    ///
    /// This is called after game removal or cache changes to ensure the
    /// canonical head remains accurate. Falls back to the anchor block
    /// if no games exist in the cache.
    fn recompute_canonical_head(&mut self) {
        let mut best_game: Option<Game> = None;

        for game in self.games.values() {
            if best_game.as_ref().map(|current| game.l2_block > current.l2_block).unwrap_or(true) {
                best_game = Some(game.clone());
            }
        }

        if let Some(game) = best_game {
            self.set_canonical_head(&game);
        } else if let Some(anchor_block) = self.anchor_l2_block {
            self.canonical_head_index = None;
            self.canonical_head_l2_block = Some(anchor_block);
        } else {
            self.canonical_head_index = None;
            self.canonical_head_l2_block = None;
        }
    }

    /// Finds a game's index given its on-chain address.
    ///
    /// Returns None if the game is not in the cache.
    fn find_index_by_address(&self, address: Address) -> Option<U256> {
        self.games
            .iter()
            .find_map(|(index, game)| if game.address == address { Some(*index) } else { None })
    }

    /// Retain only games whose ancestry leads back to the current anchor.
    ///
    /// This function performs a graph traversal starting from the anchor to find
    /// all reachable games. Any games not reachable from the anchor are removed.
    ///
    /// The traversal handles both direct parent relationships (via parent_index)
    /// and special handling for games with parent_index = u32::MAX that should
    /// connect to the anchor.
    ///
    /// After pruning, the canonical head is recomputed to ensure consistency.
    fn retain_descendants_of_anchor(&mut self) {
        let anchor_index =
            self.anchor_index.expect("anchor index must be set before pruning descendants");

        let mut reachable: HashSet<U256> = HashSet::new();
        let mut stack = vec![anchor_index];

        while let Some(index) = stack.pop() {
            if !reachable.insert(index) {
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
                        if anchor_index == index {
                            Some(game.index)
                        } else {
                            None
                        }
                    } else if U256::from(game.parent_index) == index {
                        Some(game.index)
                    } else {
                        None
                    }
                })
                .collect();

            stack.extend(children);
        }

        self.games.retain(|index, _| reachable.contains(index));

        self.recompute_canonical_head();
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
                        None
                    }
                })
                .collect();

            stack.extend(children);
        }

        for index in to_remove {
            self.games.remove(&index);
        }

        self.recompute_canonical_head();
    }

    /// Checks if a game's parent is in a resolved state, allowing this game to be resolved.
    ///
    /// A game can only be resolved after its parent is no longer IN_PROGRESS.
    /// Root games (parent_index = u32::MAX) are always considered ready.
    fn parent_ready(&self, game: &Game) -> bool {
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
    fn proposer_ready(game: &Game, now_ts: u64) -> bool {
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
    fn challenger_ready(game: &Game, now_ts: u64) -> bool {
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
            .filter(|game| self.parent_ready(game))
            .filter(|game| match mode {
                Mode::Proposer => Self::proposer_ready(game, now_ts),
                Mode::Challenger => Self::challenger_ready(game, now_ts),
            })
            .map(|game| game.index)
            .collect()
    }

    /// Returns games where the proposer can claim their bond.
    ///
    /// A game is claimable when:
    /// 1. The proposer won (DEFENDER_WINS)
    /// 2. The game has been finalized on-chain
    /// 3. Credit is still available (not already claimed)
    ///
    /// Returns tuples of (game_index, game_address) sorted by index.
    fn claimable_games(&self) -> Vec<(U256, Address)> {
        let mut games: Vec<&Game> = self.games.values().collect();
        games.sort_by_key(|game| game.index);

        games
            .into_iter()
            .filter(|game| game.status == GameStatus::DEFENDER_WINS)
            .filter(|game| game.finalized)
            .filter(|game| game.proposer_credit_available)
            .map(|game| (game.index, game.address))
            .collect()
    }

    /// Marks a game's credit as claimed to prevent duplicate claim attempts.
    fn mark_credit_claimed(&mut self, index: U256) {
        if let Some(game) = self.games.get_mut(&index) {
            game.proposer_credit_available = false;
        }
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
            state: Arc::new(Mutex::new(ProposerState::default())),
        })
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
    ) -> Result<(Address, U256)> {
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

        Ok((game_address, game_index))
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
        let anchor_registry_address =
            self.factory.get_anchor_state_registry_address(self.config.game_type).await?;
        let anchor_registry =
            AnchorStateRegistry::new(anchor_registry_address, self.l1_provider.clone());
        let anchor_game = anchor_registry.anchorGame().call().await?;

        if anchor_game == Address::ZERO {
            let anchor_root_result = anchor_registry.getAnchorRoot().call().await?;
            let anchor_l2_block = U256::from(anchor_root_result._1);

            {
                let mut state = self.state.lock().await;
                state.anchor_index = None;
                state.anchor_address = None;
                state.anchor_l2_block = Some(anchor_l2_block);

                if state.canonical_head_index.is_none() {
                    state.canonical_head_l2_block = Some(anchor_l2_block);
                }

                if state.games.is_empty() {
                    state.cursor = None;
                }
            }

            self.load_new_games().await?;
            self.refresh_cached_game_statuses().await?;
            return Ok(());
        }

        let mut needs_reset = false;
        {
            let mut state = self.state.lock().await;

            if state.anchor_address == Some(anchor_game) {
                // Anchor unchanged; nothing to do here.
            } else if let Some(anchor_index) = state.find_index_by_address(anchor_game) {
                if let Some(anchor) = state.games.get(&anchor_index).cloned() {
                    state.anchor_index = Some(anchor_index);
                    state.anchor_address = Some(anchor.address);
                    state.anchor_l2_block = Some(anchor.l2_block);
                    state.retain_descendants_of_anchor();
                }
            } else {
                state.reset();
                needs_reset = true;
            }
        }

        if needs_reset {
            self.initialize_anchor(anchor_game).await?;
        }

        self.load_new_games().await?;
        self.refresh_cached_game_statuses().await?;

        Ok(())
    }

    /// Initializes the state with a new anchor game fetched from the factory.
    ///
    /// Called when the anchor changes to a game not in our cache, requiring
    /// us to fetch the game details and set it as our new baseline.
    async fn initialize_anchor(&self, anchor_game: Address) -> Result<()> {
        let Some(anchor_index) = self.find_game_index_by_address(anchor_game).await? else {
            tracing::warn!("Anchor game {:?} not found in factory listings", anchor_game);
            return Ok(());
        };

        let Some(game) = self.fetch_game(anchor_index).await? else {
            tracing::warn!("Anchor game {:?} failed validation when fetching", anchor_game);
            return Ok(());
        };

        {
            let mut state = self.state.lock().await;
            state.set_anchor(&game);
        }

        Ok(())
    }

    /// Incrementally loads new games from the factory starting from the cursor.
    ///
    /// This function:
    /// 1. Determines the starting index (cursor + 1 or 0 if no cursor)
    /// 2. Fetches the latest game index from factory
    /// 3. Loads and processes each game in sequence
    /// 4. Updates the cursor after each game
    ///
    /// Games are validated (correct type, valid output root) before being added.
    async fn load_new_games(&self) -> Result<()> {
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
            if let Some(game) = self.fetch_game(next_index).await? {
                self.process_loaded_game(game).await;
            } else {
                let mut state = self.state.lock().await;
                state.set_cursor(next_index);
            }
            next_index += U256::from(1);
        }

        Ok(())
    }

    /// Processes a loaded game and updates the cache accordingly.
    ///
    /// The processing logic depends on the game's status:
    /// - CHALLENGER_WINS: Remove the game and its entire subtree
    /// - Valid parent: Add to cache and potentially update canonical head
    /// - Invalid parent: Skip (parent not in cache yet)
    ///
    /// The cursor is always updated regardless of whether the game is cached.
    async fn process_loaded_game(&self, game: Game) {
        let mut state = self.state.lock().await;

        if game.status == GameStatus::CHALLENGER_WINS {
            state.remove_subtree(game.index);
            state.set_cursor(game.index);
            return;
        }

        let parent_index_valid = if game.parent_index == u32::MAX {
            true
        } else {
            state.games.contains_key(&U256::from(game.parent_index))
        };

        if parent_index_valid {
            state.games.insert(game.index, game.clone());
            state.update_canonical_head_if_better(&game);
        } else {
            tracing::debug!(
                game_index = %game.index,
                parent_index = game.parent_index,
                "Skipping game with missing parent in cache"
            );
        }

        state.set_cursor(game.index);
    }

    /// Refreshes the status of all cached games from on-chain data.
    ///
    /// For each cached game, this function:
    /// 1. Fetches current on-chain status and proposal state
    /// 2. Checks finalization status in the anchor registry
    /// 3. Checks for available credit (for claimable games)
    /// 4. Updates the cached game state
    /// 5. Removes invalid games (CHALLENGER_WINS) and their subtrees
    ///
    /// This ensures our cache reflects the current on-chain state of all games.
    async fn refresh_cached_game_statuses(&self) -> Result<()> {
        let targets: Vec<(U256, Address)> = {
            let state = self.state.lock().await;
            state.games.values().map(|game| (game.index, game.address)).collect()
        };

        for (index, game_address) in targets {
            let contract = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
            let claim_data = contract.claimData().call().await?;
            let status = contract.status().call().await?;
            let deadline = U256::from(claim_data.deadline).to::<u64>();
            let registry_address = contract.anchorStateRegistry().call().await?;
            let registry = AnchorStateRegistry::new(registry_address, self.l1_provider.clone());
            let is_finalized = registry.isGameFinalized(game_address).call().await?;

            let mut proposer_credit_available = false;
            if is_finalized && status == GameStatus::DEFENDER_WINS {
                proposer_credit_available =
                    contract.credit(self.prover_address).call().await? > U256::ZERO;
            }

            let mut state = self.state.lock().await;
            let mut remove = false;

            if let Some(game) = state.games.get_mut(&index) {
                if status == GameStatus::CHALLENGER_WINS {
                    remove = true;
                } else {
                    game.status = status;
                    game.proposal_status = claim_data.status;
                    game.deadline = deadline;
                    game.finalized = is_finalized;
                    game.proposer_credit_available = proposer_credit_available;
                }
            }

            if remove {
                state.remove_subtree(index);
            }
        }

        Ok(())
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

    async fn mark_credit_claimed(&self, index: U256) {
        let mut state = self.state.lock().await;
        state.mark_credit_claimed(index);
    }

    async fn fetch_game(&self, index: U256) -> Result<Option<Game>> {
        let game = self.factory.gameAtIndex(index).call().await?;

        if game.gameType != self.config.game_type {
            return Ok(None);
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
            return Ok(None);
        }

        let claim_data = contract.claimData().call().await?;
        let status = contract.status().call().await?;
        let deadline = U256::from(claim_data.deadline).to::<u64>();

        Ok(Some(Game {
            index,
            address: game_address,
            parent_index: claim_data.parentIndex,
            l2_block,
            status,
            proposal_status: claim_data.status,
            deadline,
            finalized: false,
            proposer_credit_available: false,
        }))
    }

    async fn find_game_index_by_address(&self, address: Address) -> Result<Option<U256>> {
        let game_count = self.factory.gameCount().call().await?;

        if game_count == U256::ZERO {
            return Ok(None);
        }

        let mut index = game_count - U256::from(1);

        loop {
            let game = self.factory.gameAtIndex(index).call().await?;
            if game.proxy == address {
                return Ok(Some(index));
            }

            if index == U256::ZERO {
                break;
            }

            index -= U256::from(1);
        }

        Ok(None)
    }

    async fn state_snapshot(&self) -> Option<StateSnapshot> {
        let state = self.state.lock().await;
        state.snapshot()
    }

    /// Handles the creation of a new game if conditions are met.
    /// Returns the address of the created game, if one was created.
    #[tracing::instrument(name = "[[Proposing]]", skip(self))]
    pub async fn handle_game_creation(&self) -> Result<Option<Address>> {
        let snapshot = {
            let state = self.state.lock().await;
            state.snapshot()
        };

        let Some(snapshot) = snapshot else {
            tracing::info!("State not initialized; skipping game creation");
            return Ok(None);
        };

        let latest_proposed_block_number = snapshot.canonical_head_l2_block;
        let next_l2_block_number_for_proposal =
            latest_proposed_block_number + U256::from(self.config.proposal_interval_in_blocks);
        let parent_game_index =
            snapshot.canonical_head_index.map(|index| index.to::<u32>()).unwrap_or(u32::MAX);

        let finalized_l2_head_block_number = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, latest_proposed_block_number.to::<u64>())
            .await?;

        // There's always a new game to propose, as the chain is always moving forward from the
        // genesis block set for the game type. Only create a new game if the finalized L2
        // head block number is at least the next L2 block number for proposal.
        if let Some(finalized_block) = finalized_l2_head_block_number {
            if U256::from(finalized_block) >= next_l2_block_number_for_proposal {
                let (game_address, game_index) =
                    self.create_game(next_l2_block_number_for_proposal, parent_game_index).await?;

                if let Some(game) = self.fetch_game(game_index).await? {
                    self.process_loaded_game(game).await;
                } else {
                    let mut state = self.state.lock().await;
                    state.set_cursor(game_index);
                }

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
            self.mark_credit_claimed(index).await;
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
                self.mark_credit_claimed(index).await;
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
        if let Some(snapshot) = self.state_snapshot().await {
            ProposerGauge::LatestGameL2BlockNumber
                .set(snapshot.canonical_head_l2_block.to::<u64>() as f64);

            if let Some(finalized_l2_block_number) = self
                .host
                .get_finalized_l2_block_number(
                    &self.fetcher,
                    snapshot.canonical_head_l2_block.to::<u64>(),
                )
                .await?
            {
                ProposerGauge::FinalizedL2BlockNumber.set(finalized_l2_block_number as f64);
            }

            ProposerGauge::AnchorGameL2BlockNumber.set(snapshot.anchor_l2_block.to::<u64>() as f64);
        } else {
            tracing::info!("No state snapshot available for metrics update");
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

    /// Runs the proposer indefinitely.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        tracing::info!("OP Succinct Proposer running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        // Spawn a dedicated task for continuous metrics collection
        self.spawn_metrics_collector();

        loop {
            interval.tick().await;

            if let Err(e) = self.refresh_state().await {
                tracing::warn!("Failed to refresh proposer state: {:?}", e);
            }

            // 1. Handle completed tasks
            if let Err(e) = self.handle_completed_tasks().await {
                tracing::warn!("Failed to handle completed tasks: {:?}", e);
            }

            // 2. Spawn new work (non-blocking)
            if let Err(e) = self.spawn_pending_operations().await {
                tracing::warn!("Failed to spawn pending operations: {:?}", e);
            }

            // 3. Log task statistics
            self.log_task_stats().await;
        }
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
        let should_create = self.should_create_game().await?;
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

        // Get the next proposal block for task info
        let next_block = self.get_next_proposal_block().await.unwrap_or(U256::ZERO);
        let task_info = TaskInfo::GameCreation { block_number: next_block };

        self.tasks.lock().await.insert(task_id, (handle, task_info));
        tracing::info!("Spawned game creation task {} for block {}", task_id, next_block);
        Ok(true)
    }

    /// Check if we should create a game
    async fn should_create_game(&self) -> Result<bool> {
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
                return Ok(false);
            }
        }

        // Get the next L2 block number for proposal
        let next_l2_block_number_for_proposal = self.get_next_proposal_block().await?;

        // Get the latest proposed block number to use for finalized block check
        // If there's no snapshot, fall back to the anchor block
        let latest_proposed_block_number = if let Some(snapshot) = self.state_snapshot().await {
            snapshot.canonical_head_l2_block
        } else {
            // Use anchor block as the reference point when no snapshot exists
            self.factory.get_anchor_l2_block_number(self.config.game_type).await?
        };

        let finalized_l2_head_block_number = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, latest_proposed_block_number.to::<u64>())
            .await?;

        Ok(finalized_l2_head_block_number
            .map(|finalized_block| U256::from(finalized_block) >= next_l2_block_number_for_proposal)
            .unwrap_or(false))
    }

    /// Get the next proposal block number
    async fn get_next_proposal_block(&self) -> Result<U256> {
        if let Some(snapshot) = self.state_snapshot().await {
            Ok(snapshot.canonical_head_l2_block +
                U256::from(self.config.proposal_interval_in_blocks))
        } else {
            let anchor_l2_block_number =
                self.factory.get_anchor_l2_block_number(self.config.game_type).await?;
            Ok(anchor_l2_block_number
                .checked_add(U256::from(self.config.proposal_interval_in_blocks))
                .unwrap())
        }
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
        let candidate = {
            let state = self.state.lock().await;
            state.claimable_games().into_iter().next()
        };

        let Some((index, game_address)) = candidate else {
            return Ok(false);
        };

        let proposer = self.clone();
        let task_id = self.next_task_id.fetch_add(1, Ordering::Relaxed);

        let handle = tokio::spawn(async move {
            match proposer.claim_bond_for_game(index, game_address).await {
                Ok(Action::Performed) => {
                    ProposerGauge::GamesBondsClaimed.increment(1.0);
                    Ok(())
                }
                Ok(Action::Skipped) => Ok(()),
                Err(e) => Err(e),
            }
        });

        let task_info = TaskInfo::BondClaim;
        self.tasks.lock().await.insert(task_id, (handle, task_info));
        tracing::info!("Spawned bond claim task {}", task_id);
        Ok(true)
    }
}
