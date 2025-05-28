# Troubleshooting

## Common Issues

### Missing Trie Node Error

**Error Message:**
```
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
```
Failed to validate L2 block #<block_number> with output root <hash>
```

**Cause:**
This error occurs when the L1 head block selected is too close to the batch posting block, causing the derivation process to fail. The L2 node may have an inconsistent view of the safe head state, requiring additional L1 blocks to properly derive and validate the L2 blocks.

**Solution:**
This has been resolved through DA-aware L1 head calculation:

1. **For Ethereum DA**: Uses a small buffer of 20 blocks after the batch posting block
2. **For Celestia DA**: Uses blobstream commitment logic to ensure data availability by finding L1 blocks where batches with committed Celestia data were posted

**Technical Details:**
The system now automatically calculates the appropriate L1 head based on the DA type:

- **Ethereum DA**: Simple offset logic since data is immediately available once the L1 transaction is included
- **Celestia DA**: Sophisticated blobstream-aware logic that checks which Celestia blocks have been committed to Ethereum and finds the corresponding L1 blocks where safe batches were posted

The calculation happens in the host layer (`calculate_safe_l1_head` method) rather than using hardcoded offsets in the fetcher layer. This ensures each DA type gets the appropriate treatment without heuristic values.

Reference: [Host Implementation](utils/host/src/host.rs) and [Celestia Blobstream Utils](utils/celestia/host/src/blobstream_utils.rs)
