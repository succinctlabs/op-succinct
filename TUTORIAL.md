This tutorial assumes you have an OP Stack chain running. If you don’t, follow [Optimism’s tutorial](https://docs.optimism.io/builders/chain-operators/tutorials/create-l2-rollup) to get that set up first.

Requirements:
- You should have access to the Admin keys needed to upgrade L1 contracts.
- Fault proofs should be turned off (ie you should use the `L2OutputOracle`, not `FaultDisputeGameFactory` for output roots).
- You should no longer have the original `op-proposer` running (you can simply exit from the command line, it won’t impact any of the other components).
- You’ve funded your `PROPOSER` wallet with at least 1 ETH (the Optimism tutorial suggests just 0.2 ETH, but it's more expensive to verify proofs than just post data).
- You have all the dependencies from the Optimism tutorial installed, as well as …
- Your L2 geth node must be running with the flags `--gcmode=archive` and `--state.scheme=hash` ([here’s an example](https://github.com/anton-rs/ops-anton/blob/main/L2/op-mainnet/op-geth/op-geth.sh)).

## Overview

Before getting into the details, it’s useful to understand a high level overview of what we’re trying to accomplish. If you’re not a total nerd, feel free to skip this section.

When an OP Stack chain is running, there are four main components interacting:
1) **op-geth** in sequencer mode takes in transactions from users, and uses them to generate and execute blocks.
2) **op-batcher** takes these blocks, compresses the transactions, and submits batches (in the form of blobs or calldata) onto L1.
3) **op-node** reads this batch data from L1 and uses it to drive op-geth of non-sequencer nodes to perform the same transactions (it also does this over P2P in an unsafe way, but we can ignore that for our purposes).
4) **op-proposer** posts an output root to L1 at regular intervals, which captures the L2 state so that withdrawals can be processed.

For OP to become a full ZK validity chain, parts 1-3 don’t need to change at all.

This tutorial focuses on replacing `op-proposer` (and the contracts it speaks with) for your chain, in a way that only allows the chain to progress with ZK proven blocks.

## Setup

Clone the SP Stack repo and checkout the tutorial branch:
```
git clone https://github.com/succinctlabs/op-succinct.git
git checkout tutorial
```
Copy the `.env.example` and `.env.server.example` files into an `.env` and `.env.server` file.
```
cp .env.example .env
cp .env.server.example .env.server
```
Fill in the values in both files. Instructions are provided in the comments.

Fill in the `contracts/zkconfig.json` file:
- startingBlockNumber: should be 0 for new chains
- l2RollupNode: should be running locally at http://localhost:9545 after tutorial
- submissionInterval: number of blocks per on-chain proof
- l2BlockTime: time per L2 block, should match what you used when deploying the chain
- proposer: if you want proposals to be permissioned, otherwise use address(0)
- challenger: permissioned role to undo proposals, otherwise use address(0)
- finalizationPeriod: how long after a state is proven can users withdraw?
- chainId: your L2 chain id (if you aren’t sure, try `cast rpc --rpc-url http://localhost:8551 eth_chainId | tr -d '"' | xargs cast 2d`)
- owner: address that can update the verification key and contracts
- vkey: initial verification key (you can call `cargo run --bin vkey` from the root of this repo to generate it)
- verifierGateway: address of verification contract, deployed on most chains at 0x3B6041173B80E77f038f3F2C0f9744f04837185e
- l2OutputOracleProxy: your already deployed L2OutputOracle proxy contract to upgrade

## Step 1: Deploy ZKL2OutputOracle

The first step is to replace your chain’s L2OutputOracle with a ZKL2OutputOracle.

The old L2OutputOracle contract allows a permissioned `proposer` role to submit output roots at any point. The permissioned `challenger` role can undo them. There are no checks for the validity of these claims.

The ZKL2OutputOracle is a small diff to this contract that makes the following changes:
- Allows anyone to submit proofs if `proposer == address(0)`.
- Requires a proof to be passed when proposing a new output root, which is verified using SP1’s Verifier Gateway.

To perform this upgrade, use the `ZKUpgrader.s.sol` script in the repo. This will deploy a new ZKL2OutputOracle contract and use the Admin private key from your `.env` file to upgrade the existing address to point to this new implementation. It will then call `initialize()` to set all the contract parameters appropriately.

```
cd contracts
forge script script/ZKUpgrader.s.sol:ZKUpgrader  --rpc-url <L1 RPC> --private-key <ADMIN PK> --verify --verifier etherscan --etherscan-api-key <ETHERSCAN API> --broadcast --slow --vvvv
```
(Equivalently, you can call `just upgrade-l2oo <L1_RPC> <ADMIN_PK> <Optional: ETHERSCAN_API_KEY>` from the root of the project.)

Once we have the contract upgraded, output roots will only be accepted when they come along with a ZK proof of their validity. But how are we going to generate these proofs?

## Step 2: Launch Kona SP1 Server

[Kona](https://github.com/ethereum-optimism/kona/) is a Rust implementation of the Optimism state transition function, used in some of their fault dispute games. [SP1](https://github.com/succinctlabs/sp1) is a performant zkVM, which can create ZK proofs for arbitrary Rust code. I think you see where this is going.

The Kona SP1 server exposes an API that, when called, kicks off the proof generation process:
1) It runs Kona for the range of blocks we want to prove in “witness generation” mode, storing preimages for all necessary state.
2) It sends all this data to Succinct’s prover network to generate proofs.
3) The prover network reruns Kona inside the SP1 zkVM, verifying the integrity of all the data passed and returning a proof.

The result is a fully ZK proven execution of the full state transition function.

To launch the server in a Docker image, simply call:
```
docker compose build
docker compose up -d
```

## Step 3: Upgrade the Proposer

The final step is to upgrade `op-proposer` itself.

As a reminder, you should already have shut down the old proposer in your existing OP stack implementation. It will be trying to submit output roots without proofs, so they will all be rejected.

The new proposer does the same thing as the old one (submit output roots to progress the chain), but is slightly more complex:
1) It watches the state of L1 to determine when the right amount of data has been posted to optimally request a proof. This is usually on the order of 1-5 minutes.
2) It requests a proof from the Kona SP1 server.
3) Once proofs have been generated for sufficient blocks that we are ready to post on chain (defined as `SUBMISSION_INTERVAL` in your `zkconfig.json` file), it creates an aggregate proof across all of these subproofs and submits the aggregate proof on chain.

Your new `op-proposer` is actually already running. The Docker command from the last section pulled [a branch from our Optimism monorepo fork](https://github.com/succinctlabs/optimism/tree/zk-proposer) that has modified the proposer in the above way, and then started it running inside of Docker.

## That’s It

Were you waiting for the complicated part of the tutorial?

There isn’t one.

You now have an upgraded contract on L1 that verifies proofs. You have a server running that speaks to SP1’s prover network to generate proofs for you. And you have an updated op-proposer that uses that server to generate proofs and posts them to L1.

Congrats. You are now running a zkOP chain.
