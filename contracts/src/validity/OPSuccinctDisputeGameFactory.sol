// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {LibCWIA} from "@solady-v0.0.281/utils/legacy/LibCWIA.sol";
import {Claim, GameType, LibGameId, Timestamp} from "src/dispute/lib/Types.sol";

// Interfaces
import {IDisputeGame} from "@optimism/src/dispute/interfaces/IDisputeGame.sol";
import {ISemver} from "@optimism/src/universal/interfaces/ISemver.sol";

contract OPSuccinctDisputeGameFactory is ISemver {
    using LibCWIA for address;

    /// @notice The owner of the contract, who has admin permissions.
    address public owner;

    /// @notice The address of the OP Succinct DisputeGame implementation contract.
    address public gameImpl;

    /// @notice Semantic version.
    /// @custom:semver v1.0.0-beta
    string public constant version = "v1.0.0-beta";

    /// @notice An append-only array of disputeGames that have been created. Used by offchain game solvers to
    ///         efficiently track dispute games.
    IDisputeGame[] internal _disputeGameList;

    ////////////////////////////////////////////////////////////
    //                        Modifiers                       //
    ////////////////////////////////////////////////////////////

    modifier onlyOwner() {
        require(msg.sender == owner, "OPSuccinctDisputeGameFactory: caller is not the owner");
        _;
    }

    ////////////////////////////////////////////////////////////
    //                        Functions                       //
    ////////////////////////////////////////////////////////////

    /// @notice Constructs the OPSuccinctDisputeGameFactory contract.
    constructor(address _owner, address _gameImpl) {
        owner = _owner;
        gameImpl = _gameImpl;
    }

    /// @notice `gameAtIndex` returns the dispute game contract address at the given index.
    /// @param _index The index of the dispute game.
    /// @return proxy_ The clone of the `DisputeGame` created with the given parameters.
    ///         Returns `address(0)` if nonexistent.
    function gameAtIndex(uint256 _index) external view returns (IDisputeGame proxy_) {
        proxy_ = _disputeGameList[_index];
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

        _disputeGameList.push(game);
    }

    /// @notice Finds the `_n` most recent `IDisputeGame`'s starting at `_start`. If there are less than
    ///         `_n` games starting at `_start`, then the returned array will be shorter than `_n`.
    /// @param _start The index to start the reverse search from.
    /// @param _n The number of games to find.
    function findLatestGames(uint256 _start, uint256 _n) external view returns (IDisputeGame[] memory games_) {
        // If the `_start` index is greater than or equal to the game array length or `_n == 0`, return an empty array.
        if (_start >= _disputeGameList.length || _n == 0) return games_;

        // Allocate enough memory for the full array, but start the array's length at `0`. We may not use all of the
        // memory allocated, but we don't know ahead of time the final size of the array.
        assembly {
            games_ := mload(0x40)
            mstore(0x40, add(games_, add(0x20, shl(0x05, _n))))
        }

        // Perform a reverse linear search for the `_n` most recent games.
        for (uint256 i = _start; i >= 0 && i <= _start;) {
            IDisputeGame game = _disputeGameList[i];

            // Increase the size of the `games_` array by 1.
            // SAFETY: We can safely lazily allocate memory here because we pre-allocated enough memory for the max
            //         possible size of the array.
            assembly {
                mstore(games_, add(mload(games_), 0x01))
            }

            games_[games_.length - 1] = game;
            if (games_.length >= _n) break;

            unchecked {
                i--;
            }
        }
    }

    /// Updates the owner address.
    /// @param _owner The new owner address.
    function transferOwnership(address _owner) external onlyOwner {
        owner = _owner;
    }

    /// @notice Sets the implementation address.
    /// @param _implementation New implementation address.
    function setImplementation(address _implementation) external onlyOwner {
        gameImpl = _implementation;
    }
}
