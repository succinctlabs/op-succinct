// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {IDisputeGame} from "@optimism/src/dispute/interfaces/IDisputeGame.sol";
import {LibCWIA} from "@solady/utils/legacy/LibCWIA.sol";

contract OPSuccinctDisputeGameFactory {
    using LibCWIA for address;

    /// @notice The address of the OP Succinct DisputeGame implementation contract.
    address public gameImpl;

    /// @notice Constructs the OPSuccinctDisputeGameFactory contract.
    constructor(address _gameImpl) {
        gameImpl = _gameImpl;
    }

    /// @notice Creates a new DisputeGame proxy contract.
    function create(bytes32 _rootClaim, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof)
        external
        payable
    {
        IDisputeGame game = IDisputeGame(
            gameImpl.clone(
                abi.encodePacked(msg.sender, _rootClaim, bytes32(0), abi.encode(_l2BlockNumber, _l1BlockNumber, _proof))
            )
        );

        game.initialize{value: msg.value}();
    }
}
