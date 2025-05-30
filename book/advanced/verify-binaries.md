# Verify the OP Succinct binaries

When deploying OP Succinct in production, it's important to ensure that the SP1 programs used when generating proofs are reproducible.

## Introduction

Recall there are two programs used in OP Succinct:
- `range`: Proves the correctness of an OP Stack derivation + STF for a range of blocks.
- `aggregation`: Aggregates multiple range proofs into a single proof. This is the proof that lands on-chain. The aggregation proof ensures that all `range` proofs in a given block range are linked and use the `rangeVkeyCommitment` from the `L2OutputOracleProxy` as the verification key.

## Prerequisites

To reproduce the OP Succinct program binaries, you first need to install the [cargo prove](https://docs.succinct.xyz/docs/sp1/getting-started/install#option-1-prebuilt-binaries-recommended) toolchain.

Ensure that you have the latest version of the toolchain by running:

```bash
sp1up
```

Confirm that you have the toolchain installed by running:

```bash
cargo prove --version
```

### Verify the SP1 binaries

To build the SP1 binaries, first ensure that Docker is running.

```bash
docker ps
```

Then build the binaries:

```bash
cd programs/range
# Build the range elfs
cargo prove build --elf-name range-elf-bump --docker --tag v5.0.0
cargo prove build --elf-name range-elf-embedded --docker --tag v5.0.0 --features embedded

cd ../aggregation
# Build the aggregation-elf
cargo prove build --elf-name aggregation-elf --docker --tag v5.0.0
```

Now, you can verify the binaries. The `config` script outputs the rollup config hash, aggregation verification key, and range verification key commitment based on the ELFs in `/elf`.

```bash
cargo run --bin config --release
```
