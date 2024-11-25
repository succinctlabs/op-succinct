// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Libraries
import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";
// import { FixedPointMathLib } from "@solady/utils/FixedPointMathLib.sol";
// import { Clone } from "@solady/utils/Clone.sol";
import { Clone } from "./Vendor.sol";
import { Types } from "@optimism/src/libraries/Types.sol";
import { Hashing } from "@optimism/src/libraries/Hashing.sol";
import { RLPReader } from "@optimism/src/libraries/rlp/RLPReader.sol";
import {
    GameStatus,
    GameType,
    Claim,
    Clock,
    Duration,
    Timestamp,
    Hash,
    OutputRoot,
    LibClock,   
    LocalPreimageKey,
    VMStatuses
} from "@optimism/src/dispute/lib/Types.sol";
import { Position, LibPosition } from "@optimism/src/dispute/lib/LibPosition.sol";
import {
    InvalidParent,
    ClaimAlreadyExists,
    ClaimAlreadyResolved,
    OutOfOrderResolution,
    MaxDepthTooLarge,
    AnchorRootNotFound,
    AlreadyInitialized,
    UnexpectedRootClaim,
    GameNotInProgress,
    InvalidPrestate,
    ValidStep,
    GameDepthExceeded,
    L2BlockNumberChallenged,
    InvalidDisputedClaimIndex,
    ClockTimeExceeded,
    DuplicateStep,
    CannotDefendRootClaim,
    IncorrectBondAmount,
    InvalidLocalIdent,
    BlockNumberMatches,
    InvalidHeaderRLP,
    ClockNotExpired,
    BondTransferFailed,
    NoCreditToClaim,
    InvalidOutputRootProof,
    ClaimAboveSplit
} from "@optimism/src/dispute/lib/Errors.sol";

// Interfaces
import { ISemver } from "@optimism/src/universal/ISemver.sol";
import { IDelayedWETH } from "@optimism/src/dispute/interfaces/IDelayedWETH.sol";
import { IBigStepper, IPreimageOracle } from "@optimism/src/dispute/interfaces/IBigStepper.sol";
import { IAnchorStateRegistry } from "@optimism/src/dispute/interfaces/IAnchorStateRegistry.sol";
import {ISP1VerifierGateway} from "@sp1-contracts/src/ISP1VerifierGateway.sol";


error NoClaimProven();

