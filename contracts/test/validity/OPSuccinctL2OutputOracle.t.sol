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
        l2oo.checkpointBlockHash(checkpointedL1BlockNum);
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
    uint256 constant STARTING_TIMESTAMP = 1000000;
    uint256 constant FINALIZATION_PERIOD = 7 days;
    uint256 constant FALLBACK_TIMEOUT = 2 days;
    
    SP1MockVerifier verifier;
    bytes proof = hex"";
    address proverAddress = address(0x7890);
    
    function setUp() public {
        verifier = new SP1MockVerifier();
        
        Config memory cfg = Config({
            aggregationVkey: aggregationVkey,
            challenger: challenger,
            finalizationPeriod: FINALIZATION_PERIOD,
            l2BlockTime: L2_BLOCK_TIME,
            opSuccinctL2OutputOracleImpl: address(0),
            owner: owner,
            proposer: approvedProposer,
            proxyAdmin: address(0),
            rangeVkeyCommitment: rangeVkeyCommitment,
            rollupConfigHash: rollupConfigHash,
            startingBlockNumber: STARTING_BLOCK_NUMBER,
            startingOutputRoot: startingOutputRoot,
            startingTimestamp: STARTING_TIMESTAMP,
            submissionInterval: SUBMISSION_INTERVAL,
            verifier: address(verifier),
            fallbackProposalTimeout: FALLBACK_TIMEOUT
        });
        
        address l2ooAddress = deployWithConfig(cfg);
        l2oo = OPSuccinctL2OutputOracle(l2ooAddress);
        
        // Set the timestamp to after the starting timestamp
        vm.warp(STARTING_TIMESTAMP + 1000);
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
        vm.roll(currentL1Block + 1);
        l2oo.checkpointBlockHash(currentL1Block);
        
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
        vm.roll(currentL1Block + 1);
        l2oo.checkpointBlockHash(currentL1Block);
        
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
        vm.roll(currentL1Block + 1);
        l2oo.checkpointBlockHash(currentL1Block);
        
        // Approved proposer should still be able to propose before timeout
        vm.prank(approvedProposer);
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
        vm.roll(currentL1Block + 1);
        l2oo.checkpointBlockHash(currentL1Block);
        
        // Approved proposer should still be able to propose after timeout
        vm.prank(approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);
        
        // Verify the proposal was accepted
        assertEq(l2oo.latestBlockNumber(), nextBlockNumber);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, outputRoot);
    }
    
    function testLastProposalTimestamp_InitialState() public {
        // Initially, lastProposalTimestamp should return the starting timestamp
        assertEq(l2oo.lastProposalTimestamp(), STARTING_TIMESTAMP);
    }
    
    function testLastProposalTimestamp_AfterProposal() public {
        // Make a proposal
        uint256 nextBlockNumber = l2oo.nextBlockNumber();
        bytes32 outputRoot = keccak256("test_output");
        uint256 proposalTime = STARTING_TIMESTAMP + 5000;
        
        vm.warp(proposalTime);
        
        // Checkpoint the current block hash
        uint256 currentL1Block = block.number;
        vm.roll(currentL1Block + 1);
        l2oo.checkpointBlockHash(currentL1Block);
        
        vm.prank(approvedProposer);
        l2oo.proposeL2Output(outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);
        
        // lastProposalTimestamp should now return the proposal time
        assertEq(l2oo.lastProposalTimestamp(), proposalTime);
    }
    
    function testFallbackProposalTimeout_Getter() public {
        // Test that the getter returns the correct timeout value
        assertEq(l2oo.FALLBACK_PROPOSAL_TIMEOUT(), FALLBACK_TIMEOUT);
        assertEq(l2oo.fallbackProposalTimeout(), FALLBACK_TIMEOUT);
    }
}
