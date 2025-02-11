// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// Libraries
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {GameType, Claim} from "src/dispute/lib/Types.sol";
import {BadAuth, BondTransferFailed, NoCreditToClaim} from "src/dispute/lib/Errors.sol";

// Interfaces
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";

// Contracts
import {OPSuccinctFaultDisputeGame} from "./OPSuccinctFaultDisputeGame.sol";

/**
 * @title OPSuccinctEntryPoint
 * @notice An entrypoint contract for `OPSuccinctFaultDisputeGame` that:
 *         1) Stores whitelisted addresses for proposers and challengers.
 *         2) Calls the `disputeGameFactory` to create, challenge, prove,
 *            and resolve `OPSuccinctFaultDisputeGame`s.
 *         3) Holds credits for users when games are resolved.
 */
contract OPSuccinctEntryPoint is OwnableUpgradeable {
    ////////////////////////////////////////////////////////////////
    //                         Events                           //
    ////////////////////////////////////////////////////////////////

    event ProposerWhitelisted(address indexed proposer, bool allowed);
    event ChallengerWhitelisted(address indexed challenger, bool allowed);
    event CreditAdded(address indexed user, uint256 amount);
    event CreatedOPSuccinctFaultDisputeGame(address indexed game, address indexed creator, Claim rootClaim);

    ////////////////////////////////////////////////////////////////
    //                         State Vars                         //
    ////////////////////////////////////////////////////////////////

    /// @notice The DisputeGameFactory that clones `OPSuccinctFaultDisputeGame` when creating new games.
    IDisputeGameFactory public disputeGameFactory;

    /// @notice The gameType used by the factory to pick the correct implementation.
    GameType public gameType;

    /// @notice Tracks whitelisted proposers.
    mapping(address => bool) public proposers;

    /// @notice Tracks whitelisted challengers.
    mapping(address => bool) public challengers;

    /// @notice Minimum bond required to create a new game (if you want bonding).
    uint256 public createBond;

    /// @notice Amount of credit each address has.
    mapping(address => uint256) public credit;

    ////////////////////////////////////////////////////////////////
    //                         Modifiers                          //
    ////////////////////////////////////////////////////////////////

    /// @notice Modifier to check if the caller is a whitelisted proposer.
    /// @dev Whitelisting zero address allows permissionless proposer system.
    modifier onlyProposer() {
        if (!proposers[msg.sender] && !proposers[address(0)]) {
            revert BadAuth();
        }
        _;
    }

    /// @notice Modifier to check if the caller is a whitelisted challenger.
    /// @dev Whitelisting zero address allows permissionless challenger system.
    modifier onlyChallenger() {
        if (!challengers[msg.sender] && !challengers[address(0)]) {
            revert BadAuth();
        }
        _;
    }

    ////////////////////////////////////////////////////////////////
    //                         Implementation                     //
    ////////////////////////////////////////////////////////////////

    /**
     * @notice Initializer for upgradeable pattern.
     * @param _disputeGameFactory The factory contract address.
     * @param _gameType The game type associated with your `OPSuccinctFaultDisputeGame`.
     */
    function initialize(IDisputeGameFactory _disputeGameFactory, GameType _gameType) external initializer {
        __Ownable_init();
        disputeGameFactory = _disputeGameFactory;
        gameType = _gameType;
    }

    /**
     * @notice Allows the owner to whitelist or un-whitelist proposers.
     * @param _proposer The address to set in the proposers mapping.
     * @param _allowed True if whitelisting, false otherwise.
     */
    function setProposer(address _proposer, bool _allowed) external onlyOwner {
        proposers[_proposer] = _allowed;
        emit ProposerWhitelisted(_proposer, _allowed);
    }

    /**
     * @notice Allows the owner to whitelist or un-whitelist challengers.
     * @param _challenger The address to set in the challengers mapping.
     * @param _allowed True if whitelisting, false otherwise.
     */
    function setChallenger(address _challenger, bool _allowed) external onlyOwner {
        challengers[_challenger] = _allowed;
        emit ChallengerWhitelisted(_challenger, _allowed);
    }

    /**
     * @notice Adds credit to a user's account.
     *
     * @param _user The address to add credit to.
     * @param _amount The amount of credit to add.
     *
     * @dev Credit is added when a game is resolved.
     */
    function addCredit(address _user, uint256 _amount) external payable {
        credit[_user] += _amount;
        emit CreditAdded(_user, _amount);
    }

    /**
     * @notice Allows a user to claim all of their credits accumulated.
     */
    function claimCredit(address _recipient) external {
        uint256 creditToClaim = credit[_recipient];
        credit[_recipient] = 0;

        if (creditToClaim == 0) revert NoCreditToClaim();

        (bool success,) = _recipient.call{value: creditToClaim}(hex"");
        if (!success) revert BondTransferFailed();
    }

    /**
     * @notice Creates a new `OPSuccinctFaultDisputeGame` via the `disputeGameFactory`, passing `_rootClaim` and `_extraData`.
     * @param _rootClaim The root claim to initialize the new game with.
     * @param _extraData The extra data to initialize the new game with.
     * @dev Only whitelisted proposers are allowed to call this function.
     * @dev The extra data includes the l2BlockNumber and the parentIndex.
     *      Address of the entry point is appended to the extra data to ensure
     *      the game is created through the entry point.
     */
    function createGame(Claim _rootClaim, bytes calldata _extraData)
        external
        payable
        onlyProposer
        returns (address newGameAddress)
    {
        // Append address(msg.sender) to the extraData
        bytes memory extraDataWithClaimant = abi.encodePacked(_extraData, msg.sender);

        // Call the factory to create the game
        // Append address(msg.sender) to the extraData to record the claimant
        IDisputeGame newGame = disputeGameFactory.create{value: msg.value}(gameType, _rootClaim, extraDataWithClaimant);

        // Emit an event with the new game address and the rootClaim
        emit CreatedOPSuccinctFaultDisputeGame(newGameAddress = address(newGame), msg.sender, _rootClaim);
    }

    /**
     * @notice Challenges an `OPSuccinctFaultDisputeGame`.
     * @dev Only whitelisted challengers can call this function.
     * @dev Exact amount of ETH for proof reward is required when challenging.
     */
    function challengeGame(IDisputeGame _game) external payable onlyChallenger {
        OPSuccinctFaultDisputeGame(address(_game)).challenge{value: msg.value}(msg.sender);
    }

    /**
     * @notice Proves an `OPSuccinctFaultDisputeGame`.
     * @dev Anyone can be a prover.
     */
    function proveGame(IDisputeGame _game, bytes calldata proofBytes) external {
        OPSuccinctFaultDisputeGame(address(_game)).prove(msg.sender, proofBytes);
    }

    /**
     * @notice Resolves an `OPSuccinctFaultDisputeGame`.
     * @dev Anyone can resolve a game.
     */
    function resolveGame(IDisputeGame _game) external {
        OPSuccinctFaultDisputeGame(address(_game)).resolve();
    }
}
