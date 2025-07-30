// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// The game type for the OP Succinct Fault Dispute Game.
// Eventually will be enshrined in the game type enum.
uint32 constant OP_SUCCINCT_FAULT_DISPUTE_GAME_TYPE = 42;

/// @notice The public values committed to for an OP Succinct aggregation program.
struct AggregationOutputs {
    bytes32 l1Head;
    bytes32 l2PreRoot;
    bytes32 claimRoot;
    uint256 claimBlockNum;
    bytes32 rollupConfigHash;
    bytes32 rangeVkeyCommitment;
    address proverAddress;
}

/// @notice Retrieves the address from the code of a proxy contract.
/// @param code The code of the proxy contract.
/// @return The address of the implementation contract.
function _getAddressFromCode(bytes memory code) pure returns (address) {
    address implementation;

    assembly {
        // Point to the memory location of the address data.
        let pointer := add(add(code, 0x20), 65)
        // Load 32 bytes from the pointer.
        let data := mload(pointer)
        // Right-shift by 96 bits to align the 20-byte address.
        implementation := shr(96, data)
    }
    return implementation;
}
