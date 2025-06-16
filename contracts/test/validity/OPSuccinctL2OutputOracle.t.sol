// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console} from "forge-std/Test.sol";
import {Utils} from "../helpers/Utils.sol";
import {OPSuccinctL2OutputOracle} from "../../src/validity/OPSuccinctL2OutputOracle.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

contract OPSuccinctL2OutputOracleTest is Test, Utils {
    // Example proof data for a mock proof for Phala Testnet. Tx: https://sepolia.etherscan.io/tx/0x640441cfcba322574a0b153fa3a520bc7bbf1591fdee32f7984dfcf4e18fde4f
    uint256 checkpointedL1BlockNum = 7931837;
    bytes32 claimedOutputRoot = 0xfb2b5dde22744d80ef752a49227a8a4927f998999a66338a22b06f093e9ccd3c;
    uint256 claimedL2BlockNum = 1432001;
    bytes proof = hex"";
    address proverAddress = 0x788c45CafaB3ea427b9079889BE43D7d3cd7500C;

    // The owner of the L2OO.
    address OWNER = 0x788c45CafaB3ea427b9079889BE43D7d3cd7500C;

    OPSuccinctL2OutputOracle l2oo;

    function setUp() public {
        // Note: L1_RPC should be a valid Sepolia RPC.
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);
    }

    // Test the L2OO contract.
    function testOPSuccinctL2OOFork() public {
        l2oo = OPSuccinctL2OutputOracle(0x5f0c7178CF4d7520f347d1334e5fc219da9b8Da4);
        checkpointAndRoll(l2oo, checkpointedL1BlockNum);
        vm.prank(OWNER);
        l2oo.proposeL2Output(claimedOutputRoot, claimedL2BlockNum, checkpointedL1BlockNum, proof, proverAddress);
    }
}