/// @title FaultDisputeGame
/// @notice An implementation of the `IFaultDisputeGame` interface.
contract FaultDisputeGame is Clone, ISemver {
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
        Position position;
        Clock clock;
    }

    /// @notice Parameters to initialize the contract.
    struct InitParams {
        bytes32 aggregationVkey;
        bytes32 rangeVkeyCommitment;
        address verifierGateway;
        bytes32 startingOutputRoot;
        bytes32 rollupConfigHash;
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

    /// @notice Emitted when a new claim is added to the DAG by `claimant`
    /// @param parentIndex The index within the `claimData` array of the parent claim
    /// @param claim The claim being added
    /// @param claimant The address of the claimant
    event Move(uint256 indexed parentIndex, Claim indexed claim, address indexed claimant);

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////

    /// @notice The maximum duration that may accumulate on a team's chess clock before they may no longer respond.
    Duration internal immutable MAX_CLOCK_DURATION;

    /// @notice The game type ID.
    GameType internal immutable GAME_TYPE;

    /// @notice WETH contract for holding ETH.
    IDelayedWETH internal immutable WETH;

    /// @notice The anchor state registry.
    IAnchorStateRegistry internal immutable ANCHOR_STATE_REGISTRY;

    /// @notice The chain ID of the L2 network this contract argues about.
    uint256 internal immutable L2_CHAIN_ID;

    /// @notice The initial parameters related to SP1 verification and the rollup configuration hash. 
    /// @dev These parameters should be set in the constructor and cannot be changed after the contract is deployed.
    InitParams internal INIT_PARAMS;

    /// @notice The index of the block number in the RLP-encoded block header.
    /// @dev Consensus encoding reference:
    /// https://github.com/paradigmxyz/reth/blob/5f82993c23164ce8ccdc7bf3ae5085205383a5c8/crates/primitives/src/header.rs#L368
    uint256 internal constant HEADER_BLOCK_NUMBER_INDEX = 8;

    /// @notice Semantic version.
    /// @custom:semver 1.3.1-beta.7
    string public constant version = "1.3.1-beta.7";

    /// @notice The starting timestamp of the game
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice Returns the current status of the game.
    GameStatus public status;

    /// @notice Flag for the `initialize` function to prevent re-initialization.
    bool internal initialized;

    /// @notice Flag for whether or not the L2 block number claim has been invalidated via `challengeRootL2Block`.
    bool public l2BlockNumberChallenged;

    /// @notice The challenger of the L2 block number claim. Should always be `address(0)` if `l2BlockNumberChallenged`
    ///         is `false`. Should be the address of the challenger if `l2BlockNumberChallenged` is `true`.
    address public l2BlockNumberChallenger;

    /// @notice An append-only array of all claims made during the dispute game.
    /// @dev We note that in single-step ZK fault proofs, this array only ever has one element.
    ClaimData[] public claimData;

    /// @notice Credited balances for winning participants.
    mapping(address => uint256) public credit;

    /// @notice A mapping to allow for constant-time lookups of existing claims.
    mapping(Hash => bool) public claims;

    /// @notice The latest finalized output root, serving as the anchor for output bisection.
    OutputRoot public startingOutputRoot;

    /// @notice The proven claim.
    Claim public provenClaim;

    /// @param _gameType The type ID of the game.
    /// @param _maxClockDuration The maximum amount of time that may accumulate on a team's chess clock.
    /// @param _weth WETH contract for holding ETH.
    /// @param _anchorStateRegistry The contract that stores the anchor state for each game type.
    /// @param _l2ChainId Chain ID of the L2 network this contract argues about.
    constructor(
        GameType _gameType,
        Duration _maxClockDuration,
        IDelayedWETH _weth,
        IAnchorStateRegistry _anchorStateRegistry,
        uint256 _l2ChainId,
        InitParams memory _initParams
    ) {
        // Set up initial game state.
        GAME_TYPE = _gameType;
        MAX_CLOCK_DURATION = _maxClockDuration;
        WETH = _weth;
        ANCHOR_STATE_REGISTRY = _anchorStateRegistry;
        L2_CHAIN_ID = _l2ChainId;
        INIT_PARAMS = _initParams;
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
        startingOutputRoot = OutputRoot({ l2BlockNumber: rootBlockNumber, root: root });

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
        if (l2BlockNumber() <= rootBlockNumber) revert UnexpectedRootClaim(rootClaim());

        // TODO: in the case that `extraData` is set to be a proof, then we can verify the proof and resolve the 
        // game immediately, thereby making `FaultDisputeGame` have the same functionality as `L2OutputOracle`.
        // Otherwise, we just store the new claim as the claimData to be resolved.

        // Set the root claim
        claimData.push(
            ClaimData({
                parentIndex: type(uint32).max,
                counteredBy: address(0),
                claimant: gameCreator(),
                bond: uint128(msg.value),
                claim: rootClaim(),
                position: Position.wrap(uint128(0)),  
                clock: LibClock.wrap(Duration.wrap(0), Timestamp.wrap(uint64(block.timestamp)))
            })
        );

        // Set the game as initialized.
        initialized = true;

        // Deposit the bond.
        WETH.deposit{ value: msg.value }();

        // Set the game's starting timestamp
        createdAt = Timestamp.wrap(uint64(block.timestamp));
    }

    ////////////////////////////////////////////////////////////////
    //                  `IFaultDisputeGame` impl                  //
    ////////////////////////////////////////////////////////////////

    /// @notice Generic move function, used for both `attack` and `defend` moves.
    /// @param _disputed The disputed `Claim`.
    function prove(Claim _disputed, bytes memory _proof) public payable virtual {
        // INVARIANT: Moves cannot be made unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        AggregationOutputs memory publicValues = AggregationOutputs({
            l1Head: l1Head().raw(),
            l2PreRoot: startingRootHash().raw(),
            claimRoot: _disputed.raw(),
            // TODO: I think we have to ensure the originally proposed l2 block number is resaonable so there are no DDOS attacks.
            claimBlockNum: l2BlockNumber(), 
            rollupConfigHash: INIT_PARAMS.rollupConfigHash,
            rangeVkeyCommitment: INIT_PARAMS.rangeVkeyCommitment
        });

        ISP1VerifierGateway(INIT_PARAMS.verifierGateway).verifyProof(INIT_PARAMS.aggregationVkey, abi.encode(publicValues), _proof);

        // INVARIANT: No moves against the root claim can be made after it has been challenged with
        //            `challengeRootL2Block`.`
        if (l2BlockNumberChallenged) revert L2BlockNumberChallenged();  

        // TODO: add a condition that if the time has expired, then the game is resolved automatically.
        // INVARIANT: A move can never be made once its clock has exceeded `MAX_CLOCK_DURATION`
        //            seconds of time.
        // if (nextDuration.raw() == MAX_CLOCK_DURATION.raw()) revert ClockTimeExceeded();

        // Emit the appropriate event for the attack or defense.
        emit Move(l2BlockNumber(), _disputed, msg.sender);  

        provenClaim = _disputed;
    }

    /// @notice The l2BlockNumber of the disputed output root in the `L2OutputOracle`.
    function l2BlockNumber() public pure returns (uint256 l2BlockNumber_) {
        l2BlockNumber_ = _getArgUint256(0x54);
    }

    /// @notice Only the starting block number of the game.
    function startingBlockNumber() public view returns (uint256 startingBlockNumber_) {
        startingBlockNumber_ = startingOutputRoot.l2BlockNumber;
    }

    /// @notice Starting output root and block number of the game.
    function startingRootHash() public view returns (Hash startingRootHash_) {
        startingRootHash_ = startingOutputRoot.root;
    }

    /// @notice Challenges the root L2 block number by providing the preimage of the output root and the L2 block header
    ///         and showing that the committed L2 block number is incorrect relative to the claimed L2 block number.
    /// @param _outputRootProof The output root proof.
    /// @param _headerRLP The RLP-encoded L2 block header.
    function challengeRootL2Block(
        Types.OutputRootProof calldata _outputRootProof,
        bytes calldata _headerRLP
    )
        external
    {
        // INVARIANT: Moves cannot be made unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        // The root L2 block claim can only be challenged once.
        if (l2BlockNumberChallenged) revert L2BlockNumberChallenged();

        // Verify the output root preimage.
        if (Hashing.hashOutputRootProof(_outputRootProof) != rootClaim().raw()) revert InvalidOutputRootProof();

        // Verify the block hash preimage.
        if (keccak256(_headerRLP) != _outputRootProof.latestBlockhash) revert InvalidHeaderRLP();

        // Decode the header RLP to find the number of the block. In the consensus encoding, the timestamp
        // is the 9th element in the list that represents the block header.
        RLPReader.RLPItem[] memory headerContents = RLPReader.readList(RLPReader.toRLPItem(_headerRLP));
        bytes memory rawBlockNumber = RLPReader.readBytes(headerContents[HEADER_BLOCK_NUMBER_INDEX]);

        // Sanity check the block number string length.
        if (rawBlockNumber.length > 32) revert InvalidHeaderRLP();

        // Convert the raw, left-aligned block number to a uint256 by aligning it as a big-endian
        // number in the low-order bytes of a 32-byte word.
        //
        // SAFETY: The length of `rawBlockNumber` is checked above to ensure it is at most 32 bytes.
        uint256 blockNumber;
        assembly {
            blockNumber := shr(shl(0x03, sub(0x20, mload(rawBlockNumber))), mload(add(rawBlockNumber, 0x20)))
        }

        // Ensure the block number does not match the block number claimed in the dispute game.
        if (blockNumber == l2BlockNumber()) revert BlockNumberMatches();

        // Issue a special counter to the root claim. This counter will always win the root claim subgame, and receive
        // the bond from the root claimant.
        l2BlockNumberChallenger = msg.sender;
        l2BlockNumberChallenged = true;
    }

    ////////////////////////////////////////////////////////////////
    //                    `IDisputeGame` impl                     //
    ////////////////////////////////////////////////////////////////

    /// @notice If all necessary information has been gathered, this function should mark the game
    ///         status as either `CHALLENGER_WINS` or `DEFENDER_WINS` and return the status of
    ///         the resolved game. It is at this stage that the bonds should be awarded to the
    ///         necessary parties.
    /// @dev May only be called if the `status` is `IN_PROGRESS`.
    /// @return status_ The status of the game after resolution.
    function resolve() external returns (GameStatus status_) {
        // INVARIANT: Resolution cannot occur unless the game is currently in progress.
        if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();

        if (provenClaim.raw() == bytes32(0)) revert NoClaimProven();

        // TODO: insert condition that if the time has expired, then the game is resolved automatically.

        // Update the global game status; The dispute has concluded.
        status_ = provenClaim.raw() == rootClaim().raw() ? GameStatus.DEFENDER_WINS : GameStatus.CHALLENGER_WINS;
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));

        // Update the status and emit the resolved event, note that we're performing an assignment here.
        emit Resolved(status = status_);

        // Try to update the anchor state, this should not revert.
        ANCHOR_STATE_REGISTRY.tryUpdateAnchorState();
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
    
    // TODO: I think that we can remove this function?
    /// @notice Claim the credit belonging to the recipient address.
    /// @param _recipient The owner and recipient of the credit.
    function claimCredit(address _recipient) external {
        // Remove the credit from the recipient prior to performing the external call.
        uint256 recipientCredit = credit[_recipient];
        credit[_recipient] = 0;

        // Revert if the recipient has no credit to claim.
        if (recipientCredit == 0) revert NoCreditToClaim();

        // Try to withdraw the WETH amount so it can be used here.
        WETH.withdraw(_recipient, recipientCredit);

        // Transfer the credit to the recipient.
        (bool success,) = _recipient.call{ value: recipientCredit }(hex"");
        if (!success) revert BondTransferFailed();
    }

    /// @notice Returns the length of the `claimData` array.
    function claimDataLen() external view returns (uint256 len_) {
        len_ = claimData.length;
    }

    ////////////////////////////////////////////////////////////////
    //                     IMMUTABLE GETTERS                      //
    ////////////////////////////////////////////////////////////////

    /// @notice Returns the max clock duration.
    function maxClockDuration() external view returns (Duration maxClockDuration_) {
        maxClockDuration_ = MAX_CLOCK_DURATION;
    }

    /// @notice Returns the WETH contract for holding ETH.
    function weth() external view returns (IDelayedWETH weth_) {
        weth_ = WETH;
    }

    /// @notice Returns the anchor state registry contract.
    function anchorStateRegistry() external view returns (IAnchorStateRegistry registry_) {
        registry_ = ANCHOR_STATE_REGISTRY;
    }

    /// @notice Returns the chain ID of the L2 network this contract argues about.
    function l2ChainId() external view returns (uint256 l2ChainId_) {
        l2ChainId_ = L2_CHAIN_ID;
    }

    ////////////////////////////////////////////////////////////////
    //                          HELPERS                           //
    ////////////////////////////////////////////////////////////////

    // TODO: I think that we can remove this function?
    /// @notice Pays out the bond of a claim to a given recipient.
    /// @param _recipient The recipient of the bond.
    /// @param _bonded The claim to pay out the bond of.
    function _distributeBond(address _recipient, ClaimData storage _bonded) internal {
        // Set all bits in the bond value to indicate that the bond has been paid out.
        uint256 bond = _bonded.bond;

        // Increase the recipient's credit.
        credit[_recipient] += bond;

        // Unlock the bond.
        WETH.unlock(_recipient, bond);
    }

}