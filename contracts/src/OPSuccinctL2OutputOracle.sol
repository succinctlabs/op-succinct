// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import {ISemver} from "@optimism/src/universal/ISemver.sol";
import {Types} from "@optimism/src/libraries/Types.sol";
import {Constants} from "@optimism/src/libraries/Constants.sol";
import {ISP1VerifierGateway} from "@sp1-contracts/src/ISP1VerifierGateway.sol";

/// @custom:proxied
/// @title OPSuccinctL2OutputOracle
/// @notice The OPSuccinctL2OutputOracle contains an array of L2 state outputs, where each output is a
///         commitment to the state of the L2 chain. Other contracts like the OptimismPortal use
///         these outputs to verify information about the state of L2. The outputs posted to this contract
///         are proved to be valid with `op-succinct`.
contract OPSuccinctL2OutputOracle is Initializable, ISemver {
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

    /// @notice The number of the first L2 block recorded in this contract.
    uint256 public startingBlockNumber;

    /// @notice The timestamp of the first L2 block recorded in this contract.
    uint256 public startingTimestamp;

    /// @notice An array of L2 output proposals.
    Types.OutputProposal[] internal l2Outputs;

    /// @notice The interval in L2 blocks at which checkpoints must be submitted.
    /// @custom:network-specific
    uint256 public submissionInterval;

    /// @notice The time between L2 blocks in seconds. Once set, this value MUST NOT be modified.
    /// @custom:network-specific
    uint256 public l2BlockTime;

    /// @notice The address of the challenger. Can be updated via upgrade.
    /// @custom:network-specific
    address public challenger;

    /// @notice The address of the proposer. Can be updated via upgrade.
    /// @custom:network-specific
    address public proposer;

    /// @notice The minimum time (in seconds) that must elapse before a withdrawal can be finalized.
    /// @custom:network-specific
    uint256 public finalizationPeriodSeconds;

    /// @notice The verification key of the aggregation SP1 program.
    bytes32 public aggregationVkey;

    /// @notice The 32 byte commitment to the BabyBear representation of the verification key of the range SP1 program. Specifically,
    /// this verification is the output of converting the [u32; 8] range BabyBear verification key to a [u8; 32] array.
    bytes32 public rangeVkeyCommitment;

    /// @notice The deployed SP1VerifierGateway contract to request proofs from.
    address public verifierGateway;

    /// @notice The hash of the chain's rollup config, which ensures the proofs submitted are for the correct chain.
    bytes32 public rollupConfigHash;

    /// @notice A trusted mapping of block numbers to block hashes.
    mapping(uint256 => bytes32) public historicBlockHashes;

    ////////////////////////////////////////////////////////////
    //                         Events                         //
    ////////////////////////////////////////////////////////////

    /// @notice Emitted when an output is proposed.
    /// @param outputRoot    The output root.
    /// @param l2OutputIndex The index of the output in the l2Outputs array.
    /// @param l2BlockNumber The L2 block number of the output root.
    /// @param l1Timestamp   The L1 timestamp when proposed.
    event OutputProposed(
        bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp
    );

    /// @notice Emitted when outputs are deleted.
    /// @param prevNextOutputIndex Next L2 output index before the deletion.
    /// @param newNextOutputIndex  Next L2 output index after the deletion.
    event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex);

    ////////////////////////////////////////////////////////////
    //                         Errors                         //
    ////////////////////////////////////////////////////////////

    /// @notice The L1 block hash is not available. If the block hash requested is not in the last 256 blocks,
    ///         it is not available.
    error L1BlockHashNotAvailable();

    /// @notice The L1 block hash is not checkpointed.
    error L1BlockHashNotCheckpointed();

    /// @notice Semantic version.
    /// @custom:semver 0.1.0
    string public constant version = "0.1.0";

    ////////////////////////////////////////////////////////////
    //                        Functions                       //
    ////////////////////////////////////////////////////////////

    /// @notice Constructs the OPSuccinctL2OutputOracle contract. Disables initializers.
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializer.
    /// @param _submissionInterval  Interval in blocks at which checkpoints must be submitted.
    /// @param _l2BlockTime         The time per L2 block, in seconds.
    /// @param _startingBlockNumber The number of the first L2 block.
    /// @param _startingTimestamp   The timestamp of the first L2 block.
    /// @param _proposer            The address of the proposer.
    /// @param _challenger          The address of the challenger.
    /// @param _finalizationPeriodSeconds The minimum time (in seconds) that must elapse before a withdrawal
    ///                                   can be finalized.
    /// @param _initParams          The aggregation vkey, range vkey commitment, verifier gateway, rollup config hash, and starting output root for the contract.
    /// @dev Starting block number, timestamp and output root are ignored for upgrades, because these values already exist.
    function initialize(
        uint256 _submissionInterval,
        uint256 _l2BlockTime,
        uint256 _startingBlockNumber,
        uint256 _startingTimestamp,
        address _proposer,
        address _challenger,
        uint256 _finalizationPeriodSeconds,
        InitParams memory _initParams
    ) public reinitializer(2) {
        require(_submissionInterval > 0, "L2OutputOracle: submission interval must be greater than 0");
        require(_l2BlockTime > 0, "L2OutputOracle: L2 block time must be greater than 0");
        require(
            _startingTimestamp <= block.timestamp,
            "L2OutputOracle: starting L2 timestamp must be less than current time"
        );

        submissionInterval = _submissionInterval;
        l2BlockTime = _l2BlockTime;

        // For proof verification to work, there must be an initial output.
        // Disregard the _startingBlockNumber and _startingTimestamp parameters during upgrades, as they're already set.
        if (l2Outputs.length == 0) {
            l2Outputs.push(
                Types.OutputProposal({
                    outputRoot: _initParams.startingOutputRoot,
                    timestamp: uint128(_startingTimestamp),
                    l2BlockNumber: uint128(_startingBlockNumber)
                })
            );

            startingBlockNumber = _startingBlockNumber;
            startingTimestamp = _startingTimestamp;
        }

        proposer = _proposer;
        challenger = _challenger;
        finalizationPeriodSeconds = _finalizationPeriodSeconds;

        // OP Succinct initialization parameters.
        aggregationVkey = _initParams.aggregationVkey;
        rangeVkeyCommitment = _initParams.rangeVkeyCommitment;
        verifierGateway = _initParams.verifierGateway;
        rollupConfigHash = _initParams.rollupConfigHash;
    }

    /// @notice Getter for the submissionInterval.
    ///         Public getter is legacy and will be removed in the future. Use `submissionInterval` instead.
    /// @return Submission interval.
    /// @custom:legacy
    function SUBMISSION_INTERVAL() external view returns (uint256) {
        return submissionInterval;
    }

    /// @notice Getter for the l2BlockTime.
    ///         Public getter is legacy and will be removed in the future. Use `l2BlockTime` instead.
    /// @return L2 block time.
    /// @custom:legacy
    function L2_BLOCK_TIME() external view returns (uint256) {
        return l2BlockTime;
    }

    /// @notice Getter for the challenger address.
    ///         Public getter is legacy and will be removed in the future. Use `challenger` instead.
    /// @return Address of the challenger.
    /// @custom:legacy
    function CHALLENGER() external view returns (address) {
        return challenger;
    }

    /// @notice Getter for the proposer address.
    ///         Public getter is legacy and will be removed in the future. Use `proposer` instead.
    /// @return Address of the proposer.
    /// @custom:legacy
    function PROPOSER() external view returns (address) {
        return proposer;
    }

    /// @notice Getter for the finalizationPeriodSeconds.
    ///         Public getter is legacy and will be removed in the future. Use `finalizationPeriodSeconds` instead.
    /// @return Finalization period in seconds.
    /// @custom:legacy
    function FINALIZATION_PERIOD_SECONDS() external view returns (uint256) {
        return finalizationPeriodSeconds;
    }

    /// @notice Deletes all output proposals after and including the proposal that corresponds to
    ///         the given output index. Only the challenger address can delete outputs.
    /// @param _l2OutputIndex Index of the first L2 output to be deleted.
    ///                       All outputs after this output will also be deleted.
    function deleteL2Outputs(uint256 _l2OutputIndex) external {
        require(msg.sender == challenger, "L2OutputOracle: only the challenger address can delete outputs");

        // Make sure we're not *increasing* the length of the array.
        require(
            _l2OutputIndex < l2Outputs.length, "L2OutputOracle: cannot delete outputs after the latest output index"
        );

        // Do not allow deleting any outputs that have already been finalized.
        require(
            block.timestamp - l2Outputs[_l2OutputIndex].timestamp < finalizationPeriodSeconds,
            "L2OutputOracle: cannot delete outputs that have already been finalized"
        );

        uint256 prevNextL2OutputIndex = nextOutputIndex();

        // Use assembly to delete the array elements because Solidity doesn't allow it.
        assembly {
            sstore(l2Outputs.slot, _l2OutputIndex)
        }

        emit OutputsDeleted(prevNextL2OutputIndex, _l2OutputIndex);
    }

    /// @notice Accepts an outputRoot and the timestamp of the corresponding L2 block.
    ///         The timestamp must be equal to the current value returned by `nextTimestamp()` in
    ///         order to be accepted. This function may only be called by the Proposer.
    /// @param _outputRoot    The L2 output of the checkpoint block.
    /// @param _l2BlockNumber The L2 block number that resulted in _outputRoot.
    /// @param _l1BlockNumber The block number with the specified block hash.
    /// @dev Modified the function signature to exclude the `_l1BlockHash` parameter, as it's redundant
    /// for OP Succinct given the `_l1BlockNumber` parameter.
    function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof)
        external
        payable
    {
        require(
            msg.sender == proposer || proposer == address(0),
            "L2OutputOracle: only the proposer address can propose new outputs"
        );

        require(
            _l2BlockNumber >= nextBlockNumber(),
            "L2OutputOracle: block number must be greater than or equal to next expected block number"
        );

        require(
            computeL2Timestamp(_l2BlockNumber) < block.timestamp,
            "L2OutputOracle: cannot propose L2 output in the future"
        );

        require(_outputRoot != bytes32(0), "L2OutputOracle: L2 output proposal cannot be the zero hash");

        bytes32 l1BlockHash = historicBlockHashes[_l1BlockNumber];
        if (l1BlockHash == bytes32(0)) {
            revert L1BlockHashNotCheckpointed();
        }

        AggregationOutputs memory publicValues = AggregationOutputs({
            l1Head: l1BlockHash,
            l2PreRoot: l2Outputs[latestOutputIndex()].outputRoot,
            claimRoot: _outputRoot,
            claimBlockNum: _l2BlockNumber,
            rollupConfigHash: rollupConfigHash,
            rangeVkeyCommitment: rangeVkeyCommitment
        });

        ISP1VerifierGateway(verifierGateway).verifyProof(aggregationVkey, abi.encode(publicValues), _proof);

        emit OutputProposed(_outputRoot, nextOutputIndex(), _l2BlockNumber, block.timestamp);

        l2Outputs.push(
            Types.OutputProposal({
                outputRoot: _outputRoot,
                timestamp: uint128(block.timestamp),
                l2BlockNumber: uint128(_l2BlockNumber)
            })
        );
    }

    /// @notice Checkpoints a block hash at a given block number.
    /// @param _blockNumber Block number to checkpoint the hash at.
    /// @dev If the block hash is not available, this will revert.
    function checkpointBlockHash(uint256 _blockNumber) external {
        bytes32 blockHash = blockhash(_blockNumber);
        if (blockHash == bytes32(0)) {
            revert L1BlockHashNotAvailable();
        }
        historicBlockHashes[_blockNumber] = blockHash;
    }

    /// @notice Returns an output by index. Needed to return a struct instead of a tuple.
    /// @param _l2OutputIndex Index of the output to return.
    /// @return The output at the given index.
    function getL2Output(uint256 _l2OutputIndex) external view returns (Types.OutputProposal memory) {
        return l2Outputs[_l2OutputIndex];
    }

    /// @notice Returns the index of the L2 output that checkpoints a given L2 block number.
    ///         Uses a binary search to find the first output greater than or equal to the given
    ///         block.
    /// @param _l2BlockNumber L2 block number to find a checkpoint for.
    /// @return Index of the first checkpoint that commits to the given L2 block number.
    function getL2OutputIndexAfter(uint256 _l2BlockNumber) public view returns (uint256) {
        // Make sure an output for this block number has actually been proposed.
        require(
            _l2BlockNumber <= latestBlockNumber(),
            "L2OutputOracle: cannot get output for a block that has not been proposed"
        );

        // Make sure there's at least one output proposed.
        require(l2Outputs.length > 0, "L2OutputOracle: cannot get output as no outputs have been proposed yet");

        // Find the output via binary search, guaranteed to exist.
        uint256 lo = 0;
        uint256 hi = l2Outputs.length;
        while (lo < hi) {
            uint256 mid = (lo + hi) / 2;
            if (l2Outputs[mid].l2BlockNumber < _l2BlockNumber) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        return lo;
    }

    /// @notice Returns the L2 output proposal that checkpoints a given L2 block number.
    ///         Uses a binary search to find the first output greater than or equal to the given
    ///         block.
    /// @param _l2BlockNumber L2 block number to find a checkpoint for.
    /// @return First checkpoint that commits to the given L2 block number.
    function getL2OutputAfter(uint256 _l2BlockNumber) external view returns (Types.OutputProposal memory) {
        return l2Outputs[getL2OutputIndexAfter(_l2BlockNumber)];
    }

    /// @notice Returns the number of outputs that have been proposed.
    ///         Will revert if no outputs have been proposed yet.
    /// @return The number of outputs that have been proposed.
    function latestOutputIndex() public view returns (uint256) {
        return l2Outputs.length - 1;
    }

    /// @notice Returns the index of the next output to be proposed.
    /// @return The index of the next output to be proposed.
    function nextOutputIndex() public view returns (uint256) {
        return l2Outputs.length;
    }

    /// @notice Returns the block number of the latest submitted L2 output proposal.
    ///         If no proposals been submitted yet then this function will return the starting
    ///         block number.
    /// @return Latest submitted L2 block number.
    function latestBlockNumber() public view returns (uint256) {
        return l2Outputs.length == 0 ? startingBlockNumber : l2Outputs[l2Outputs.length - 1].l2BlockNumber;
    }

    /// @notice Computes the block number of the next L2 block that needs to be checkpointed.
    /// @return Next L2 block number.
    function nextBlockNumber() public view returns (uint256) {
        return latestBlockNumber() + submissionInterval;
    }

    /// @notice Returns the L2 timestamp corresponding to a given L2 block number.
    /// @param _l2BlockNumber The L2 block number of the target block.
    /// @return L2 timestamp of the given block.
    function computeL2Timestamp(uint256 _l2BlockNumber) public view returns (uint256) {
        return startingTimestamp + ((_l2BlockNumber - startingBlockNumber) * l2BlockTime);
    }
}
