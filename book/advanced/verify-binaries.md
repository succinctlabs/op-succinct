# Verify the OP Succinct binaries

When deploying OP Succinct in production, the verification process relies on deterministic builds, so it's important to ensure that the SP1 programs used when generating proofs are reproducible.

Recall there are two programs used in OP Succinct:

-   `range`
    -   Proves the correctness of an OP Stack derivation + STF for a range of blocks.
-   `aggregation`
    -   Aggregates multiple range proofs into a single proof. This is the proof that lands on-chain. The aggregation proof ensures that all `range` proofs in a given block range are linked and use the `rangeVkeyCommitment` from the `L2OutputOracleProxy` as the verification key.

Anything that changes these programs requires reproducing the binaries. This includes changes to the program logic or bumping program dependencies.

## Prerequisites

Install the [`cargo prove`](https://docs.succinct.xyz/docs/sp1/getting-started/install#option-1-prebuilt-binaries-recommended) CLI version that matches the checked-out release.
For this revision, use SP1 6.10.0:

```bash
sp1up -v 6.10.0
```

and

```bash
cargo prove --version
```

Also, make sure Docker is running:

```bash
docker ps
```

## Reproduce the program binaries

To build the binaries, run:

```bash
# Build the range elfs
just build-range-elfs

# Build the aggregation elf
just build-agg-elf
```

The updated binaries will be saved in the [`/elf`](https://github.com/succinctlabs/op-succinct/tree/main/elf) directory.

## Verify the program binaries

To verify the binaries, run:

```bash
cargo run --bin config --release
```

This will log the rollup config hash, aggregation verification key, and range verification key commitment based on the latest ELFs in the [`/elf`](https://github.com/succinctlabs/op-succinct/tree/main/elf) directory.

For the `agglayer` images, `config` is shipped as `op-succinct-vkeys`, so the keys can be read with:

```
$ docker run --rm --entrypoint op-succinct-vkeys <image>
Range Verification Key Hash: 0x7e6d4611456d99f96b25940f43ca5cf33807d6bf5991a9cf2d86093362fc164f
Aggregation Verification Key Hash: 0x009eb5031cd753aa3cec78e5e8621a5ee59fe2540c8756cb5dcb9a5746b2f8e8
Aggregation Verification Key: 0x00001970000078040000000042f6…
```

## Update the contract

After reproducing the binaries, you must either [deploy](../validity/contracts/deploy.md) a new OPSuccinctL2OutputOracle contract or perform a [rolling update](../validity/contracts/update-parameters.md) on an existing contract.
