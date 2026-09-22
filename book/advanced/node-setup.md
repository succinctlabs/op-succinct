# Node Setup

## Overview

To run OP Succinct or OP Succinct Lite, you will need the following RPCs in your `.env`:

- `L1_RPC`: L1 Execution Archive Node
- `L2_RPC`: L2 Execution Node (`op-reth`)
- `L2_NODE_RPC`: L2 Rollup Node (`op-node`)

> Note: If your integration requires access to consensus-layer data, set the `L1_BEACON_RPC` (L1 Beacon Node). This is optional and not required by default.

<div class="warning">
When running the proposer in production, it is recommended that your L2 nodes are in the same region/network to minimize latency. Otherwise, tasks like witness generation can take significantly longer and become a bottleneck.
</div>

## Required Accessible Endpoints

The RPCs must support the following endpoints:

| RPC | Endpoints | Description |
|-----|-----------|-------------|
| `L1_RPC` | `debug_getRawHeader`, `debug_getRawReceipts` | L1 Execution Archive Node |
| `L2_RPC` | `debug_getRawHeader`, `debug_getRawBlock`, `eth_getProof`, and `debug_executePayload` or `debug_dbGet` | L2 Execution Node (`op-reth`) |
| `L2_NODE_RPC` | `optimism_outputAtBlock`, `optimism_rollupConfig`, `optimism_safeHeadAtL1Block` | L2 Rollup Node (`op-node`) |

The Kona host first requests a complete execution witness with `debug_executePayload`.
If that witness is unavailable or incomplete, it falls back to `debug_dbGet` for individual state and code preimages.
The L2 execution RPC must expose at least one complete path for serving those preimages.

Proving historical ranges also requires the L2 execution node to retain state and proof data for the target blocks.
For `op-reth`, configure `--rpc.eth-proof-window` for short lookbacks or enable [historical proofs](https://docs.optimism.io/node-operators/tutorials/reth-historical-proofs) for a larger retention window.

`op-geth` reached end of support on May 31, 2026 and does not support the Karst hardfork.
New deployments should use `op-reth`; see the [Optimism deprecation notice](https://docs.optimism.io/notices/op-geth-deprecation) for migration guidance.

## External RPC Provider

First, we'd recommend asking your RPC provider if they support these endpoints. If they do, you can use them.

> **Note:** While some RPC providers support the required L1 endpoints after enabling them, public L2 endpoints commonly restrict `debug_executePayload`, `debug_dbGet`, or the raw block and header methods.
> You will likely need an allowlisted provider endpoint or your own L2 nodes.

## Running Your Own L2 Nodes

If you don't have access to these endpoints, you can run your own L2 nodes.

### Instructions
1. Follow the Optimism guide to [build and run an OP Stack node from source](https://docs.optimism.io/node-operators/tutorials/run-node-from-source).
2. Run `op-reth` with a proof window that covers the block range you plan to prove.

Your `op-reth` endpoint is available on its configured HTTP RPC port, which is `8545` by default (e.g. `http://localhost:8545`).

For `op-reth` proofs-history, follow the [historical proofs setup](https://docs.optimism.io/node-operators/tutorials/reth-historical-proofs) and confirm that `debug_proofsSyncStatus` covers the target range before proving it.

Your `op-node` endpoint is available on its configured RPC port, which is `9545` by default (e.g. `http://localhost:9545`).

#### Check Sync Status

After a few hours, your node should be fully synced and you can use it to begin generating ZKPs.

To check your node's sync status, you can run the following commands:

**L2 execution node:**

```bash
curl -H "Content-Type: application/json" -X POST --data '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}' http://localhost:8545
```

**op-node:**

```bash
curl -H "Content-Type: application/json" -X POST --data '{"jsonrpc":"2.0","method":"optimism_syncStatus","params":[],"id":1}' http://localhost:9545
```
