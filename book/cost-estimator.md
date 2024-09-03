# Cycle Counts [Cost Estimator]

We provide a convenient CLI tool to estimate the RISC-V cycle counts (and cost) for generating ZKPs for a range of blocks for a given rollup.

## Overview

First, add the following RPCs to your `.env` file for your rollup:

```bash
# L1 RPC
L1_RPC=
# L1 Consensus RPC
L1_BEACON_RPC=
# L2 Archive Node (OP-Geth)
L2_RPC=
```

It is required that the L2 RPC is an archival node for your OP stack rollup, with the "debug_dbGet" endpoint enabled.

Then run the following command:
```shell
RUST_LOG=info just cost-estimator <start_l2_block> <end_l2_block>
```

This command will execute `op-succinct` as if it's in production. First, it will divide the entire block range
into smaller ranges optimized along the span batch boundaries. Then it will fetch the required data for generating the ZKP for each of these ranges, and execute the SP1 `span` program. Once each program finishes, it will collect the statistics and output the aggregate statistics
for the entire block range. From this data, you can extrapolate the cycle count to a cost based on the cost per billion cycles.

## Example Block Range

On OP Sepolia, generating a proof from 15840000 to 15840050 (50 blocks) generates 4 span proofs, takes ~1.8B cycles and takes
~2 minutes to execute.

```bash
RUST_LOG=info just cost-estimator 15840000 15840050

...Execution Logs...

+--------------------------------+---------------------------+
| Metric                         | Value                     |
+--------------------------------+---------------------------+
| Total Cycles                   |             1,502,329,547 |
| Block Execution Cycles         |             1,009,112,508 |
| Total Blocks                   |                        51 |
| Total Transactions             |                       202 |
| Cycles per Block               |                19,786,519 |
| Cycles per Transaction         |                 4,995,606 |
| Transactions per Block         |                         3 |
| Total Gas Used                 |                52,647,751 |
| Gas Used per Block             |                 1,032,308 |
| Gas Used per Transaction       |                   260,632 |
+--------------------------------+---------------------------+
```

## Misc
- For large enough block ranges, the RISC-V SP1 program will surpass the SP1 memory limit. Recommended limit is 20-30 blocks.
- Your L2 node must have been synced for the blocks in the range you are proving. 

