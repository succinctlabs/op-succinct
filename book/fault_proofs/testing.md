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
- **Challenge Duration**: 2 minutes (configurable)
- **Prove Duration**: 3 minutes (configurable)
- **Bond Claim Delay**: 1 minute (configurable)
- **Total Test Time**: ~5 minutes per full lifecycle

## Prerequisites

Before running the tests, ensure you have:

1. **Rust toolchain installed**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Environment variables configured**
   ```bash
   export L1_RPC=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
   export L1_BEACON_RPC=https://sepolia.beaconcha.in
   export L2_RPC=https://sepolia.optimism.io
   export L2_NODE_RPC=$L2_RPC  # Can be same as L2_RPC
   ```

3. **Built binaries**
   ```bash
   cargo build --release --bin proposer
   cargo build --release --bin challenger
   ```

## Available Tests

### End-to-End Tests (`fault_proof/tests/e2e.rs`)

#### 1. Honest Proposer Full Lifecycle
`test_honest_proposer()`: Tests the complete lifecycle when proposer creates valid games:
- **Phase 1**: Proposer creates games naturally
- **Phase 2**: Time warp to challenge deadline (no challenges submitted)
- **Phase 3**: Games resolve in proposer's favor
- **Phase 4**: Proposer claims bonds after delay period

#### 2. Honest Challenger Full Lifecycle
`test_honest_challenger()`: Tests challenger winning against invalid games:
- **Phase 1**: Test creates invalid games manually
- **Phase 2**: Challenger detects and challenges automatically
- **Phase 3**: Games resolve in challenger's favor (no defense proof)
- **Phase 4**: Challenger claims bonds after delay period

### Integration Tests (`fault_proof/tests/integration.rs`)

#### 1. Proposer Defense Scenario
`test_proposer_defends_successfully()`: Tests successful defense against malicious challenges:
- Proposer creates valid game
- Malicious challenger submits invalid challenge
- Proposer automatically defends with valid proof
- Game resolves in proposer's favor

## Running the Tests

### Basic Test Execution
```bash
# Run all end-to-end tests
cargo test --test e2e

# Run specific test
cargo test --test e2e test_honest_proposer

# Run with detailed logging
RUST_LOG=info cargo test --test e2e -- --nocapture

# Run with debug logging for troubleshooting
RUST_LOG=debug cargo test --test e2e -- --nocapture
```

### Test Options
```bash
# Run tests with custom thread count (recommended: 1 for E2E tests)
cargo test --test e2e -- --test-threads=1

# Run in release mode for faster execution
cargo test --release --test e2e

# Show test output even on success
cargo test --test e2e -- --nocapture
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

## Test Structure

### Common Test Utilities (`fault_proof/tests/common/`)
- `anvil.rs`: Anvil fork management and time manipulation
- `contracts.rs`: Smart contract deployment helpers
- `env.rs`: Test environment setup and configuration
- `monitor.rs`: Event monitoring and game state tracking
- `process.rs`: Binary process management

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
   Solution: Build the binaries first with `cargo build --bin proposer`

3. **AccessManager Deployment Fails**
   ```
   ⚠️ AccessManager deployment reverted
   ```
   Note: This is a known issue on Anvil forks and is handled gracefully

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
   The tests capture stdout/stderr from binaries. Look for `[proposer stderr]` or `[challenger stderr]` in logs.

## CI Integration

Tests can be run in CI environments with the following considerations:

1. **Timeouts**: E2E tests may take 5-10 minutes each
2. **Resources**: Tests require ~4GB RAM for Anvil fork
3. **Parallelization**: Run E2E tests with `--test-threads=1` to avoid conflicts
4. **Secrets**: Ensure RPC URLs are configured as CI secrets

Example GitHub Actions configuration:
```yaml
- name: Run E2E Tests
  run: cargo test --release --test e2e
  env:
    L1_RPC: ${{ secrets.L1_RPC }}
    L1_BEACON_RPC: ${{ secrets.L1_BEACON_RPC }}
    L2_RPC: ${{ secrets.L2_RPC }}
    L2_NODE_RPC: ${{ secrets.L2_NODE_RPC }}
```
