# Prove Scripts

The prove scripts generate range and aggregation proofs for OP Succinct using the SP1 network prover.

## Overview

OP Succinct uses a two-tier proving architecture:

1. **Range Proofs** (`multi`): Generate compressed proofs for a range of L2 blocks.

2. **Aggregation Proofs** (`agg`): Combine multiple range proofs into a single aggregation proof, reducing on-chain verification costs.

> **Note:** All range proofs must be in compressed mode for aggregation. The prove scripts handle this automatically.

For cost estimation without proving, see [Cost Estimation Tools](./cost-estimation-tools.md).

## Setup

### Environment Configuration

Create a `.env` file in the project root directory:

```bash
# RPC Endpoints
L1_RPC=<YOUR_L1_RPC_ENDPOINT>
L1_BEACON_RPC=<YOUR_L1_BEACON_RPC_ENDPOINT>
L2_RPC=<YOUR_L2_RPC_ENDPOINT>
L2_NODE_RPC=<YOUR_L2_NODE_RPC_ENDPOINT>

# Network Prover Configuration
NETWORK_PRIVATE_KEY=<YOUR_NETWORK_PRIVATE_KEY>

# Proof Strategy Configuration
RANGE_PROOF_STRATEGY=reserved    # Options: reserved, hosted, auction
AGG_PROOF_STRATEGY=reserved      # Options: reserved, hosted, auction
AGG_PROOF_MODE=plonk             # Options: plonk, groth16
```

### Environment Variables

#### Required

| Variable | Description |
|----------|-------------|
| `L1_RPC` | L1 Archive Node endpoint |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node endpoint |
| `L2_RPC` | L2 Execution Node (`op-geth`) endpoint |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`) endpoint |
| `NETWORK_PRIVATE_KEY` | Private key for the Succinct Prover Network. See the [Succinct Prover Network Quickstart](https://docs.succinct.xyz/docs/sp1/prover-network/quickstart) for setup instructions. |

#### Optional (`multi` script)

| Variable | Description | Default |
|----------|-------------|---------|
| `RANGE_PROOF_STRATEGY` | Proof fulfillment strategy for range proofs | `reserved` |
| `USE_KMS_REQUESTER` | Use AWS KMS for network signing (`NETWORK_PRIVATE_KEY` becomes a KMS key ARN) | `false` |

#### Optional (`agg` script)

| Variable | Description | Default |
|----------|-------------|---------|
| `AGG_PROOF_STRATEGY` | Proof fulfillment strategy for aggregation proofs | `reserved` |
| `AGG_PROOF_MODE` | Proof mode for aggregation proofs (`plonk` or `groth16`) | `plonk` |
| `USE_KMS_REQUESTER` | Use AWS KMS for network signing (`NETWORK_PRIVATE_KEY` becomes a KMS key ARN) | `false` |

Each script reads only its own strategy env var, so `RANGE_PROOF_STRATEGY` and `AGG_PROOF_STRATEGY` can be set independently.

**Proof Strategies:**
- `reserved`: Uses reserved SP1 network capacity
- `hosted`: Uses hosted proof generation service
- `auction`: Uses auction-based proof fulfillment

**Proof Modes:**
- `plonk`: PLONK proof system (default)
- `groth16`: Groth16 proof system

### Getting Started with the Prover Network

1. Follow the [Succinct Prover Network Quickstart](https://docs.succinct.xyz/docs/sp1/prover-network/quickstart) to set up your account and obtain a private key.

2. Set the `NETWORK_PRIVATE_KEY` environment variable:
   ```bash
   NETWORK_PRIVATE_KEY=0x...
   ```

3. Run the prove scripts. The binaries will automatically use the network prover with your configured key.

## Generating Range Proofs

The `multi` binary generates compressed range proofs for a specified block range.

### Usage

```bash
cargo run --bin multi --release -- \
    --start <START_BLOCK> \
    --end <END_BLOCK> \
    --prove
```

### Example

```bash
# Generate a compressed range proof for blocks 1000-1300
cargo run --bin multi --release -- \
    --start 1000 \
    --end 1300 \
    --prove
```

### Output

Range proofs are saved to `data/{chain_id}/proofs/{start_block}-{end_block}.bin`.

## Generating Aggregation Proofs

The `agg` binary aggregates multiple compressed range proofs into a single aggregation proof.

### Usage

```bash
cargo run --bin agg --release -- \
    --proofs <PROOF_1>,<PROOF_2>,<PROOF_N> \
    --prover <PROVER_ADDRESS> \
    --prove
