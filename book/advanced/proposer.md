# Proposer

The `op-succinct` service consists of two containers:
- [`op-succinct/succinct-proposer`](https://ghcr.io/succinctlabs/op-succinct/succinct-proposer): Receives proof requests from the `op-succinct/op-proposer`, generates the witness for the proof, and submits the proof to the Succinct Prover Network. Handles the communication with the [Succinct's Prover Network](https://docs.succinct.xyz/docs/generating-proofs/prover-network) to fetch the proof status and completed proof data.
- [`op-succinct/op-proposer`](https://ghcr.io/succinctlabs/op-succinct/op-proposer): Monitors L1 state to determine when to request a proof. Sends proof requests to the `op-succinct/succinct-proposer`. Once proofs have been generated for a sufficiently large range, aggregates range proofs into an aggregation proof. Submits the aggregation proof to the `OPSuccinctL2OutputOracle` contract which includes the L2 state outputs.

We've packaged the `op-succinct` service in a docker compose file to make it easier to run.

# Prerequisites

## RPC Requirements

Confirm that your RPC's have all of the required endpoints. More details can be found in the [prerequisites](../quick-start/prerequisites.md#requirements) section.

## Hardware Requirements

We recommend the following hardware configuration for the `op-succinct` service containers:

Using the docker compose file:

- Full `op-succinct` service: 16 vCPUs, 64GB RAM.
- Mock `op-succinct` service: 32 vCPUs, 128GB RAM. Increased memory because the machine is executing the proofs locally.

Running as separate containers:

- [`op-succinct/succinct-proposer`](https://ghcr.io/succinctlabs/op-succinct/succinct-proposer):
    - Full `op-succinct` service: 16 vCPUs, 64GB RAM.
    - Mock `op-succinct` service: 32 vCPUs, 128GB RAM. Increased memory because the machine is executing the proofs locally.
- [`op-succinct/op-proposer`](https://ghcr.io/succinctlabs/op-succinct/op-proposer): 1 vCPU, 4GB RAM

For advanced configurations, depending on the number of concurrent requests you expect, you may need to increase the number of vCPUs and memory allocated to the `op-succinct-server` container.

# Environment Setup

If you are running the `op-succinct` service as a Docker Compose service, you should include *all* of the required environment variables in the `.env` file for both the `op-succinct/succinct-proposer` and `op-succinct/op-proposer` services.

Before starting the proposer, ensure you have deployed the L2 Output Oracle and have the address of the proxy contract ready. Follow the steps in the [Contract Configuration](../contracts/configuration.md) section.

## Required Environment Variables

### `op-succinct/succinct-proposer`

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
| `NETWORK_PRIVATE_KEY` | Key for the Succinct Prover Network. Get access [here](https://docs.succinct.xyz/docs/generating-proofs/prover-network). |

### `op-succinct/op-proposer`

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
| `L2OO_ADDRESS` | Address of the `OPSuccinctL2OutputOracle` contract. |
| `PRIVATE_KEY` | Private key for the account that will be posting output roots to L1. |

## Advanced Environment Variables

### `op-succinct/succinct-proposer`

| Parameter | Description |
|-----------|-------------|
| `NETWORK_RPC_URL` | Default: `https://rpc.production.succinct.xyz`. RPC URL for the Succinct Prover Network. |
| `RANGE_PROOF_STRATEGY` | Default: `reserved`. Set to `hosted` to use hosted proof strategy. |
| `AGG_PROOF_STRATEGY` | Default: `reserved`. Set to `hosted` to use hosted proof strategy. |
| `AGG_PROOF_MODE` | Default: `groth16`. Set to `plonk` to use PLONK proof type. Note: The verifier gateway contract address must be updated to use PLONK proofs. |

### `op-succinct/op-proposer`

| Parameter | Description |
|-----------|-------------|
| `MAX_CONCURRENT_PROOF_REQUESTS` | Default: `3`. The maximum number of concurrent proof requests to send to the `op-succinct-server`. |
| `MAX_CONCURRENT_WITNESS_GEN` | Default: `3`. The maximum number of concurrent witness generation processes to run on the `op-succinct-server`. |
| `RANGE_PROOF_INTERVAL` | Default: `1800`. The number of blocks to include in each range proof. For chains with high throughput, you need to decrease this value. |
| `OP_SUCCINCT_MOCK` | Default: `false`. Set to `true` to run in mock proof mode. The `OPSuccinctL2OutputOracle` contract must be configured to use an `SP1MockVerifier`. |
| `METRICS_PORT` | Default: `8080`. The port to run the metrics server on. |
| `DB_PATH` | Default: `/usr/local/bin/dbdata`. The path to the database directory within the container. |
| `LOOP_INTERVAL` | Default: `60`. The interval (in seconds) between each iteration of the OP Succinct service. |

# Build the Proposer Service

Build the OP Succinct validity service.

```bash
docker compose build
```

# Run the Proposer

Run the OP Succinct validity service.

```bash
docker compose up
```

To see the logs of the OP Succinct services, run:

```bash
docker compose logs -f
```

After a few minutes, you should see the OP Succinct services start to generate range proofs. Once enough range proofs have been generated, they will be verified in an aggregate proof and submitted to the L1.

To stop the OP Succinct services, run:

```bash
docker compose stop
```