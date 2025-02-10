// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Libraries
import {Clone} from "@solady/utils/Clone.sol";
import {
    Claim,
    Clock,
    Duration,
    GameStatus,
    GameType,
    Hash,
    LibClock,
    OutputRoot,
    Timestamp
} from "src/dispute/lib/Types.sol";
import {
    AlreadyInitialized,
    AnchorRootNotFound,
    BondTransferFailed,
    ClaimAlreadyResolved,
    ClockNotExpired,
    ClockTimeExceeded,
    GameNotInProgress,
    IncorrectBondAmount,
    NoCreditToClaim,
    UnexpectedRootClaim
} from "src/dispute/lib/Errors.sol";
import "src/fp/lib/Errors.sol";
import {AggregationOutputs} from "src/lib/Types.sol";
import {LibString} from "@solady/utils/LibString.sol";

// Interfaces
import {ISemver} from "src/universal/interfaces/ISemver.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IAnchorStateRegistry} from "src/dispute/interfaces/IAnchorStateRegistry.sol";

// Contracts
import {OPSuccinctEntryPoint} from "src/fp/OPSuccinctEntryPoint.sol";

/// @title OPSuccinctFaultDisputeGame
/// @notice An implementation of the `IFaultDisputeGame` interface.
contract OPSuccinctFaultDisputeGame is Clone, ISemver {
    ////////////////////////////////////////////////////////////////
    //                         Enums                              //
    ////////////////////////////////////////////////////////////////

    enum ProposalStatus {
        // The initial state of a new proposal
        Unchallenged,
        // A proposal that has been challenged but not yet proven
        Challenged,
        // An unchallenged proposal that has been proven valid with a verified proof
        UnchallengedAndValidProofProvided,
        // A challenged proposal that has been proven valid with a verified proof
        ChallengedAndValidProofProvided,
        // The final state after resolution, either GameStatus.CHALLENGER_WINS or GameStatus.DEFENDER_WINS.
        Resolved
    }

    ////////////////////////////////////////////////////////////////
    //                         Structs                            //
    ////////////////////////////////////////////////////////////////

    /// @notice The `ClaimData` struct represents the data associated with a Claim.
    struct ClaimData {
        uint32 parentIndex;
        address counteredBy;
        address prover;
        Claim claim;
        ProposalStatus status;
        Timestamp deadline;
    }

    ////////////////////////////////////////////////////////////////
    //                         Events                             //
    ////////////////////////////////////////////////////////////////

    /// @notice Emitted when the game is challenged.
    /// @param challenger The address of the challenger.
    event Challenged(address indexed challenger);

    /// @notice Emitted when the game is proved.
    /// @param prover The address of the prover.
    event Proved(address indexed prover);

    /// @notice Emitted when the game is resolved.
    /// @param status The status of the game after resolution.
    event Resolved(GameStatus indexed status);

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////

    /// @notice The maximum duration allowed for a challenger to challenge a game.
    Duration internal immutable MAX_CHALLENGE_DURATION;

    /// @notice The maximum duration allowed for a proposer to prove against a challenge.
    Duration internal immutable MAX_PROVE_DURATION;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    /// @notice The dispute game factory.
    IDisputeGameFactory internal immutable DISPUTE_GAME_FACTORY;

    /// @notice The SP1 verifier.
    ISP1Verifier internal immutable SP1_VERIFIER;

    /// @notice The rollup config hash.
    bytes32 internal immutable ROLLUP_CONFIG_HASH;

    /// @notice The vkey for the aggregation program.
    bytes32 internal immutable AGGREGATION_VKEY;

    /// @notice The 32 byte commitment to the BabyBear representation of the verification key of the range SP1 program. Specifically,
    /// this verification is the output of converting the [u32; 8] range BabyBear verification key to a [u8; 32] array.
    bytes32 internal immutable RANGE_VKEY_COMMITMENT;

    /// @notice The proof reward for the game. This is the amount of the bond that the challenger has to bond to challenge and
    ///         is the amount of the bond that is distributed to the prover when proven with a valid proof.
    uint256 internal immutable PROOF_REWARD;

    /// @notice The address of the entry point contract for the game.
    address payable internal immutable ENTRY_POINT;

    /// @notice The address of the anchor state registry.
    address internal immutable ANCHOR_STATE_REGISTRY;

    /// @notice Semantic version.
    /// @custom:semver 1.0.0
    string public constant version = "1.0.0";

    /// @notice The starting timestamp of the game
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice Returns the current status of the game.
    GameStatus public status;

    /// @notice Flag for the `initialize` function to prevent re-initialization.
    bool internal initialized;

    /// @notice The claim made by the proposer.
    ClaimData public claimData;

    /// @notice The starting output root of the game that is proven from in case of a challenge.
    /// @dev This should match the claim root of the parent game.
    OutputRoot public startingOutputRoot;

    /// @param _maxChallengeDuration The maximum duration allowed for a challenger to challenge a game.
    /// @param _maxProveDuration The maximum duration allowed for a proposer to prove against a challenge.
    /// @param _disputeGameFactory The factory that creates the dispute games.
    /// @param _sp1Verifier The address of the SP1 verifier that verifies the proof for the aggregation program.
    /// @param _rollupConfigHash The rollup config hash for the L2 network.
    /// @param _aggregationVkey The vkey for the aggregation program.
    /// @param _rangeVkeyCommitment The commitment to the range vkey.
    /// @param _proofReward The proof reward for the game.
    /// @param _entryPoint The address of the entry point contract for the game.
    /// @param _anchorStateRegistry The address of the anchor state registry.
    constructor(
        Duration _maxChallengeDuration,
        Duration _maxProveDuration,
        IDisputeGameFactory _disputeGameFactory,
        ISP1Verifier _sp1Verifier,
        bytes32 _rollupConfigHash,
        bytes32 _aggregationVkey,
        bytes32 _rangeVkeyCommitment,
        uint256 _proofReward,
        address payable _entryPoint,
        address _anchorStateRegistry
    ) {
        // Set up initial game state.
        GAME_TYPE = GameType.wrap(42);
        MAX_CHALLENGE_DURATION = _maxChallengeDuration;
        MAX_PROVE_DURATION = _maxProveDuration;
        DISPUTE_GAME_FACTORY = _disputeGameFactory;
        SP1_VERIFIER = _sp1Verifier;
        ROLLUP_CONFIG_HASH = _rollupConfigHash;
        AGGREGATION_VKEY = _aggregationVkey;
        RANGE_VKEY_COMMITMENT = _rangeVkeyCommitment;
        PROOF_REWARD = _proofReward;
        ENTRY_POINT = _entryPoint;
        ANCHOR_STATE_REGISTRY = _anchorStateRegistry;
    }

    /// @notice Extracts the proof bytes from the extra data.
    /// @dev The extra data is the calldata of the `createGame` function from the `OPSuccinctEntryPoint` contract.
    /// @dev Expected calldata length without proof bytes: 0x92
    //      - 0x04 selector
    //      - 0x14 creator address
    //      - 0x20 root claim
    //      - 0x20 l1 head
    //      - 0x20 extraData (l2BlockNumber)
    //      - 0x04 extraData (parentIndex)
    //      - 0x14 extraData (entryPoint)
    //      - 0x02 CWIA bytes
    //      - 0x?? proof bytes
    /// @dev There can be arbitrary length of optional proof bytes following the CWIA bytes.
    /// @dev 1. If the calldata size is less than 0x92, will revert with `BadExtraData()`.
    /// @dev 2. If the calldata size is exactly 0x92, will return an empty `proofBytes`.
    /// @dev 3. If the calldata size is greater than 0x92, will return `proofBytes` from the 0x92 index to the end of the calldata.
    function _extractProofFromExtraData() private pure returns (bytes memory proofBytes) {
        assembly {
            // The total size of the calldata *including* the 4-byte function selector.
            let size := calldatasize()

            // If calldatasize < 0x92, revert with `BadExtraData()`.
            if lt(size, 0x92) {
                // Store the selector for `BadExtraData()` and revert.
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }

            // 2. If calldatasize == 0x92, return an empty bytes array.
            if eq(size, 0x92) {
                // Allocate free memory for an empty bytes array of length 0.
                proofBytes := mload(0x40) // Fetch current free memory pointer.
                mstore(proofBytes, 0) // Set length = 0 in the 32 bytes length slot.
                mstore(0x40, add(proofBytes, 0x20)) // Advance free ptr by 32 (just for the length slot).
            }

            // 3. If calldatasize > 0x92, interpret everything beyond 0x92 as proof bytes.
            if gt(size, 0x92) {
                let proofLen := sub(size, 0x92)

                // Allocate a new bytes array in free memory
                proofBytes := mload(0x40)
                mstore(proofBytes, proofLen) // Set array length
                mstore(0x40, add(proofBytes, add(proofLen, 0x20))) // Advance free mem pointer (proofLen + 32 bytes for length)

                // Copy the proof from calldata[0x92...size] into memory[proofBytes+0x20]
                calldatacopy(add(proofBytes, 0x20), 0x92, proofLen)
            }
        }
    }

    /// @notice Initializes the contract.
    /// @dev This function may only be called once.
    function initialize() external payable virtual {
        // SAFETY: Any revert in this function will bubble up to the DisputeGameFactory and
        // prevent the game from being created.
        //
        // Implicit assumptions:
        // - The `gameStatus` state variable defaults to 0, which is `GameStatus.IN_PROGRESS`
        // - The dispute game factory will enforce the required bond to initialize the game.
        //
        // Explicit checks:
        // - The game must not have already been initialized.
        // - An output root cannot be proposed at or before the starting block number.

        // INVARIANT: The game must not have already been initialized.
        if (initialized) revert AlreadyInitialized();

        // INVARIANT: Games can only be created through the entry point contract.
        require(
            gameCreator() == ENTRY_POINT,
            string.concat(
                "Invalid entry point: expected ",
                LibString.toHexString(ENTRY_POINT),
                " but got ",
                LibString.toHexString(gameCreator())
            )
        );

        // The first game is initialized with a parent index of uint32.max
        if (parentIndex() != type(uint32).max) {
            // For subsequent games, get the parent game's information
            (,, IDisputeGame proxy) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());

            startingOutputRoot = OutputRoot({
                l2BlockNumber: OPSuccinctFaultDisputeGame(address(proxy)).l2BlockNumber(),
                root: Hash.wrap(OPSuccinctFaultDisputeGame(address(proxy)).rootClaim().raw())
            });

            // INVARIANT: The parent game must be a valid game.
            if (proxy.status() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();
        } else {
            // When there is no parent game, the starting output root is the anchor state for the game type.
            (startingOutputRoot.root, startingOutputRoot.l2BlockNumber) =
                IAnchorStateRegistry(ANCHOR_STATE_REGISTRY).anchors(GAME_TYPE);
        }

        // Do not allow the game to be initialized if the root claim corresponds to a block at or before the
        // configured starting block number.
        if (l2BlockNumber() <= startingOutputRoot.l2BlockNumber) {
            revert UnexpectedRootClaim(rootClaim());
        }

        // Set the root claim
        claimData = ClaimData({
            parentIndex: parentIndex(),
            counteredBy: address(0),
            prover: address(0),
            claim: rootClaim(),
            status: ProposalStatus.Unchallenged,
            deadline: Timestamp.wrap(uint64(block.timestamp + MAX_CHALLENGE_DURATION.raw()))
        });

        // Set the game as initialized.
        initialized = true;

        // Set the game's starting timestamp
        createdAt = Timestamp.wrap(uint64(block.timestamp));

        ////////////////////////////////////////////////////////////////
        //                     FAST-FINALITY MODE                    //
        ////////////////////////////////////////////////////////////////

        // Extract the proof bytes from the extra data.
        bytes memory proofBytes = _extractProofFromExtraData();

        // If the proof bytes are not empty, the game is created in fast-finality mode.
        if (proofBytes.length > 0) {
            this.prove(proofBytes);
        }
    }

    /// @notice The l2BlockNumber of the disputed output root in the `L2OutputOracle`.
    function l2BlockNumber() public pure returns (uint256 l2BlockNumber_) {
        l2BlockNumber_ = _getArgUint256(0x54);
    }

    /// @notice The parent index of the game.
    function parentIndex() public pure returns (uint32 parentIndex_) {
        parentIndex_ = _getArgUint32(0x74);
    }

    /// @notice The claimant of the game.
    function claimant() public pure returns (address claimant_) {
        claimant_ = _getArgAddress(0x78);
    }

    /// @notice Only the starting block number of the game.
    function startingBlockNumber() external view returns (uint256 startingBlockNumber_) {
        startingBlockNumber_ = startingOutputRoot.l2BlockNumber;
    }

    /// @notice Starting output root of the game.
    function startingRootHash() external view returns (Hash startingRootHash_) {
        startingRootHash_ = startingOutputRoot.root;
    }

    ////////////////////////////////////////////////////////////////
    //                    `IDisputeGame` impl                     //
    ////////////////////////////////////////////////////////////////

    /// @notice Challenges the game.
    function challenge() external payable returns (ProposalStatus) {
        // INVARIANT: Can only challenge a game that has not been challenged yet.
        if (claimData.status != ProposalStatus.Unchallenged) revert ClaimAlreadyChallenged();

        // INVARIANT: Cannot challenge a game if the clock has already expired.
        if (uint64(block.timestamp) > claimData.deadline.raw()) revert ClockTimeExceeded();

        // If the required bond is not met, revert.
        if (msg.value != PROOF_REWARD) revert IncorrectBondAmount();

        // Update the counteredBy address
        claimData.counteredBy = msg.sender;

        // Update the status of the proposal
        claimData.status = ProposalStatus.Challenged;

        // Update the clock to the current block timestamp, which marks the start of the challenge.
        claimData.deadline = Timestamp.wrap(uint64(block.timestamp + MAX_PROVE_DURATION.raw()));

        emit Challenged(claimData.counteredBy);

        return claimData.status;
    }

    /// @notice Proves the game.
    /// @param proofBytes The proof bytes to validate the claim.
    function prove(bytes calldata proofBytes) external returns (ProposalStatus) {
        // INVARIANT: Cannot prove a game if the clock has timed out.
        if (uint64(block.timestamp) > claimData.deadline.raw()) revert ClockTimeExceeded();

        // INVARIANT: Cannot prove a claim that has already been proven
        if (claimData.prover != address(0)) revert AlreadyProven();

        // Decode the public values to check the claim root
        AggregationOutputs memory publicValues = AggregationOutputs({
            l1Head: Hash.unwrap(l1Head()),
            l2PreRoot: Hash.unwrap(startingOutputRoot.root),
            claimRoot: rootClaim().raw(),
            claimBlockNum: l2BlockNumber(),
            rollupConfigHash: ROLLUP_CONFIG_HASH,
            rangeVkeyCommitment: RANGE_VKEY_COMMITMENT
        });

        SP1_VERIFIER.verifyProof(AGGREGATION_VKEY, abi.encode(publicValues), proofBytes);

        // Update the prover address
        claimData.prover = msg.sender;

        // Update the status of the proposal
        if (claimData.counteredBy == address(0)) {
            claimData.status = ProposalStatus.UnchallengedAndValidProofProvided;
        } else {
            claimData.status = ProposalStatus.ChallengedAndValidProofProvided;
        }

        emit Proved(claimData.prover);

        return claimData.status;
    }

    /// @notice Resolves the game after the clock expires.
    ///         `DEFENDER_WINS` when no one has challenged the proposer's claim and `MAX_CHALLENGE_DURATION` has passed
    ///         or there is a challenge but the prover has provided a valid proof within the `MAX_PROVE_DURATION`.
    ///         `CHALLENGER_WINS` when the proposer's claim has been challenged, but the proposer has not proven
    ///         its claim within the `MAX_PROVE_DURATION`.
    function resolve() external returns (GameStatus) {
        // INVARIANT: Resolution cannot occur unless the game has already been resolved.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        // INVARIANT: Cannot resolve a game if the parent game has not been resolved.
        GameStatus parentGameStatus;
        if (parentIndex() != type(uint32).max) {
            (,, IDisputeGame parentGame) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());
            parentGameStatus = parentGame.status();
        }
        // If this is the first dispute game (i.e. parent game index is `uint32.max`),
        // then the parent game's status is considered as `DEFENDER_WINS`.
        else {
            parentGameStatus = GameStatus.DEFENDER_WINS;
        }

        if (parentGameStatus == GameStatus.IN_PROGRESS) revert ParentGameNotResolved();

        // INVARIANT: If the parent game's claim is invalid, then the current game's claim is invalid.
        // INVARIANT: If there is no challenger, i.e. `claimData.counteredBy == address(0)`, the bond is burnt.
        if (parentGameStatus == GameStatus.CHALLENGER_WINS) {
            claimData.status = ProposalStatus.Resolved;
            status = GameStatus.CHALLENGER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));

            // Add challenger's reward to the entry point contract.
            (bool success,) = ENTRY_POINT.call{value: address(this).balance}(
                abi.encodeWithSelector(
                    OPSuccinctEntryPoint.addCredit.selector, claimData.counteredBy, address(this).balance
                )
            );
            if (!success) revert CreditTransferFailed();

            emit Resolved(status);

            return status;
        }

        if (claimData.status == ProposalStatus.Unchallenged) {
            if (claimData.deadline.raw() >= uint64(block.timestamp)) revert ClockNotExpired();

            claimData.status = ProposalStatus.Resolved;
            status = GameStatus.DEFENDER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));

            // Return the proposer's bond to the entry point contract.
            (bool success,) = ENTRY_POINT.call{value: address(this).balance}(
                abi.encodeWithSelector(OPSuccinctEntryPoint.addCredit.selector, claimant(), address(this).balance)
            );
            if (!success) revert CreditTransferFailed();

            emit Resolved(status);
        } else if (claimData.status == ProposalStatus.Challenged) {
            if (claimData.deadline.raw() >= uint64(block.timestamp)) revert ClockNotExpired();
            claimData.status = ProposalStatus.Resolved;
            status = GameStatus.CHALLENGER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));

            // Add challenger's reward to the entry point contract.
            (bool success,) = ENTRY_POINT.call{value: address(this).balance}(
                abi.encodeWithSelector(
                    OPSuccinctEntryPoint.addCredit.selector, claimData.counteredBy, address(this).balance
                )
            );
            if (!success) revert CreditTransferFailed();

            emit Resolved(status);
        } else if (claimData.status == ProposalStatus.UnchallengedAndValidProofProvided) {
            claimData.status = ProposalStatus.Resolved;
            status = GameStatus.DEFENDER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));

            // Return the proposer's bond to the entry point contract.
            (bool success,) = ENTRY_POINT.call{value: address(this).balance}(
                abi.encodeWithSelector(OPSuccinctEntryPoint.addCredit.selector, claimant(), address(this).balance)
            );
            if (!success) revert CreditTransferFailed();

            emit Resolved(status);
        } else if (claimData.status == ProposalStatus.ChallengedAndValidProofProvided) {
            claimData.status = ProposalStatus.Resolved;
            status = GameStatus.DEFENDER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));

            // Add prover's reward to the entry point contract.
            (bool success,) = ENTRY_POINT.call{value: PROOF_REWARD}(
                abi.encodeWithSelector(OPSuccinctEntryPoint.addCredit.selector, claimData.prover, PROOF_REWARD)
            );
            if (!success) revert CreditTransferFailed();

            // Return the proposer's bond to the entry point contract.
            (success,) = ENTRY_POINT.call{value: address(this).balance}(
                abi.encodeWithSelector(OPSuccinctEntryPoint.addCredit.selector, claimant(), address(this).balance)
            );
            if (!success) revert CreditTransferFailed();

            emit Resolved(status);
        }

        return status;
    }

    /// @notice Getter for the game type.
    /// @dev The reference impl should be entirely different depending on the type (fault, validity)
    ///      i.e. The game type should indicate the security model.
    /// @return gameType_ The type of proof system being used.
    function gameType() public view returns (GameType gameType_) {
        gameType_ = GAME_TYPE;
    }

    /// @notice Getter for the creator of the dispute game.
    /// @dev `clones-with-immutable-args` argument #1
    /// @return creator_ The creator of the dispute game.
    function gameCreator() public pure returns (address creator_) {
        creator_ = _getArgAddress(0x00);
    }

    /// @notice Getter for the root claim.
    /// @dev `clones-with-immutable-args` argument #2
    /// @return rootClaim_ The root claim of the DisputeGame.
    function rootClaim() public pure returns (Claim rootClaim_) {
        rootClaim_ = Claim.wrap(_getArgBytes32(0x14));
    }

    /// @notice Getter for the parent hash of the L1 block when the dispute game was created.
    /// @dev `clones-with-immutable-args` argument #3
    /// @return l1Head_ The parent hash of the L1 block when the dispute game was created.
    function l1Head() public pure returns (Hash l1Head_) {
        l1Head_ = Hash.wrap(_getArgBytes32(0x34));
    }

    /// @notice Getter for the extra data.
    /// @dev `clones-with-immutable-args` argument #4
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    /// FIXME(fakedev9999): Wrong length of extra data considering below facts.
    /// @dev Expected calldata length without proof bytes: 0x92
    //      - 0x04 selector
    //      - 0x14 creator address
    //      - 0x20 root claim
    //      - 0x20 l1 head
    //      - 0x20 extraData (l2BlockNumber)
    //      - 0x04 extraData (parentIndex)
    //      - 0x14 extraData (claimant)
    //      - 0x02 CWIA bytes
    //      - 0x?? proof bytes
    function extraData() public pure returns (bytes memory extraData_) {
        // The extra data starts at the second word within the cwia calldata and
        // is 32 bytes long.
        extraData_ = _getArgBytes(0x54, 0x20);
    }

    /// @notice A compliant implementation of this interface should return the components of the
    ///         game UUID's preimage provided in the cwia payload. The preimage of the UUID is
    ///         constructed as `keccak256(gameType . rootClaim . extraData)` where `.` denotes
    ///         concatenation.
    /// @return gameType_ The type of proof system being used.
    /// @return rootClaim_ The root claim of the DisputeGame.
    /// @return extraData_ Any extra data supplied to the dispute game contract by the creator.
    function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_) {
        gameType_ = gameType();
        rootClaim_ = rootClaim();
        extraData_ = extraData();
    }

    ////////////////////////////////////////////////////////////////
    //                     IMMUTABLE GETTERS                      //
    ////////////////////////////////////////////////////////////////

    /// @notice Returns the max challenge duration.
    function maxChallengeDuration() external view returns (Duration maxChallengeDuration_) {
        maxChallengeDuration_ = MAX_CHALLENGE_DURATION;
    }

    /// @notice Returns the max prove duration.
    function maxProveDuration() external view returns (Duration maxProveDuration_) {
        maxProveDuration_ = MAX_PROVE_DURATION;
    }

    /// @notice Returns the dispute game factory.
    function disputeGameFactory() external view returns (IDisputeGameFactory disputeGameFactory_) {
        disputeGameFactory_ = DISPUTE_GAME_FACTORY;
    }
}
