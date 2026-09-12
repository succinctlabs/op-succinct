# Experimental Features

This section covers experimental features for OP Succinct (Validity).

## EigenDA DA

The `op-succinct-eigenda` service monitors the state of an OP Stack chain with EigenDA enabled, uses an EigenDA Proxy to retrieve and validate blobs from DA certificates, requests proofs from the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro) and submits them to L1.

For detailed setup instructions, see the [EigenDA DA](./eigenda.md) section.

## AltDA Batch Input Size

For Keccak256 AltDA, set `alt_da.da_max_input_size` in the rollup configuration to limit each resolved batch, in bytes.
If you omit this field, OP Succinct uses 130,672 bytes, the op-alt-da default.
An explicit zero causes pipeline setup to fail.
Inputs at the limit are accepted.
The derivation pipeline skips larger inputs and continues with the next batch.

Use the same fixed value in OP Succinct, op-node, and op-batcher for the rollup's history.
Check that all three versions support this field before you configure a custom limit.
Adding or changing the field changes the rollup config hash.
Generate the configuration with the `altda` feature and update the [on-chain parameters](../contracts/update-parameters.md).
Validate historical derivation before changing an existing chain's limit.
