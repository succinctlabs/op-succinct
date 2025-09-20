# Run `run-multi`

The `run-multi` helper executes the OP Succinct range program for a span of L2 blocks and can
optionally generate artifacts for later reuse.

## Prerequisites

- Populate a `.env` file at the repository root with the RPC endpoints required by the host:

  ```env
  L1_RPC=<YOUR_L1_RPC_ENDPOINT>
  L1_BEACON_RPC=<YOUR_L1_BEACON_RPC_ENDPOINT>
  L2_RPC=<YOUR_L2_RPC_ENDPOINT>
  L2_NODE_RPC=<YOUR_L2_NODE_RPC_ENDPOINT>
  ```


- Install [`just`](https://github.com/casey/just).

## Command

```bash
just run-multi <start-block> <end-block> \
  use-cache=true \
  prove=false \
  save-artifacts=true
```

- `start-block` / `end-block` define the inclusive range executed by the host.
- `use-cache=true` reuses cached witness data when available (set to `false` to recompute).
- `prove=true` produces a compressed proof saved to
  `data/<l2-chain-id>/proofs/<start>-<end>.bin`.
- `save-artifacts=true` stores the embedded program and serialized stdin at
  `data/<l2-chain-id>/binaries/<start>-<end>_{program,stdin}.bin`. Set to `false` to skip writing
  these files when you only need execution stats. The `data/` directory is gitignored, so saved artifacts will not appear in `git diff` output.

## Choose a Block Range

Use the latest finalized L2 block as an upper bound and ensure the range size fits within your
proving capacity. You can query the finalized block with Foundry's `cast` CLI:

```bash
cast block finalized --json --rpc-url "$L2_RPC" | jq '.number'
```

You can use the value as the `end-block`. Set `start-block` to an earlier finalized block so that `end-block - start-block` matches the number of blocks you want to execute.

``` admonish info
The `data/` directory is gitignored, so saved artifacts and proofs will not appear in `git diff`
output. Copy files elsewhere if you need to share them.
```
