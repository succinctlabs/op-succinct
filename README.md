# op-succinct

OP Succinct is the production-grade proving engine for the OP Stack, powered by SP1.

With support for both validity proofs, with OP Succinct, and ZK fault proofs, with OP Succinct Lite, OP Succinct enables seamless upgrades for OP Stack rollups to a type-1 zkEVM rollup.

**[Docs](https://succinctlabs.github.io/op-succinct)**

## Repository Overview

> [!CAUTION]
> `main` is the development branch and may contain unstable code.
> For production use, please use the [latest release](https://github.com/succinctlabs/op-succinct/releases).

The repository is organized into the following directories:

- `book`: The documentation for OP Succinct users and developers.
- `contracts`: The solidity contracts for posting state roots to L1.
- `programs`: The programs for proving the execution and derivation of the L2 state transitions and proof aggregation.
- `validity`: The implementation of the `op-succinct/op-succinct` service.
- `fault-proof`: The implementation of the `op-succinct/fault-proof` service.
- `scripts`: Scripts for testing and deploying OP Succinct.
- `utils`: Shared utilities for the host, client, and proposer.

## Celo Modifications

Following modifications were introduced by Celo:
- additional env vars:
    - `CELO_SUPERCHAIN_CONFIG_ADDRESS` - if `OPTIMISM_PORTAL2_ADDRESS` is not specified - it allows to avoid deploying new SuperchainConifg / CeloSuperchainConfig
    - `ANCHOR_STATE_REGISTRY_ADDRESS` - if provided it allows to avoid deploying new AnchorStateRegistry
    - `DISPUTE_GAME_FACTORY_ADDRESS` - if provided it allows to avoid deploying new DisputeGameFactory
    - `CONFIGURE_CONTRACTS` - if `true` performs deployment of contracts, registering of new games on factory & setting respected game type on optimism portal, if `false` performs just deployment of contracts without configuration
- separation of justfile methods:
    - `deploy-fdg-contracts .{env}` - works like before (fetches config & deploys contracts)
    - `fetch-fdg-config .{env}` - allows to explicitly fetch config (useful when fetching live network config & deploying contracts over forked network in anvil)
    - `_deploy-fdg-contracts .{env} {config}.json` - allows to deploy contracts with specified environment & explicitly defined config
- introduction of multiple ways to deploy & configure contracts:
    - `CONFIGURE_CONTRACTS=true just deploy-fdg-contracts` - default behaviour that invokes `DeployOPSuccinctFDG.s.sol` underneath to deploy & configure contracts (requires providing single private key that is owner of DisputeGameFactory & guardian of OptimismPortal for new Game contract deployment) - for more details check: [deploy.md](/book/fault_proofs/deploy.md)
    - `CONFIGURE_CONTRACTS=false just deploy-fdg-contracts` + `PORTAL=0x... ConfigureDeploymentSafe.s.sol` - uses `DeployOPSuccinctFDG.s.sol` just for pre-deployment & actual configuration happens through Safe transaction (requires that Safe is owner of DisputeGameFactory & guardian of OptimismPortal, but allows to provide separate key for new Game contract deployment)
    - `CONFIGURE_CONTRACTS=false just deploy-fdg-contracts` + `PORTAL=address(0) ConfigureDeploymentSafe.s.sol` + `set-respected-game-type.sh` - divides deployment into 3 separate steps:
        - pre-deploys new Game contract with dedicated private key
        - registers new Game on DisputeGameFactory with Safe (Safe needs to be owner of DisputeGameFactory)
        - sets respected game type in OptimismPortal with dedicated private key (private key of guardian of OptimismPortal)

## Celo Remarks

Important is the fact of 2-stage config generation:
- first step is `.env` file generation that consists of:
    - configuration secrets like private key, rpc urls, api keys...
    - behavior flags like `CONFIGURE_CONTRACTS` or `OP_SUCCINCT_MOCK`
    - logic driving addresses like `OPTIMISM_PORTAL2_ADDRESS`, `CELO_SUPERCHAIN_CONFIG_ADDRESS`...
    - game configuration like `GAME_TYPE`, `PERMISSIONLESS_MODE`, `DISPUTE_GAME_FINALITY_DELAY_SECONDS`...
- second step is to call `fetch-fdg-config .{env}` that connects with L1/L2 RPCs & constructs JSON config file (`opsuccinctfdgconfig.json`) with additional rollup derived parameters:
    - `aggregationVkey` - The verification key for the aggregation SP1 program. Used to verify aggregation proofs that combine multiple range proofs into a single proof
    - `rangeVkeyCommitment` - A 32-byte commitment to the BabyBear representation of the verification key for the range SP1 program. Used to verify range proofs that prove the execution of a specific block range
    - `rollupConfigHash` - Hash of the chain's rollup configuration. Ensures proofs submitted are for the correct chain and prevents replay attacks
    - `startingL2BlockNumber` - The L2 block number from which the fault dispute game starts. Defines the genesis block for the dispute game system
    - `startingRoot` - The initial output root hash that serves as the starting point. Used to initialize the contract from an empty state, as OP Succinct requires a starting output root from which to prove the next state transitions

For more details on environment variables check: [configuration.md](book/contracts/configuration.md)

## Acknowledgments

This repo would not exist without:
* [OP Stack](https://docs.optimism.io/stack/getting-started): Modular software components for building L2 blockchains.
* [Kona](https://github.com/anton-rs/kona/tree/main): A portable implementation of the OP Stack rollup state transition, namely the derivation pipeline and the block execution logic.
* [SP1](https://github.com/succinctlabs/sp1): The fastest, most feature-complete zkVM for developers.
