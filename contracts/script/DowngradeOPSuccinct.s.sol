// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";
import {Utils} from "../test/helpers/Utils.sol";
import {L2OutputOracle} from "@optimism/src/L1/L2OutputOracle.sol";
import {OPSuccinctL2OutputOracle} from "../src/OPSuccinctL2OutputOracle.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";

contract DowngradeOPSuccinct is Script, Utils {
    function run() external returns (address) {
        vm.startBroadcast();

        Config memory config = readJson("opsuccinctl2ooconfig.json");

        OPSuccinctL2OutputOracle oracleImpl = new OPSuccinctL2OutputOracle();
        Proxy proxy = new Proxy(msg.sender);

        upgradeAndInitialize(address(oracleImpl), config, address(proxy), true);

        SP1MockVerifier verifier = new SP1MockVerifier();

        vm.stopBroadcast();

        return address(verifier);
    }
}
