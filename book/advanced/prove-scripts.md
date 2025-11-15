# Prove Scripts

The prove scripts allow you to manually generate range and aggregation proofs for OP Succinct. These are useful for testing proof generation workflows and debugging.

## Overview

There are two main proving binaries:
- **multi.rs**: Generates range proofs for multiple blocks
- **agg.rs**: Aggregates multiple range proofs into a single aggregation proof

Both binaries use the SP1 network prover by default.

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

The `multi.rs` binary generates range proofs for a specified block range.

### Usage

```bash
cargo run --bin multi --release -- \
    --start <START_BLOCK> \
    --end <END_BLOCK> \
    --prove
```

### Example

```bash
# Generate a range proof for blocks 1000-1300
cargo run --bin multi --release -- \
    --start 1000 \
    --end 1300 \
    --prove
```

### Output

Range proofs are saved to `data/{chain_id}/proofs/{start_block}-{end_block}.bin`

## Generating Aggregation Proofs

The `agg.rs` binary aggregates multiple range proofs into a single aggregation proof.

### Usage

```bash
cargo run --bin agg --release -- \
    --proofs <PROOF_1>,<PROOF_2>,<PROOF_N> \
    --prover <PROVER_ADDRESS> \
    --prove
```

### Example

```bash
# Aggregate three range proofs
cargo run --bin agg --release -- \
    --proofs 1000-1300,1300-1600,1600-1900 \
    --prover 0x1234567890abcdef1234567890abcdef12345678 \
    --prove
```

### Parameters

| Parameter | Description | Required |
|-----------|-------------|----------|
| `--proofs` | Comma-separated list of proof names (without `.bin` extension) | Yes |
| `--prover` | Prover wallet address | Yes |
| `--prove` | Generate proof (omit to only execute) | No |
| `--env-file` | Path to environment file (default: `.env`) | No |

### Requirements

- Proof files must exist in `data/fetched_proofs/` directory
- Proof names should match the range format: `{start_block}-{end_block}`
- All range proofs must be verified before aggregation

## Local Development

For local development and testing, omit the `--prove` flag to execute the program without generating proofs. This allows you to verify correctness without incurring proving costs.

```bash
# Execute without proving
cargo run --bin multi --release -- \
    --start 1000 \
    --end 1300
```

This will run the full execution and report cycle counts without submitting proof requests to the network.
