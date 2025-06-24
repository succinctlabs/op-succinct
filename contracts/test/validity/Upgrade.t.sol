// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console} from "forge-std/Test.sol";
import {OPSuccinctUpgrader} from "../../script/validity/OPSuccinctUpgrader.s.sol";
import {OPSuccinctL2OutputOracle} from "../../src/validity/OPSuccinctL2OutputOracle.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {Types} from "@optimism/src/libraries/Types.sol";
import {Utils} from "../helpers/Utils.sol";

contract UpgradeTest is Test, Utils {
    function testFreshDeployment() public {
        vm.startBroadcast();

        bytes32 exampleOutputRoot = keccak256("output root");
        vm.warp(12345678);
        uint256 exampleTimestamp = block.timestamp - 1;

        Config memory config = Config({
            challenger: address(0),
            finalizationPeriod: 0,
            l2BlockTime: 10,
            owner: address(0xDEd0000E32f8F40414d3ab3a830f735a3553E18e),
            proposer: address(0xDEd0000E32f8F40414d3ab3a830f735a3553E18e),
            rollupConfigHash: bytes32(0x71241d0f92749d7365aaaf6a015de550816632a4e4e84e273f865f582e8190aa),
            startingBlockNumber: 132003,
            startingOutputRoot: bytes32(0x0cde567c088a52c8ddc32c76d954c6def0cf3418524e9d70bb05e713d9b07586),
            startingTimestamp: 1733438634,
            submissionInterval: 2,
            verifier: address(0x397A5f7f3dBd538f23DE225B51f532c34448dA9B),
            aggregationVkey: bytes32(0x00ea4171dbd0027768055bee7f6d64e17e9cec99b29aad5d18e5d804b967775b),
            rangeVkeyCommitment: bytes32(0x1a4ebe5c47d55436319c425951eb1a7e04f560945e29eb454215d30b30987bbb),
            proxyAdmin: address(0x0000000000000000000000000000000000000000),
            opSuccinctL2OutputOracleImpl: address(0x0000000000000000000000000000000000000000),
            fallbackProposalTimeout: 3600
        });

        // This is never called, so we just need to add some code to the address so the check passes.
        config.verifier = address(new Proxy(address(this)));
        config.startingOutputRoot = exampleOutputRoot;
        config.startingTimestamp = exampleTimestamp;
        OPSuccinctL2OutputOracle l2oo = OPSuccinctL2OutputOracle(deployWithConfig(config));

        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).outputRoot, exampleOutputRoot);
        assertEq(l2oo.getL2Output(l2oo.latestOutputIndex()).timestamp, exampleTimestamp);

        vm.stopBroadcast();
    }

    function testUpgradeExistingContract() public {
        // Fork Sepolia to test with real deployed contract
        vm.createSelectFork(vm.envString("L1_RPC"));

        // This contract was deployed with release tag v2.3.0. 
        // https://github.com/succinctlabs/op-succinct/tree/v2.3.0
        address existingL2OOProxy = 0xD810CbD4bD0BB01EcFD1064Aa4636436B96f8632;

        console.log("Testing Upgrade of Existing Contract");
        console.log("Existing contract address:", existingL2OOProxy);

        // Read current state before upgrade
        OPSuccinctL2OutputOracle existingContract = OPSuccinctL2OutputOracle(existingL2OOProxy);

        // Capture pre-upgrade state
        uint256 preLatestOutputIndex = existingContract.latestOutputIndex();
        uint256 preStartingBlockNumber = existingContract.startingBlockNumber();
        uint256 preStartingTimestamp = existingContract.startingTimestamp();
        uint256 preLatestBlockNumber = existingContract.latestBlockNumber();
        address preChallenger = existingContract.challenger();
        uint256 preSubmissionInterval = existingContract.submissionInterval();
        uint256 preL2BlockTime = existingContract.l2BlockTime();
        uint256 preFinalizationPeriod = existingContract.finalizationPeriodSeconds();
        address preOwner = existingContract.owner();
        Types.OutputProposal memory preFirstOutput = existingContract.getL2Output(0);

        console.log("Pre-upgrade state captured:");

        // Create config similar to testFreshDeployment but preserving existing state
        Config memory config = Config({
            challenger: preChallenger, // Keep existing challenger
            finalizationPeriod: preFinalizationPeriod, // Keep existing finalization period
            l2BlockTime: preL2BlockTime, // Keep existing L2 block time
            owner: preOwner, // Keep existing owner
            proposer: preOwner, // Use owner as proposer
            rollupConfigHash: bytes32(0x1111111111111111111111111111111111111111111111111111111111111111),
            startingBlockNumber: preStartingBlockNumber, // Keep existing starting block
            startingOutputRoot: preFirstOutput.outputRoot, // Keep existing starting output
            startingTimestamp: preStartingTimestamp, // Keep existing starting timestamp
            submissionInterval: preSubmissionInterval, // Keep existing submission interval
            verifier: address(0x1234567890123456789012345678901234567890), // Test verifier address
            aggregationVkey: bytes32(0x2222222222222222222222222222222222222222222222222222222222222222),
            rangeVkeyCommitment: bytes32(0x3333333333333333333333333333333333333333333333333333333333333333),
            proxyAdmin: address(0x0000000000000000000000000000000000000000),
            opSuccinctL2OutputOracleImpl: address(0x0000000000000000000000000000000000000000),
            fallbackProposalTimeout: 3600
        });

        // Deploy mock verifier contract
        vm.etch(
            config.verifier,
            hex"6080604052348015600f57600080fd5b506004361060285760003560e01c8063b8e2f40314602d575b600080fd5b60336035565b005b50565b"
        );

        vm.startBroadcast(0x4b713049Fc139df09A20F55f5b76c08184135DF8);

        // Deploy new implementation
        config.opSuccinctL2OutputOracleImpl = address(new OPSuccinctL2OutputOracle());
        console.log("New implementation deployed at:", config.opSuccinctL2OutputOracleImpl);

        // Execute actual upgrade, and reinitialize the contract.
        console.log("Executing Upgrade");
        Proxy existingProxy = Proxy(payable(existingL2OOProxy));
        upgradeAndInitialize(config, address(existingProxy), true);

        vm.stopBroadcast();

        console.log("Post-Upgrade Verification");

        // Verify state is preserved after upgrade
        assertEq(existingContract.latestOutputIndex(), preLatestOutputIndex, "Latest output index should be preserved");
        assertEq(
            existingContract.startingBlockNumber(), preStartingBlockNumber, "Starting block number should be preserved"
        );
        assertEq(existingContract.startingTimestamp(), preStartingTimestamp, "Starting timestamp should be preserved");
        assertEq(existingContract.latestBlockNumber(), preLatestBlockNumber, "Latest block number should be preserved");
        assertEq(existingContract.challenger(), preChallenger, "Challenger should be preserved");
        assertEq(existingContract.owner(), preOwner, "Owner should be preserved");
    }
}
