use alloy_primitives::{Address, U256};
use alloy_provider::ProviderBuilder;
use anyhow::{Context, Result};
use clap::Parser;
use fault_proof::contract::{
    DisputeGameFactory::DisputeGameFactoryInstance, OPSuccinctFaultDisputeGame,
};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};

const GAME_TYPE: u32 = 42;
const MAX_RETRIES: u32 = 3;
/// Kill a process if its log file is this many times larger than the median of peers.
const LOG_VOLUME_KILL_MULTIPLIER: f64 = 10.0;
/// Minimum number of completion history entries required to perform median comparison.
const MEDIAN_THRESHOLD: usize = 3;
/// Kill a process if its time-per-block exceeds this multiplier of the median of completed
/// processes.
const RUNTIME_KILL_MULTIPLIER: f64 = 5.0;

/// Arguments for the game monitor.
#[derive(Debug, Clone, Parser)]
pub struct GameMonitorArgs {
    /// The environment file to use. This file should contain the following environment variables:
    ///
    /// - DISPUTE_GAME_FACTORY_ADDRESS: The address of the dispute game factory contract.
    ///
    /// - L1_RPC: The URL of the L1 RPC endpoint.
    ///
    /// - L1_BEACON_RPC: The URL of the L1 beacon RPC endpoint.
    ///
    /// - L2_RPC: The URL of the L2 RPC endpoint.
    ///
    /// - L2_NODE_RPC: The URL of the L2 node RPC endpoint.
    ///
    /// - EIGENDA_PROXY_ADDRESS: The address of the eigenda proxy service.
    ///
    /// - OP_SUCCINCT_MOCK: Must be 'true'
    ///
    /// - SP1_PROVER: Must be 'mock'
    #[arg(long, default_value = ".env")]
    pub env_file: PathBuf,

    /// The polling interval in seconds.
    #[arg(long, default_value = "30")]
    pub poll_interval: u64,

    /// Maximum number of concurrent cost estimator processes.
    #[arg(long, default_value = "5")]
    pub max_concurrent: usize,

    /// The path to the cost estimator binary.
    #[arg(long, default_value = "cost-estimator")]
    pub cost_estimator_binary_path: PathBuf,

    /// The directory under which to store the logs.
    #[arg(long, default_value = "logs")]
    pub logs_dir: PathBuf,

    /// Maximum total size in megabytes for the logs directory. When this limit
    /// is exceeded, the oldest log files (by game index) are deleted until the
    /// total size is within the limit. Log files for currently running processes
    /// are never deleted. A value of 0 disables the limit.
    #[arg(long, default_value = "0")]
    pub max_logs_size_mb: u64,

    /// The index of the game to start checking from. If unset the monitor will start with the most
    /// recently created game.
    #[arg(long, default_value = None)]
    pub start_index: Option<u64>,

    /// The time in seconds to wait between discovering a game index and fetching its details
    /// from L1/L2. This delay mitigates node-desync issues that occur when accessing L1 or L2
    /// via a proxy with multiple backends (e.g. gameCount sees a game on one backend but
    /// gameAtIndex fails on another). The default value of 10 minutes should be safe given the
    /// default values used when running op stack nodes.
    #[arg(long, default_value = "600")]
    pub delay: u64,

    /// Maximum duration in seconds before a cost estimator process is killed.
    /// When completion history is available, a relative check (based on time-per-block
    /// vs completed processes) may kill sooner. This value acts as the absolute ceiling.
    #[arg(long, default_value = "10800")]
    pub max_process_duration_secs: u64,

    /// Maximum number of entries retained in the completion history used for anomaly
    /// detection (time-per-block and log-size outliers). Older entries are discarded first.
    #[arg(long, default_value = "50")]
    pub max_history_length: usize,

    /// Path to the completion history file. Defaults to `<logs_dir>/completion_history.json`.
    #[arg(long)]
    pub history_file: Option<PathBuf>,
}

/// Represents a running cost estimator process for a game.
struct RunningEstimator {
    started_at: Instant,
    process: Child,
    log_file: PathBuf,
    block_range: u64,
    retries: u32,
}

