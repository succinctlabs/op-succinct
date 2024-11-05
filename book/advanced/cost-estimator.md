# Cost Estimator

We provide a convenient CLI tool to fetch the RISC-V instruction counts for generating ZKPs for a range of blocks for a given rollup. We recommend running the cost estimator on a remote machine (500+ Mbps) with fast network connectivity because witness generation is bandwidth-intensive.

## Overview

In the root directory, add the following RPCs to your `.env` file for your rollup:

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |

More details on the RPC requirements can be found in the [prerequisites](../getting-started/prerequisites.md) section.

## Running the Cost Estimator

```shell
RUST_LOG=info just cost-estimator <start_l2_block> <end_l2_block>
```

**Example Arguments:**
- `start_l2_block`: Subtract 100 from the latest finalized block. `cast block finalized -f number --rpc-url <L2_RPC>`
- `end_l2_block`: The latest finalized block.

> Note: You can execute unfinalized blocks as long as they're included in an L1 block.

### Run the Cost Estimator

To run the cost estimator over a block range using your local `.env` file for environment variables, run the following command:

```shell
RUST_LOG=info just cost-estimator <start_l2_block> <end_l2_block>
```

This command will split the block range into smaller ranges to model the workload run by `op-succinct`. It will then fetch the required data for generating the ZKP for each of these ranges, and execute the SP1 `range` program. Once each program finishes, it will collect the statistics and output the aggregate statistics.

Once the execution of the range is complete, the cost estimator will output the aggregate statistics and write them to a CSV file in `execution-reports/{chain_id}/{start_block}-{end_block}.csv`.

### Advanced Usage

There are a few optional flags that can be used to customize the cost estimator:

| Flag | Description |
|-----------|-------------|
| `--batch-size` | The number of blocks to execute in a single batch. |
| `--use-cache` | Use cached witness generation. |
| `--env-file` | The path to the environment file to use. (Ex. `.env.opmainnet`) |

```shell
RUST_LOG=info cargo run --bin cost-estimator --release <start_l2_block> <end_l2_block> --env-file <path_to_env_file>
```

> Running the cost estimator for a large block range may be slow on machines with limited network bandwidth to the L2 node. For optimal performance, we recommend using a remote machine with high-speed connectivity to avoid slow witness generation.

## Example

On Optimism Sepolia, proving the block range 17664000 to 17664125 (125 blocks) takes 4 range proofs and ~11.1B cycles.

```bash
RUST_LOG=info just cost-estimator 17664000 17664125

...Execution Logs...

 +--------------------------------+---------------------------+
| Metric                         | Value                     |
+--------------------------------+---------------------------+
| Batch Start                    |                17,664,125 |
| Batch End                      |                17,664,250 |
| Execution Duration (seconds)   |                       606 |
| Total Instruction Count        |            11,055,051,645 |
| Oracle Verify Cycles           |               832,566,844 |
| Derivation Cycles              |             1,089,859,924 |
| Block Execution Cycles         |             8,959,507,779 |
| Blob Verification Cycles       |               338,156,173 |
| Total SP1 Gas                  |            13,075,527,707 |
| Number of Blocks               |                       126 |
| Number of Transactions         |                       856 |
| Ethereum Gas Used              |               416,711,464 |
| Cycles per Block               |                87,738,505 |
| Cycles per Transaction         |                12,914,779 |
| Transactions per Block         |                         6 |
| Gas Used per Block             |                 3,307,233 |
| Gas Used per Transaction       |                   486,812 |
+--------------------------------+---------------------------+
```
