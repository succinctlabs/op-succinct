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
    UnexpectedRootClaim
} from "src/dispute/lib/Errors.sol";
import "src/fp/lib/Errors.sol";
import {AggregationOutputs} from "src/lib/Types.sol";

// Interfaces
import {ISemver} from "src/universal/interfaces/ISemver.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
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
        Claim claim;
        Clock clock;
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

    /// @notice The maximum duration allowed for a challenger to challenge a game.
    Duration internal immutable MAX_CHALLENGE_DURATION;

    /// @notice The maximum duration allowed for a proposer to prove against a challenge.
    Duration internal immutable MAX_PROVE_DURATION;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    /// @notice The dispute game factory.
    IDisputeGameFactory internal immutable DISPUTE_GAME_FACTORY;

    /// @notice The chain ID of the L2 network this contract argues about.
    uint256 internal immutable L2_CHAIN_ID;

    /// @notice The SP1 verifier.
    ISP1Verifier internal immutable SP1_VERIFIER;

    /// @notice The rollup config hash.
    bytes32 internal immutable ROLLUP_CONFIG_HASH;

    /// @notice The vkey for the aggregation program.
    bytes32 internal immutable AGGREGATION_VKEY;

    /// @notice The 32 byte commitment to the BabyBear representation of the verification key of the range SP1 program. Specifically,
    /// this verification is the output of converting the [u32; 8] range BabyBear verification key to a [u8; 32] array.
    bytes32 internal immutable RANGE_VKEY_COMMITMENT;

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

    /// @notice Modifier to ensure that the caller is the proposer.
    modifier onlyProposer() {
        if (msg.sender != claimData.claimant) revert NotProposer();
        _;
    }

    /// @param _maxChallengeDuration The maximum duration allowed for a challenger to challenge a game.
    /// @param _maxProveDuration The maximum duration allowed for a proposer to prove against a challenge.
    /// @param _disputeGameFactory The factory that creates the dispute games.
    /// @param _l2ChainId Chain ID of the L2 network this contract argues about.
    /// @param _sp1Verifier The address of the SP1 verifier that verifies the proof for the aggregation program.
    /// @param _rollupConfigHash The rollup config hash for the L2 network.
    /// @param _aggregationVkey The vkey for the aggregation program.
    /// @param _rangeVkeyCommitment The commitment to the range vkey.
    constructor(
        Duration _maxChallengeDuration,
        Duration _maxProveDuration,
        IDisputeGameFactory _disputeGameFactory,
        uint256 _l2ChainId,
        ISP1Verifier _sp1Verifier,
        bytes32 _rollupConfigHash,
        bytes32 _aggregationVkey,
        bytes32 _rangeVkeyCommitment
    ) {
        // Set up initial game state.
        GAME_TYPE = GameType.wrap(42);
        MAX_CHALLENGE_DURATION = _maxChallengeDuration;
        MAX_PROVE_DURATION = _maxProveDuration;
        DISPUTE_GAME_FACTORY = _disputeGameFactory;
        L2_CHAIN_ID = _l2ChainId;
        SP1_VERIFIER = _sp1Verifier;
        ROLLUP_CONFIG_HASH = _rollupConfigHash;
        AGGREGATION_VKEY = _aggregationVkey;
        RANGE_VKEY_COMMITMENT = _rangeVkeyCommitment;
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

        // Revert if the calldata size is not the expected length.
        //
        // This is to prevent adding extra or omitting bytes from to `extraData` that result in a different game UUID
        // in the factory, but are not used by the game, which would allow for multiple dispute games for the same
        // output proposal to be created.
        //
        // Expected length: 0x7E
        // - 0x04 selector
        // - 0x14 creator address
        // - 0x20 root claim
        // - 0x20 l1 head
        // - 0x20 extraData (l2BlockNumber)
        // - 0x04 extraData (parentIndex)
        // - 0x02 CWIA bytes
        assembly {
            if iszero(eq(calldatasize(), 0x7E)) {
                // Store the selector for `BadExtraData()` & revert
                mstore(0x00, 0x9824bdab)
                revert(0x1C, 0x04)
            }
        }

        // TODO(fakedev9999): Come up with a better way to handle the first game
        if (l2BlockNumber() != uint256(1234567890)) {
            // Set the starting output root.
            (GameType gameType, Timestamp timestamp, IDisputeGame proxy) =
                DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());
            startingOutputRoot = OutputRoot({
                l2BlockNumber: OPSuccinctFaultDisputeGame(address(proxy)).l2BlockNumber(),
                root: Hash.wrap(OPSuccinctFaultDisputeGame(address(proxy)).rootClaim().raw())
            });

            // INVARIANT: The parent game must have the same game type as the current game.
            if (gameType.raw() != GAME_TYPE.raw()) revert UnexpectedGameType();

            // INVARIANT: The parent game must be a valid game.
            if (proxy.status() == GameStatus.CHALLENGER_WINS) revert InvalidParentGame();

            // Do not allow the game to be initialized if the root claim corresponds to a block at or before the
            // configured starting block number.
            if (l2BlockNumber() <= startingOutputRoot.l2BlockNumber) {
                revert UnexpectedRootClaim(rootClaim());
            }

            // Set the root claim
            claimData = ClaimData({
                parentIndex: parentIndex(),
                counteredBy: address(0),
                claimant: gameCreator(),
                claim: rootClaim(),
                clock: LibClock.wrap(Duration.wrap(0), Timestamp.wrap(uint64(block.timestamp)))
            });
        } else {
            // Set the root claim
            claimData = ClaimData({
                parentIndex: 0,
                counteredBy: address(0),
                claimant: gameCreator(),
                claim: rootClaim(),
                clock: LibClock.wrap(Duration.wrap(0), Timestamp.wrap(uint64(block.timestamp)))
            });
        }

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

    /// @notice The parent index of the game.
    function parentIndex() public pure returns (uint32 parentIndex_) {
        parentIndex_ = _getArgUint32(0x74);
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
    function challenge() external payable {
        // INVARIANT: Cannot challenge the first game
        (,, IDisputeGame proxy) = DISPUTE_GAME_FACTORY.gameAtIndex(0);
        if (address(proxy) == address(this)) revert FirstGameCannotBeChallenged();

        // INVARIANT: Cannot challenge a game if the game has already been resolved.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        // INVARIANT: Cannot challenge a game if the game has already been challenged.
        if (claimData.counteredBy != address(0)) revert ClaimAlreadyChallenged();

        // INVARIANT: Cannot challenge a game if the clock has already expired.
        if (getChallengeDuration().raw() == MAX_CHALLENGE_DURATION.raw()) {
            revert ClockTimeExceeded();
        }

        // Update the counteredBy address
        claimData.counteredBy = msg.sender;

        // Update the clock to the current block timestamp, which marks the start of the challenge.
        claimData.clock = LibClock.wrap(Duration.wrap(0), Timestamp.wrap(uint64(block.timestamp)));

        // If the required bond is not met, revert.
        // TODO(fakedev9999): Have a separate bond for challenging.
        if (msg.value != DISPUTE_GAME_FACTORY.initBonds(GAME_TYPE)) revert IncorrectBondAmount();

        // Hold the bond in the contract.
        payable(address(this)).transfer(msg.value);
    }

    function prove(bytes calldata publicValues, bytes calldata proofBytes)
        external
        onlyProposer
        returns (GameStatus status_)
    {
        // INVARIANT: Cannot prove a game if the game is not challenged.
        if (claimData.counteredBy == address(0)) revert ClaimNotChallenged();

        // INVARIANT: Cannot prove a game if the game has already been resolved.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        // INVARIANT: Cannot prove a game if the clock has timed out.
        if (getChallengeDuration().raw() == MAX_CHALLENGE_DURATION.raw()) {
            revert ClockTimeExceeded();
        }

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

        status_ = GameStatus.DEFENDER_WINS;
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));

        emit Resolved(status = status_);

        // Distribute the bond to the proposer
        (bool success,) = msg.sender.call{value: address(this).balance}("");
        if (!success) revert BondTransferFailed();
    }

    /// @notice Resolves the game after the clock expires.
    ///         `DEFENDER_WINS` when no one has challenged the proposer's claim.
    ///         `CHALLENGER_WINS` when the proposer's claim has been challenged, but the proposer has not proven
    ///         its claim within the `MAX_PROVE_DURATION`.
    function resolve() external returns (GameStatus status_) {
        // INVARIANT: First game is always resolved as `DEFENDER_WINS`
        (,, IDisputeGame firstGame) = DISPUTE_GAME_FACTORY.gameAtIndex(0);
        if (address(firstGame) == address(this)) {
            status_ = GameStatus.DEFENDER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));
            emit Resolved(status = status_);

            // Distribute the bond back to the proposer
            (bool success,) = claimData.claimant.call{value: address(this).balance}("");
            if (!success) revert BondTransferFailed();

            return status_;
        }

        // INVARIANT: Resolution cannot occur unless the game has already been resolved.
        if (status != GameStatus.IN_PROGRESS) revert ClaimAlreadyResolved();

        // INVARIANT: Cannot resolve a game if the parent game has not been resolved.
        (,, IDisputeGame proxy) = DISPUTE_GAME_FACTORY.gameAtIndex(parentIndex());
        if (proxy.status() == GameStatus.IN_PROGRESS) revert ParentGameNotResolved();

        // INVARIANT: If the parent game is an invalid game, then the current game is invalid.
        if (proxy.status() == GameStatus.CHALLENGER_WINS) {
            status_ = GameStatus.CHALLENGER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));
            emit Resolved(status = status_);

            return status_;
        }

        // The only case left is that the parent game is a valid game (i.e. parent game's status is `DEFENDER_WINS`)
        if (claimData.counteredBy != address(0)) {
            // INVARIANT: Cannot resolve a game unless the clock has expired.
            if (getProveDuration().raw() < MAX_PROVE_DURATION.raw()) {
                revert ClockNotExpired();
            }

            status_ = GameStatus.CHALLENGER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));
            emit Resolved(status = status_);

            // Distribute the bond to the challenger
            (bool success,) = payable(claimData.counteredBy).call{value: address(this).balance}("");
            if (!success) revert BondTransferFailed();

            return status_;
        } else {
            // INVARIANT: Cannot resolve a game unless the clock has expired.
            if (getChallengeDuration().raw() < MAX_CHALLENGE_DURATION.raw()) {
                revert ClockNotExpired();
            }

            status_ = GameStatus.DEFENDER_WINS;
            resolvedAt = Timestamp.wrap(uint64(block.timestamp));
            emit Resolved(status = status_);

            // Distribute the bond back to the proposer
            (bool success,) = claimData.claimant.call{value: address(this).balance}("");
            if (!success) revert BondTransferFailed();
        }
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

    /// @notice Returns the amount of time elapsed on the proposer's chess clock. Maxes out at
    ///         `MAX_PROVE_DURATION`.
    /// @return duration_ The time elapsed on the proposer's chess clock.
    function getProveDuration() public view returns (Duration duration_) {
        // INVARIANT: The game must be in progress to query the remaining time.
        if (status != GameStatus.IN_PROGRESS) {
            revert GameNotInProgress();
        }

        // INVARIANT: The game must have been challenged to query the remaining time.
        if (claimData.counteredBy == address(0)) {
            revert ClaimNotChallenged();
        }

        // Compute the duration elapsed of the proposer's clock.
        uint64 proveDuration = uint64(block.timestamp - claimData.clock.timestamp().raw());
        duration_ = proveDuration > MAX_PROVE_DURATION.raw() ? MAX_PROVE_DURATION : Duration.wrap(proveDuration);
    }

    /// @notice Returns the amount of time elapsed on the potential challenger's chess clock. Maxes
    ///         out at `MAX_CHALLENGE_DURATION`.
    /// @return duration_ The time elapsed on the potential challenger's chess clock.
    function getChallengeDuration() public view returns (Duration duration_) {
        // INVARIANT: The game must be in progress to query the remaining time to respond to a given claim.
        if (status != GameStatus.IN_PROGRESS) {
            revert GameNotInProgress();
        }

        // Compute the duration elapsed of the potential challenger's clock.
        uint64 challengeDuration = uint64(block.timestamp - claimData.clock.timestamp().raw());
        duration_ =
            challengeDuration > MAX_CHALLENGE_DURATION.raw() ? MAX_CHALLENGE_DURATION : Duration.wrap(challengeDuration);
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

    /// @notice Returns the chain ID of the L2 network this contract argues about.
    function l2ChainId() external view returns (uint256 l2ChainId_) {
        l2ChainId_ = L2_CHAIN_ID;
    }

    /// @notice Returns the dispute game factory.
    function disputeGameFactory() external view returns (IDisputeGameFactory disputeGameFactory_) {
        disputeGameFactory_ = DISPUTE_GAME_FACTORY;
    }
}