/// A game index discovered from the factory, waiting for its delay to elapse before
/// fetching game details and spawning the cost estimator.
struct PendingGame {
    discovered_at: Instant,
    game_index: u64,
    retries: u32,
}

struct GameData {
    game_index: u64,
    game_address: Address,
    start_block: u64,
    end_block: u64,
}

impl GameData {
    fn block_range(&self) -> u64 {
        self.end_block.saturating_sub(self.start_block)
    }
}

enum ProcessAction {
    Success { duration: Duration, block_range: u64 },
    Kill { reason: String },
    Retry { reason: String },
}

/// A record of a successfully completed cost estimator process, used for anomaly detection.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CompletionRecord {
    /// Execution duration.
    duration: Duration,
    /// Final log file size in bytes.
    log_size: u64,
    /// Block count
    block_range: u64,
}

struct MonitorState {
    running_processes: HashMap<u64, RunningEstimator>,
    pending_games: VecDeque<PendingGame>,
    next_game_index: u64,
    completion_history: VecDeque<CompletionRecord>,
    max_process_duration_secs: u64,
    max_history_length: usize,
    history_file: PathBuf,
}

impl MonitorState {
    fn new(
        next_game_index: u64,
        max_process_duration_secs: u64,
        max_history_length: usize,
        history_file: PathBuf,
    ) -> Self {
        let completion_history = Self::load_history(&history_file, max_history_length);
        info!(
            "Loaded {} completion history entries from {}",
            completion_history.len(),
            history_file.display()
        );
        Self {
            running_processes: HashMap::new(),
            pending_games: VecDeque::new(),
            next_game_index,
            completion_history,
            max_process_duration_secs,
            max_history_length,
            history_file,
        }
    }

