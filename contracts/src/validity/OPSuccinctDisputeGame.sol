// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {OPSuccinctL2OutputOracle} from "./OPSuccinctL2OutputOracle.sol";
import {Clone} from "@solady/utils/Clone.sol";

import {ISemver} from "interfaces/universal/ISemver.sol";
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {Claim, GameStatus, GameType, GameTypes, Hash, Timestamp} from "@optimism/src/dispute/lib/Types.sol";
import {GameNotInProgress, OutOfOrderResolution} from "@optimism/src/dispute/lib/Errors.sol";

contract OPSuccinctDisputeGame is ISemver, Clone, IDisputeGame {

    /// @notice The address of the L2 output oracle proxy contract.
    address internal immutable l2OutputOracle;

    /// @notice The timestamp of the game's global creation.
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice Returns the current status of the game.
    GameStatus public status;

    /// @notice A boolean for whether or not the game type was respected when the game was created.
    bool public wasRespectedGameTypeWhenCreated;

    /// @notice Semantic version.
    /// @custom:semver v1.0.0-beta
    string public constant version = "v1.0.0-beta";

    constructor(address _l2OutputOracle) {
        l2OutputOracle = _l2OutputOracle;
    }

    ////////////////////////////////////////////////////////////
    //                    IDisputeGame impl                   //
    ////////////////////////////////////////////////////////////

    function initialize() external payable {
        // TODO (aleph) - Need to check that this is actually needed, since the main branch does not have it
        require(Timestamp.unwrap(createdAt) == 0, "Already initialized");

        createdAt = Timestamp.wrap(uint64(block.timestamp));
        status = GameStatus.IN_PROGRESS;
        wasRespectedGameTypeWhenCreated = true;

        (bool proposed, bool finalized) = OPSuccinctL2OutputOracle(l2OutputOracle).checkIfFinalized(rootClaim().raw());

        // We can check ahead of time
        if (proposed && finalized) {
            _resolve(GameStatus.DEFENDER_WINS);
        } else if (!proposed) {
            // The sequencer is REQUIRED to sequence the claimed root into the L2 output oracle BEFORE submitting to sequencing
            _resolve(GameStatus.CHALLENGER_WINS);
        }
        // If the game hasn't been finalized we need to wait for the finalization so we leave the game as pending
    }

    /// @notice Getter for the game type.
    /// @dev The reference impl should be entirely different depending on the type (fault, validity)
    ///      i.e. The game type should indicate the security model.
    /// @return gameType_ The type of proof system being used.
    function gameType() public pure returns (GameType) {
        // TODO: Once a new version of the Optimism contracts containing the PR below is released,
        // update this to return the correct game type: GameTypes.OP_SUCCINCT
        // https://github.com/ethereum-optimism/optimism/pull/13780
        return GameType.wrap(6);
    }

    /// @notice Getter for the creator of the dispute game.
    /// @dev `clones-with-immutable-args` argument #1
    /// @return The creator of the dispute game.
    function gameCreator() public pure returns (address) {
        return _getArgAddress(0x00);
    }

    /// @notice Getter for the root claim.
    /// @dev `clones-with-immutable-args` argument #2
    /// @return The root claim of the DisputeGame.
    function rootClaim() public pure returns (Claim) {
        return Claim.wrap(_getArgBytes32(0x14));
    }

    /// @notice Getter for the parent hash of the L1 block when the dispute game was created.
    /// @dev `clones-with-immutable-args` argument #3
    /// @return The parent hash of the L1 block when the dispute game was created.
    function l1Head() public pure returns (Hash) {
        return Hash.wrap(_getArgBytes32(0x34));
    }


    /// @notice If all necessary information has been gathered, this function should mark the game
    ///         status as either `CHALLENGER_WINS` or `DEFENDER_WINS` and return the status of
    ///         the resolved game. It is at this stage that the bonds should be awarded to the
    ///         necessary parties.
    /// @dev May only be called if the `status` is `IN_PROGRESS`.
    /// @return status_ The status of the game after resolution.
    function resolve() external returns (GameStatus status_) {
        // INVARIANT: Resolution cannot occur unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        (bool proposed, bool finalized) = OPSuccinctL2OutputOracle(l2OutputOracle).checkIfFinalized(rootClaim().raw());

        // We pick the game resolution based on if a root with higher number has finalized or this one has finalized
        if (proposed && finalized) {
            _resolve(GameStatus.DEFENDER_WINS);
            return(GameStatus.DEFENDER_WINS);
        } else if (!proposed) {
            // We hit this case only when the root was proposed into the L2 oracle but was challenged successfully, so should be rolled back
            _resolve(GameStatus.CHALLENGER_WINS);
            return(GameStatus.CHALLENGER_WINS);
        } else {
            revert("Cannot Resolve");
        }
    }

    /// @notice Internal resolve function: sets the status, emits a resolution and sets all state variables
    /// @param _status The status to resolve as
    function _resolve(GameStatus _status) internal {
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));
        status = _status;

        emit Resolved(_status);
    }

    /// @notice Getter for the extra data.
    /// @dev `clones-with-immutable-args` argument #4
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function extraData() public pure returns (bytes memory extraData_) {
        return new bytes(0);
    }

    /// @notice The l2BlockNumber of the disputed output root in the `L2OutputOracle`.
    function l2BlockNumber() public pure returns (uint256 l2BlockNumber_) {
        l2BlockNumber_ = _getArgUint256(0x54);
    }


    /// @notice A compliant implementation of this interface should return the components of the
    ///         game UUID's preimage provided in the cwia payload. The preimage of the UUID is
    ///         constructed as `keccak256(gameType . rootClaim . extraData)` where `.` denotes
    ///         concatenation.
    /// @return gameType_ The type of proof system being used.
    /// @return rootClaim_ The root claim of the DisputeGame.
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function gameData() external pure returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_) {
        gameType_ = gameType();
        rootClaim_ = rootClaim();
        extraData_ = new bytes(0);
    }
}
