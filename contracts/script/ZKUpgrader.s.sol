// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { VmSafe } from "forge-std/Vm.sol";
import { Script } from "forge-std/Script.sol";

import { console } from "forge-std/console.sol";
import { stdJson } from "forge-std/StdJson.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract ZKUpgrader is Script {
    ////////////////////////////////////////////////////////////////
    //                      Initial State                         //
    ////////////////////////////////////////////////////////////////

    // ZTODO: Pull all these from a zkconfig.json
    uint constant STARTING_BLOCK_NUMBER = 0;
    string constant L2_ROLLUP_NODE = "http://45.250.254.215:5059";
    uint constant SUBMISSION_INTERVAL = 900;
    uint constant L2_BLOCK_TIME = 2;
    address constant PROPOSER = address(0);
    address constant CHALLENGER = address(0);
    uint constant FINALIZATION_PERIOD = 0;
    uint constant CHAIN_ID = 10;
    address constant OWNER = address(0);
    bytes32 constant VKEY = 0x0;
    address constant VERIFIER_GATEWAY = address(0);

    ////////////////////////////////////////////////////////////////
    //                        Modifiers                           //
    ////////////////////////////////////////////////////////////////

    /// @notice Modifier that wraps a function in broadcasting.
    modifier broadcast() {
        // ZTODO: This has to be the admin wallet for upgrading
        vm.startBroadcast(msg.sender);
        _;
        vm.stopBroadcast();
    }

    ////////////////////////////////////////////////////////////////
    //                        Functions                           //
    ////////////////////////////////////////////////////////////////

    function run() public broadcast {
        // // deploy an implementation of the ZK L2OO
        // ZKL2OutputOracle zkL2OutputOracleImpl = new ZKL2OutputOracle();

        // // get the address of the L2OutputOracleProxy
        // address l2OutputOracleProxy = mustGetAddress("L2OutputOracleProxy");

        // // require that we have permission to upgrade the contract
        // require(Proxy(l2OutputOracleProxy).admin() == msg.sender, "ZKUpgrader: not admin");

        // string[] memory inputs = new string[](6);
        // inputs[0] = "cast";
        // inputs[1] = "rpc";
        // inputs[2] = "--rpc-url";
        // inputs[3] = L2_ROLLUP_NODE;
        // inputs[4] = "optimism_outputAtBlock";
        // // ZTODO: This needs to do the right dropping of 0s
        // inputs[5] = Strings.toHexString(STARTING_BLOCK_NUMBER);

        // bytes memory res = vm.ffi(inputs);
        // // ZTODO: This will actually require parsing the JSON.
        // (bytes32 startingOutputRoot, uint startingTimestamp) = abi.decode(res, (bytes32, uint256));

        // console.log("Initializing with starting output root:");
        // console.logBytes32(outputRoot);

        // // ZTODO: Check for codesize at verifier gateway to confirm it's deployed

        // Proxy(l2OutputOracleAddress).upgradeToAndCall(
        //     address(zkL2OutputOracleImpl),
        //     abi.encodeCall(L2OutputOracle.initialize, (
        //         SUBMISSION_INTERVAL,
        //         L2_BLOCK_TIME,
        //         STARTING_BLOCK_NUMBER,
        //         startingTimestamp,
        //         startingOutputRoot,
        //         PROPOSER,
        //         CHALLENGER,
        //         FINALIZATION_PERIOD,
        //         CHAIN_ID,
        //         OWNER,
        //         VKEY,
        //         VERIFIER_GATEWAY
        //     ))
        // );
    }
}
