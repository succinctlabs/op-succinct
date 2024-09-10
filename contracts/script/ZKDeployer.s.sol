// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {ZKL2OutputOracle} from "../src/ZKL2OutputOracle.sol";
import {Utils} from "../test/helpers/Utils.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";

contract ZKDeployer is Script, Utils {
    function run() public returns (address) {
        vm.startBroadcast();

        // Get the latest rollup config hash.
        updateRollupConfigHash();

        Config memory config = readJsonWithRPCFromEnv("zkconfig.json");

        // If starting block number is 0, set it to the latest block number - 10.

        // TODO: This seems wrong. Why are we using the msg.sender as a proxy?
        config.l2OutputOracleProxy = address(new Proxy(msg.sender));

        address zkL2OutputOracleImpl = address(new ZKL2OutputOracle());

        upgradeAndInitialize(zkL2OutputOracleImpl, config, address(0), bytes32(0), 0);

        vm.stopBroadcast();

        return config.l2OutputOracleProxy;
    }
}
