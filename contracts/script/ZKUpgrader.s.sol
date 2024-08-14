// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { VmSafe } from "forge-std/Vm.sol";
import { Script } from "forge-std/Script.sol";

import { Test, console } from "forge-std/Test.sol";
import { stdJson } from "forge-std/StdJson.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";
import { OutputRootDecoder } from "./OutputRootDecoder.sol";
import { Proxy } from "@optimism/src/universal/Proxy.sol";
import { SP1VerifierGateway } from "@sp1-contracts/src/SP1VerifierGateway.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract ZKUpgrader is Script, Test {
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

    function run() public broadcast {
        upgradeToZK(address(0));
    }

    function upgradeToZK(address spoofedAdmin) public {

        /////////////////////////////
        //          INPUTS         //
        /////////////////////////////

        // grab the config from the zkconfig.json file
        Config memory config = readJson("script/zkconfig.json");

        /////////////////////////////
        //      SAFETY CHECKS      //
        /////////////////////////////

        // requier that the verifier gateway is deployed
        require(address(config.verifierGateway).code.length > 0, "ZKUpgrader: verifier gateway not deployed");

        /////////////////////////////
        // OUTPUT ROOT COMPUTATION //
        /////////////////////////////

        (bytes32 startingOutputRoot, uint startingTimestamp) = fetchOutputRoot(config);

        /////////////////////////////
        //     CONTRACT UPGRADE    //
        /////////////////////////////

        // deploy an implementation of the ZK L2OO to the L1
        ZKL2OutputOracle zkL2OutputOracleImpl = new ZKL2OutputOracle();

        ZKL2OutputOracle.ZKInitParams memory zkInitParams = ZKL2OutputOracle.ZKInitParams({
            chainId: config.chainId,
            verifierGateway: config.verifierGateway,
            vkey: config.vkey,
            owner: config.owner,
            startingOutputRoot: startingOutputRoot
        });

        if (spoofedAdmin != address(0)) vm.startPrank(spoofedAdmin);

        // upgrade the proxy to the new implementation
        Proxy(payable(config.l2OutputOracleProxy)).upgradeToAndCall(
            address(zkL2OutputOracleImpl),
            abi.encodeCall(ZKL2OutputOracle.initialize, (
                config.submissionInterval,
                config.l2BlockTime,
                config.startingBlockNumber,
                startingTimestamp,
                config.proposer,
                config.challenger,
                config.finalizationPeriod,
                zkInitParams
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

    function fetchOutputRoot(Config memory config) public returns (bytes32 startingOutputRoot, uint startingTimestamp) {
        string memory hexStartingBlockNumber = createHexString(config.startingBlockNumber);

        string[] memory inputs = new string[](6);
        inputs[0] = "cast";
        inputs[1] = "rpc";
        inputs[2] = "--rpc-url";
        inputs[3] = config.l2RollupNode;
        inputs[4] = "optimism_outputAtBlock";
        inputs[5] = hexStartingBlockNumber;

        string memory jsonRes = string(vm.ffi(inputs));
        bytes memory outputRootBytes = vm.parseJson(jsonRes, ".outputRoot");
        bytes memory startingTimestampBytes = vm.parseJson(jsonRes, ".blockRef.timestamp");

        startingOutputRoot = abi.decode(outputRootBytes, (bytes32));
        startingTimestamp = abi.decode(startingTimestampBytes, (uint));
    }

    function createHexString(uint256 value) public returns (string memory) {
        string memory hexStartingBlockNum = Strings.toHexString(value);
        bytes memory startingBlockNumAsBytes = bytes(hexStartingBlockNum);
        require(
            startingBlockNumAsBytes.length >= 4 &&
            startingBlockNumAsBytes[0] == '0' &&
            startingBlockNumAsBytes[1] == 'x',
            "Invalid input"
        );

        if (startingBlockNumAsBytes[2] == '0') {
            bytes memory result = new bytes(startingBlockNumAsBytes.length - 1);
            result[0] = '0';
            result[1] = 'x';
            for (uint i = 3; i < startingBlockNumAsBytes.length; i++) {
                result[i - 1] = startingBlockNumAsBytes[i];
            }
            return string(result);
        }
        return hexStartingBlockNum;
    }
}
