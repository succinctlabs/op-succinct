# 🚀 op-succinct

OP Succinct turns any OP stack rollup into a full type-1 zkEVM Rollup in 1 hour using SP1.

** 📚 [Docs](https://succinctlabs.github.io/op-succinct)**

## 🛠️ Getting Started

Today, you can already use OP Succinct to upgrade any existing OP Stack rollup to a type-1 zkEVM rollup. To get started, make sure you have [Rust](https://rustup.rs/), [Foundry](https://book.getfoundry.sh/), and [Docker](https://docs.docker.com/engine/install/) installed. Then, follow the steps in the [book](https://succinctlabs.github.io/op-succinct/) to deploy the `OPSuccinctL2OutputOracle` contract and start the OP Succinct service.

## 📁 Repository Overview

> 🔴 CAUTION 🔴
> 
> `main` is the development branch and may contain unstable code.
> For production use, please use the [latest release](https://github.com/succinctlabs/op-succinct/releases).

The repository is organized into the following directories:

- 📚`book`: The documentation for OP Succinct users and developers.
- 📜`contracts`: The solidity contracts for posting state roots to L1.
- 🧩`programs`: The programs for proving the execution and derivation of the L2 state transitions and proof aggregation.
- 📤`proposer`: The implementation of the `op-succinct/op-proposer` and `op-succinct/succinct-proposer` services.
- 🖥️`scripts`: The scripts for proving and generating witnesses.

## 🙏Acknowledgments

This repo could not exist without:
* [OP Stack](https://docs.optimism.io/stack/getting-started): Modular software components for building L2 blockchains.
* [Kona](https://github.com/anton-rs/kona/tree/main): A portable implementation of the OP Stack rollup state transition, namely the derivation pipeline and the block execution logic.
* [SP1](https://github.com/succinctlabs/sp1): A fast, feature-complete zkVM for developers that can prove the execution of arbitrary Rust (or any LLVM-compiled) program.
