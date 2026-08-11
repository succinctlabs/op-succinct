# Fault Proof Challenger

The fault proof challenger is a component responsible for monitoring and challenging invalid OP-Succinct fault dispute games on the L1 chain. It continuously scans for invalid games and challenges them to maintain L2 state validity.

## Prerequisites

Before running the challenger, ensure you have:

1. Rust toolchain installed (latest stable version)
2. Access to L1, L2 execution, and L2 rollup nodes
3. The DisputeGameFactory contract deployed (See [Deploy](./deploy.md))
4. Sufficient ETH balance for:
   - Transaction fees
   - Challenger bonds (proof rewards)
5. Required environment variables properly configured (See [Configuration](#configuration))

## Overview

The challenger performs several key functions:

1. **Game Monitoring**: Continuously syncs new games from the factory, validates their output roots, and marks ones that inherit a challenger win from their parent.
2. **Game Challenging**: Submits challenges for games flagged by the sync step and supports optional malicious testing.
3. **Game Resolution**: Resolves games the challenger countered once their deadlines pass and the parent dispute has settled.
4. **Bond Management**: Tracks finalized games and claims the challenger's credit before removing them from the cache.

## Configuration

The challenger is configured through environment variables.

`L1_RPC` and the contract settings configure the shared challenger lifecycle. `L2_RPC` and
`L2_NODE_RPC` are consumed by the default OP Stack game-validation backend; challengers that inject
a different `GameValidator` do not require those two endpoints.

Create a `.env.challenger` file in the `fault-proof` directory with all required variables. This single file is used by:
- Docker Compose (for both variable substitution and runtime configuration)
- Direct binary execution (`cargo run --bin challenger` from the `fault-proof` directory; the binary automatically loads `.env.challenger`)

### Required Environment Variables

| Variable | Description |
|----------|-------------|
| `L1_RPC` | L1 RPC endpoint URL |
| `L2_RPC` | L2 RPC endpoint URL |
| `L2_NODE_RPC` | RPC endpoint for the single op-node paired with `L2_RPC` |
| `ANCHOR_STATE_REGISTRY_ADDRESS` | Address of the AnchorStateRegistry contract |
| `FACTORY_ADDRESS` | Address of the DisputeGameFactory contract |
| `GAME_TYPE` | Type identifier for the dispute game |

Either `PRIVATE_KEY` or both `SIGNER_URL` and `SIGNER_ADDRESS` must be set for transaction signing:

| Variable | Description |
|----------|-------------|
| `PRIVATE_KEY` | Private key for transaction signing (if using private key signer) |
| `SIGNER_URL` | URL of the web3 signer service (if using web3 signer) |
| `SIGNER_ADDRESS` | Address of the account managed by the web3 signer (if using web3 signer) |

### Optional Environment Variables

| Variable | Description | Default Value |
|----------|-------------|---------------|
| `FETCH_INTERVAL` | Polling interval in seconds | `30` |
| `CHALLENGER_METRICS_PORT` | The port to expose metrics on. Update prometheus.yml to use this port, if using docker compose. | `9001` |
| `MALICIOUS_CHALLENGE_PERCENTAGE` | Percentage (0.0-100.0) of valid games to challenge for testing defense mechanisms | `0.0` |
| `SYNC_L1_CONFIRMATIONS` | Number of L1 blocks behind `latest` used for pinned state reads. The operator must choose a value appropriate for the chain's reorg assumptions; no challenge-window upper bound is enforced. | `0` |
| `TX_CONFIRMATION_TIMEOUT` | Maximum time (in seconds) to wait for an L1 transaction to reach the required number of confirmations. Setting this too low risks timeout-triggered retries that can lead to redundant operations. | `60` |

```env
# Required Configuration
L1_RPC=                              # L1 RPC endpoint URL
L2_RPC=                              # L2 RPC endpoint URL
L2_NODE_RPC=                         # RPC URL of the op-node paired with L2_RPC
ANCHOR_STATE_REGISTRY_ADDRESS=       # Address of the AnchorStateRegistry contract
FACTORY_ADDRESS=                     # Address of the DisputeGameFactory contract
GAME_TYPE=                           # Type identifier for the dispute game
PRIVATE_KEY=                         # Private key for transaction signing

# Optional Configuration
FETCH_INTERVAL=30                     # Polling interval in seconds
CHALLENGER_METRICS_PORT=9001          # The port to expose metrics on

# Testing Configuration (Optional)
MALICIOUS_CHALLENGE_PERCENTAGE=0.0    # Percentage of valid games to challenge for testing (0.0 = disabled)

# L1 State Snapshot Configuration (Optional)
SYNC_L1_CONFIRMATIONS=0               # L1 blocks behind latest used for pinned sync reads

# Transaction Configuration (Optional)
TX_CONFIRMATION_TIMEOUT=60            # L1 tx confirmation timeout in seconds (raise for congested L1s)
```

Each synchronization cycle pins factory, game, parent, registry, and credit reads to one canonical
L1 block at `latest - SYNC_L1_CONFIRMATIONS`. Deadline decisions continue to use the timestamp from
the cycle's `latest` block. Before submitting a challenge, resolution, or credit claim, the
challenger rechecks the relevant eligibility at a fresh canonical `latest` block and skips the
transaction if that preflight is unavailable or stale.

Before entering the main loop, the challenger verifies that the op-node rollup configuration has
the same L1 and L2 chain IDs as `L1_RPC` and `L2_RPC`, that SafeDB is enabled and populated, and
that its safe head exists with the same hash on the paired execution node. Startup validation is
retried until this fixed node pair is healthy.

The shared challenger lifecycle delegates chain-specific claim checks to a `GameValidator`.
The standard constructor installs the OP Stack validator, which owns the op-node and paired L2
execution providers and performs all of the SafeDB checks described below. Custom integrations may
inject another validator without changing game discovery, retry, deadline, or transaction handling.

For every active unchallenged game, the challenger resolves `game.l1Head` to its canonical L1
block number `X`, then queries `optimism_safeHeadAtL1Block(X)` after confirming that
`optimism_syncStatus.current_l1` has processed past `X`. A claim above this historical local-safe
head is invalid even if the execution node now contains the same block as unsafe. Node lag,
missing SafeDB history, execution history/state pruning, and L1/L2 hash mismatches remain
`Unavailable`: they are retried and alerted on, never challenged as unknown data.

`L2_NODE_RPC` must point directly to one dedicated op-node, not a node pool. `L2_RPC` must point to
the execution node paired with it. The op-node must have SafeDB enabled with `--safedb.path`, must
retain history covering the active challenge window, and must not use FollowSource or
SuperAuthority because the challenger relies on SafeDB having historical local-safe semantics.

If the confirmed L1 height moves backwards relative to the highest snapshot accepted by the
challenger, it fails closed and skips that cycle. The high-water mark is recorded before applying
the snapshot, so a cancelled or partially failed sync cannot later admit an older snapshot. An
unchanged confirmed height is still processed so unavailable validation and failed actions continue
to retry.

## Running

To run the challenger from the `fault-proof` directory:
```bash
# Uses .env.challenger by default
cargo run --bin challenger

# Or specify a custom environment file
cargo run --bin challenger -- --env-file custom.env
```

The challenger will run indefinitely, monitoring for invalid games and challenging them as needed.

## Testing Defense Mechanisms

The challenger supports **malicious challenging** of valid games for defense mechanisms testing purposes.

### Configuration

Set `MALICIOUS_CHALLENGE_PERCENTAGE` to enable malicious challenging:

```bash
# Production mode (default) - only challenge invalid games
MALICIOUS_CHALLENGE_PERCENTAGE=0.0

# Testing mode - challenge all valid games
MALICIOUS_CHALLENGE_PERCENTAGE=100.0

# Fine-grained testing - challenge 0.1% of valid games
MALICIOUS_CHALLENGE_PERCENTAGE=0.1

# Mixed testing - challenge 25.5% of valid games
MALICIOUS_CHALLENGE_PERCENTAGE=25.5
```

### Behavior

When malicious challenging is enabled:

1. **Priority 1**: Challenge invalid games (honest challenger behavior)
2. **Priority 2**: Challenge valid games at the configured percentage (defense mechanisms testing behavior); the percentage acts as a per-iteration probability gate.

The challenger will always prioritize challenging invalid games first, then optionally challenge valid games based on the configured percentage.

### Logging

The challenger relies on structured `tracing` logs:
- Honest challenges are logged as `Game challenged successfully` within the `[[Challenging]]` span.
- Malicious attempts emit `\x1b[31m[MALICIOUS CHALLENGE]\x1b[0m` so they stand out in the logs.

## Features

### Game Monitoring
- Incrementally pulls new games from the factory using an on-chain index cursor
- Checks game validity against the L2 state commitment
- Filters to the configured OP Succinct fault dispute game type that was respected at creation time
- Marks games for challenging, resolution, or bond claiming based on proposal status, parent outcomes, and deadlines

### Output Root Validation

The dispute game exposes its claimed L2 block number as a `uint256`, while the execution RPC block
number is limited to `u64`. Values up to and including `u64::MAX` are accepted. Larger values are
treated as invalid without issuing an L2 RPC request, rather than being truncated or causing the
challenger to panic.

For representable block numbers, the challenger requests the L2 header without full transaction
bodies. Post-Isthmus headers provide the L2-to-L1 message passer storage root through
`withdrawalsRoot`; for earlier blocks, the challenger falls back to `eth_getProof` at the same block
number when calculating the output root.

### Validation Outcomes and Retries

Output-root validation records one of three outcomes for each monitored game:

- `Valid`: the locally computed output root matches the claim.
- `Invalid`: the output root mismatches, the claimed L2 block number exceeds `u64::MAX`, or the
  claim is above the historical local-safe head at `game.l1Head`.
- `Unavailable`: validation could not complete because required RPC data could not be read.

Only `Invalid` authorizes an automatic challenge based on output-root validation. An `Unavailable`
game is never treated as invalid: while its challenge window remains open, the challenger retries it
on every polling cycle. At or after the deadline, it keeps the unavailable reason for observability
but stops fetching the output root and does not submit a challenge based on an unknown result.
Challenges inherited from a parent game that resolved in favor of the challenger remain independent
of this output-root classification.

Game discovery and validation retries are tracked independently from the factory cursor. A failed
index is retained for retry while the cursor continues to later indices, so one malformed or
temporarily unavailable game cannot starve subsequent discovery. Individual game refresh failures
are also isolated so other cached games can still be synchronized and acted on.

The following metrics expose unavailable validation and isolated synchronization failures:

- `op_succinct_fp_challenger_unverifiable_games`: in-progress, unchallenged games whose output root
  remains unavailable.
- `op_succinct_fp_challenger_nearest_unverifiable_deadline_seconds`: seconds until the nearest such
  deadline (`-1` when none and `0` once expired).
- `op_succinct_fp_challenger_sync_failures_total`: isolated discovery, refresh, and synchronization
  failures.
- `op_succinct_fp_challenger_confirmed_l1_head`: L1 block number selected for the latest pinned sync
  snapshot.
- `op_succinct_fp_challenger_l1_confirmation_lag_blocks`: block distance between `latest` and that
  confirmed snapshot.
- `op_succinct_fp_challenger_preflight_errors_total`: latest-state action preflight RPC errors.
- `op_succinct_fp_challenger_preflight_skips_total`: stale actions rejected by latest-state
  preflight.

### Game Challenging
- Submits challenges for games flagged by the sync step
- Challenges games that are in progress and either invalid or the parent is challenger wins
- Supports malicious challenging of valid games when enabled

### Game Resolution
The challenger:
- Tracks only games it countered
- Resolves games after their deadline once the parent dispute is resolved and it is own game

### Bond Claiming
- Flags challenger-win games once the anchor registry marks them finalized and there is credit to claim
- Claims credit for the challenger's address and removes games from the cache after claiming credit

## Architecture

The challenger is built around the `OPSuccinctChallenger` struct which manages:
- Configuration state
- Wallet management for transactions
- Game challenging and resolution logic
- Chain monitoring and interval management

Key components:
- `ChallengerConfig`: Handles environment-based configuration
- `sync_state`: Keeps the in-memory cache in sync with on-chain state, marking games for challenge, resolution, or bond claims
- `handle_game_challenging`: Submits challenge transactions for games flagged by the sync step and supports malicious testing
- `handle_game_resolution`: Resolves flagged games once they are eligible based on deadlines, parent outcomes and whether it is own game
- `handle_bond_claiming`: Claims challenger credit from finalized games and trims settled entries from the cache
- `run`: Main loop that orchestrates state syncing, challenging, resolution, and bond claiming at the configured interval while isolating task failures

## Error Handling

The challenger includes robust error handling for:
- RPC connection issues
- Transaction failures
- Contract interaction errors
- Invalid configurations

Errors are logged with appropriate context to aid in debugging.

## Development

When developing or modifying the challenger:
1. Ensure all environment variables are properly set
2. Test with a local L1/L2 setup first
3. Monitor logs for proper operation
4. Test challenging and resolution separately
5. Verify proper handling of edge cases
6. Test with various game states and conditions
