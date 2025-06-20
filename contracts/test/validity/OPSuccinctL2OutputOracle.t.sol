// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test} from "forge-std/Test.sol";
import {Utils} from "../helpers/Utils.sol";
import {OPSuccinctL2OutputOracle} from "../../src/validity/OPSuccinctL2OutputOracle.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";
import {console} from "forge-std/console.sol";

contract OPSuccinctL2OutputOracleTest is Test, Utils {
    // Example proof data for a mock proof for Phala Testnet. Tx:
    // https://sepolia.etherscan.io/tx/0x5cc14297049cbabb00c93ac56f27a41a1b94d122af1a2fc6962eca9688cd18c2
    // If you update the l2outputoracle contract, you should also update this test. To speed up this
    // process, set RANGE_PROOF_INTERVAL and SUBMISSION_INTERVAL to small numbers, and set
    // STARTING_BLOCK_NUMBER well before the latest L2 finalized block.
    // 1. Deploy a new L2OO to sepolia, using `just deploy-oracle`.
    // 2. Run a proposer pointing to the new L2OO.
    // 3. Wait until an aggregation proof is submitted to the L2.
    // 4. Examine the logs to find out the checkpointed L1 block number and claimed L2 block number
    //      associated with that aggregation proof. The claimed output root is the block hash of the
    //      claimedL2BlockNumber.
    uint256 checkpointedL1BlockNum = 8592429;
    bytes32 claimedOutputRoot = 0xb5dc455ccb443fa49db9c58d70d008398c604e2295cf3f61d6a97e06117a4bce;
    uint256 claimedL2BlockNum = 2243468;
    bytes proof = hex"";
    address proverAddress = 0x4b713049Fc139df09A20F55f5b76c08184135DF8;

    // The owner of the L2OO.
    address OWNER = 0x4b713049Fc139df09A20F55f5b76c08184135DF8;

    OPSuccinctL2OutputOracle l2oo;

    function setUp() public {
        // Note: L1_RPC should be a valid Sepolia RPC.
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);
    }

    // Test the L2OO contract.
    function testOPSuccinctL2OOFork() public {
        // https://sepolia.etherscan.io/address/0x0bf8068136928AF30ffF09EBC441636f103C5bd6
        l2oo = OPSuccinctL2OutputOracle(0x0bf8068136928AF30ffF09EBC441636f103C5bd6);
        bytes32 genesisConfigName = l2oo.GENESIS_CONFIG_NAME();
        checkpointAndRoll(l2oo, checkpointedL1BlockNum);
        vm.prank(OWNER, OWNER);
        l2oo.proposeL2Output(
            genesisConfigName, claimedOutputRoot, claimedL2BlockNum, checkpointedL1BlockNum, proof, proverAddress
        );
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

    bytes32 genesisConfigName = bytes32(0);

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
        genesisConfigName = l2oo.GENESIS_CONFIG_NAME();
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
        l2oo.proposeL2Output(genesisConfigName, outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

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
        l2oo.proposeL2Output(genesisConfigName, outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);
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
        l2oo.proposeL2Output(genesisConfigName, outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

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
        l2oo.proposeL2Output(genesisConfigName, outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

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
        l2oo.proposeL2Output(genesisConfigName, outputRoot, nextBlockNumber, currentL1Block, proof, proverAddress);

        // lastProposalTimestamp should now return the proposal time
        assertEq(l2oo.lastProposalTimestamp(), proposalTime);
    }

    function testFallbackProposalTimeout_Getter() public view {
        // Test that the getter returns the correct timeout value
        assertEq(l2oo.fallbackTimeout(), FALLBACK_TIMEOUT);
    }
}

contract OPSuccinctConfigManagementTest is Test, Utils {
    OPSuccinctL2OutputOracle l2oo;

    address owner = address(0x1234);
    address nonOwner = address(0x5678);

    bytes32 constant TEST_CONFIG_NAME = keccak256("test_config");
    bytes32 constant NEW_AGGREGATION_VKEY = keccak256("new_aggregation_key");
    bytes32 constant NEW_RANGE_VKEY = keccak256("new_range_key");
    bytes32 constant NEW_ROLLUP_CONFIG = keccak256("new_rollup_config");

    bytes32 genesisConfigName = bytes32(0);

    function setUp() public {
        // Deploy L2OutputOracle using Utils helper function
        address verifier = address(new SP1MockVerifier());
        OPSuccinctL2OutputOracle.InitParams memory initParams =
            createStandardInitParams(verifier, address(0x1111), address(0x2222), owner);

        l2oo = deployL2OutputOracle(initParams);
        genesisConfigName = l2oo.GENESIS_CONFIG_NAME();
    }

    function testUpdateOpSuccinctConfig_NewConfig() public {
        vm.prank(owner);

        l2oo.addOpSuccinctConfig(TEST_CONFIG_NAME, NEW_ROLLUP_CONFIG, NEW_AGGREGATION_VKEY, NEW_RANGE_VKEY);

        // Verify the configuration was stored
        (bytes32 aggVkey, bytes32 rangeVkey, bytes32 rollupConfig) = l2oo.opSuccinctConfigs(TEST_CONFIG_NAME);
        assertEq(aggVkey, NEW_AGGREGATION_VKEY);
        assertEq(rangeVkey, NEW_RANGE_VKEY);
        assertEq(rollupConfig, NEW_ROLLUP_CONFIG);
    }

    function testUpdateOpSuccinctConfig_DuplicateConfigName() public {
        // First create a test configuration
        vm.prank(owner);
        l2oo.addOpSuccinctConfig(TEST_CONFIG_NAME, NEW_ROLLUP_CONFIG, NEW_AGGREGATION_VKEY, NEW_RANGE_VKEY);

        // Try to add another configuration with the same name
        vm.prank(owner);
        vm.expectRevert("L2OutputOracle: config already exists");
        l2oo.addOpSuccinctConfig(
            TEST_CONFIG_NAME,
            keccak256("different_rollup_config"),
            keccak256("different_agg_key"),
            keccak256("different_range_key")
        );
    }

    function testDeleteOpSuccinctConfig_Success() public {
        // First create a test configuration
        vm.prank(owner);
        l2oo.addOpSuccinctConfig(TEST_CONFIG_NAME, NEW_ROLLUP_CONFIG, NEW_AGGREGATION_VKEY, NEW_RANGE_VKEY);

        // Now delete it
        vm.prank(owner);
        l2oo.deleteOpSuccinctConfig(TEST_CONFIG_NAME);

        // Verify it's deleted
        (,, bytes32 rollupConfig) = l2oo.opSuccinctConfigs(TEST_CONFIG_NAME);
        assertEq(rollupConfig, bytes32(0));
    }
}
