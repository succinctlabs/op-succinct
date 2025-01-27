// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {OPSuccinctL2OutputOracle} from "./OPSuccinctL2OutputOracle.sol";
import {CWIA} from "@solady/utils/legacy/CWIA.sol";
import {LibBytes} from "@solady/utils/LibBytes.sol";
import {ISemver} from "@optimism/src/universal/ISemver.sol";
import {IDisputeGame} from "@optimism/src/dispute/interfaces/IDisputeGame.sol";
import {Claim, GameStatus, GameType, GameTypes, Hash, Timestamp} from "@optimism/src/dispute/lib/Types.sol";
import {GameNotInProgress, OutOfOrderResolution} from "@optimism/src/dispute/lib/Errors.sol";

// Add these at the top of the contract, after the imports
error AlreadyInitialized();
error UnexpectedRootClaim(Claim claim);
error L1BlockHashNotCheckpointed();

// Add this struct definition
struct AggregationOutputs {
    bytes32 l1Head;
    bytes32 l2PreRoot;
    bytes32 claimRoot;
    uint256 claimBlockNum;
    bytes32 rollupConfigHash;
    bytes32 rangeVkeyCommitment;
}

contract OPSuccinctFaultDisputeGame is ISemver, CWIA, IDisputeGame {
    using LibBytes for bytes;

    /// @notice Parameters to initialize the OPSuccinctFaultDisputeGame contract.
    struct InitParams {
        uint256 challengeTime;
        uint256 provingTime;
        bytes32 aggregationVkey;
        bytes32 rangeVkeyCommitment;
        bytes32 rollupConfigHash;
        bytes32 startingOutputRoot;
        uint256 startingBlockNumber;
        // TODO add more init params here, if needed.
    }

    enum ProposalState {
        // By default, the proposal is unchallenged.
        Unchallenged,
        // The proposal has been challenged.
        Challenged,
        // The proposal has been challenged, and a valid proof has been provided.
        ChallengedAndValidProofProvided
    }

    struct ProposalData {
        address proposer;
        address parentGame;
        uint40 deadline;
        uint8 version;
        ProposalState state;
        address challenger;
        uint96 blockNum;
    }

    /// @notice The time period for which the game can be challenged.
    uint256 internal immutable challengeTime;

    /// @notice The time period to allow for proving a challenge.
    uint256 internal immutable provingTime;

    /// @notice The verification key of the aggregation SP1 program.
    bytes32 internal immutable aggregationVkey;

    /// @notice The 32 byte commitment to the BabyBear representation of the verification key of the range SP1 program. Specifically,
    /// this verification is the output of converting the [u32; 8] range BabyBear verification key to a [u8; 32] array.
    bytes32 internal immutable rangeVkeyCommitment;

    /// @notice The hash of the chain's rollup config, which ensures the proofs submitted are for the correct chain.
    bytes32 internal immutable rollupConfigHash;

    /// @notice The starting block number.
    uint256 internal immutable firstBlockNumber;

    /// @notice The starting block number.
    bytes32 internal firstOutputRoot;

    /// @notice The timestamp of the game's global creation.
    Timestamp public createdAt;

    /// @notice The timestamp of the game's global resolution.
    Timestamp public resolvedAt;

    /// @notice Returns the current status of the game.
    GameStatus public status;

    bytes32 public startingOutputRoot;

    ProposalState public proposalState;

    ProposalData public proposal;

    /// @notice A trusted mapping of block numbers to block hashes.
    mapping(uint256 => bytes32) public historicBlockHashes;

    /// @notice Semantic version.
    /// @custom:semver v1.0.0-beta
    string public constant version = "v1.0.0-beta";

    /// @notice Whether the game has been initialized.
    bool public initialized;

    /// @notice The root block number.
    uint256 public rootBlockNumber;

    /// @notice The amount of ETH required to submit a proof.
    uint256 public constant proofReward = 1 ether;

    /// @notice The amount of ETH required to submit a proposal.
    uint256 public constant proposalBond = 1 ether;

    /// @notice The WETH contract.
    IWETH public immutable WETH;

    /// @notice The SP1 verifier contract.
    ISP1Verifier public immutable verifier;

    /// @notice Mapping of addresses to their refund mode credit.
    mapping(address => uint256) public refundModeCredit;

    /// @notice Constructs the OPSuccinctFaultDisputeGame contract.
    constructor(
        InitParams memory _initParams,
        IWETH _weth,
        ISP1Verifier _verifier
    ) {
        challengeTime = _initParams.challengeTime;
        provingTime = _initParams.provingTime;
        aggregationVkey = _initParams.aggregationVkey;
        rangeVkeyCommitment = _initParams.rangeVkeyCommitment;
        rollupConfigHash = _initParams.rollupConfigHash;
        startingOutputRoot = _initParams.startingOutputRoot;
        firstBlockNumber = _initParams.startingBlockNumber;
        WETH = _weth;
        verifier = _verifier;
    }

    ////////////////////////////////////////////////////////////
    //                    IDisputeGame impl                   //
    ////////////////////////////////////////////////////////////

    function initialize() external payable {
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

        // The extra data contains the address of the parent game, and an optional proof.
        // Instead of reading the root block number from the AnchorStateRegistry, we read it from the parent game.
        (address parentGame, bytes memory optionalProof) =
            abi.decode(extraData(), (address, bytes));

        // There is an edge-case, where this is the first game in the chain. In this case, there is no parent game and we use the init params.
        startingOutputRoot = firstOutputRoot;
        if (parentGame != address(0)) {
            startingOutputRoot = IDisputeGame(parentGame).rootClaim();
        }

        // TODO: We have to adapt this to our new extra data format, esp. if it includes a proof.
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

        proposal = ProposalData({
            proposer: gameCreator(),
            parent: parentGame,
            deadline: uint40(block.timestamp + challengeTime),
            version: 0,
            state: ProposalState.Unchallenged,
            challenger: address(0)
        });

        // Set the game as initialized.
        initialized = true;

        // Deposit the bond.
        refundModeCredit[gameCreator()] += msg.value;
        WETH.deposit{ value: msg.value }();

        // Set the game's starting timestamp
        createdAt = Timestamp.wrap(uint64(block.timestamp));

        // Set whether the game type was respected when the game was created.
        // TODO: figure out if we need this.
        // wasRespectedGameTypeWhenCreated =
        //     GameType.unwrap(ANCHOR_STATE_REGISTRY.respectedGameType()) == GameType.unwrap(GAME_TYPE);
    }

    /// @notice Challenge the game.
    function challenge() public payable {
        require(msg.value == proofReward, "No challenge bond provided.");
        require(proposal.state == ProposalState.Unchallenged, "Can only challenge unchallenged proposals.");
        require(proposal.deadline > block.timestamp, "Proposal deadline passed.");

        proposal.deadline = uint40(block.timestamp + provingTime);
        proposal.state = ProposalState.Challenged;
        proposal.challenger = msg.sender;
    }

    /// @notice Prove that the proposal is correct.
    function prove(uint256 _l1BlockNumber, bytes memory proof) public {
        require(proposal.state == ProposalState.Challenged, "Proposal not challenged");
        require(proposal.deadline > block.timestamp, "Proposal deadline passed");
        // TODO: verify proof based on the public values.
        // TODO: note there is edge case here around the l2 block number being passed in.

        // Note: It IS possible to verify valid proofs against invalid parents.
        // Challengers should not challenge proofs of invalid parents, as they will lose their bonds.
        // As long as the parent is rejected, children will be rejected too during resolution.

        bytes32 l1BlockHash = historicBlockHashes[_l1BlockNumber];
        if (l1BlockHash == bytes32(0)) {
            revert L1BlockHashNotCheckpointed();
        }

        AggregationOutputs memory publicValues = AggregationOutputs({
            l1Head: l1BlockHash,
            l2PreRoot: startingOutputRoot.root,
            claimRoot: rootClaim(),
            claimBlockNum: l2BlockNumber(),
            rollupConfigHash: rollupConfigHash,
            rangeVkeyCommitment: rangeVkeyCommitment
        });

        ISP1Verifier(verifier).verifyProof(aggregationVkey, abi.encode(publicValues), proof);

        // If the proof is valid, set the proposal state to confirmed.
        proposal.state = ProposalState.ChallengedAndValidProofProvided;

        // Pay the prover their reward.
        payable(msg.sender).transfer(proofReward);
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

        // TODO: we have to check if the parent game == address(0) and add the logic for that.

        IDisputeGame parent = IDisputeGame(proposal.parent);
        if (parent.status == GameStatus.IN_PROGRESS) {
            parent.resolve();

            // TODO: extra safety check
            // require(isFinalized(parentProposal.state), "parent not finalized");
        }

        // If the challenger won the parent game, then the challenger also wins this game.
        if (parent.status == GameStatus.CHALLENGER_WINS) {
            status = GameStatus.CHALLENGER_WINS;

            // If there was a challenge on this game, then the challenger gets their bond back.
            if (proposal.challenger != address(0)) {
                payable(proposal.challenger).transfer(proposalBond + proofReward);
            } else {
                // If there was no challenge, then the resolver gets the proposer's bond.
                payable(msg.sender).transfer(proposalBond);
            }

            return;
        }

        // At this point, the parent game must have been resolved to DEFENDER_WINS.
        require(parent.status == GameStatus.DEFENDER_WINS, "Parent game not resolved");

        // There are a few cases to consider here:
        // 1. The proposal was challenged, and a valid proof was provided, meaning the proposal is confirmed.
        // Note that for all other cases, we need to ensure that the game deadline has passed.
        // 2. The proposal was challenged, the deadline passed, and no valid proof was provided, meaning the proposal is rejected.
        // 3. The proposal went unchallenged, and the deadline passed, meaning the proposal is confirmed.
        if (proposal.state == ProposalState.Confirmed) {
            // If the proposal was confirmed, then the proposer wins.
            status = GameStatus.DEFENDER_WINS;
            // Transfer the proposer their bond and the proof reward.
            payable(proposal.proposer).transfer(proposalBond + proofReward);
        }

        require(proposal.deadline < block.timestamp, "Proposal deadline not passed");

        if (proposal.state == ProposalState.Challenged) {
            // The proposal.state = Challenged and the deadline passing means that the proposal is rejected.
            status = GameStatus.CHALLENGER_WINS;
            payable(proposal.challenger).transfer(proposalBond + proofReward);
        } else {
            // The only other option is that the proposal was unchallenged and the deadline passed.
            require(proposal.state == ProposalState.Unchallenged, "Proposal not unchallenged");
            status = GameStatus.DEFENDER_WINS;
            // Transfer the proposer their original bond.
            payable(proposal.proposer).transfer(proposalBond);
        }

        // At the end, the game should be resolved.
        resolvedAt = Timestamp.wrap(uint64(block.timestamp));

        emit Resolved(status = status_);
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

    /// @notice Getter for the extra data.
    /// @dev `clones-with-immutable-args` argument #4
    /// @return Any extra data supplied to the dispute game contract by the creator.
    function extraData() public pure returns (bytes memory) {
        // The extra data starts at the second word within the cwia calldata
        return _getArgBytes().slice(0x54);
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
        extraData_ = extraData();
    }

    /// @notice Returns the L2 block number associated with the root claim.
    function l2BlockNumber() public view returns (uint256) {
        return proposal.blockNum;
    }
}

interface IWETH {
    function deposit() external payable;
    function withdraw(uint256) external;
}

interface ISP1Verifier {
    function verifyProof(bytes32 vkey, bytes memory publicValues, bytes memory proof) external view;
}
