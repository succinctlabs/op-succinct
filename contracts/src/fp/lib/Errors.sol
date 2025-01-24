// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

////////////////////////////////////////////////////////////////
//            `OPSuccinctFaultDisputeGame` Errors             //
////////////////////////////////////////////////////////////////

/// @notice Thrown when the proof has an unexpected L1 head.
error UnexpectedL1Head(bytes32 l1Head);

/// @notice Thrown when the proof has an unexpected starting output root.
error UnexpectedStartingOutputRoot(bytes32 startingOutputRoot);

/// @notice Thrown when the proof has an unexpected claim block number.
error UnexpectedClaimBlockNum(uint256 claimBlockNum);
