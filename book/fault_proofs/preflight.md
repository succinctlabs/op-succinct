# Pre-Flight Validation

**Proving will break if parameters are incorrect.** After deploying contracts, run this script to verify the contract configuration before the mainnet upgrade.

## What Does Pre-Flight Validate?

Catches configuration issues before production by testing the complete proof generation and submission pipeline end-to-end. The game creation and proof submission is simulated on a forked L1 network.

## Prerequisites

- Deployed `DisputeGameFactory` contract
- L1/L2 RPC access
- SP1 network prover access

## Required Environment Variables

Create a `.env` file with the following variables:

### Contract Configuration
```bash
# Address of the DisputeGameFactory contract on L1
FACTORY_ADDRESS=0x...

# Game type identifier for OPSuccinctFaultDisputeGames
# This must match the game type registered in the factory
GAME_TYPE=42

# L1 block number where setImplementation was called for this game type
# Must be a finalized L1 block
SET_IMPL_BLOCK=12345678
```

### Network Configuration
```bash
# L1 RPC endpoint (used for Anvil fork during validation)
L1_RPC=https://ethereum-sepolia-rpc.publicnode.com

# L1 beacon chain RPC endpoint
L1_BEACON_RPC=https://ethereum-sepolia-beacon-api.publicnode.com

# L2 RPC endpoint
L2_RPC=https://rpc-your-rollup.example.com

# L2 node RPC endpoint (often same as L2_RPC)
L2_NODE_RPC=https://rpc-your-rollup.example.com
```

### Prover Configuration
```bash
# Range proof fulfillment strategy
RANGE_PROOF_STRATEGY=auction

# Aggregation proof fulfillment strategy
AGG_PROOF_STRATEGY=auction

# Set to 'true' to use AWS KMS for key management (requires KMS configuration).
# Set to 'false' to use a local private key (requires NETWORK_PRIVATE_KEY below).
# Default: false
USE_KMS_REQUESTER=false

# SP1 network prover private key (required when USE_KMS_REQUESTER=false)
# When USE_KMS_REQUESTER=true, this should be an AWS KMS key ARN instead
NETWORK_PRIVATE_KEY=0x...
```

## Running Pre-Flight Validation

### Basic Usage

Run the preflight script from the repository root:

```bash
cargo run --release --bin preflight
```

The script will:
1. Fetch the anchor L2 block number from the factory
2. Generate a range proof for the game
3. Generate an aggregation proof for the game
4. Fork L1 at a finalized block using Anvil
5. Create a game at 10 blocks after the anchor
6. Prove the game with the aggregation proof
7. Verify the game has been validated with the aggregation proof

### Using Pre-Generated Proofs

To save time during iterative validation, you can reuse previously generated proofs:

```bash
cargo run --release --bin preflight -- \
  --range-proof "12345-12355" \
  --agg-proof "agg"
```

This skips proof generation and uses existing proofs from:
- `data/{CHAIN_ID}/proofs/range/12345-12355.bin`
- `data/{CHAIN_ID}/proofs/agg/agg.bin`