contract OPSuccinctL2OutputOracleFallbackTest is Test, Utils {
    OPSuccinctL2OutputOracle l2oo;

    address approvedProposer = address(0x1234);
    address nonApprovedProposer = address(0x5678);
    address challenger = address(0x9ABC);
    address owner = address(0xDEF0);

    bytes32 aggregationVkey = keccak256("aggregation_vkey");
    bytes32 rangeVkeyCommitment = keccak256("range_vkey");
    bytes32 rollupConfigHash = keccak256("rollup_config");
    bytes32 startingOutputRoot = keccak256("starting_output");

    uint256 constant SUBMISSION_INTERVAL = 10;
    uint256 constant L2_BLOCK_TIME = 2;
    uint256 constant STARTING_BLOCK_NUMBER = 1000;
    uint256 constant FINALIZATION_PERIOD = 7 days;
    uint256 constant FALLBACK_TIMEOUT = 2 days;

    bytes proof = hex"";
    address proverAddress = address(0x7890);
    uint256 startingTimestamp = block.timestamp;

    function setUp() public {
        // Deploy L2OutputOracle using Utils helper function with custom parameters.
        address verifier = address(new SP1MockVerifier());
        OPSuccinctL2OutputOracle.InitParams memory initParams = createInitParamsWithFallback(
            verifier,
            approvedProposer,
            challenger,
            address(this),
            SUBMISSION_INTERVAL,
            L2_BLOCK_TIME,
            STARTING_BLOCK_NUMBER,
            FINALIZATION_PERIOD,
            FALLBACK_TIMEOUT
        );

        l2oo = deployL2OutputOracle(initParams);

        // Set the timestamp to after the starting timestamp
        vm.warp(block.timestamp + 1000);
    }

    function testFallbackProposal_TimeoutElapsed_NonApprovedCanPropose() public {
        // Get the next block number to propose
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");

        // Warp time to exceed the fallback timeout
        uint256 lastProposalTime = l2oo.lastProposalTimestamp();
        vm.warp(lastProposalTime + FALLBACK_TIMEOUT + 1);

        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        checkpointAndRoll(l2oo, currentL1Block);

        // Non-approved proposer should be able to propose after timeout
        vm.prank(nonApprovedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        // Verify the proposal was accepted
        assertEq(l2oo.latestBlockNumber(), nextBlockNumber);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, outputRoot);
    }

    function testFallbackProposal_TimeoutNotElapsed_NonApprovedCannotPropose() public {
        // Get the next block number to propose
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");

        // Don't warp time - fallback timeout has not elapsed
        uint256 lastProposalTime = l2oo.lastProposalTimestamp();
        vm.warp(lastProposalTime + FALLBACK_TIMEOUT - 1); // Just before timeout

        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        checkpointAndRoll(l2oo, currentL1Block);

        // Non-approved proposer should NOT be able to propose before timeout
        vm.prank(nonApprovedProposer);
        vm.expectRevert("L2OutputOracle: only approved proposers can propose new outputs");
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);
    }

    function testFallbackProposal_TimeoutNotElapsed_ApprovedCanStillPropose() public {
        // Get the next block number to propose
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");

        // Don't warp time - fallback timeout has not elapsed
        uint256 lastProposalTime = l2oo.lastProposalTimestamp();
        vm.warp(lastProposalTime + FALLBACK_TIMEOUT - 1); // Just before timeout

        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        checkpointAndRoll(l2oo, currentL1Block);

        // Approved proposer should still be able to propose before timeout
        vm.prank(approvedProposer, approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        // Verify the proposal was accepted
        assertEq(l2oo.latestBlockNumber(), nextBlockNumber);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, outputRoot);
    }

    function testFallbackProposal_TimeoutElapsed_ApprovedCanStillPropose() public {
        // Get the next block number to propose
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");

        // Warp time to exceed the fallback timeout
        uint256 lastProposalTime = l2oo.lastProposalTimestamp();
        vm.warp(lastProposalTime + FALLBACK_TIMEOUT + 1);

        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        checkpointAndRoll(l2oo, currentL1Block);

        // Approved proposer should still be able to propose after timeout
        vm.prank(approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        // Verify the proposal was accepted
        assertEq(l2oo.latestBlockNumber(), nextBlockNumber);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, outputRoot);
    }

    function testLastProposalTimestamp_InitialState() public view {
        // Initially, lastProposalTimestamp should return the starting timestamp
        assertEq(l2oo.lastProposalTimestamp(), startingTimestamp);
    }

    function testLastProposalTimestamp_AfterProposal() public {
        // Make a proposal
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");
        uint256 proposalTime = startingTimestamp + 5000;

        vm.warp(proposalTime);

        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        checkpointAndRoll(l2oo, currentL1Block);

        vm.prank(approvedProposer, approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        // lastProposalTimestamp should now return the proposal time
        assertEq(l2oo.lastProposalTimestamp(), proposalTime);
    }

    function testFallbackProposalTimeout_Getter() public view {
        // Test that the getter returns the correct timeout value
        assertEq(l2oo.fallbackTimeout(), FALLBACK_TIMEOUT);
    }
}

