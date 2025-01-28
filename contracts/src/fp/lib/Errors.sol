// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

////////////////////////////////////////////////////////////////
//            `OPSuccinctFaultDisputeGame` Errors             //
////////////////////////////////////////////////////////////////

/// @notice Thrown when the caller is not the proposer.
error NotProposer();

/// @notice Thrown when the claim has already been challenged.
error ClaimAlreadyChallenged();

/// @notice Thrown when the game type of the parent game does not match the current game.
error UnexpectedGameType();

/// @notice Thrown when the parent game is invalid.
error InvalidParentGame();

/// @notice Thrown when the proof has an unexpected L1 head.
error UnexpectedL1Head(bytes32 l1Head);

/// @notice Thrown when the proof has an unexpected starting output root.
error UnexpectedStartingOutputRoot(bytes32 startingOutputRoot);

/// @notice Thrown when the proof has an unexpected claim block number.
error UnexpectedClaimBlockNum(uint256 claimBlockNum);

/// @notice Thrown when the proof has an unexpected rollup config hash.
error UnexpectedRollupConfigHash(bytes32 rollupConfigHash);

/// @notice Thrown when the proof has an unexpected range vkey commitment.
error UnexpectedRangeVkeyCommitment(bytes32 rangeVkeyCommitment);

/// @notice Thrown when the claim has not been challenged.
error ClaimNotChallenged();

/// @notice Thrown when the parent game is not resolved.
error ParentGameNotResolved();
