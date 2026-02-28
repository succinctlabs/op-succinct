# Prove Scripts

The prove scripts allow you to manually generate range and aggregation proofs for OP Succinct.

## Overview

OP Succinct uses a two-tier proving architecture:

1. **Range Proofs** (`multi.rs`): Generate compressed proofs for a range of L2 blocks. These proofs verify the state transition for a specific block range.

2. **Aggregation Proofs** (`agg.rs`): Combine multiple range proofs into a single aggregation proof. This reduces on-chain verification costs by verifying one proof instead of many.

Both binaries use the SP1 network prover by default.

### When to Use Aggregation

Aggregation is useful when you need to:
- Reduce on-chain verification costs by combining multiple range proofs into one proof
- Prove larger block ranges by aggregating individual range proofs from different time periods
- Combine proofs from different proving sessions into a single proof for batch submission

> **Note:** All range proofs must be generated in compressed mode for aggregation. The prove scripts handle this automatically.

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

#### Optional

| Variable | Description | Default |
|----------|-------------|---------|
| `RANGE_PROOF_STRATEGY` | Proof fulfillment strategy for range proofs | `reserved` |
| `AGG_PROOF_STRATEGY` | Proof fulfillment strategy for aggregation proofs | `reserved` |
| `AGG_PROOF_MODE` | Proof mode for aggregation proofs (`plonk` or `groth16`) | `plonk` |

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
   ```.env
   NETWORK_PRIVATE_KEY=0x...
   ```

3. Run the prove scripts. The binaries will automatically use the network prover with your configured key.

## Generating Range Proofs

The `multi.rs` binary generates compressed range proofs for a specified block range. These proofs verify the state transition function for the L2 blocks in the range.

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

The proof will be generated in compressed mode, which is required for aggregation.

### Output

Range proofs are saved to `data/{chain_id}/proofs/{start_block}-{end_block}.bin`

## Generating Aggregation Proofs

The `agg.rs` binary aggregates multiple compressed range proofs into a single aggregation proof. This allows you to verify the state transition for a large block range with a single on-chain verification.

### How Aggregation Works

1. The binary loads the specified range proofs from `data/fetched_proofs/`
2. Each range proof is verified to ensure validity
3. The proofs are aggregated into a single proof that attests to the entire block range
4. The aggregation proof can be submitted on-chain for efficient verification

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
    --proofs 1000-1300,1300-1600,1600-1900 \
    --prover 0x1234567890abcdef1234567890abcdef12345678 \
    --prove
```

This will generate a single aggregation proof that verifies the state transition from block 1000 to 1900.

### Parameters

| Parameter | Description | Required |
|-----------|-------------|----------|
| `--proofs` | Comma-separated list of proof names (without `.bin` extension) | Yes |
| `--prover` | Prover wallet address included in the aggregation proof | Yes |
| `--prove` | Generate proof (omit to only execute and verify inputs) | No |
| `--env-file` | Path to environment file (default: `.env`) | No |

### Requirements

- Proof files must exist in `data/fetched_proofs/` directory
- Proof names should match the range format: `{start_block}-{end_block}`
- Range proofs must be consecutive (e.g., 1000-1300, 1300-1600, 1600-1900)
- All range proofs are automatically verified before aggregation

## Local Development

For local development and testing, omit the `--prove` flag to execute the program without generating proofs. This allows you to verify correctness without incurring proving costs.

```bash
# Execute without proving
cargo run --bin multi --release -- \
    --start 1000 \
    --end 1300
```

This will run the full execution and report cycle counts without submitting proof requests to the network.
