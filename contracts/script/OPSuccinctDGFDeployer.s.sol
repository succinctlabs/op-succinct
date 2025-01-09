// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {OPSuccinctDisputeGame} from "../src/OPSuccinctDisputeGame.sol";
import {OPSuccinctDisputeGameFactory} from "../src/OPSuccinctDisputeGameFactory.sol";
import {Utils} from "../test/helpers/Utils.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {console} from "forge-std/console.sol";

contract OPSuccinctDFGDeployer is Script, Utils {
    function run() public returns (address) {
        vm.startBroadcast();

        address l2OutputOracleProxy = vm.envAddress("L2OO_ADDRESS");

        // This initializes the dipute game
        OPSuccinctDisputeGame gameImpl = new OPSuccinctDisputeGame(l2OutputOracleProxy);
        Proxy gameProxy = new Proxy(msg.sender);

        gameProxy.upgradeTo(address(gameImpl));

        // This initializes the dispute game factory
        OPSuccinctDisputeGameFactory factoryImpl = new OPSuccinctDisputeGameFactory(address(gameProxy));
        Proxy factoryProxy = new Proxy(msg.sender);

        factoryProxy.upgradeTo(address(factoryImpl));

        vm.stopBroadcast();

        return address(factoryProxy);
    }
}
