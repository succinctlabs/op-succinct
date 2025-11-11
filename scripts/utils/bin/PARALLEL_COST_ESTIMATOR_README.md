# Parallel Cost Estimator

A utility to run multiple `cost-estimator` instances in parallel for processing large block ranges efficiently.

## Overview

The `parallel-cost-estimator` divides a large block range into smaller chunks and processes them concurrently using multiple instances of the `cost-estimator` binary.

## Usage

### Basic Command

```bash
cargo run --release --bin parallel-cost-estimator -- \
  --from <START_BLOCK> \
  --to <END_BLOCK> \
  --batch-size <BLOCKS_PER_BATCH> \
  --concurrency <NUM_PARALLEL_WORKERS>
```

### Parameters

#### Required Parameters

- `--batch-size <SIZE>`: Number of blocks in each processing batch (default: 10)

#### Optional Parameters

- `--from <BLOCK_NUMBER>`: Starting block number (inclusive). If not provided, fetches latest finalized block from L2 RPC
- `--to <BLOCK_NUMBER>`: Ending block number (exclusive). If not provided, calculates as `(from - days * 86400)`
- `--days <NUM>`: Number of days to look back when calculating `to` (default: 14, assumes 1 second block time)
- `--concurrency <NUM>`: Number of concurrent cost_estimator instances (default: 4)
- `--reverse`: Process ranges in reverse order (highest blocks first)
- `--log-only`: Skip writing CSV files and only log execution statistics (default: true)
- `--env-file <PATH>`: Path to environment file (default: .env)

### Examples

#### Example 1: Auto-fetch latest 2 weeks of blocks

```bash
cargo run --release --bin parallel-cost-estimator -- \
  --batch-size 100 \
  --concurrency 4
```

This will:
- Fetch the latest finalized block from L2 RPC as `from`
- Calculate `to` as `from - (14 * 86400)` blocks (14 days)
- Process with 4 concurrent workers

#### Example 2: Custom time range (7 days)

```bash
cargo run --release --bin parallel-cost-estimator -- \
  --days 7 \
  --batch-size 100 \
  --concurrency 8
```

This will process the last 7 days of blocks instead of the default 14 days.

#### Example 3: Specific block range

```bash
cargo run --release --bin parallel-cost-estimator -- \
  --from 1000000 \
  --to 900000 \
  --batch-size 50 \
  --concurrency 6
```

This will process blocks 1,000,000 down to 900,000 with custom batch size.

#### Example 4: High concurrency for large ranges

```bash
cargo run --release --bin parallel-cost-estimator -- \
  --from 2000000 \
  --to 800000 \
  --batch-size 1000 \
  --concurrency 16
```

Processes 1.2 million blocks with high parallelism.

## Output

The script logs:
1. Configuration summary (ranges, concurrency)
2. Progress for each completed range
3. Errors for failed ranges
4. **Panic information for ranges that panicked**
5. Final statistics: completed, failed, panicked counts
6. Lists of all completed, failed, and panicked ranges

## Error Handling

The script tracks three categories:
- **Completed**: Ranges that processed successfully
- **Failed**: Ranges that failed with errors
- **Panicked**: Ranges where the process panicked

All three categories are logged with their specific block ranges at the end:
```
[INFO] Completed ranges: [(100, 200), (200, 300)]
[INFO] Failed ranges: [(300, 400)]
[INFO] Panicked ranges: [(400, 500)]
```

Processing continues even if some ranges fail or panic. The script exits with an error code if any failures or panics occurred.

## Building

### Native Build

```bash
cargo build --release --bin parallel-cost-estimator
```

### Docker Build

```bash
docker build -f scripts/utils/Dockerfile.parallel-cost-estimator -t parallel-cost-estimator .
```
