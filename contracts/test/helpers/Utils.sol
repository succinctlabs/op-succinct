// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Test, console } from "forge-std/Test.sol";
import { JSONDecoder } from "./JSONDecoder.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { Proxy } from "@optimism/src/universal/Proxy.sol";
import { ZKL2OutputOracle } from "src/ZKL2OutputOracle.sol";

contract Utils is Test, JSONDecoder {
    function deployWithConfig(Config memory cfg, bytes32 startingOutputRoot, uint startingTimestamp) public returns (address) {
        // deploy impl and proxy
        address zkL2OutputOracleImpl = address(new ZKL2OutputOracle());
        cfg.l2OutputOracleProxy = address(new Proxy(address(this)));

        // upgrade the proxy to point to the impl and call initialize with values from config
        upgradeAndInitialize(zkL2OutputOracleImpl, cfg, address(0), startingOutputRoot, startingTimestamp);

        // transfer ownership of proxy to owner
        Proxy(payable(cfg.l2OutputOracleProxy)).changeAdmin(cfg.owner);

        return cfg.l2OutputOracleProxy;
    }

    function upgradeAndInitialize(address impl, Config memory cfg, address _spoofedAdmin, bytes32 startingOutputRoot, uint startingTimestamp) public {
        // requier that the verifier gateway is deployed
        require(address(cfg.verifierGateway).code.length > 0, "ZKUpgrader: verifier gateway not deployed");

        // use starting output block number to compute starting timestamp and output root if not passed
        if (startingOutputRoot == bytes32(0) || startingTimestamp == 0) {
            (bytes32 returnedStartingOutputRoot, uint returnedStartingTimestamp) = fetchOutputRoot(cfg);
            if (startingOutputRoot == bytes32(0)) startingOutputRoot = returnedStartingOutputRoot;
            if (startingTimestamp == 0) startingTimestamp = returnedStartingTimestamp;
        }

        // pack the new init params into a struct
        ZKL2OutputOracle.ZKInitParams memory zkInitParams = ZKL2OutputOracle.ZKInitParams({
            chainId: cfg.chainId,
            verifierGateway: cfg.verifierGateway,
            vkey: cfg.vkey,
            owner: cfg.owner,
            startingOutputRoot: startingOutputRoot
        });

        // if spoofing admin, start prank
        if (_spoofedAdmin != address(0)) vm.startPrank(_spoofedAdmin);

        // upgrade the proxy to the newly deployed implementation
        Proxy(payable(cfg.l2OutputOracleProxy)).upgradeToAndCall(
            impl,
            abi.encodeCall(ZKL2OutputOracle.initialize, (
                cfg.submissionInterval,
                cfg.l2BlockTime,
                cfg.startingBlockNumber,
                startingTimestamp,
                cfg.proposer,
                cfg.challenger,
                cfg.finalizationPeriod,
                zkInitParams
            ))
        );
    }

    function readJson(string memory filepath) view public returns (Config memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/", filepath);
        string memory json = vm.readFile(path);
        bytes memory data = vm.parseJson(json);
        return abi.decode(data, (Config));
    }

    function readJsonWithRPCFromEnv(string memory filepath) view public returns (Config memory) {
        Config memory config = readJson(filepath);
        config.l2RollupNode = vm.envString("L2_ROLLUP_NODE");
        return config;
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

    function createHexString(uint256 value) public pure returns (string memory) {
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
