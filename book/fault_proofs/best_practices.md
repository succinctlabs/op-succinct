# Best Practices

This document covers best practices for OP Succinct Lite deployment setup.

## Key Recommendations

- Enable SafeDB for op-node and set `NO_SAFE_DB=false` in `.env.proposer` as a safeguard to ensure proper op-node configuration.

## SafeDB Configuration

### Enabling SafeDB in op-node

SafeDB is a critical component for efficient L1 head determination. When SafeDB is not enabled, the system falls back to timestamp-based L1 head estimation, which can lead to several issues:

1. Less reliable derivation
2. Potential cycle count blowup for derivation

To enable SafeDB in your op-node, see [Consensus layer configuration options (op-node)](https://docs.optimism.io/operators/node-operators/configuration/consensus-config#safedbpath).

This ensures that L1 head can be efficiently determined without relying on the more expensive fallback mechanism.

### Enforcing SafeDB with NO_SAFE_DB Environment Variable

For production environments, you can enforce a sanity check that verifies SafeDB is properly enabled in your op-node by setting the `NO_SAFE_DB` environment variable in your `.env.proposer` file:

```env
NO_SAFE_DB=false
```

When this variable is set to `false`, proposer will panic if SafeDB is not available instead of falling back to the timestamp-based estimation. This helps prevent unexpected performance issues and ensures your setup is correctly configured.

Fallback behavior (setting `NO_SAFE_DB=true`) is set by default for the proposer.
