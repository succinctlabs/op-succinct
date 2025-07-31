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

/// @notice Parses proxy contract code to extract both the prefix bytes and implementation address.
/// @param code The code of the proxy contract.
/// @return prefix The bytes preceding the implementation address (first 65 bytes).
/// @return implementation The address of the implementation contract.
function _parseProxyCode(bytes memory code) pure returns (bytes memory prefix, address implementation) {
    // Security check: ensure the code is at least 85 bytes (65 byte prefix + 20 byte address)
    require(code.length >= 85, "Code too short for proxy parsing");

    // Allocate memory for the 65-byte prefix
    prefix = new bytes(65);

    assembly {
        // Source pointer: skip the length field (0x20) to get to actual code data
        let srcPtr := add(code, 0x20)
        // Destination pointer: skip the length field of the prefix bytes
        let destPtr := add(prefix, 0x20)

        // Copy 64 bytes in two 32-byte chunks
        let chunk1 := mload(srcPtr)
        let chunk2 := mload(add(srcPtr, 0x20))
        mstore(destPtr, chunk1)
        mstore(add(destPtr, 0x20), chunk2)

        // For the 65th byte and address extraction, load the next 32 bytes
        let chunk3 := mload(add(srcPtr, 0x40))
        // Store the 65th byte (first byte of chunk3)
        mstore8(add(destPtr, 0x40), byte(0, chunk3))

        // Extract address from bytes 65-84 (already loaded in chunk3)
        // The address starts at byte 65, which is at position 1 in chunk3
        // Shift left by 8 bits to remove the first byte, then right by 96 bits for address
        implementation := shr(96, shl(8, chunk3))
    }

    return (prefix, implementation);
}

/// @notice Retrieves the address from the code of a proxy contract.
/// @param code The code of the proxy contract.
/// @return The address of the implementation contract.
function _getAddressFromCode(bytes memory code) pure returns (address) {
    (, address implementation) = _parseProxyCode(code);
    return implementation;
}

/// @notice Retrieves the proxy prefix bytes (all bytes before the address) from proxy contract code.
/// @param code The code of the proxy contract.
/// @return prefix The bytes preceding the implementation address (first 65 bytes).
function _getProxyPrefixFromCode(bytes memory code) pure returns (bytes memory prefix) {
    (prefix,) = _parseProxyCode(code);
    return prefix;
}
