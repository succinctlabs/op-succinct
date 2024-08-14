// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test, console } from "forge-std/Test.sol";
import { ZKUpgrader } from "script/ZKUpgrader.s.sol";

contract ZKUpgraderTest is Test {
    ZKUpgrader u;

    function setUp() public {
        u = new ZKUpgrader();
    }

    function testReadJsonSucceeds() public {
        ZKUpgrader.Config memory config = u.readJson("script/zkconfig.json");
        assertEq(config.l2BlockTime, 2);
        assertEq(config.proposer, address(0));
    }
}