    fn load_history(path: &Path, max_length: usize) -> VecDeque<CompletionRecord> {
        let data = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => return VecDeque::new(),
        };
        let mut records: VecDeque<CompletionRecord> = match serde_json::from_str(&data) {
            Ok(records) => records,
            Err(e) => {
                warn!("Failed to parse completion history from {}: {}", path.display(), e);
                return VecDeque::new();
            }
        };
        while records.len() > max_length {
            records.pop_front();
        }
        records
    }

    fn save_history(&self) {
        match serde_json::to_string(&self.completion_history) {
            Ok(data) => {
                if let Err(e) = fs::write(&self.history_file, data) {
                    warn!(
                        "Failed to write completion history to {}: {}",
                        self.history_file.display(),
                        e
                    );
                }
            }
            Err(e) => {
                warn!("Failed to serialize completion history: {}", e);
            }
        }
    }

    fn push_completion(&mut self, record: CompletionRecord) {
        if self.max_history_length == 0 {
            return;
        }
        if self.completion_history.len() >= self.max_history_length {
            self.completion_history.pop_front();
        }
        self.completion_history.push_back(record);
        self.save_history();
    }

    fn cleanup_finished_processes(&mut self) {
        // Calculate median log size per block
        let lpb_values: Vec<f64> = self
            .completion_history
            .iter()
            .map(|r| r.log_size as f64 / r.block_range as f64)
            .collect();
        let median_lpb: Option<f64> = median(&lpb_values, MEDIAN_THRESHOLD);

        // Calculate median time per block
        let tpb_values: Vec<f64> = self
            .completion_history
            .iter()
            .map(|r| r.duration.as_secs_f64() / r.block_range as f64)
            .collect();
        let median_tpb: Option<f64> = median(&tpb_values, MEDIAN_THRESHOLD);

        let running_log_sizes: HashMap<u64, u64> = self
            .running_processes
            .iter()
            .map(|(id, est)| {
                let size = fs::metadata(&est.log_file).map(|m| m.len()).unwrap_or(0);
                (*id, size)
            })
            .collect();

        let mut process_actions: Vec<(u64, ProcessAction)> = Vec::new();

        for (id, estimator) in self.running_processes.iter_mut() {
            let elapsed = estimator.started_at.elapsed();

            match estimator.process.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        info!(
                            "Cost estimator {} completed successfully, log file: {}",
                            id,
                            estimator.log_file.display(),
                        );
                        process_actions.push((
                            *id,
                            ProcessAction::Success {
                                duration: elapsed,
                                block_range: estimator.block_range,
                            },
                        ));
                    } else {
                        error!(
                            "Cost estimator {} failed with status {:?}, log file: {}",
                            id,
                            status,
                            estimator.log_file.display(),
                        );
                        process_actions.push((
                            *id,
                            ProcessAction::Retry { reason: format!("exit status {:?}", status) },
                        ));
                    }
                }
                Ok(None) => {
                    let kill_reason = (|| {
                        if elapsed.as_secs() > self.max_process_duration_secs {
                            return Some(format!(
                                "exceeded maximum duration of {}s",
                                self.max_process_duration_secs
                            ));
                        }
                        if estimator.block_range > 0 {
                            if let Some(med_tpb) = median_tpb {
                                let current_tpb =
                                    elapsed.as_secs_f64() / estimator.block_range as f64;
                                if current_tpb > RUNTIME_KILL_MULTIPLIER * med_tpb {
                                    return Some(format!(
                                        "time per block ({:.1}s) exceeds {:.0}x median ({:.1}s)",
                                        current_tpb, RUNTIME_KILL_MULTIPLIER, med_tpb
                                    ));
                                }
                            }
                            if let Some(med_lpb) = median_lpb {
                                let current_lpb = running_log_sizes.get(id).copied().unwrap_or(0)
                                    as f64 /
                                    estimator.block_range as f64;
                                if current_lpb > LOG_VOLUME_KILL_MULTIPLIER * med_lpb {
                                    return Some(format!(
                                        "log size ({:.1} MB) exceeds {:.0}x median ({:.1} MB)",
                                        current_lpb / (1024.0 * 1024.0),
                                        LOG_VOLUME_KILL_MULTIPLIER,
                                        med_lpb / (1024.0 * 1024.0)
                                    ));
                                }
                            }
                        }
                        None
                    })();

                    if let Some(reason) = kill_reason {
                        error!(
                            "Cost estimator {} is out of control ({}), log file: {}. Killing it.",
                            id,
                            reason,
                            estimator.log_file.display()
                        );
                        process_actions.push((*id, ProcessAction::Kill { reason }));
                    }
                }
                Err(e) => {
                    error!("Error checking process {}: {}", id, e);
                }
            }
        }

        for (id, action) in process_actions {
            match action {
                ProcessAction::Success { duration, block_range } => {
                    if let Some(est) = self.running_processes.remove(&id) {
                        let log_size = fs::metadata(&est.log_file).map(|m| m.len()).unwrap_or(0);
                        LogFile::mark_complete(&est.log_file, true);
                        if block_range > 0 {
                            self.push_completion(CompletionRecord {
                                duration,
                                log_size,
                                block_range,
                            });
                        }
                    }
                }
                ProcessAction::Kill { reason } => {
                    if let Some(mut est) = self.running_processes.remove(&id) {
                        let _ = est.process.kill();
                        LogFile::mark_complete(&est.log_file, false);
                        self.maybe_requeue(id, est.retries, &reason);
                    }
                }
                ProcessAction::Retry { reason } => {
                    if let Some(est) = self.running_processes.remove(&id) {
                        LogFile::mark_complete(&est.log_file, false);
                        self.maybe_requeue(id, est.retries, &reason);
                    }
                }
            }
        }
    }

    fn maybe_requeue(&mut self, game_index: u64, retries: u32, reason: &str) {
        if retries < MAX_RETRIES {
            let new_retries = retries + 1;
            warn!(
                "Re-queuing game {} for retry {}/{} ({})",
                game_index, new_retries, MAX_RETRIES, reason
            );
            self.pending_games.push_back(PendingGame {
                discovered_at: Instant::now(),
                game_index,
                retries: new_retries,
            });
        } else {
            error!(
                "Game {} failed after {} retries ({}), giving up.",
                game_index, MAX_RETRIES, reason
            );
        }
    }

    fn can_spawn_new(&self, max_concurrent: usize) -> bool {
        self.running_processes.len() < max_concurrent
    }

    fn shutdown(&mut self) {
        info!("Shutting down: killing {} running processes", self.running_processes.len());
        for (id, mut est) in self.running_processes.drain() {
            if let Err(e) = est.process.kill() {
                warn!("Failed to kill process for game {}: {}", id, e);
            }
            if let Err(e) = fs::remove_file(&est.log_file) {
                warn!("Failed to delete log file {}: {}", est.log_file.display(), e);
            }
        }
    }
}

