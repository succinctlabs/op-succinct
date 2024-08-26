# op-succinct

This repo provides a way to take an OP stack chain & turn it in a full ZK-validity proven chain in < 1 hour by deploying 1 smart contract and spinning up a lightweight "proposer" service that ... 

Benefits include:... 

Read more: [blog post]

Reach out if you're interested in using this in production: ... 

## Getting Started

### Use an existing OP stack rollup

* Get an existing OP stack rollup deployed using [...] or with [...].
    * Today, we require that the rollup uses Ethereum for DA (blobs or calldata).
    * We require that the rollups use `OptimismPortalGatewayV1.sol` (not the V2 of the Gateway, which is for optimistic fault proofs).

### Deploy ZK L2 Output Oracle

Our ZK L2 Output Oracle is a "ZK" version of the L2 Output Oracle used by the V1 version of the OP stack (the non-fault proof version). In particular,... 

To deploy it, you can use the following commands in the `contracts` folder of this repo:

```
```

Make sure to set your `.env` with a `PRIVATE_KEY` and ... and following the .env.example.


### Spin up the ZK op-proposer

The next step of the integration

**Get prover network API key**: follow instructions here

### Transfer L2 Output Oracle to ZK L2 Output Oracle

The final step is to ...

### Check it's working

Now you can check that it's working by examining: ... 

To troubleshooot you can: ...

## Profiling Cycle Counts

To learn how to estimate cycle counts for a given block range, check out our [Cycle Count Guide](./zkvm-host/CYCLE_COUNT.md).

## Repository Overview

**`crates`**
- `client-utils`: A suite of utilities for the client program.
- `host-utils`: A suite of utilities for constructing the host which runs the OP Succinct program.

**`sp1-kona`**
- `native-host`: The host program which runs the `op-succinct` program natively using `kona`.
- `zkvm-host`: The host program which runs the `op-succinct` program in the SP1 zkVM.
- `client-programs`: The programs proven in SP1.
    - `fault-proof` and `range` are used to verifiably derive and execute single blocks
    and batches of blocks respectively. Their binary's are first run in native mode on the `kona-host` to
    fetch the witness data, then they use SP1 to verifiably execute the program.
   - For `aggregation`, which is used to generate an aggregate proof for a set of batches,
   first generate proofs for `range` programs for each batch, then use `aggregation` to
   generate an aggregate proof.

## Acknowlements

This repo could not exist without:
* Kona
* OP Stack
* SP1

## Open Source

This code is open sourced under the [Apache 2.0 License](./LICENSE.txt).
