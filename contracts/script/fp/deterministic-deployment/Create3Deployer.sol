// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import {Create3} from "./Create3.sol";

contract Create3Deployer {
    /**
     * @notice Deploy a contract using Create3
     * @param _salt Salt for deterministic address
     * @param _creationCode Creation code of contract to deploy
     * @return The address of the deployed contract
     */
    function create3(bytes32 _salt, bytes memory _creationCode) external payable
returns (address) {
        return Create3.create3(_salt, _creationCode, msg.value);
    }

    /**
     * @notice Compute the address of a contract deployed via Create3
     * @param _salt Salt used for deployment
     * @return The deterministic address
     */
    function addressOf(bytes32 _salt) external view returns (address) {
        return Create3.addressOf(_salt);
    }
}
