# Cost Estimation Tools

This guide covers the scripts for estimating proving costs and testing execution: `multi` and `cost-estimator`. For generating actual proofs, see [Prove Scripts](./prove-scripts.md).

## Setup

Before running these scripts, set up a `.env` file in the project root:

```bash
L1_RPC=<YOUR_L1_RPC_ENDPOINT>
L2_RPC=<YOUR_L2_RPC_ENDPOINT>
L2_NODE_RPC=<YOUR_L2_NODE_RPC_ENDPOINT>
```

## Multi Script

The `multi` script executes the OP Succinct range proof program for a block range. Use it to test proof generation or generate actual proofs.

### Usage

```bash
# Execute without proving (generates execution report)
cargo run --bin multi -- --start 1000 --end 1020

# Generate compressed proofs
cargo run --bin multi -- --start 1000 --end 1020 --prove
```

### Output

- **Execution mode**: Prints execution stats and saves to `execution-reports/multi/{chain_id}/{start}-{end}.csv`
- **Prove mode**: Saves proof to `data/{chain_id}/proofs/range/{start}-{end}.bin`

## Cost Estimator

The `cost-estimator` estimates proving costs without generating proofs. It splits large ranges into batches and runs them in parallel.

### Usage

```bash
cargo run --bin cost-estimator -- \
    --start 2000000 \
    --end 2001800 \
    --batch-size 300
```

For best estimation, use a range bigger than the batcher interval with batch size equal to the range.

To run against a historical range without depending on op-node SafeDB, pass both `--no-safe-head-split` and an explicit `--l1-head`:

```bash
cargo run --bin cost-estimator -- \
    --start 2000000 \
    --end 2001800 \
    --batch-size 300 \
    --no-safe-head-split \
    --l1-head <L1_HEAD_BLOCK_HASH>
```

`--no-safe-head-split` uses fixed-size batches instead of SafeDB-aligned safe head boundaries. `--l1-head` uses the supplied L1 head hash instead of looking it up through SafeDB, and ignores `L1_BLOCK_TAG` / `L1_CONFIRMATIONS` for the estimator run.

### Output

Execution report saved to `execution-reports/{chain_id}/{start}-{end}-report.csv` with metrics:
- Total instruction count
- Oracle verification / derivation / block execution costs
- SP1 gas usage
- Transaction counts and EVM gas
- Precompile cycles (BN pair, add, mul, KZG eval, etc.)

## Witness Caching

Both scripts support `--cache` to skip the time-consuming witness generation step on subsequent runs. For full details on caching (usage, DA compatibility, cache management), see [Prove Scripts — Witness Caching](./prove-scripts.md#witness-caching).

```bash
# First run: generates witness and saves to cache
cargo run --bin multi -- --start 1000 --end 1020 --cache

# Cost estimator with caching
cargo run --bin cost-estimator -- --start 1000 --end 1100 --batch-size 10 --cache
```
