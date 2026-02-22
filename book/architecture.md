# Architecture

## System Overview

OP Succinct is a lightweight upgrade to the OP Stack that enables ZK-based finality.

This document explains the standard OP Stack components and the lightweight modifications OP Succinct adds to enable proving blocks with zero-knowledge proofs using [SP1](https://docs.succinct.xyz/docs/sp1/introduction).

## Standard OP Stack Design

In the specification of the [standard OP Stack design](https://specs.optimism.io/), there are 4 main components.

1. OP Geth: Execution engine for the L2.
2. OP Batcher: Collects and batches users transactions efficiently and posts to L1.
3. OP Node: Reads batch data from L1, and passes to OP Geth to perform state transitions.
4. OP Proposer: Posts state roots from OP Node to L1. Enables withdrawal processing.

![OP Stack Architecture](./assets/opstack_architecture_0424.png)

## OP Succinct Design

### Overview

OP Succinct is a lightweight upgrade to the OP Stack that enables ZK-based finality. Specifically, it upgrades a single on-chain contract and the `op-proposer` component. No changes are needed to `op-geth`, `op-batcher`, or `op-node`.

![OP Succinct Design](./assets/opsuccinct_architecture_0424.png)

### Service Architecture

The OP Succinct Proposer is a new service that orchestrates the proving pipeline. It monitors L1 for posted batches, generates proofs, and submits them to L1 with ZK proofs.

1. User transactions are processed by standard OP Stack components.
2. Validity proofs are generated for ranges of blocks with the range program.
3. Combine range proofs into a single aggregationproof that is cheaply verifiable on-chain.
4. OP Succinct proposer submits aggregation proof to the on-chain contract.
5. The on-chain contract verifies the proof and updates the L2 state root for withdrawals.

![OP Succinct Architecture](./assets/op-succinct-proposer-architecture.jpg)

### Proof Types

OP Succinct uses a two-stage proving pipeline, with different proof types at each stage:

#### Range Proofs (STARK — Compressed)

The range program produces **compressed STARK proofs** via SP1. This is hardcoded and not configurable — range proofs always use `SP1ProofMode::Compressed`. Compressed proofs are smaller than core STARK proofs but are not yet suitable for on-chain verification because verifying STARKs on-chain is too expensive.

#### Aggregation Proofs (SNARK — Plonk or Groth16)

Multiple range proofs are combined into a single **aggregation proof**. The aggregation step converts the STARK proofs into a **SNARK proof** (either Plonk or Groth16), which is compact and cheap to verify on-chain.

The aggregation proof mode is configurable via the `AGG_PROOF_MODE` environment variable:
- `plonk` (default) — produces a PLONK SNARK proof.
- `groth16` — produces a Groth16 SNARK proof.

> **Note:** Changing the aggregation proof mode requires updating the SP1 verifier contract address in your deployment. The on-chain verifier contract must match the proof type. See [SP1 Contract Addresses](https://docs.succinct.xyz/docs/sp1/verification/contract-addresses) for verifier addresses.

#### On-Chain Verification

The on-chain contract (`OPSuccinctL2OutputOracle` or `OPSuccinctFaultDisputeGame`) verifies only **aggregation proofs**, not individual range proofs. It calls `ISP1Verifier.verifyProof(...)` with the aggregation verification key. The `ISP1Verifier` contract must correspond to the proof mode (Plonk or Groth16) being used.

#### Summary

| Stage | Proof Mode | Proof System | Configurable | On-Chain |
|---|---|---|---|---|
| Range | `Compressed` | STARK | No | No |
| Aggregation | `Plonk` (default) or `Groth16` | SNARK | Yes (`AGG_PROOF_MODE`) | Yes |
