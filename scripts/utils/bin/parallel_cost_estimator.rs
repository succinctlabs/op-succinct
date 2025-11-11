use alloy_eips::BlockId;
use anyhow::Result;
use clap::Parser;
use futures::FutureExt;
use log::{error, info, warn};
use op_succinct_host_utils::{
    block_range::{split_range_basic, SpanBatchRange},
    fetcher::OPSuccinctDataFetcher,
};
use std::{cmp::min, panic::AssertUnwindSafe, path::PathBuf, process::Stdio, sync::Arc};
use tokio::{process::Command, sync::Mutex, task::JoinSet};

/// Parallel cost estimator that runs multiple cost_estimator instances concurrently
#[derive(Parser, Debug, Clone)]
#[command(about = "Runs cost estimator for a range of blocks with configurable concurrency")]
pub struct ParallelCostEstimatorArgs {
    /// Starting block number (inclusive). If not provided, uses latest finalized block from L2
    /// RPC.
    #[arg(long)]
    pub from: Option<u64>,

    /// Ending block number (exclusive). If not provided, calculates as (from - days *
    /// blocks_per_day).
    #[arg(long)]
    pub to: Option<u64>,

    /// Number of days to look back when calculating 'to' from 'from' (assumes 1 second block time)
    #[arg(long, default_value = "14")]
    pub days: u64,

    /// Number of concurrent cost_estimator instances to run
    #[arg(long, default_value = "4")]
    pub concurrency: usize,

    /// The number of blocks to execute in a single batch (passed to cost_estimator)
    #[arg(long, default_value = "10")]
    pub batch_size: u64,

    /// The environment file to use (passed to cost_estimator)
    #[arg(long, default_value = ".env")]
    pub env_file: PathBuf,

    /// Process ranges and batches in reverse order (from highest to lowest block)
    #[arg(long)]
    pub reverse: bool,

    /// Skip writing CSV files and only log execution statistics (passed to cost_estimator)
    #[arg(long, default_value = "true")]
    pub log_only: bool,
}

/// Statistics tracker for parallel execution
#[derive(Debug, Default)]
struct ExecutionTracker {
    completed: usize,
    failed: usize,
    panicked: usize,
    total: usize,
    completed_ranges: Vec<SpanBatchRange>,
    failed_ranges: Vec<SpanBatchRange>,
    panicked_ranges: Vec<SpanBatchRange>,
}

impl ExecutionTracker {
    fn new(total: usize) -> Self {
        Self {
            completed: 0,
            failed: 0,
            panicked: 0,
            total,
            completed_ranges: Vec::new(),
            failed_ranges: Vec::new(),
            panicked_ranges: Vec::new(),
        }
    }

    fn mark_completed(&mut self, range: SpanBatchRange) {
        info!("Completed cost_estimator for blocks {} to {}", range.start, range.end);
        self.completed += 1;
        self.completed_ranges.push(range);
    }

    fn mark_failed(&mut self, range: SpanBatchRange, error: String) {
        error!("Failed to process blocks {} to {} with error: {}", range.start, range.end, error);
        self.failed += 1;
        self.failed_ranges.push(range);
    }
    fn mark_panicked(&mut self, range: SpanBatchRange, panic_msg: String) {
        error!(
            "Panicked while processing blocks {} to {} with message: {}",
            range.start, range.end, panic_msg
        );
        self.panicked += 1;
        self.panicked_ranges.push(range);
    }
}

/// Run a single cost_estimator instance for the given range
async fn run_cost_estimator(
    range: SpanBatchRange,
    args: &ParallelCostEstimatorArgs,
) -> Result<SpanBatchRange> {
    info!("Starting cost_estimator for blocks {} to {}", range.start, range.end);

    // Find cost-estimator binary:
    // 1. Try in PATH (works in Docker or when installed)
    // 2. Try in workspace target/release (works when running locally)
    // 3. Fall back to just "cost-estimator" and let the OS find it
    let cost_estimator_bin = std::env::var("COST_ESTIMATOR_BIN")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            // Try to find in workspace
            cargo_metadata::MetadataCommand::new()
                .exec()
                .ok()
                .map(|metadata| {
                    PathBuf::from(metadata.workspace_root).join("target/release/cost-estimator")
                })
                .filter(|p| p.exists())
        })
        .unwrap_or_else(|| PathBuf::from("cost-estimator"));

    let batch_size = min(args.batch_size, range.end - range.start);
    let mut cmd = Command::new(&cost_estimator_bin);
    cmd.arg("--start")
        .arg(range.start.to_string())
        .arg("--end")
        .arg(range.end.to_string())
        .arg("--batch-size")
        .arg(batch_size.to_string())
        .arg("--default-range")
        .arg(batch_size.to_string())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if args.log_only {
        cmd.arg("--log-only");
    }

    let status = cmd.status().await?;

    if status.success() {
        Ok(range)
    } else {
        anyhow::bail!(
            "cost_estimator failed for blocks {} to {} with exit code: {:?}",
            range.start,
            range.end,
            status.code()
        );
    }
}

