// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test, console } from "forge-std/Test.sol";
import { ZKUpgrader } from "script/ZKUpgrader.s.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";
import { Types } from "@optimism/src/libraries/Types.sol";
import { Proxy } from "@optimism/src/universal/Proxy.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract ZKUpgraderTest is Test {
    ZKUpgrader u;

    function setUp() public {
        vm.createSelectFork("https://mainnet.infura.io/v3/fb419f740b7e401bad5bec77d0d285a5", 20528129);
        u = new ZKUpgrader();
    }

    function testReadJsonSucceeds() public {
        ZKUpgrader.Config memory config = u.readJson("script/zkconfig.json");
        assertEq(config.l2BlockTime, 2);
        assertEq(config.proposer, address(0));
    }

    function testFetchOutputRoot() public {
        ZKUpgrader.Config memory config = u.readJson("script/zkconfig.json");
        (bytes32 root, uint ts) = u.fetchOutputRoot(config);
        assertEq(root, 0x6a2fb9128c8bc82eed49ee590fba3e975bd67fede20535d0d20b3000ea6d99b1);
        assertEq(ts, 1691802540);
    }

    function testUpgradeWorks() public {
        ZKL2OutputOracle l2oo = ZKL2OutputOracle(0xdfe97868233d1aa22e815a266982f2cf17685a27);

        u.upgradeToZK(0x543bA4AADBAb8f9025686Bd03993043599c6fB04);

        assertEq(l2oo.owner(), address(0));
        assertEq(address(l2oo.verifierGateway()), 0x3B6041173B80E77f038f3F2C0f9744f04837185e);
        assertEq(l2oo.proposer(), address(0));
    }

    // add test: testFreshDeployWorks (sets outputroot and stuff, and above doesn't)

    function testHexString() public {
        assertEq(u.createHexString(0), "0x0");
        assertEq(u.createHexString(1), "0x1");
        assertEq(u.createHexString(15), "0xf");
        assertEq(u.createHexString(16), "0x10");
        assertEq(u.createHexString(256), "0x100");
    }
}