/// Compute the median of a slice of f64 values. Returns if the length of the slice is below the
/// threshold.
fn median(values: &[f64], threshold: usize) -> Option<f64> {
    if values.len() < threshold {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

#[derive(Debug, thiserror::Error)]
enum FetchGameError {
    #[error("game {game_index} has type {game_type}, expected {expected}")]
    WrongGameType { game_index: u64, game_type: u32, expected: u32 },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

async fn fetch_game_data<P: alloy_provider::Provider + Clone>(
    pending: &PendingGame,
    factory: &DisputeGameFactoryInstance<P>,
    l1_provider: P,
) -> Result<GameData, FetchGameError> {
    let game_index = pending.game_index;

    let game_info = factory
        .gameAtIndex(U256::from(game_index))
        .call()
        .await
        .context("failed to get game at index")?;

    let game_type = game_info.gameType;
    if game_type != GAME_TYPE {
        return Err(FetchGameError::WrongGameType { game_index, game_type, expected: GAME_TYPE });
    }

    let game_address = game_info.proxy;

    let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider);

    let l2_block_number =
        game.l2BlockNumber().call().await.context("failed to get L2 block number")?.to::<u64>();

    let start_block = game
        .startingBlockNumber()
        .call()
        .await
        .context("failed to get starting block number")?
        .to::<u64>();

    Ok(GameData { game_index, game_address, start_block, end_block: l2_block_number })
}

fn spawn_cost_estimator(
    cost_estimator_binary_path: &PathBuf,
    env_file: &Path,
    log_file: &PathBuf,
    game_data: &GameData,
) -> Result<Child> {
    let args = [
        "--start",
        &game_data.start_block.to_string(),
        "--end",
        &game_data.end_block.to_string(),
        "--batch-size",
        &game_data.block_range().to_string(),
        "--env-file",
        env_file.to_str().unwrap(),
    ];

    let cmd = format!("{} {}", cost_estimator_binary_path.display(), args.join(" "));

    // Write command and env to log file to facilitate easy re-running of the command.
    let mut log_file_handle = File::create(log_file)?;
    writeln!(log_file_handle, "=== Cost Estimator Command ===")?;
    writeln!(log_file_handle, "{}", cmd)?;
    writeln!(log_file_handle, "=== Cost Estimator ENV ===")?;
    let relevant_vars = [
        "DISPUTE_GAME_FACTORY_ADDRESS",
        "L1_RPC",
        "L1_BEACON_RPC",
        "L2_RPC",
        "L2_NODE_RPC",
        "EIGENDA_PROXY_ADDRESS",
        "OP_SUCCINCT_MOCK",
        "SP1_PROVER",
    ];
    for var in relevant_vars {
        if let Ok(value) = env::var(var) {
            writeln!(log_file_handle, "{}={}", var, value)?;
        }
    }
    writeln!(log_file_handle, "=== Output ===")?;
    writeln!(log_file_handle)?;

    // Create log file for this specific run
    let stdout_file = log_file_handle.try_clone()?;
    let stderr_file = log_file_handle.try_clone()?;

    info!("Running cost estimator: {}", cmd);
    info!("Logging to: {}", log_file.display());

    let child = Command::new(cost_estimator_binary_path)
        .args(args)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .context("Failed to spawn cost estimator process")?;

    Ok(child)
}

struct LogFile;

impl LogFile {
    fn path(logs_dir: &Path, game_index: u64, game_address: Address, retries: u32) -> PathBuf {
        if retries > 0 {
            logs_dir.join(format!(
                "cost-estimator-{}-{}-retry{}.log",
                game_index, game_address, retries
            ))
        } else {
            logs_dir.join(format!("cost-estimator-{}-{}.log", game_index, game_address))
        }
    }

    fn extract_game_index(path: &Path) -> Option<u64> {
        let filename = path.file_name()?.to_str()?;
        let stripped = filename.strip_prefix("cost-estimator-")?;
        let dash_pos = stripped.find('-')?;
        stripped[..dash_pos].parse().ok()
    }

    fn mark_complete(path: &Path, success: bool) {
        let Some(filename) = path.file_name().and_then(|f| f.to_str()) else {
            return;
        };
        let Some(stem) = filename.strip_suffix(".log") else {
            return;
        };
        let suffix = if success { "success" } else { "failure" };
        let new_path = path.with_file_name(format!("{}-{}.log", stem, suffix));
        if let Err(e) = fs::rename(path, &new_path) {
            warn!("Failed to rename log {} to {}: {}", path.display(), new_path.display(), e);
        }
    }
    fn sizes(logs_dir: &Path) -> Result<Vec<(PathBuf, u64, u64)>> {
        let mut log_files: Vec<(PathBuf, u64, u64)> = Vec::new();
        for entry in fs::read_dir(logs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let size = entry.metadata()?.len();
                // Only consider files matching our naming pattern as deletion
                // candidates.
                if let Some(game_index) = Self::extract_game_index(&path) {
                    log_files.push((path, size, game_index));
                }
            }
        }
        Ok(log_files)
    }
}

fn enforce_log_space_limit(
    max_size_bytes: u64,
    running_game_indices: &HashMap<u64, RunningEstimator>,
    logs_dir: &Path,
) {
    let mut log_files = match LogFile::sizes(logs_dir) {
        Ok(files) => files,
        Err(e) => {
            warn!("Failed to read log sizes for space enforcement: {}", e);
            return;
        }
    };
    let mut total_size: u64 = log_files.iter().map(|t| t.1).sum();

    if total_size <= max_size_bytes {
        return;
    }

    info!(
        "Log directory size ({:.2} MB) exceeds limit ({:.2} MB), cleaning up oldest logs",
        total_size as f64 / (1024.0 * 1024.0),
        max_size_bytes as f64 / (1024.0 * 1024.0),
    );

    // Sort by game index ascending (oldest first).
    log_files.sort_by_key(|(_, _, idx)| *idx);

    for (path, size, game_index) in log_files.iter() {
        if total_size <= max_size_bytes {
            break;
        }

        if running_game_indices.contains_key(game_index) {
            continue;
        }

        match fs::remove_file(path) {
            Ok(()) => {
                info!("Deleted log file: {}", path.display());
                total_size -= size;
            }
            Err(e) => {
                warn!("Failed to delete log file {}: {}", path.display(), e);
            }
        }
    }

    if total_size > max_size_bytes {
        warn!(
            "Log directory still exceeds limit after cleanup ({:.2} MB remaining), some files may belong to running processes",
            total_size as f64 / (1024.0 * 1024.0),
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = GameMonitorArgs::parse();

    // Load environment variables
    dotenv::from_path(&args.env_file).ok();
    sp1_sdk::utils::setup_logger();
    info!("Game monitor args: {:?}", args);

    // Create the logs directory if it doesn't exist
    if !args.logs_dir.exists() {
        fs::create_dir_all(&args.logs_dir).context("Failed to create logs directory")?;
    }

    info!("Starting game monitor for game type {}", GAME_TYPE);

    // Get required environment variables
    let l1_rpc = env::var("L1_RPC").context("L1_RPC not set")?;
    let dispute_game_factory_address = env::var("DISPUTE_GAME_FACTORY_ADDRESS")
        .context("DISPUTE_GAME_FACTORY_ADDRESS not set")?
        .parse::<Address>()
        .context("Invalid DISPUTE_GAME_FACTORY_ADDRESS")?;

    info!("L1 RPC: {}", l1_rpc);
    info!("Dispute Game Factory: {}", dispute_game_factory_address);

    // Set up L1 provider and factory contract
    let l1_provider = ProviderBuilder::new().connect_http(l1_rpc.parse()?);
    let factory =
        DisputeGameFactoryInstance::new(dispute_game_factory_address, l1_provider.clone());

    // If start_index is unset start from the most recent game, or game at index 0 if there are no
    // games. Otherwise use the start_index.
    let next_game_index = match args.start_index {
        Some(index) => index,
        None => {
            let initial_game_count = factory.gameCount().call().await?.to::<u64>();
            match initial_game_count {
                0 => 0,
                n => n - 1,
            }
        }
    };
    let history_file =
        args.history_file.clone().unwrap_or_else(|| args.logs_dir.join("completion_history.json"));
    let mut state = MonitorState::new(
        next_game_index,
        args.max_process_duration_secs,
        args.max_history_length,
        history_file,
    );

    let poll_interval = Duration::from_secs(args.poll_interval);
    let delay = Duration::from_secs(args.delay);

    let mut sigterm =
        signal(SignalKind::terminate()).context("Failed to register SIGTERM handler")?;
    let mut sigint =
        signal(SignalKind::interrupt()).context("Failed to register SIGINT handler")?;

    'outer: loop {
        tokio::select! {
            _ = sleep(poll_interval) => {}
            _ = sigterm.recv() => {
                info!("Received SIGTERM");
                break;
            }
            _ = sigint.recv() => {
                info!("Received SIGINT");
                break;
            }
        }

        state.cleanup_finished_processes();

        if args.max_logs_size_mb > 0 {
            enforce_log_space_limit(
                args.max_logs_size_mb * 1024 * 1024,
                &state.running_processes,
                &args.logs_dir,
            );
        }

        info!(
            "Running: {}/{}, Pending: {}",
            state.running_processes.len(),
            args.max_concurrent,
            state.pending_games.len()
        );

        // Process pending games whose delay has elapsed: fetch game info and spawn.
        while let Some(pending) = state.pending_games.front() {
            if !state.can_spawn_new(args.max_concurrent) {
                break;
            }
            // Retries skip the discovery delay.
            if pending.retries == 0 && pending.discovered_at.elapsed() < delay {
                break;
            }

            let game_data = match fetch_game_data(pending, &factory, l1_provider.clone()).await {
                Ok(data) => data,
                Err(FetchGameError::WrongGameType { game_index, game_type, expected }) => {
                    debug!(
                        "Skipping game at index {} (type {} != {})",
                        game_index, game_type, expected
                    );
                    state.pending_games.pop_front();
                    continue;
                }
                Err(e) => {
                    warn!(
                        "Failed to fetch game data for index {}: {:#}. Retrying",
                        pending.game_index, e
                    );
                    continue 'outer;
                }
            };

            let pending = state.pending_games.pop_front().unwrap();

            info!(
                "Game {} covers L2 blocks {} to {}",
                game_data.game_address, game_data.start_block, game_data.end_block
            );

            let log_file = LogFile::path(
                &args.logs_dir,
                game_data.game_index,
                game_data.game_address,
                pending.retries,
            );

            let child = spawn_cost_estimator(
                &args.cost_estimator_binary_path,
                &args.env_file,
                &log_file,
                &game_data,
            )?;
            info!(
                "Started cost estimator for game {} at index {} (blocks {}-{})",
                game_data.game_address,
                game_data.game_index,
                game_data.start_block,
                game_data.end_block
            );
            state.running_processes.insert(
                game_data.game_index,
                RunningEstimator {
                    started_at: Instant::now(),
                    process: child,
                    log_file,
                    block_range: game_data.block_range(),
                    retries: pending.retries,
                },
            );
        }

        // Discover new game indices and queue them for deferred processing.
        let current_game_count = match factory.gameCount().call().await {
            Ok(count) => count.to::<u64>(),
            Err(e) => {
                warn!(
                    "Failed to fetch gameCount from factory {}: {}. Retrying",
                    dispute_game_factory_address, e
                );
                continue;
            }
        };
        while state.next_game_index < current_game_count {
            let game_index = state.next_game_index;
            state.next_game_index += 1;

            info!(
                "Discovered new game at index {}, queuing for processing after {:?} delay",
                game_index, delay
            );
            state.pending_games.push_back(PendingGame {
                discovered_at: Instant::now(),
                game_index,
                retries: 0,
            });
        }
    }

    state.shutdown();
    Ok(())
}
