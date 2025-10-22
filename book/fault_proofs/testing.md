# Testing Guide

This guide explains how to run and understand the test suite for the OP Succinct fault dispute game system.

## Overview

The fault-proof crate includes comprehensive end-to-end tests that run actual proposer and challenger binaries against a forked Ethereum network. These tests validate the complete lifecycle of dispute games including creation, challenges, resolution, and bond claims.

## Prerequisites

Before running the tests, ensure you have:

1. **Rust toolchain installed**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

<<<<<<< HEAD
2. **Nightly Foundry installed** (required for contract bindings)
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup --install nightly-d592b3e0f142d694c3be539702704a4a73238773
   ```

   As of July 11, 2025, you need to use Forge nightly for binding generation.

3. **Environment variables configured**
   ```bash
   export L1_RPC=<YOUR_L1_RPC>
   export L1_BEACON_RPC=<YOUR_L1_BEACON_RPC>
   export L2_RPC=<YOUR_L2_RPC>
   export L2_NODE_RPC=<YOUR_L2_NODE_RPC>
   ```
||||||| ae1b78c
See [Proposer Configuration](./proposer.md#configuration) and [Challenger Configuration](./challenger.md#configuration) for more information on how to configure the proposer and challenger properly.
=======
2. **Nightly Foundry installed** (required for contract bindings)
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup --install nightly-d592b3e0f142d694c3be539702704a4a73238773
   ```

   As of July 11, 2025, you need to use Forge nightly for binding generation.

3. **Environment variables configured**
   ```bash
   export L1_RPC=<YOUR_L1_RPC>
   export L2_RPC=<YOUR_L2_RPC>
   export L2_NODE_RPC=<YOUR_L2_NODE_RPC>
   ```
>>>>>>> upstream/main

## Available Tests

### End-to-End Tests (`fault-proof/tests/e2e.rs`)

<<<<<<< HEAD
The test suite includes two comprehensive end-to-end tests that validate the complete fault dispute game lifecycle:
||||||| ae1b78c
#### 1. Proposer Defense Scenario
`test_proposer_defends_successfully()`: Tests the scenario where:
- The proposer creates a valid game
- A malicious challenger challenges it
- The proposer successfully defends with a valid proof
=======
The asynchronous end-to-end suite spins up real proposer and challenger services, interacts with
the dispute game factory, and warps Anvil time to exercise full lifecycles. Each test uses the
`_native` services and validates that contracts and coordination logic behave as expected.
>>>>>>> upstream/main

<<<<<<< HEAD
#### 1. Honest Proposer Full Lifecycle
`test_honest_proposer()`: Tests the complete lifecycle when proposer creates valid games:
- **Phase 1: Game Creation**: Proposer creates games naturally based on L2 state
- **Phase 2: Challenge Period**: Time warp to near end of challenge duration (no challenges submitted)
- **Phase 3: Resolution**: Games automatically resolve in proposer's favor after challenge period
- **Phase 4: Bond Claims**: Proposer claims bonds after airgap period (7 days warped to seconds)
||||||| ae1b78c
### End-to-End Tests
=======
#### Lifecycle Coverage
>>>>>>> upstream/main

<<<<<<< HEAD
#### 2. Honest Challenger Full Lifecycle
`test_honest_challenger()`: Tests challenger winning against invalid games:
- **Phase 1: Create Invalid Games**: Test manually creates games with invalid output roots
- **Phase 2: Challenge Period**: Challenger automatically detects and challenges invalid games
- **Phase 3: Resolution**: Time warp past prove deadline, games resolve in challenger's favor
- **Phase 4: Bond Claims**: Challenger claims bonds from all defeated games after airgap period
||||||| ae1b78c
#### 1. Proposer Wins Scenario
`test_e2e_proposer_wins()`: Tests the happy path where:
- The honest proposer creates valid games
- No challenges are submitted
- Games are resolved successfully in favor of the proposer after timeout

#### 2. Challenger Wins Scenario
`test_e2e_challenger_wins()`: Tests the scenario where:
- The malicious proposer creates invalid games
- The challenger successfully challenges them
- Games resolve in favor of the challenger after timeout
=======
- `test_honest_proposer_native()`: Covers the proposer happy path end-to-end.
  - **Phase 1: Game Creation** – Proposer seeds three canonical games from L2 state.
  - **Phase 2: Challenge Period** – Time is warped to the end of `MAX_CHALLENGE_DURATION`.
  - **Phase 3: Resolution** – Games auto-resolve to `ProposerWins`.
  - **Phase 4: Bond Claims** – Proposer claims the bonds after
    `DISPUTE_GAME_FINALITY_DELAY_SECONDS`.
- `test_honest_challenger_native()`: Validates that a challenger finds and defeats invalid roots.
  - **Phase 1: Create Invalid Games** – Helper code submits outputs with random invalid roots.
  - **Phase 2: Challenges** – Challenger service files disputes for each invalid game.
  - **Phase 3: Resolution** – The clock is warped past both challenge and prove windows, ensuring
    `ChallengerWins`.
  - **Phase 4: Bond Claims** – Challenger recovers bonds once the finality delay elapses.

#### Game Type Transition

- `test_game_type_transition_skips_legacy_games()`: Seeds the factory with legacy permissioned
  games, ensures the respected game type is restored, and verifies the proposer starts producing
  fresh `TEST_GAME_TYPE` games without touching legacy ones. The test also asserts that historical
  mock games remain `IN_PROGRESS`.

#### Game Chain Validation Scenarios

These tests focus on anchor selection, parent validation, and handling of invalid chains:

- `test_game_chain_validation_invalid_parent()`: Builds a chain where a valid child points to an
  invalid parent. When the proposer relaunches, it skips the poisoned branch and continues from a
  clean anchor.
- `test_game_chain_validation_challenged_parent()`: Creates a valid parent/child pair, allows the
  parent to be challenged and resolved to `ChallengerWins`, and confirms the proposer restarts from
  a fresh anchor instead of extending the tainted chain.
- `test_game_chain_validation_anchor_reset()`: Constructs two branches, finalizes an alternate
  branch, manually resets the anchor registry, and verifies the proposer begins building on the new
  canonical ancestor (`parentIndex == b1_index`).
>>>>>>> upstream/main

## Running the Tests

### Basic Test Execution
```bash
<<<<<<< HEAD
# Run all end-to-end tests
cargo test --release --test e2e
```
||||||| ae1b78c
# For e2e tests
cargo test --test e2e <TEST_NAME>

# For integration tests
cargo test --test integration <TEST_NAME>
```

For example:
```bash
# Run the proposer defense test
cargo test --test integration test_proposer_defends_successfully

# Run the proposer wins e2e test
cargo test --test e2e test_e2e_proposer_wins
```
=======
# Run all end-to-end tests with single thread and no capture
cargo test --release --test e2e -- --test-threads=1 --nocapture
```
>>>>>>> upstream/main
