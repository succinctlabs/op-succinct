// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

// Interfaces
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";

// Contracts
import {AccessManager} from "src/fp/AccessManager.sol";

contract AccessManagerFactory {
    function createAccessManager(
        uint256 _fallbackTimeout,
        IDisputeGameFactory _factory,
        address _owner,
        bytes32 _salt
    ) external returns (AccessManager accessManager_) {
        accessManager_ = new AccessManager{salt: _salt}(_fallbackTimeout, _factory);
        accessManager_.transferOwnership(_owner);
    }
}