contract OPSuccinctL2OutputOracleStagingTest is Test, Utils {
    OPSuccinctL2OutputOracle l2oo;
    SP1MockVerifier verifier;

    address approvedProposer = address(0x1234);
    address challenger = address(0x9ABC);
    address owner = address(0xDEF0);

    // Real parameters from opsuccinctl2ooconfig.json
    bytes32 aggregationVkey = 0x003991487ea72a40a1caa7c234b12c0da52fc4ccc748a07f6ebd354bbb54772e;
    bytes32 rangeVkeyCommitment = 0x2ebb1e0d5380158f22adf3750cc6056100a133d274fd7c5b457148ff29dfe173;
    bytes32 rollupConfigHash = 0xc9c7547506227136eb0bb56a7d1b2d7d3bec6e760cf574d5b523d5c4b4118a45;
    bytes32 startingOutputRoot = keccak256("starting_output");

    // Staging versions (different values for testing)
    bytes32 stagingAggregationVkey = 0x004991487ea72a40a1caa7c234b12c0da52fc4ccc748a07f6ebd354bbb54772f;
    bytes32 stagingRangeVkeyCommitment = 0x3ebb1e0d5380158f22adf3750cc6056100a133d274fd7c5b457148ff29dfe174;
    bytes32 stagingRollupConfigHash = 0xd9c7547506227136eb0bb56a7d1b2d7d3bec6e760cf574d5b523d5c4b4118a46;

    uint256 constant SUBMISSION_INTERVAL = 10;
    uint256 constant L2_BLOCK_TIME = 2;
    uint256 constant STARTING_BLOCK_NUMBER = 1000;
    uint256 constant FINALIZATION_PERIOD = 7 days;
    uint256 constant FALLBACK_TIMEOUT = 2 days;

    // Real proof data for testing (empty proof works with SP1MockVerifier)
    bytes proof = hex"";
    address proverAddress = address(0x7890);

    function setUp() public {
        verifier = new SP1MockVerifier();
        OPSuccinctL2OutputOracle.InitParams memory initParams = createInitParamsWithFallback(
            address(verifier),
            approvedProposer,
            challenger,
            owner,
            SUBMISSION_INTERVAL,
            L2_BLOCK_TIME,
            STARTING_BLOCK_NUMBER,
            FINALIZATION_PERIOD,
            FALLBACK_TIMEOUT
        );

        l2oo = deployL2OutputOracle(initParams);
        vm.warp(block.timestamp + 1000);
    }

    function testSetStagingAggregationVkey() public {
        vm.prank(owner);
        l2oo.setStagingAggregationVkey(stagingAggregationVkey);

        assertEq(l2oo.stagingAggregationVkey(), stagingAggregationVkey);
    }

    function testSetStagingAggregationVkey_OnlyOwner() public {
        vm.prank(approvedProposer);
        vm.expectRevert("L2OutputOracle: caller is not the owner");
        l2oo.setStagingAggregationVkey(stagingAggregationVkey);
    }

    function testPromoteStagingAggregationVkeyToCurrent() public {
        vm.prank(owner);
        l2oo.setStagingAggregationVkey(stagingAggregationVkey);

        vm.prank(owner);
        l2oo.promoteStagingAggregationVkeyToCurrent();

        assertEq(l2oo.aggregationVkey(), stagingAggregationVkey);
        assertEq(l2oo.stagingAggregationVkey(), bytes32(0));
    }

    function testPromoteStagingAggregationVkeyToCurrent_NoStagingSet() public {
        vm.prank(owner);
        vm.expectRevert("L2OutputOracle: no staging aggregation vkey set");
        l2oo.promoteStagingAggregationVkeyToCurrent();
    }

    function testSetStagingRangeVkeyCommitment() public {
        vm.prank(owner);
        l2oo.setStagingRangeVkeyCommitment(stagingRangeVkeyCommitment);

        assertEq(l2oo.stagingRangeVkeyCommitment(), stagingRangeVkeyCommitment);
    }

    function testPromoteStagingRangeVkeyCommitmentToCurrent() public {
        vm.prank(owner);
        l2oo.setStagingRangeVkeyCommitment(stagingRangeVkeyCommitment);

        vm.prank(owner);
        l2oo.promoteStagingRangeVkeyCommitmentToCurrent();

        assertEq(l2oo.rangeVkeyCommitment(), stagingRangeVkeyCommitment);
        assertEq(l2oo.stagingRangeVkeyCommitment(), bytes32(0));
    }

    function testSetStagingRollupConfigHash() public {
        vm.prank(owner);
        l2oo.setStagingRollupConfigHash(stagingRollupConfigHash);

        assertEq(l2oo.stagingRollupConfigHash(), stagingRollupConfigHash);
    }

    function testPromoteStagingRollupConfigHashToCurrent() public {
        vm.prank(owner);
        l2oo.setStagingRollupConfigHash(stagingRollupConfigHash);

        vm.prank(owner);
        l2oo.promoteStagingRollupConfigHashToCurrent();

        assertEq(l2oo.rollupConfigHash(), stagingRollupConfigHash);
        assertEq(l2oo.stagingRollupConfigHash(), bytes32(0));
    }

    function testProposeL2Output_WithCurrentParams() public {
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");
        uint256 currentL1Block = block.number;

        checkpointAndRoll(l2oo, currentL1Block);

        vm.prank(approvedProposer, approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        assertEq(l2oo.latestBlockNumber(), nextBlockNumber);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, outputRoot);
    }

    function testStagingParams_GettersReturnCorrectValues() public {
        // Initially staging params should be zero
        assertEq(l2oo.stagingAggregationVkey(), bytes32(0));
        assertEq(l2oo.stagingRangeVkeyCommitment(), bytes32(0));
        assertEq(l2oo.stagingRollupConfigHash(), bytes32(0));

        // Set staging params
        vm.prank(owner);
        l2oo.setStagingAggregationVkey(stagingAggregationVkey);
        vm.prank(owner);
        l2oo.setStagingRangeVkeyCommitment(stagingRangeVkeyCommitment);
        vm.prank(owner);
        l2oo.setStagingRollupConfigHash(stagingRollupConfigHash);

        // Verify getters return correct values
        assertEq(l2oo.stagingAggregationVkey(), stagingAggregationVkey);
        assertEq(l2oo.stagingRangeVkeyCommitment(), stagingRangeVkeyCommitment);
        assertEq(l2oo.stagingRollupConfigHash(), stagingRollupConfigHash);
    }

    function testStagingWorkflow_FullCycle() public {
        bytes32 originalAggVkey = l2oo.aggregationVkey();
        bytes32 originalRangeCommitment = l2oo.rangeVkeyCommitment();
        bytes32 originalRollupHash = l2oo.rollupConfigHash();

        // Set all staging params
        vm.startPrank(owner);
        l2oo.setStagingAggregationVkey(stagingAggregationVkey);
        l2oo.setStagingRangeVkeyCommitment(stagingRangeVkeyCommitment);
        l2oo.setStagingRollupConfigHash(stagingRollupConfigHash);

        // Verify staging params are set
        assertEq(l2oo.stagingAggregationVkey(), stagingAggregationVkey);
        assertEq(l2oo.stagingRangeVkeyCommitment(), stagingRangeVkeyCommitment);
        assertEq(l2oo.stagingRollupConfigHash(), stagingRollupConfigHash);

        // Current params should remain unchanged
        assertEq(l2oo.aggregationVkey(), originalAggVkey);
        assertEq(l2oo.rangeVkeyCommitment(), originalRangeCommitment);
        assertEq(l2oo.rollupConfigHash(), originalRollupHash);

        // Promote staging to current
        l2oo.promoteStagingAggregationVkeyToCurrent();
        l2oo.promoteStagingRangeVkeyCommitmentToCurrent();
        l2oo.promoteStagingRollupConfigHashToCurrent();
        vm.stopPrank();

        // Verify current params are updated and staging params are cleared
        assertEq(l2oo.aggregationVkey(), stagingAggregationVkey);
        assertEq(l2oo.rangeVkeyCommitment(), stagingRangeVkeyCommitment);
        assertEq(l2oo.rollupConfigHash(), stagingRollupConfigHash);

        assertEq(l2oo.stagingAggregationVkey(), bytes32(0));
        assertEq(l2oo.stagingRangeVkeyCommitment(), bytes32(0));
        assertEq(l2oo.stagingRollupConfigHash(), bytes32(0));
    }
}
