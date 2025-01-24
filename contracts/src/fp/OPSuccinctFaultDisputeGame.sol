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
    UnexpectedRootClaim
} from "src/dispute/lib/Errors.sol";
import "src/fp/lib/Errors.sol";

// Interfaces
import {ISemver} from "src/universal/interfaces/ISemver.sol";
import {IAnchorStateRegistry} from "src/dispute/interfaces/IAnchorStateRegistry.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

/// @title OPSuccinctFaultDisputeGame
/// @notice An implementation of the `IFaultDisputeGame` interface.
contract OPSuccinctFaultDisputeGame is Clone, ISemver {
    ////////////////////////////////////////////////////////////////
    //                         Structs                            //
    ////////////////////////////////////////////////////////////////

    /// @notice The `ClaimData` struct represents the data associated with a Claim.
    struct ClaimData {
        uint32 parentIndex;
        address counteredBy;
        address claimant;
        uint128 bond;
        Claim claim;
        Clock clock;
    }

    /// @notice The public values committed to for an OP Succinct aggregation program.
    struct AggregationOutputs {
        bytes32 l1Head;
        bytes32 l2PreRoot;
        bytes32 claimRoot;
        uint256 claimBlockNum;
        bytes32 rollupConfigHash;
        bytes32 rangeVkeyCommitment;
    }

    ////////////////////////////////////////////////////////////////
    //                         Events                             //
    ////////////////////////////////////////////////////////////////

    /// @notice Emitted when the game is resolved.
    /// @param status The status of the game after resolution.
    event Resolved(GameStatus indexed status);

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////

    /// @notice The absolute prestate of the instruction trace. This is a constant that is defined
    ///         by the program that is being used to execute the trace.
    Claim internal immutable ABSOLUTE_PRESTATE;

    /// @notice The maximum duration that may accumulate on a team's chess clock before they may no longer respond.
    Duration internal immutable MAX_CLOCK_DURATION;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    /// @notice The anchor state registry.
    IAnchorStateRegistry internal immutable ANCHOR_STATE_REGISTRY;

    /// @notice The chain ID of the L2 network this contract argues about.
    uint256 internal immutable L2_CHAIN_ID;

    /// @notice The SP1 verifier.
    ISP1Verifier internal immutable SP1_VERIFIER;

    /// @notice The vkey for the aggregation program.
    bytes32 internal immutable AGGREGATION_VKEY;

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

    /// @notice The latest finalized output root, serving as the anchor for output bisection.
    OutputRoot public startingOutputRoot;

    /// @param _absolutePrestate The absolute prestate of the instruction trace.
    /// @param _maxClockDuration The maximum amount of time that may accumulate on a team's chess clock.
    /// @param _anchorStateRegistry The contract that stores the anchor state for each game type.
    /// @param _l2ChainId Chain ID of the L2 network this contract argues about.
    constructor(
        Claim _absolutePrestate,
        Duration _maxClockDuration,
        IAnchorStateRegistry _anchorStateRegistry,
        uint256 _l2ChainId,
        ISP1Verifier _sp1Verifier,
        bytes32 _aggregationVkey
    ) {
        // Set up initial game state.
        GAME_TYPE = GameType.wrap(42);
        ABSOLUTE_PRESTATE = _absolutePrestate;
        MAX_CLOCK_DURATION = _maxClockDuration;
        ANCHOR_STATE_REGISTRY = _anchorStateRegistry;
        L2_CHAIN_ID = _l2ChainId;
        SP1_VERIFIER = _sp1Verifier;
        AGGREGATION_VKEY = _aggregationVkey;
    }

    /// @notice Initializes the contract.
    /// @dev This function may only be called once.
    function initialize() public payable virtual {
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

        // Grab the latest anchor root.
        (Hash root, uint256 rootBlockNumber) = ANCHOR_STATE_REGISTRY.anchors(GAME_TYPE);

        // Should only happen if this is a new game type that hasn't been set up yet.
        if (root.raw() == bytes32(0)) revert AnchorRootNotFound();

        // Set the starting output root.
        startingOutputRoot = OutputRoot({l2BlockNumber: rootBlockNumber, root: root});

        // Revert if the calldata size is not the expected length.
        //
        // This is to prevent adding extra or omitting bytes from to `extraData` that result in a different game UUID
        // in the factory, but are not used by the game, which would allow for multiple dispute games for the same
        // output proposal to be created.
        //
        // Expected length: 0x7A
        // - 0x04 selector
        // - 0x14 creator address
        // - 0x20 root claim
        // - 0x20 l1 head
        // - 0x20 extraData
        // - 0x02 CWIA bytes
        assembly {
            if iszero(eq(calldatasize(), 0x7A)) {
                // Store the selector for `BadExtraData()` & revert
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }
        }

        // Do not allow the game to be initialized if the root claim corresponds to a block at or before the
        // configured starting block number.
        if (l2BlockNumber() <= rootBlockNumber) {
            revert UnexpectedRootClaim(rootClaim());
        }

        // Set the root claim
        claimData = ClaimData({
            parentIndex: type(uint32).max,
            counteredBy: address(0),
            claimant: gameCreator(),
            bond: uint128(msg.value),
            claim: rootClaim(),
            clock: LibClock.wrap(Duration.wrap(0), Timestamp.wrap(uint64(block.timestamp)))
        });

        // Hold the bond in the contract.
        payable(address(this)).transfer(msg.value);

        // Set the game as initialized.
        initialized = true;

        // Set the game's starting timestamp
        createdAt = Timestamp.wrap(uint64(block.timestamp));
    }

    /// @notice The l2BlockNumber of the disputed output root in the `L2OutputOracle`.
    function l2BlockNumber() public pure returns (uint256 l2BlockNumber_) {
        l2BlockNumber_ = _getArgUint256(0x54);
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

    /// @notice Resolves the game after the clock expires. The proposer wins and the bond is returned back to the proposer.
    function resolve() external returns (GameStatus status_) {
        // INVARIANT: Resolution cannot occur unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        Duration challengeClockDuration = getChallengerDuration();

        // INVARIANT: Cannot resolve a subgame unless the clock of its would-be counter has expired
        // INVARIANT: Assuming ordered subgame resolution, challengeClockDuration is always >= MAX_CLOCK_DURATION if all
        // descendant subgames are resolved
        if (challengeClockDuration.raw() < MAX_CLOCK_DURATION.raw()) {
            revert ClockNotExpired();
        }

        // Update the global game status; The dispute has concluded.
        status_ = GameStatus.DEFENDER_WINS;
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));

        // Update the status and emit the resolved event, note that we're performing an assignment here.
        emit Resolved(status = status_);

        // Try to update the anchor state, this should not revert.
        ANCHOR_STATE_REGISTRY.tryUpdateAnchorState();

        // Distribute the bond back to the proposer
        (bool success,) = claimData.claimant.call{value: claimData.bond}("");
        if (!success) revert BondTransferFailed();
    }

    /// @notice Resolves the game immediately with a proof. The honest challenger wins and the bond is rewarded to the challenger.
    /// @param publicValues The public values committed to for an OP Succinct aggregation program.
    /// @param proofBytes The proof of the program execution the SP1 zkVM encoded as bytes.
    function resolveWithProof(bytes calldata publicValues, bytes calldata proofBytes)
        external
        returns (GameStatus status_)
    {
        // INVARIANT: Resolution cannot occur unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        Duration challengeClockDuration = getChallengerDuration();

        // INVARIANT: Cannot resolve a game with a proof if clock has timed out.
        if (challengeClockDuration.raw() == MAX_CLOCK_DURATION.raw()) {
            revert ClockTimeExceeded();
        }

        // Decode the public values to check the claim root
        AggregationOutputs memory outputs = abi.decode(publicValues, (AggregationOutputs));

        // The proof must have the same l1 head committed as the game's l1 head
        if (outputs.l1Head != Hash.unwrap(l1Head())) {
            revert UnexpectedL1Head(outputs.l1Head);
        }

        // The proof must have the same starting output root committed as the game's starting output root
        if (outputs.l2PreRoot != Hash.unwrap(startingOutputRoot.root)) {
            revert UnexpectedStartingOutputRoot(outputs.l2PreRoot);
        }

        // The proof must have the same claim block number committed as the starting output root's block number
        if (outputs.claimBlockNum != l2BlockNumber()) {
            revert UnexpectedClaimBlockNum(outputs.claimBlockNum);
        }

        // The proof must show a different claim root than what was originally claimed
        if (outputs.claimRoot == Claim.unwrap(rootClaim())) {
            revert UnexpectedRootClaim(rootClaim());
        }

        SP1_VERIFIER.verifyProof(AGGREGATION_VKEY, publicValues, proofBytes);

        claimData.counteredBy = msg.sender;

        status_ = GameStatus.CHALLENGER_WINS;
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));

        emit Resolved(status = status_);

        // Distribute the bond to the challenger
        (bool success,) = msg.sender.call{value: claimData.bond}("");
        if (!success) revert BondTransferFailed();
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
    //                       MISC EXTERNAL                        //
    ////////////////////////////////////////////////////////////////

    /// @notice Returns the amount of time elapsed on the potential challenger to `claimData`'s chess clock. Maxes
    ///         out at `MAX_CLOCK_DURATION`.
    /// @return duration_ The time elapsed on the potential challenger to `_claimIndex`'s chess clock.
    function getChallengerDuration() public view returns (Duration duration_) {
        // INVARIANT: The game must be in progress to query the remaining time to respond to a given claim.
        if (status != GameStatus.IN_PROGRESS) {
            revert GameNotInProgress();
        }

        // Compute the duration elapsed of the potential challenger's clock.
        uint64 challengeDuration = uint64((block.timestamp - claimData.clock.timestamp().raw()));
        duration_ = challengeDuration > MAX_CLOCK_DURATION.raw() ? MAX_CLOCK_DURATION : Duration.wrap(challengeDuration);
    }

    ////////////////////////////////////////////////////////////////
    //                     IMMUTABLE GETTERS                      //
    ////////////////////////////////////////////////////////////////

    /// @notice Returns the absolute prestate of the instruction trace.
    function absolutePrestate() external view returns (Claim absolutePrestate_) {
        absolutePrestate_ = ABSOLUTE_PRESTATE;
    }

    /// @notice Returns the max clock duration.
    function maxClockDuration() external view returns (Duration maxClockDuration_) {
        maxClockDuration_ = MAX_CLOCK_DURATION;
    }

    /// @notice Returns the anchor state registry contract.
    function anchorStateRegistry() external view returns (IAnchorStateRegistry registry_) {
        registry_ = ANCHOR_STATE_REGISTRY;
    }

    /// @notice Returns the chain ID of the L2 network this contract argues about.
    function l2ChainId() external view returns (uint256 l2ChainId_) {
        l2ChainId_ = L2_CHAIN_ID;
    }
}
