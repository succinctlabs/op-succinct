// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "forge-std/console.sol";

// Contracts
import {OPSuccinctFaultDisputeGame} from "src/fp/OPSuccinctFaultDisputeGame.sol";

// Libraries
import {Duration, GameStatus} from "src/dispute/lib/Types.sol";
import {BadAuth} from "src/dispute/lib/Errors.sol";

// Interfaces
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";

/// @title OPSuccinctPermissionedFaultDisputeGame
/// @notice OPSuccinctPermissionedFaultDisputeGame is a contract that inherits from `OPSuccinctFaultDisputeGame`, and contains two roles:
///         - The `challenger` role, which is allowed to challenge a game.
///         - The `proposer` role, which is allowed to create proposals and participate in their game.
///         This contract exists as a way for networks to support the fault proof iteration of the OptimismPortal
///         contract without needing to support a fully permissionless system. Permissionless systems can introduce
///         costs that certain networks may not wish to support. This contract can also be used as a fallback mechanism
///         in case of a failure in the permissionless fault proof system in the stage one release.
contract OPSuccinctPermissionedFaultDisputeGame is OPSuccinctFaultDisputeGame {
    /// @notice An address that is allowed to challenge games.
    address internal immutable CHALLENGER;
    /// @notice An address that is allowed to propose games.
    address internal immutable PROPOSER;

    modifier onlyChallenger() {
        if (msg.sender != CHALLENGER) {
            revert BadAuth();
        }
        _;
    }

    modifier onlyProposer() {
        if (msg.sender != PROPOSER) {
            revert BadAuth();
        }
        _;
    }

    /// @param _maxChallengeDuration The maximum duration allowed for a challenger to challenge a game.
    /// @param _maxProveDuration The maximum duration allowed for a proposer to prove against a challenge.
    /// @param _disputeGameFactory The factory that creates the dispute games.
    /// @param _sp1Verifier The address of the SP1 verifier that verifies the proof for the aggregation program.
    /// @param _rollupConfigHash The rollup config hash for the L2 network.
    /// @param _aggregationVkey The vkey for the aggregation program.
    /// @param _rangeVkeyCommitment The commitment to the range vkey.
    /// @param _genesisL2BlockNumber The L2 block number of the genesis block.
    /// @param _genesisL2OutputRoot The L2 output root of the genesis block.
    /// @param _proofReward The proof reward for the game.
    /// @param _challenger An address that is allowed to challenge games.
    /// @param _proposer An address that is allowed to propose games.
    constructor(
        Duration _maxChallengeDuration,
        Duration _maxProveDuration,
        IDisputeGameFactory _disputeGameFactory,
        ISP1Verifier _sp1Verifier,
        bytes32 _rollupConfigHash,
        bytes32 _aggregationVkey,
        bytes32 _rangeVkeyCommitment,
        uint256 _genesisL2BlockNumber,
        bytes32 _genesisL2OutputRoot,
        uint256 _proofReward,
        address _challenger,
        address _proposer
    )
        OPSuccinctFaultDisputeGame(
            _maxChallengeDuration,
            _maxProveDuration,
            _disputeGameFactory,
            _sp1Verifier,
            _rollupConfigHash,
            _aggregationVkey,
            _rangeVkeyCommitment,
            _genesisL2BlockNumber,
            _genesisL2OutputRoot,
            _proofReward
        )
    {
        CHALLENGER = _challenger;
        PROPOSER = _proposer;
    }

    /// @inheritdoc OPSuccinctFaultDisputeGame
    function initialize() public payable override {
        if (tx.origin != PROPOSER) revert BadAuth();
        super.initialize();
    }

    /// @inheritdoc OPSuccinctFaultDisputeGame
    function challenge() public payable override onlyChallenger returns (ProposalStatus) {
        return super.challenge();
    }

    /// @inheritdoc OPSuccinctFaultDisputeGame
    function prove(bytes calldata proofBytes) public override onlyProposer returns (ProposalStatus) {
        return super.prove(proofBytes);
    }

    /// @inheritdoc OPSuccinctFaultDisputeGame
    function resolve() public override onlyProposer onlyChallenger returns (GameStatus) {
        return super.resolve();
    }
}