/// Process all ranges with controlled concurrency
async fn process_ranges(
    ranges: Vec<SpanBatchRange>,
    args: &ParallelCostEstimatorArgs,
) -> Result<()> {
    let total_ranges = ranges.len();
    info!("Processing {} ranges with concurrency of {}", total_ranges, args.concurrency);

    let tracker = Arc::new(Mutex::new(ExecutionTracker::new(total_ranges)));
    let mut handles = JoinSet::new();
    let mut range_iter = ranges.into_iter();

    // Helper function to spawn a new task
    let spawn_task = |handles: &mut JoinSet<_>,
                      range: SpanBatchRange,
                      args: ParallelCostEstimatorArgs,
                      tracker: Arc<Mutex<ExecutionTracker>>| {
        let range_clone = range.clone();
        handles.spawn(async move {
            // Wrap the execution in catch_unwind to handle panics gracefully
            let result =
                AssertUnwindSafe(run_cost_estimator(range.clone(), &args)).catch_unwind().await;

            let mut tracker = tracker.lock().await;
            match result {
                Ok(Ok(span_range)) => {
                    tracker.mark_completed(span_range.clone());
                    Ok(span_range)
                }
                Ok(Err(e)) => {
                    tracker.mark_failed(range_clone, e.to_string());
                    Err(e)
                }
                Err(panic_err) => {
                    let panic_msg = if let Some(s) = panic_err.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = panic_err.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "Unknown panic".to_string()
                    };
                    tracker.mark_panicked(range_clone, panic_msg.clone());
                    Err(anyhow::anyhow!("Task panicked: {}", panic_msg))
                }
            }
        });
    };

    // Spawn initial batch of tasks
    for _ in 0..args.concurrency {
        if let Some(range) = range_iter.next() {
            spawn_task(&mut handles, range, args.clone(), tracker.clone());
        }
    }

    // Process results and spawn new tasks as slots become available
    while let Some(result) = handles.join_next().await {
        match result {
            Ok(_) => {}
            Err(e) => {
                warn!("Tokio task error (critical): {}", e);
                let mut t = tracker.lock().await;
                t.panicked += 1;
            }
        }

        // Spawn next task if available (regardless of success/failure/panic)
        if let Some(range) = range_iter.next() {
            spawn_task(&mut handles, range, args.clone(), tracker.clone());
        }
    }

    let final_tracker = tracker.lock().await;
    info!(
        "All tasks completed. Success: {}, Failed: {}, Total: {}",
        final_tracker.completed, final_tracker.failed, final_tracker.total
    );

    info!("Completed ranges: {:?}", final_tracker.completed_ranges);
    info!("Failed ranges: {:?}", final_tracker.failed_ranges);
    info!("Panicked ranges: {:?}", final_tracker.panicked_ranges);
    info!("Total ranges: {}", final_tracker.total);
    if final_tracker.failed > 0 || final_tracker.panicked > 0 {
        anyhow::bail!(
            "{} ranges failed, {} ranges panicked, out of {} total ranges",
            final_tracker.failed,
            final_tracker.panicked,
            final_tracker.total
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Info).init();

    let args = ParallelCostEstimatorArgs::parse();

    let from_block = if args.from.is_none() {
        // Only to provided, fetch from from L2 RPC
        info!("'from' not provided, fetching latest finalized block from L2 RPC...");
        dotenv::from_path(&args.env_file).ok();
        let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
        let finalized_header = data_fetcher.get_l2_header(BlockId::finalized()).await?;
        let from = finalized_header.number;
        info!("Using latest finalized block: {}", from);
        from
    } else {
        args.from.unwrap()
    };
    let to_block = if args.to.is_none() {
        // Calculate number of blocks based on days (assuming 1 second block time)
        // blocks = days * 24 hours * 60 minutes * 60 seconds
        let blocks_to_subtract = args.days * 24 * 60 * 60;
        let to = from_block.saturating_sub(blocks_to_subtract);
        info!(
            "'to' not provided, using from ({}) - {} days ({} blocks) = {}",
            from_block, args.days, blocks_to_subtract, to
        );
        to
    } else {
        args.to.unwrap()
    };

    // Validate arguments
    if from_block <= to_block {
        anyhow::bail!(
            "'from' block ({}) must be > 'to' block ({}). Note: 'to' is the older block when processing backwards.",
            from_block, to_block
        );
    }

    if args.concurrency == 0 {
        anyhow::bail!("'concurrency' must be greater than 0");
    }

    info!(
        "Starting parallel cost estimator for blocks {} to {} with batch size {} and concurrency {}",
        from_block, to_block, args.batch_size, args.concurrency
    );

    // Split the overall range into sub-ranges
    let mut ranges = split_range_basic(to_block, from_block, args.batch_size);
    if args.reverse {
        ranges.reverse();
    }

    info!("Split into {} ranges: {:?}", ranges.len(), ranges);

    // Process all ranges with controlled concurrency
    process_ranges(ranges, &args).await?;

    info!("Parallel cost estimator completed successfully!");

    Ok(())
}
