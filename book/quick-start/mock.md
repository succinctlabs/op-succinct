# Mock OP Succinct

Running OP Succinct in mock mode is useful for testing your configuration is correct before generating proofs.

## Overview

### 1) Set environment variables.

In the root directory, create a file called `.env` (mirroring `.env.example`) and set the following environment variables:

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
| `PRIVATE_KEY` | Private key for the account that will be deploying the contract. |
| `ETHERSCAN_API_KEY` | Etherscan API key for verifying the deployed contracts. |


### 2) Deploy an `SP1MockVerifier` contract which can verify mock SP1 proofs.

```bash
just deploy-mock-verifier
```

If successful, you should see the following output:

```
% just deploy-mock-verifier
[⠊] Compiling...
[⠑] Compiling 1 files with Solc 0.8.15
[⠘] Solc 0.8.15 finished in 615.84ms
Compiler run successful!
Script ran successfully.

== Return ==
0: address 0x4cb20fa9e6FdFE8FDb6CE0942c5f40d49c898646

## Setting up 1 EVM.

==========================

Chain 11155111

Estimated gas price: 3.851705636 gwei

Estimated total gas used for script: 171869

Estimated amount required: 0.000661988795953684 ETH

==========================
....
```

In these deployment logs, `0x4cb20fa9e6FdFE8FDb6CE0942c5f40d49c898646` is the address of the `SP1MockVerifier` contract.


#### Custom Environment

If you have multiple environments, you can specify the environment file to use with the `--env-file` flag.

```bash
just deploy-mock-verifier <env_file>
```

### 3) Deploy the `OPSuccinctL2OutputOracle` contract.

This contract is a modification of the `L2OutputOracle` contract which verifies a proof along with the proposed state root.

First, add the address of the `SP1MockVerifier` contract from the previous step to the `.env` file in the root directory.

| Parameter | Description |
|-----------|-------------|
| `VERIFIER_ADDRESS` | The address of the `SP1MockVerifier` contract. |

Then, deploy the `OPSuccinctL2OutputOracle` contract.

```bash
just deploy-oracle
```

