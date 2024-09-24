# Cost Estimator

We provide a convenient CLI tool to fetch the RISC-V cycle counts for generating ZKPs for a range of blocks for a given rollup.

## Overview

In the root directory, add the following RPCs to your `.env` file for your rollup:

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |

More details on the RPC requirements can be found in the [prerequisites](./getting-started/prerequisites.md) section.

Then run the following command:
```shell
just cost-estimator <start_l2_block> <end_l2_block>
```

This command will split the block range into smaller ranges as if the `op-succinct-proposer` service was running. It will then fetch the required data for generating the ZKP for each of these ranges, and execute the SP1 `range` program. Once each program finishes, it will collect the statistics and output the aggregate statistics.

## Example

On Optimism Sepolia, proving the block range 15840000 to 15840050 (50 blocks) takes 2 range proofs, ~1.8B cycles and
~2 minutes to execute.

```bash
RUST_LOG=info just cost-estimator 15840000 15840050

...Execution Logs...

+--------------------------------+---------------------------+
| Metric                         | Value                     |
+--------------------------------+---------------------------+
| Batch Start                    |                16,240,000 |
| Batch End                      |                16,240,050 |
| Execution Duration (seconds)   |                       130 |
| Total Instruction Count        |             1,776,092,063 |
| Oracle Verify Cycles           |               237,150,812 |
| Derivation Cycles              |               493,177,851 |
| Block Execution Cycles         |               987,885,587 |
| Blob Verification Cycles       |                84,995,660 |
| Total SP1 Gas                  |             2,203,604,618 |
| Number of Blocks               |                        51 |
| Number of Transactions         |                       160 |
| Ethereum Gas Used              |                43,859,242 |
| Cycles per Block               |                74,736,691 |
| Cycles per Transaction         |                23,422,603 |
| Transactions per Block         |                        11 |
| Gas Used per Block             |                 3,509,360 |
| Gas Used per Transaction       |                 1,105,066 |
| BN Pair Cycles                 |                         0 |
| BN Add Cycles                  |                         0 |
| BN Mul Cycles                  |                         0 |
| KZG Eval Cycles                |                         0 |
| EC Recover Cycles              |                 9,407,847 |
+--------------------------------+---------------------------+
```
