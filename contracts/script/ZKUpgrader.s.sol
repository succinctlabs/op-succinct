// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Script } from "forge-std/Script.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";
import { Utils } from "test/helpers/Utils.sol";


contract ZKUpgrader is Script, Utils {
    ////////////////////////////////////////////////////////////////
    //                        Modifiers                           //
    ////////////////////////////////////////////////////////////////

    /// @notice Modifier that wraps a function in broadcasting.
    modifier broadcast() {
        vm.startBroadcast(vm.envUint("ADMIN_PK"));
        _;
        vm.stopBroadcast();
    }

    ////////////////////////////////////////////////////////////////
    //                        Functions                           //
    ////////////////////////////////////////////////////////////////

    function run() public broadcast {
        // grab the config from the zkconfig.json file
        Config memory config = readJson("zkconfig.json");

        // deploy an implementation of the ZK L2OO to the L1
        address zkL2OutputOracleImpl = address(new ZKL2OutputOracle());

        // upgrade the existing proxy contract to the new impl
        upgradeAndInitialize(zkL2OutputOracleImpl, config, address(0), bytes32(0), 0);
    }
}
