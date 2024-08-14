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

    struct Config {
        uint chainId;
        address challenger;
        uint finalizationPeriod;
        uint l2BlockTime;
        address l2OutputOracleProxy;
        string l2RollupNode;
        address owner;
        address proposer;
        uint startingBlockNumber;
        uint submissionInterval;
        address verifierGateway;
        bytes32 vkey;

    }

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

    function run() public {

        /////////////////////////////
        //          INPUTS         //
        /////////////////////////////

        // grab the config from the zkconfig.json file
        Config memory config = readJson("script/zkconfig.json");

        /////////////////////////////
        //      SAFETY CHECKS      //
        /////////////////////////////

        // require that we have permission to upgrade the contract
        require(Proxy(config.l2OutputOracleProxy).admin() == msg.sender, "ZKUpgrader: not admin");

        // requier that the verifier gateway is deployed
        assembly {
            let verifierGateway := mload(config.verifierGateway)
            if iszero(extcodesize(verifierGateway)) {
                revert(0, 0)
            }
        }

        /////////////////////////////
        // OUTPUT ROOT COMPUTATION //
        /////////////////////////////

        string[] memory inputs = new string[](6);
        inputs[0] = "cast";
        inputs[1] = "rpc";
        inputs[2] = "--rpc-url";
        inputs[3] = config.l2RollupNode;
        inputs[4] = "optimism_outputAtBlock";
        // ZTODO: This needs to do the right dropping of 0s
        inputs[5] = Strings.toHexString(startingBlockNumber);

        bytes memory json_res = vm.ffi(inputs);
        // ZTODO: This will actually require parsing the JSON.
        (bytes32 startingOutputRoot, uint startingTimestamp) = abi.decode(res, (bytes32, uint256));

        console.log("Initializing with starting output root:");
        console.logBytes32(outputRoot);

        /////////////////////////////
        //     CONTRACT UPGRADE    //
        /////////////////////////////

        // deploy an implementation of the ZK L2OO to the L1
        ZKL2OutputOracle zkL2OutputOracleImpl = new ZKL2OutputOracle();

        // upgrade the proxy to the new implementation
        Proxy(config.l2OutputOracleProxy).upgradeToAndCall(
            address(zkL2OutputOracleImpl),
            abi.encodeCall(L2OutputOracle.initialize, (
                config.submissionInterval,
                config.l2BlockTime,
                config.startingBlockNumber,
                startingTimestamp,
                startingOutputRoot,
                config.proposer,
                config.challenger,
                config.finalizationPeriod,
                config.chainId,
                config.owner,
                config.vkey,
                config.verifierGateway
            ))
        );
    }


    ////////////////////////////////////////////////////////////////
    //                         Helpers                            //
    ////////////////////////////////////////////////////////////////

    function readJson(string memory filepath) view public returns (Config memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/", filepath);
        string memory json = vm.readFile(path);
        bytes memory data = vm.parseJson(json);
        return abi.decode(data, (Config));
    }
}
