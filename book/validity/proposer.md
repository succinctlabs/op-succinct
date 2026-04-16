# Proposer

The `op-succinct` service monitors the state of the L2 chain, requests proofs from the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro) and submits them to the L1.

## RPC Requirements

Confirm that your RPC's have all of the required endpoints as specified in the [prerequisites](../advanced/node-setup.md#required-accessible-endpoints) section.

## Hardware Requirements

We recommend the following hardware configuration for the default `op-succinct` validity service (1 concurrent proof request & 1 concurrent witness generation thread):

Using the docker compose file:

- Full `op-succinct` service: 2 vCPUs, 4GB RAM.
- Mock `op-succinct` service: 2 vCPUs, 8GB RAM. Increased memory because the machine is executing the proofs locally.

Depending on the number of concurrent requests you want to run, you may need to increase the number of vCPUs and memory allocated to the `op-succinct` container.

## Environment Setup

Make sure to include *all* of the required environment variables in the `.env` file.

Before starting the proposer, ensure you have deployed the relevant contracts and have the address of the proxy contract ready. Follow the steps in the [Environment Variables](./contracts/environment.md) section.

### Required Environment Variables

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
| `NETWORK_PRIVATE_KEY` | Private key for the Succinct Prover Network. See the [Succinct Prover Network Quickstart](https://docs.succinct.xyz/docs/sp1/prover-network/quickstart) for setup instructions. |
| `L2OO_ADDRESS` | Address of the `OPSuccinctL2OutputOracle` contract. |
| `PRIVATE_KEY` | Private key for the account that will be posting output roots to L1. |

### Optional Environment Variables

| Parameter | Description |
|-----------|-------------|
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. Could be required for integrations that access consensus-layer data. |
| `NETWORK_RPC_URL` | Default: `https://rpc.production.succinct.xyz`. RPC URL for the Succinct Prover Network. |
| `DATABASE_URL` | Default: `postgres://op-succinct@postgres:5432/op-succinct`. The address of a Postgres database for storing the intermediate proposer state. |
| `L1_CONFIG_DIR` | Default: `<project-root>/configs/L1`. The directory containing the L1 chain configuration files. |
| `L2_CONFIG_DIR` | Default: `<project-root>/configs/L2`. Directory containing L2 chain configuration files. On first run, the rollup config is fetched from the node RPC and cached here. On subsequent runs, the cached file is used. Delete the cached file and restart to force a refresh (e.g., after a hardfork activates). |
| `DGF_ADDRESS` | Address of the `DisputeGameFactory` contract. Note: If set, the proposer will create a dispute game with the DisputeGameFactory, rather than the `OPSuccinctL2OutputOracle`. Compatible with `OptimismPortal2`. |
| `RANGE_PROOF_STRATEGY` | Default: `reserved`. Set to `hosted` to use hosted proof strategy. |
| `AGG_PROOF_STRATEGY` | Default: `reserved`. Set to `hosted` to use hosted proof strategy. |
| `AGG_PROOF_MODE` | Default: `plonk`. Set to `groth16` to use Groth16 proof type. **Note:** Changing the proof mode requires updating the verifier gateway contract address in your L2OutputOracle contract deployment. See [SP1 Contract Addresses](https://docs.succinct.xyz/docs/sp1/verification/contract-addresses) for verifier addresses. |
| `SUBMISSION_INTERVAL` | Default: `1800`. The number of L2 blocks that must be proven before a proof is submitted to the L1. Note: The interval used by the validity service is always >= to the `submissionInterval` configured on the L2OO contract. To allow for the validity service to configure this parameter entirely, set the `submissionInterval` in the contract to `1`. |
| `RANGE_PROOF_INTERVAL` | Default: `1800`. The number of blocks to include in each range proof. For chains with high throughput, you need to decrease this value. |
| `RANGE_PROOF_EVM_GAS_LIMIT` | Default: `0`. The total amount of ethereum gas allowed to be in each range proof. If 0, uses the `RANGE_PROOF_INTERVAL` instead to do a fixed number of blocks interval. NOTE: if both `RANGE_PROOF_INTERVAL` and `RANGE_PROOF_EVM_GAS_LIMIT` are set, the number of blocks to include in each range proof is determined either when the cumulative gas reaches `RANGE_PROOF_EVM_GAS_LIMIT` or the number of blocks reaches `RANGE_PROOF_INTERVAL`, whichever occurs first. |
| `MAX_CONCURRENT_PROOF_REQUESTS` | Default: `1`. The maximum number of concurrent proof requests (in mock and real mode). |
| `MAX_CONCURRENT_WITNESS_GEN` | Default: `1`. The maximum number of concurrent witness generation requests. |
| `OP_SUCCINCT_MOCK` | Default: `false`. Set to `true` to run in mock proof mode. The `OPSuccinctL2OutputOracle` contract must be configured to use an `SP1MockVerifier`. |
| `METRICS_PORT` | Default: `8080`. The port to run the metrics server on. |
| `LOOP_INTERVAL` | Default: `60`. The interval (in seconds) between each iteration of the OP Succinct service. |
| `SIGNER_URL` | URL for the Web3Signer. Note: This takes precedence over the `PRIVATE_KEY` environment variable. |
| `SIGNER_ADDRESS` | Address of the account that will be posting output roots to L1. Note: Only set this if the signer is a Web3Signer. Note: Required if `SIGNER_URL` is set. |
| `SAFE_DB_FALLBACK` | Default: `false`. Whether to fallback to timestamp-based L1 head estimation even though SafeDB is not activated for op-node.  When `false`, proposer will panic if SafeDB is not available. It is by default `false` since using the fallback mechanism will result in higher proving cost. |
| `L1_BLOCK_TAG` | Default: `finalized`. Which L1 block to anchor proof generation against. One of `finalized` (Casper FFG finalized, ~13 min), `safe` (FFG justified checkpoint, ~6.4 min), `latest` (chain tip). Non-default values trade cryptoeconomic finality for latency and must only be used by operators who understand the L1-reorg implications for validity-mode chains. See the [L1 block selection](#l1-block-selection) section below. |
| `L1_CONFIRMATIONS` | Default: `0`. Number of additional L1 block confirmations to wait behind the block returned by `L1_BLOCK_TAG` (e.g. `L1_BLOCK_TAG=latest` with `L1_CONFIRMATIONS=4` proves against `latest - 4`). Saturates at 0 if the offset would underflow. |
| `OP_SUCCINCT_CONFIG_NAME` | Default: `"opsuccinct_genesis"`. The name of the configuration the proposer will interact with on chain. |
| `OTLP_ENABLED` | Default: `false`. Whether to export logs to [OTLP](https://opentelemetry.io/docs/specs/otel/protocol/). |
| `LOGGER_NAME` | Default: `op-succinct`. This will be the `service.name` exported in the OTLP logs. |
| `OTLP_ENDPOINT` | Default: `http://localhost:4317`. The endpoint to forward OTLP logs to. |
| `USE_KMS_REQUESTER` | Default: ``.  Whether to expect NETWORK_PRIVATE_KEY to be an AWS KMS key ARN instead of a plaintext private key. |
| `MAX_PRICE_PER_PGU` | Default: `300,000,000`. The maximum price per pgu for proving. |
| `PROVING_TIMEOUT` | Default: `14400` (4 hours). The timeout to use for proving (in seconds). |
| `NETWORK_CALLS_TIMEOUT` | Default: `15` (15 seconds). The timeout for network prover calls (in seconds). |
| `RANGE_CYCLE_LIMIT` | Default: `1,000,000,000,000`. The cycle limit to use for range proofs. |
| `RANGE_GAS_LIMIT` | Default: `1,000,000,000,000`. The gas limit to use for range proofs. |
| `AGG_CYCLE_LIMIT` | Default: `1,000,000,000,000`. The cycle limit to use for aggregation proofs. |
| `AGG_GAS_LIMIT` | Default: `1,000,000,000,000`. The gas limit to use for aggregation proofs. |
| `WHITELIST` | Default: ``. The list of prover addresses that are allowed to bid on proof requests. |
| `MIN_AUCTION_PERIOD` | Default: `1`. The minimum auction period (in seconds). |
| `AUCTION_TIMEOUT` | Default: `60` (1 minute). How long to wait before canceling a proof request that hasn't been assigned (in seconds). |
| `TX_CONFIRMATION_TIMEOUT` | Default: `60`. Maximum time (in seconds) to wait for an L1 transaction to reach the required number of confirmations. Raise on congested L1s to avoid timeout-triggered retries. |

## Build the Proposer Service

Build the OP Succinct validity service.

```bash
docker compose build
```

## Run the Proposer

Run the OP Succinct validity service.

```bash
docker compose up
```

To see the logs of the OP Succinct services, run:

```bash
docker compose logs -f
```

After several minutes, the validity service will start to generate range proofs. Once enough range proofs have been generated, an aggregation proof will be created. Once the aggregation proof is complete, it will be submitted to the L1.

To stop the OP Succinct validity service, run:

```bash
docker compose stop
```

## L1 block selection

The proposer decides which L1 block to anchor each proof against using the `L1_BLOCK_TAG` and `L1_CONFIRMATIONS` environment variables. The default (`finalized`, `0`) preserves historical behavior.

| `L1_BLOCK_TAG` | Lag from tip | Security | Reorg handling |
|---|---|---|---|
| `finalized` (default) | ~12.8 min (2 epochs) | Cryptoeconomic (Casper FFG) | Not possible to reorg under the 1/3+ slashing assumption. |
| `safe` | ~6.4 min (1 epoch) | Cryptoeconomic (2/3 validator attestations) | Justified checkpoints have never been reorged on Ethereum mainnet; reverting requires 1/3+ adversarial stake. |
| `latest` | 0s (+ `L1_CONFIRMATIONS`) | Probabilistic, depth-based | Reorgs at small depths are rare post-merge but not cryptoeconomically bounded. |

`L1_CONFIRMATIONS` subtracts additional blocks from the selected tag (e.g. `L1_BLOCK_TAG=latest` with `L1_CONFIRMATIONS=4` resolves to `latest.number - 4`).

### SafeDB requirement for non-default selections (Ethereum / EigenDA)

For Ethereum and EigenDA backends, any non-default selection (tag != `finalized` or `confirmations != 0`) resolves the max provable L2 block via `optimism_safeHeadAtL1Block(resolved_l1_number)`. This RPC requires SafeDB to be activated on the op-node. The proposer hard-fails at startup if SafeDB is unavailable under a non-default selection on these backends. `SAFE_DB_FALLBACK` only applies to the default selection; it does not provide a fallback for the non-default L1 -> L2 resolution path.

### Celestia backend: non-default selection is rejected at startup

Celestia's proving path is driven by Blobstream commitments and the op-celestia-indexer, not by an L1 block tag. `CelestiaOPSuccinctHost::calculate_safe_l1_head` and `CelestiaOPSuccinctHost::get_finalized_l2_block_number` do not read `L1_BLOCK_TAG` / `L1_CONFIRMATIONS`. To avoid silently accepting a knob that would not actually change those decisions, the production proposer binaries and the covered operator-facing utility scripts under `scripts/` hard-fail at startup when Celestia is configured together with a non-default selection.

On Celestia, only the default selection (`finalized`, `0`) is allowed for those entrypoints. Test harnesses and internal tools that construct a fetcher/host directly do not invoke the shared enforcement helper; they are not considered operator-facing and are out of scope for this policy.

### Operational notes

- Existing Prometheus metric names (e.g. those reporting the latest "finalized" L2 block number) are unchanged for dashboard compatibility. Under a non-default selection their semantics shift from "L2 block whose L1 origin is finalized" to "L2 block whose L1 origin matches the configured selection".
- Invalid values for `L1_BLOCK_TAG` or `L1_CONFIRMATIONS` cause the proposer to exit cleanly at startup with an error naming the offending env var and value.
- Utility scripts (under `scripts/`) read the same env vars through the shared fetcher; invalid values cause a panic with the same naming in the error message. Double-check env values before running scripts.
