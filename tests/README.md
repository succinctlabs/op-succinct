# E2E Tests

This directory hosts the end-to-end tests for `op-succinct`, built on top of
Optimism's devstack. Use the `justfile` here to prepare contract artifacts and
run the suite with the expected environment.

## Layout

- `e2e/`: Go e2e tests (currently covers the Succinct validity proposer flow).
- `artifacts/`: Contract artifacts; a compressed tarball lives at
`artifacts/compressed/artifacts.tzst` and is unpacked into `artifacts/src`
before tests.
- `optimism/`: Vendored `succinctlabs/optimism` repository containing
`op-succinct` integration code with Optimism devstack.
- `bindings/`, `presets/`, `utils/`: Presets, helpers and generated code used by
the tests.

## Prerequisites

- Go 1.23+ (matches `go.mod`).
- Rust toolchain (to build the `validity` binary).
- `just`, `zstd`, and `tar` available on your PATH.

## Setup

1) From this directory, fetch/update contract artifacts:

   ```just
   just update-packages
   ```

   This rebuilds artifacts via the vendored Optimism deployer and saves them to
   `artifacts/compressed/artifacts.tzst`.

2) Unpack artifacts (auto-run by the test target, but available standalone):

   ```just
   just unzip-contract-artifacts
   ```

## Running the e2e suite
- Recommended (builds the validity proposer, unpacks artifacts):

  ```just
  just test-e2e-sysgo validity
  ```

- Run a single test with a filter:

  ```just
  just test-e2e-sysgo validity TestValidityProposer_ProveSingleRange
  ```