```

### Example

```bash
# Aggregate three consecutive range proofs covering blocks 1000-1900
cargo run --bin agg --release -- \
    --proofs 1000_1300,1300_1600,1600_1900 \
    --prover 0x1234567890abcdef1234567890abcdef12345678 \
    --prove
```

### Parameters

| Parameter | Description | Required |
|-----------|-------------|----------|
| `--proofs` | Comma-separated list of proof names (without `.bin` extension) | Yes |
| `--prover` | Prover wallet address included in the aggregation proof | Yes |
| `--prove` | Generate proof (omit to only execute and verify inputs) | No |
| `--env-file` | Path to environment file (default: `.env`) | No |

### Requirements

- Proof files must exist in `data/fetched_proofs/` directory
- Proof names should match the range format: `{start_block}_{end_block}`
- Range proofs must be consecutive (e.g., 1000_1300, 1300_1600, 1600_1900)

### Output

Aggregation proofs are saved to `data/{chain_id}/proofs/agg/{proof_names}.bin`.

## End-to-End Workflow

The full proving pipeline involves three steps: generating range proofs, fetching them from the network, and aggregating them.

### 1. Generate Range Proofs

Run `multi --prove` for each block range. Proofs are submitted to the SP1 network and saved locally to `data/{chain_id}/proofs/{start}-{end}.bin`.

```bash
cargo run --bin multi --release -- --start 1000 --end 1300 --prove
cargo run --bin multi --release -- --start 1300 --end 1600 --prove
cargo run --bin multi --release -- --start 1600 --end 1900 --prove
```

### 2. Fetch Proofs from the Network

Use the `fetch_and_save_proof` utility to download completed proofs from the network into `data/fetched_proofs/`. Pass the `--start` and `--end` flags to name the files with the block range.

```bash
cargo run --bin fetch_and_save_proof --release -- \
    --request-id <REQUEST_ID> --start 1000 --end 1300
```

This saves the proof as `data/fetched_proofs/1000_1300.bin`. Repeat for each range proof.

> **Note:** `fetch_and_save_proof` uses underscore-separated naming (`{start}_{end}.bin`), not dash-separated. When passing proof names to `agg --proofs`, use the underscore format (e.g., `1000_1300`).

### 3. Aggregate Proofs

Run `agg --prove` with the proof names (without `.bin` extension) matching the files in `data/fetched_proofs/`.

```bash
cargo run --bin agg --release -- \
    --proofs 1000_1300,1300_1600,1600_1900 \
    --prover 0x1234567890abcdef1234567890abcdef12345678 \
    --prove
```

The aggregation proof is saved to `data/{chain_id}/proofs/agg/`.

## Witness Caching

Witness generation (`host.run()`) fetches L1/L2 data and executes blocks, which can take **hours** for large ranges. Caching saves the generated witness to disk so subsequent runs skip this step.

```
host.run() → WitnessData → get_sp1_stdin() → SP1Stdin
   [hours]                    [milliseconds]
```

### Usage

Use `--cache` to enable caching. On the first run, the witness is generated and saved. On subsequent runs, the cached witness is loaded instantly.

```bash
# First run: generates witness and saves to cache
cargo run --bin multi --release -- --start 1000 --end 1020 --cache

# Second run: loads from cache (instant), then proves
cargo run --bin multi --release -- --start 1000 --end 1020 --cache --prove
```

### Cache Location

```
data/{chain_id}/witness-cache/{start_block}-{end_block}-stdin.bin
```

### DA Compatibility

| DA Type | Compatible With |
|---------|-----------------|
| Ethereum (default) | Celestia |
| Celestia | Ethereum |
| EigenDA | EigenDA only |

Cache files are compatible between Ethereum and Celestia, but **not** with EigenDA. Don't mix cache files across incompatible DA types.

### Cache Management

```bash
# Clear all cache for a chain
rm -rf data/{chain_id}/witness-cache/

# Clear specific range
rm data/{chain_id}/witness-cache/{start}-{end}-stdin.bin
```

Cache files are typically 100MB-1GB per range.

## Local Development

For testing without incurring proving costs, omit the `--prove` flag:

```bash
# Execute range proof program without proving
cargo run --bin multi --release -- \
    --start 1000 \
    --end 1300

# Execute aggregation program without proving
cargo run --bin agg --release -- \
    --proofs 1000_1300,1300_1600 \
    --prover 0x1234567890abcdef1234567890abcdef12345678
```

This runs execution and reports cycle counts without submitting proof requests to the network.
