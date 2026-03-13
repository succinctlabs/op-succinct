use alloy_primitives::{Address, U256};
use alloy_provider::ProviderBuilder;
use anyhow::{Context, Result};
use clap::Parser;
use fault_proof::contract::{
    DisputeGameFactory::DisputeGameFactoryInstance, OPSuccinctFaultDisputeGame,
};
use log::{error, info, warn};
use std::{
    collections::{HashMap, VecDeque},
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use tokio::time::sleep;

const GAME_TYPE: u32 = 42;
// How long should we let a cost estimator run before killing it?
const VALID_ESTIMATOR_DURATION_IN_SECONDS: u64 = 60 * 60 * 3; // 3 hours
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

    // The index of the game to start checking from. If unset the monitor will
    #[arg(long, default_value = None)]
    pub start_index: Option<u64>,

    /// The time in seconds to wait between discovering a game index and fetching its details
    /// from L1/L2. This delay mitigates node-desync issues that occur when accessing L1 or L2
    /// via a proxy with multiple backends (e.g. gameCount sees a game on one backend but
    /// gameAtIndex fails on another). The default value of 10 minutes should be safe given the
    /// default values used when running op stack nodes.
    #[arg(long, default_value = "600")]
    pub delay: u64,
}

/// Represents a running cost estimator process for a game.
struct RunningEstimator {
    started_at: Instant,
    process: Child,
    log_file: PathBuf,
}

/// A game index discovered from the factory, waiting for its delay to elapse before
/// fetching game details and spawning the cost estimator.
struct PendingGame {
    discovered_at: Instant,
    game_index: u64,
}

struct MonitorState {
    running_processes: HashMap<u64, RunningEstimator>,
    pending_games: VecDeque<PendingGame>,
    next_game_index: u64,
}

impl MonitorState {
    fn new(next_game_index: u64) -> Self {
        Self { running_processes: HashMap::new(), pending_games: VecDeque::new(), next_game_index }
    }

    /// Clean up finished processes and return their results.
    fn cleanup_finished_processes(&mut self) {
        let mut finished = Vec::new();

        for (id, estimator) in self.running_processes.iter_mut() {
            match estimator.process.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        info!(
                            "Cost estimator {} completed successfully, log file: {}",
                            id,
                            estimator.log_file.display(),
                        );
                    } else {
                        error!(
                            "Cost estimator {} failed with status {:?}, log file: {})",
                            id,
                            status,
                            estimator.log_file.display(),
                        );
                    }
                    finished.push(*id);
                }
                Ok(None) => {
                    let duration = Instant::now().duration_since(estimator.started_at);
                    if duration.as_secs() > VALID_ESTIMATOR_DURATION_IN_SECONDS {
                        error!("Cost estimator {} is still running for more than 3 hours, log file: {}. Killing it", id, estimator.log_file.display());
                        let _ = estimator.process.kill();
                        finished.push(*id);
                    }
                }
                Err(e) => {
                    error!("Error checking process {}: {}", id, e);
                    finished.push(*id);
                }
            }
        }

        for id in finished {
            self.running_processes.remove(&id);
        }
    }

    /// Check if we can spawn a new process.
    fn can_spawn_new(&self, max_concurrent: usize) -> bool {
        self.running_processes.len() < max_concurrent
    }
}

