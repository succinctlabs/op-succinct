# Troubleshooting

## Common Issues

### Missing Trie Node Error

**Error Message:**

```text
Error: server returned an error response: error code -32000: missing trie node <hash> (path) state <hash> is not available, not found
```

**Cause:**
This error occurs when your L1 archive node has not fully synced all historical state data. It's common when:
- The archive node was recently synced
- You're attempting to prove blocks where the complete state data is not yet archived in the node

**Solution:**
1. Wait for your L1 archive node to fully sync more historical state data (typically takes about an hour, depending on your batcher interval)
2. Try proving blocks that are definitely included in your node's archived data:
   - With more recent blocks
   - Or wait longer for older blocks to be fully synced

### L2 Block Validation Failure

**Error Message:**

```text
Failed to validate L2 block #<block_number> with output root <hash>
```

**Cause:**
This error means the derived L2 output root does not match the expected root.
An L1 head that does not include all required batch data can cause this error.
Check that your nodes are synced and use the expected chain and rollup configuration.

**Solution:**
1. If derivation logs show exhausted L1 data, test a larger `L1_HEAD_BUFFER`.
   The default is `0`; set `20` to restore the previous buffer:

```bash
L1_HEAD_BUFFER=20
```

2. If you're still encountering this error, you can try:
   - Waiting for more L1 blocks to be produced and retry
   - Using a different L2 node with a more consistent safe head state
   - For development/testing, you can increase the buffer further (e.g., to 150 blocks)

**Technical Details:**
SafeDB identifies the L1 block with enough data to derive the target L2 block.
The default buffer of `0` uses that block without adding extra L1 blocks.
A positive buffer can help diagnose an inconsistent SafeDB view, but it does not fix missing RPC data or node configuration errors.
The selected L1 head remains capped by `L1_BLOCK_TAG` and `L1_CONFIRMATIONS`.

Reference: [Proposer configuration](validity/proposer.md#optional-environment-variables)

### Cached Rollup Config Differs from Node RPC

**Warning Message:**

```text
WARN Cached rollup config differs from node RPC — expected during hardfork transitions
```

**Cause:**
The proposer caches the rollup config on first run and reuses it on subsequent startups. This warning appears when the cached config doesn't match what the node RPC returns — typically because the node was upgraded before hardfork activation.

**Solution:**
No action needed. The cached config is correct for the proposer's current identity. The on-chain vkey check (`on_chain_vkeys_match`) independently prevents the proposer from creating games with a mismatched config.
