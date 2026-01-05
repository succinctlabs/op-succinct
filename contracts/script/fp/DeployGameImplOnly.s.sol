// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Duration} from "src/dispute/lib/Types.sol";

import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";

import {AccessManager} from "../../src/fp/AccessManager.sol";
import {OPSuccinctFaultDisputeGame} from "../../src/fp/OPSuccinctFaultDisputeGame.sol";

/// @title DeployGameImplOnly
/// @notice Deploys only the OPSuccinctFaultDisputeGame implementation contract.
/// @dev Used for testing hardfork scenarios where the game implementation needs to be
///      upgraded with different vkeys mid-test. This script deploys the implementation
///      without modifying the factory, allowing the caller to upgrade via setImplementation.
///
/// Required environment variables:
///   - FACTORY_PROXY: Address of the DisputeGameFactory proxy
///   - VERIFIER_ADDRESS: Address of the SP1 verifier
///   - ANCHOR_STATE_REGISTRY: Address of the anchor state registry
///   - ACCESS_MANAGER: Address of the access manager
///   - AGGREGATION_VKEY: Custom aggregation verification key (bytes32)
///   - RANGE_VKEY_COMMITMENT: Custom range vkey commitment (bytes32)
///   - ROLLUP_CONFIG_HASH: Rollup config hash (bytes32)
///   - MAX_CHALLENGE_DURATION: Maximum challenge duration in seconds
///   - MAX_PROVE_DURATION: Maximum prove duration in seconds
///   - CHALLENGER_BOND_WEI: Challenger bond amount in wei
contract DeployGameImplOnly is Script {
    function run() public returns (address gameImpl) {
        // Read required addresses from environment
        address factoryProxy = vm.envAddress("FACTORY_PROXY");
        address verifierAddress = vm.envAddress("VERIFIER_ADDRESS");
        address anchorStateRegistry = vm.envAddress("ANCHOR_STATE_REGISTRY");
        address accessManager = vm.envAddress("ACCESS_MANAGER");

        // Read vkeys from environment (allows custom values for testing)
        bytes32 aggregationVkey = vm.envBytes32("AGGREGATION_VKEY");
        bytes32 rangeVkeyCommitment = vm.envBytes32("RANGE_VKEY_COMMITMENT");
        bytes32 rollupConfigHash = vm.envBytes32("ROLLUP_CONFIG_HASH");

        // Read configuration parameters
        uint64 maxChallengeDuration = uint64(vm.envUint("MAX_CHALLENGE_DURATION"));
        uint64 maxProveDuration = uint64(vm.envUint("MAX_PROVE_DURATION"));
        uint256 challengerBondWei = vm.envUint("CHALLENGER_BOND_WEI");

        console.log("Deploying OPSuccinctFaultDisputeGame implementation with custom vkeys");
        console.log("  Factory:", factoryProxy);
        console.log("  Verifier:", verifierAddress);
        console.log("  Anchor State Registry:", anchorStateRegistry);
        console.log("  Access Manager:", accessManager);
        console.log("  Max Challenge Duration:", maxChallengeDuration);
        console.log("  Max Prove Duration:", maxProveDuration);
        console.log("  Challenger Bond Wei:", challengerBondWei);

        vm.startBroadcast();

        OPSuccinctFaultDisputeGame impl = new OPSuccinctFaultDisputeGame(
            Duration.wrap(maxChallengeDuration),
            Duration.wrap(maxProveDuration),
            IDisputeGameFactory(factoryProxy),
            ISP1Verifier(verifierAddress),
            rollupConfigHash,
            aggregationVkey,
            rangeVkeyCommitment,
            challengerBondWei,
            IAnchorStateRegistry(anchorStateRegistry),
            AccessManager(accessManager)
        );

        vm.stopBroadcast();

        gameImpl = address(impl);

        // Output in parseable format for sysgo
        console.log("gameImpl: address", gameImpl);

        return gameImpl;
    }
}