/// Spawns a cost estimator process for the given block range.
fn spawn_cost_estimator(
    cost_estimator_binary_path: &PathBuf,
    env_file: &Path,
    log_file: &PathBuf,
    start_block: u64,
    end_block: u64,
    batch_size: u64,
) -> Result<Child> {
    let args = [
        "--start",
        &start_block.to_string(),
        "--end",
        &end_block.to_string(),
        "--batch-size",
        &batch_size.to_string(),
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

fn extract_game_index(path: &Path) -> Option<u64> {
    let filename = path.file_name()?.to_str()?;
    let stripped = filename.strip_prefix("cost-estimator-")?;
    let dash_pos = stripped.find('-')?;
    stripped[..dash_pos].parse().ok()
}

fn enforce_log_space_limit(
    logs_dir: &Path,
    max_size_bytes: u64,
    running_game_indices: &HashMap<u64, RunningEstimator>,
) -> Result<()> {
    let mut log_files: Vec<(PathBuf, u64, u64)> = Vec::new();
    let mut total_size: u64 = 0;

    for entry in fs::read_dir(logs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let size = entry.metadata()?.len();
            total_size += size;
            // Only consider files matching our naming pattern as deletion
            // candidates.
            if let Some(game_index) = extract_game_index(&path) {
                log_files.push((path, size, game_index));
            }
        }
    }

    if total_size <= max_size_bytes {
        return Ok(());
    }

    info!(
        "Log directory size ({:.2} MB) exceeds limit ({:.2} MB), cleaning up oldest logs",
        total_size as f64 / (1024.0 * 1024.0),
        max_size_bytes as f64 / (1024.0 * 1024.0),
    );

    // Sort by game index ascending (oldest first).
    log_files.sort_by_key(|(_, _, idx)| *idx);

    for (path, size, game_index) in &log_files {
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

    Ok(())
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
    info!("Environment file: {}", args.env_file.display());
    info!("Polling interval: {}s", args.poll_interval);
    info!("Max concurrent processes: {}", args.max_concurrent);
    info!("Cost estimator binary path: {}", args.cost_estimator_binary_path.display());
    if args.max_logs_size_mb > 0 {
        info!("Max logs size: {} MB", args.max_logs_size_mb);
    } else {
        info!("Max logs size: unlimited");
    }

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
    let mut state = MonitorState::new(next_game_index);

    let poll_interval = Duration::from_secs(args.poll_interval);
    let delay = Duration::from_secs(args.delay);

    // Main monitoring loop
    loop {
        state.cleanup_finished_processes();

        if args.max_logs_size_mb > 0 {
            if let Err(e) = enforce_log_space_limit(
                &args.logs_dir,
                args.max_logs_size_mb * 1024 * 1024,
                &state.running_processes,
            ) {
                error!("Failed to enforce log space limit: {}", e);
            }
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
            if pending.discovered_at.elapsed() < delay {
                break;
            }

            let pending = state.pending_games.pop_front().unwrap();
            let game_index = pending.game_index;

            let game_info = match factory.gameAtIndex(U256::from(game_index)).call().await {
                Ok(info) => info,
                Err(e) => {
                    error!("Failed to get game at index {}: {}. Skipping.", game_index, e);
                    continue;
                }
            };

            let game_type = game_info.gameType;
            let game_address = game_info.proxy;

            if game_type != GAME_TYPE {
                info!(
                    "Skipping game at index {} (type {} != {})",
                    game_index, game_type, GAME_TYPE
                );
                continue;
            }

            info!("Processing game {} at index {}", game_address, game_index);

            let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider.clone());

            let l2_block_number = match game.l2BlockNumber().call().await {
                Ok(block) => block.to::<u64>(),
                Err(e) => {
                    error!(
                        "Failed to get L2 block number for game {} at index {}: {}. Skipping.",
                        game_address, game_index, e
                    );
                    continue;
                }
            };

            let start_block = match game.startingBlockNumber().call().await {
                Ok(block) => block.to::<u64>(),
                Err(e) => {
                    error!(
                        "Failed to get staring block number for game {} at index {}: {}. Skipping.",
                        game_address, game_index, e
                    );
                    continue;
                }
            };
            let end_block = l2_block_number;

            info!("Game {} covers L2 blocks {} to {}", game_address, start_block, end_block);

            let log_file =
                args.logs_dir.join(format!("cost-estimator-{}-{}.log", game_index, game_address));

            match spawn_cost_estimator(
                &args.cost_estimator_binary_path,
                &args.env_file,
                &log_file,
                start_block,
                end_block,
                end_block - start_block,
            ) {
                Ok(child) => {
                    info!(
                        "Started cost estimator for game {} at index {} (blocks {}-{})",
                        game_address, game_index, start_block, end_block
                    );
                    state.running_processes.insert(
                        game_index,
                        RunningEstimator { started_at: Instant::now(), process: child, log_file },
                    );
                }
                Err(e) => {
                    error!("Failed to spawn cost estimator for game {}: {}", game_address, e);
                }
            }
        }

        // Discover new game indices and queue them for deferred processing.
        let current_game_count = match factory.gameCount().call().await {
            Ok(count) => count.to::<u64>(),
            Err(e) => {
                error!(
                    "Failed to Fetch gameCount from factory {}: {}. Retrying",
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
            state
                .pending_games
                .push_back(PendingGame { discovered_at: Instant::now(), game_index });
        }

        sleep(poll_interval).await;
    }
}
