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
        console.log("Deploying OPSuccinctFaultDisputeGame implementation with custom vkeys");

        vm.startBroadcast();

        // Inline all env reads directly into constructor to avoid stack-too-deep error.
        // The EVM stack limit is 16 slots; having 12+ local variables exceeds this.
        OPSuccinctFaultDisputeGame impl = new OPSuccinctFaultDisputeGame(
            Duration.wrap(uint64(vm.envUint("MAX_CHALLENGE_DURATION"))),
            Duration.wrap(uint64(vm.envUint("MAX_PROVE_DURATION"))),
            IDisputeGameFactory(vm.envAddress("FACTORY_PROXY")),
            ISP1Verifier(vm.envAddress("VERIFIER_ADDRESS")),
            vm.envBytes32("ROLLUP_CONFIG_HASH"),
            vm.envBytes32("AGGREGATION_VKEY"),
            vm.envBytes32("RANGE_VKEY_COMMITMENT"),
            vm.envUint("CHALLENGER_BOND_WEI"),
            IAnchorStateRegistry(vm.envAddress("ANCHOR_STATE_REGISTRY")),
            AccessManager(vm.envAddress("ACCESS_MANAGER"))
        );

        vm.stopBroadcast();

        gameImpl = address(impl);
        console.log("gameImpl: address", gameImpl);

        return gameImpl;
    }
}
