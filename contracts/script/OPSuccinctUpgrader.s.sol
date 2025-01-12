// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {OPSuccinctL2OutputOracle} from "../src/OPSuccinctL2OutputOracle.sol";
import {Utils} from "../test/helpers/Utils.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {console} from "forge-std/console.sol";

contract OPSuccinctUpgrader is Script, Utils {
    function run() public {
        Config memory cfg = readJson("opsuccinctl2ooconfig.json");

        address l2OutputOracleProxy = vm.envAddress("L2OO_ADDRESS");
        bool executeUpgradeCall = vm.envOr("EXECUTE_UPGRADE_CALL", true);

        address OPSuccinctL2OutputOracleImpl = vm.envOr("OP_SUCCINCT_L2_OUTPUT_ORACLE_IMPL", address(0));

        address proxyAdmin = vm.envOr("PROXY_ADMIN", address(0));

        uint256 adminPk = vm.envUint("ADMIN_PK");
        // optionally use a different key for deployment
        uint256 deployPk = vm.envOr("DEPLOY_PK", adminPk);

        if (OPSuccinctL2OutputOracleImpl == address(0)) {
            vm.startBroadcast(deployPk);

            console.log("Deploying new logic");
            OPSuccinctL2OutputOracleImpl = address(new OPSuccinctL2OutputOracle());

            vm.stopBroadcast();
        }

        vm.startBroadcast(adminPk);

        upgradeAndInitialize(OPSuccinctL2OutputOracleImpl, cfg, l2OutputOracleProxy, executeUpgradeCall, proxyAdmin);

        vm.stopBroadcast();
    }
}
