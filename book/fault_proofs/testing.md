# Testing Guide

This guide explains how to run and understand the test suite for the OP Succinct fault dispute game system.

## Overview

The fault-proof crate includes comprehensive end-to-end tests that run actual proposer and challenger binaries against a forked Ethereum network. These tests validate the complete lifecycle of dispute games including creation, challenges, resolution, and bond claims.

## Test Architecture

### Test Environment
- **L1**: Anvil fork of Sepolia (enables time manipulation for faster testing)
- **L2**: Real L2 network (e.g., OP Sepolia)
- **Binaries**: Actual `proposer` and `challenger` binaries running as separate processes
- **Contracts**: Full dispute game contract suite deployed on the forked network

### Test Timing
Tests use time warping to compress days of real-world dispute game timing into minutes:
- **Challenge Duration**: 1 hour (MAX_CHALLENGE_DURATION)
- **Prove Duration**: 12 hours (MAX_PROVE_DURATION)
- **Bond Claim Delay (Airgap)**: 7 days (DISPUTE_GAME_FINALITY)
- **Total Test Time**: ~2-3 minutes per full lifecycle (using time warping)

## Prerequisites

Before running the tests, ensure you have:

1. **Rust toolchain installed**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Foundry installed** (required for contract bindings)
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

   **Note**: As of July 8, 2025, you need to use Forge nightly for binding generation:
   ```bash
   foundryup --install nightly
   ```

3. **Environment variables configured**
   ```bash
   export L1_RPC=<YOUR_L1_RPC>
   export L1_BEACON_RPC=<YOUR_L1_BEACON_RPC>
   export L2_RPC=<YOUR_L2_RPC>
   export L2_NODE_RPC=<YOUR_L2_NODE_RPC>
   ```

4. **Built binaries**
   ```bash
   cargo build --release --bin proposer
   cargo build --release --bin challenger
   ```

## Available Tests

### End-to-End Tests (`fault_proof/tests/e2e.rs`)

The test suite includes two comprehensive end-to-end tests that validate the complete fault dispute game lifecycle:

#### 1. Honest Proposer Full Lifecycle
`test_honest_proposer()`: Tests the complete lifecycle when proposer creates valid games:
- **Phase 1: Game Creation**: Proposer creates games naturally based on L2 state
- **Phase 2: Challenge Period**: Time warp to near end of challenge duration (no challenges submitted)
- **Phase 3: Resolution**: Games automatically resolve in proposer's favor after challenge period
- **Phase 4: Bond Claims**: Proposer claims bonds after airgap period (7 days warped to seconds)

#### 2. Honest Challenger Full Lifecycle
`test_honest_challenger()`: Tests challenger winning against invalid games:
- **Phase 1: Create Invalid Games**: Test manually creates games with invalid output roots
- **Phase 2: Challenge Period**: Challenger automatically detects and challenges invalid games
- **Phase 3: Resolution**: Time warp past prove deadline, games resolve in challenger's favor
- **Phase 4: Bond Claims**: Challenger claims bonds from all defeated games after airgap period

## Running the Tests

### Basic Test Execution
```bash
# Run all end-to-end tests (thread count 1 is recommended for E2E tests)
cargo test --release --test e2e -- --test-threads=1 --nocapture

# Run specific test
cargo test --release --test e2e test_honest_proposer

```

### Logging Levels
The tests use structured logging with different levels:
- `info` (default): Major test milestones and results
- `debug`: Detailed contract deployment and transaction info
- `trace`: Very detailed execution traces

Example with module-specific logging:
```bash
RUST_LOG=info,e2e::common::contracts=debug cargo test --test e2e
```

Enable process stdout logging for debugging:
```bash
TEST_LOG_STDOUT=true cargo test --test e2e -- --nocapture
```

## Test Structure

### Common Test Utilities (`fault_proof/tests/common/`)
- `anvil.rs`: Anvil fork management and time manipulation
- `contracts.rs`: Smart contract deployment helpers
- `env.rs`: Test environment setup and configuration
- `monitor.rs`: Event monitoring and game state tracking
- `process.rs`: Binary process management

### Test Configuration

Key test constants defined in `fault_proof/tests/common/contracts.rs`:
- **TEST_GAME_TYPE**: 42 (test-specific game type)
- **INIT_BOND**: 0.01 ETH (minimal bond for testing)
- **CHALLENGER_BOND**: 1 ETH (bond required for challenges)
- **MAX_CHALLENGE_DURATION**: 1 hour (time to submit a challenge)
- **MAX_PROVE_DURATION**: 12 hours (time to submit proof after challenge)
- **DISPUTE_GAME_FINALITY**: 7 days (delay before bonds can be claimed)

### Test Phases
Each full lifecycle test follows these phases:
1. **Setup**: Fork L1, deploy contracts, start binaries
2. **Game Creation**: Monitor and track game creation
3. **Challenge Period**: Time warp and monitor challenges
4. **Resolution**: Verify correct game resolution
5. **Bond Claims**: Verify bonds are claimed properly

## Troubleshooting

### Common Issues

1. **Environment Variables Missing**
   ```
   thread 'test_honest_proposer' panicked at 'L1_RPC must be set'
   ```
   Solution: Ensure all required environment variables are set

2. **Binary Not Found**
   ```
   Could not find proposer binary
   ```
   Solution: Build the binaries first with `cargo build --release --bin proposer --bin challenger`

3. **Contract Binding Generation Fails**
   ```
   error: failed to run custom build command for `bindings v0.1.0`
   ```
   Solution: Ensure Foundry is installed and `forge` is in your PATH. As of July 8, 2025, you must use Forge nightly:
   ```bash
   foundryup --install nightly
   ```

4. **Test Timeouts**
   ```
   Error: Timeout waiting for bond claims
   ```
   Solution: This may indicate a bug in the bond claiming logic. Check that all games are being processed correctly.

### Debug Tips

1. **Enable Debug Logging**
   ```bash
   RUST_LOG=debug cargo test --test e2e -- --nocapture
   ```

2. **Run Single Test**
   ```bash
   cargo test --test e2e test_honest_proposer -- --exact
   ```

3. **Check Process Output**
   By default, tests only capture stderr from binaries. Look for `[proposer stderr]` or `[challenger stderr]` in logs.
   To enable stdout logging for debugging:
   ```bash
   TEST_LOG_STDOUT=true cargo test --test e2e -- --nocapture
   ```
