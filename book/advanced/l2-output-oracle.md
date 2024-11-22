# OP Succinct L2 Output Oracle

## Overview

The `OPSuccinctL2OutputOracle` contract is a modification of the `L2OutputOracle` contract that is used to verify the state roots of the OP Stack rollup.

### Modifications to `L2OutputOracle`

The original `L2OutputOracle` contract can be found [here](https://github.com/ethereum-optimism/optimism/blob/3e68cf018d8b9b474e918def32a56d1dbf028d83/packages/contracts-bedrock/src/L1/L2OutputOracle.sol#L199-L202).

The changes introduced in the `OPSuccinctL2OutputOracle` contract are:

1. The `submissionInterval` parameter is now the minimum interval in L2 blocks at which checkpoints must be submitted. An aggregation proof can be posted after this interval has passed.
2. The addition of the `aggregationVkey`, `rangeVkeyCommitment`, `verifierGateway`, `startingOutputRoot`, and `rollupConfigHash` parameters. `startingOutputRoot` is used for initalizing the contract from an empty state, because `op-succinct` requires a starting output root from which to prove the next state root. The other parameters are used for verifying the proofs posted to the contract.
3. The addition of `historicBlockHashes` to store the L1 block hashes which the `op-succinct` proofs are anchored to. Whenever a proof is posted, the merkle proof verification will use these L1 block hashes to verify the state of the L2 which is posted as blobs or calldata to the L1.
4. The new `checkpointBlockHash` function which checkpoints the L1 block hash at a given L1 block number using the `blockhash` function.
5. The `proposeL2Output` function now takes an additional `_proof` parameter, which is the proof that is posted to the contract, and removes the unnecessary `_l1BlockHash` parameter (which is redundant given the `historicBlockHashes` mapping). This function also verifies the proof using the `ISP1VerifierGateway` contract.