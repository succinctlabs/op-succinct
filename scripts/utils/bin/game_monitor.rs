use alloy_primitives::{Address, U256};
use alloy_provider::ProviderBuilder;
use anyhow::{Context, Result};
use clap::Parser;
use fault_proof::contract::{
    DisputeGameFactory::DisputeGameFactoryInstance, OPSuccinctFaultDisputeGame,
};
use log::{error, info, warn};
use std::{
    collections::{HashMap, HashSet},
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

    // The index of the game to start checking from. If unset the monitor will
    #[arg(long, default_value = None)]
    pub start_index: Option<u64>,
}

/// Represents a running cost estimator process for a game.
struct RunningEstimator {
    started_at: Instant,
    process: Child,
    log_file: PathBuf,
}

/// Tracks the state of the game monitor.
struct MonitorState {
    /// Set of game addresses we've already spawned estimators for.
    processed_games: HashSet<Address>,
    /// Currently running estimator processes.
    running_processes: HashMap<u64, RunningEstimator>,
    /// The next game index to check.
    next_game_index: u64,
}

impl MonitorState {
    fn new(next_game_index: u64) -> Self {
        Self { processed_games: HashSet::new(), running_processes: HashMap::new(), next_game_index }
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

    info!("Running cost estimator:: {}", cmd);
    info!("Logging to: {:}", log_file.display());

    let child = Command::new(cost_estimator_binary_path)
        .args(args)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .context("Failed to spawn cost estimator process")?;

    Ok(child)
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
    // Main monitoring loop
    loop {
        // Clean up any finished processes
        state.cleanup_finished_processes();

        info!("Running processes: {}/{}", state.running_processes.len(), args.max_concurrent);
        // Get current game count
        let current_game_count = factory.gameCount().call().await?.to::<u64>();
        if state.can_spawn_new(args.max_concurrent) {
            // Check for new games
            let game_index = state.next_game_index;
            if current_game_count > game_index {
                state.next_game_index = game_index + 1;
                // Get game info
                let game_info = match factory.gameAtIndex(U256::from(game_index)).call().await {
                    Ok(info) => info,
                    Err(e) => {
                        error!("Failed to get game at index {}: {}", game_index, e);
                        continue;
                    }
                };

                let game_type = game_info.gameType;
                let game_address = game_info.proxy;
                // Check if it's the game type we're monitoring
                if game_type != GAME_TYPE {
                    info!(
                        "Skipping game {} at index {} (type {} != {})",
                        game_address, game_index, game_type, GAME_TYPE
                    );
                    continue;
                }

                // Check if we've already processed this game
                if state.processed_games.contains(&game_address) {
                    info!("Already processed game {}, skipping", game_address);
                    continue;
                }

                info!(
                    "Found new game of type {} at index {}: {}",
                    game_type, game_index, game_address
                );

                // Get the game contract
                let game = OPSuccinctFaultDisputeGame::new(game_address, l1_provider.clone());

                // Get the L2 block number for this game
                let l2_block_number = match game.l2BlockNumber().call().await {
                    Ok(block) => block.to::<u64>(),
                    Err(e) => {
                        error!("Failed to get L2 block number for game {}: {}", game_address, e);
                        continue;
                    }
                };

                // Get the start block from the game contract
                let start_block = match game.startingBlockNumber().call().await {
                    Ok(block) => block.to::<u64>(),
                    Err(e) => {
                        warn!(
                            "Failed to get starting block number for game {}: {}",
                            game_address, e
                        );
                        0
                    }
                };
                let end_block = l2_block_number;

                info!("Game {} covers L2 blocks {} to {}", game_address, start_block, end_block);

                let mut log_file =
                    PathBuf::from(format!("cost-estimator-{}-{}.log", game_index, game_address));
                log_file = args.logs_dir.join(log_file);
                // Spawn the cost estimator process
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
                            "Started cost estimator {} for game {} (blocks {}-{})",
                            game_index, game_address, start_block, end_block
                        );

                        state.running_processes.insert(
                            game_index,
                            RunningEstimator {
                                started_at: Instant::now(),
                                process: child,
                                log_file,
                            },
                        );
                        state.processed_games.insert(game_address);
                    }
                    Err(e) => {
                        error!("Failed to spawn cost estimator for game {}: {}", game_address, e);
                    }
                }
            }
        } else {
            info!("Max concurrent processes reached, waiting for one to finish...");
        }

        // If there are no more games to process or we don't have any capacity to spawn a new
        // process then wait.
        if state.next_game_index >= current_game_count || !state.can_spawn_new(args.max_concurrent)
        {
            sleep(poll_interval).await;
        }
    }
}
