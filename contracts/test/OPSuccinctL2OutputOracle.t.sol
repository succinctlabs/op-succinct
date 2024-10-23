// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console} from "forge-std/Test.sol";
import {Utils} from "./helpers/Utils.sol";
import {OPSuccinctL2OutputOracle} from "../src/OPSuccinctL2OutputOracle.sol";

contract OPSuccinctL2OutputOracleTest is Test, Utils {
    // The checkpointed block number for the proof.
    uint256 checkpointedL1BlockNum = 6919874;
    bytes32 claimedOutputRoot = 0x918d98ae75c4aa89ac9098a85dc18e0777bf073d1943d991e1f1f54a1f92450c;
    uint256 claimedL2BlockNum = 3381733;
    bytes proof =
        hex"91ff06f30532505692c10b9a952d3392017a67fc9c5997246150379c74ef11b28087b1912a2e591c0b3d5a80ed7b224a7cecfc0b882dca09ad9bcc4ac26a76dfbecc2e0f1aac3b85587c867bed8eea3402f10d381f5df4ddb54249dd6c29d967a458cb6c2b09d136197641e2894503a52dc095bcb3c783c6171ed0bbc77d05a967e8d15e1f31fe6af7cf3d669f1e62b5c1c66404baa7c7717ac57fdd803e0938bab3369822889c5b8eb4ab50dfbcdc6f99ac0d3b3290876a17d36604cb5c88ec5c8780c10ef7d49135d9b835e8f49308b86576cbf4d3e3b955807d6c592c0c42817099b617bea8309f49d31ba6630f1b73ba10f051942a43d22cd118612b6d3c252f15b6";

    // The owner of the L2OO.
    address OWNER = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;

    OPSuccinctL2OutputOracle l2oo;
    Config config;

    function setUp() public {
        // Note: L1_RPC should be a valid Sepolia RPC.
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);
    }

    // TODO: Once we have a new contract deployed with the new L2OO interface, we should use that here.
    function testOPSuccinctL2OOFork() public {
        console.log(block.number);
        l2oo = OPSuccinctL2OutputOracle(0xd9979DD3cbE74C46fdD8bDB122775Fc7D0DF0BCE);
        l2oo.checkpointBlockHash(checkpointedL1BlockNum);
        vm.prank(OWNER);
        l2oo.proposeL2Output(claimedOutputRoot, claimedL2BlockNum, checkpointedL1BlockNum, proof);
    }
}
