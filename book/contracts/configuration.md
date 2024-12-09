# Configuration

When deploying or upgrading the `OPSuccinctL2OutputOracle` contract, you will need to set the configuration parameters in your `.env` file.

## Required Parameters

When deploying or upgrading the `OPSuccinctL2OutputOracle` contract, the following parameters are required to be set in your `.env` file:

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L1_BEACON_RPC` | L1 Consensus (Beacon) Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
| `PRIVATE_KEY` | Private key for the account that will be deploying the contract. |
| `ETHERSCAN_API_KEY` | Etherscan API key used for verifying the contract (optional). |

## Optional Advanced Parameters

You can configure additional parameters when deploying or upgrading the `OPSuccinctL2OutputOracle` contract in your `.env` file.

| Parameter | Description |
|-----------|-------------|
| `VERIFIER_ADDRESS` | Default: Succinct's official Groth16 VerifierGateway. Address of the `ISP1Verifier` contract used to verify proofs. For mock proofs, this is the address of the `SP1MockVerifier` contract. |
| `STARTING_BLOCK_NUMBER` | Default: The finalized block number on L2. The block number to initialize the contract from. OP Succinct will start proving state roots from this block number. |
| `SUBMISSION_INTERVAL` | Default: `1000`. The minimum interval in L2 blocks at which checkpoints must be submitted. An aggregation proof can be posted for any range larger than this interval. |
| `FINALIZATION_PERIOD` | Default: `0`. The time period (in seconds) after which a proposed output becomes finalized and withdrawals can be processed. |
| `PROPOSER` | Default: The address of the account associated with `PRIVATE_KEY`. An Ethereum address authorized to submit proofs. Set to `address(0)` to allow permissionless submissions. **Note: Use `addProposer` and `removeProposer` functions to update the list of approved proposers.** |
| `CHALLENGER` | Default: `address(0)`, no one can dispute proofs. Ethereum address authorized to dispute proofs. |
| `OWNER` | Default: The address of the account associated with `PRIVATE_KEY`. Ethereum address authorized to update the `aggregationVkey`, `rangeVkeyCommitment`, `verifier`, and `rollupConfigHash` parameters. Can also transfer ownership of the contract and update the approved proposers. In a production setting, set to the governance smart contract or multi-sig of the chain. |
