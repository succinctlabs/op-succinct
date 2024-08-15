// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test, console } from "forge-std/Test.sol";
import { ZKUpgrader } from "script/ZKUpgrader.s.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";
import { Types } from "@optimism/src/libraries/Types.sol";
import { Proxy } from "@optimism/src/universal/Proxy.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { Utils } from "./helpers/Utils.sol";

contract UpgradeTest is Test, Utils {
    function setUp() public {
        vm.createSelectFork("https://eth.llamarpc.com", 20528129);
    }

    function testReadJsonSucceeds() public {
        Config memory config = readJson("zkconfig.json");
        assertEq(config.l2BlockTime, 2);
        assertEq(config.proposer, address(0));
    }

    function testFetchOutputRoot() public {
        Config memory config = readJson("zkconfig.json");
        (bytes32 root, uint ts) = fetchOutputRoot(config);
        assertEq(root, 0x6a2fb9128c8bc82eed49ee590fba3e975bd67fede20535d0d20b3000ea6d99b1);
        assertEq(ts, 1691802540);
    }

    function testUpgradeWorks() public {
        Config memory config = readJson("zkconfig.json");
        config.l2OutputOracleProxy = 0xdfe97868233d1aa22e815a266982f2cf17685a27;

        address optimismProxyAdmin = 0x543bA4AADBAb8f9025686Bd03993043599c6fB04;
        address newImpl = address(new ZKL2OutputOracle());

        upgradeAndInitialize(newImpl, config, optimismProxyAdmin, bytes32(0), 0);

        ZKL2OutputOracle l2oo = ZKL2OutputOracle(config.l2OutputOracleProxy);
        assertEq(l2oo.owner(), address(0));
        assertEq(address(l2oo.verifierGateway()), 0x3B6041173B80E77f038f3F2C0f9744f04837185e);
        assertEq(l2oo.proposer(), address(0));
    }

    function testHexString() public {
        assertEq(createHexString(0), "0x0");
        assertEq(createHexString(1), "0x1");
        assertEq(createHexString(15), "0xf");
        assertEq(createHexString(16), "0x10");
        assertEq(createHexString(256), "0x100");
    }

    // ZTODO: add tests:
    // testFreshDeployWorks (sets outputroot and stuff, and above doesn't)
    // test upgrade works (confirm above test doesn't set that stuff)
}
