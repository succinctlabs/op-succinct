# Data Availability

OP Succinct (Validity) supports alternative data availability layers in addition to Ethereum DA.

## Celestia

The `op-succinct-celestia` service monitors the state of an OP Stack chain with Celestia DA enabled, requests proofs from the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro) and submits them to the L1.

For detailed setup instructions, see the [Celestia](./celestia.md) section.

## EigenDA

The `op-succinct-eigenda` service monitors the state of an OP Stack chain with EigenDA enabled, uses an EigenDA Proxy to retrieve and validate blobs from DA certificates, requests proofs from the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro) and submits them to L1.

For detailed setup instructions, see the [EigenDA](./eigenda.md) section.
