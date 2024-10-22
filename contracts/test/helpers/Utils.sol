// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {JSONDecoder} from "./JSONDecoder.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {OPSuccinctL2OutputOracle} from "src/OPSuccinctL2OutputOracle.sol";

contract Utils is Test, JSONDecoder {
    function deployWithConfig(Config memory cfg) public returns (address) {
        address OPSuccinctL2OutputOracleImpl = address(new OPSuccinctL2OutputOracle());
        address l2OutputOracleProxy = address(new Proxy(address(this)));

        // Upgrade the proxy to point to the implementation and call initialize().
        // Override the starting output root and timestmp with the passed values.
        upgradeAndInitialize(OPSuccinctL2OutputOracleImpl, cfg, l2OutputOracleProxy, address(0), true);

        // Transfer ownership of proxy to owner specified in the config.
        Proxy(payable(l2OutputOracleProxy)).changeAdmin(cfg.owner);

        return l2OutputOracleProxy;
    }

    // If `executeUpgradeCall` is false, the upgrade call will not be executed.
    function upgradeAndInitialize(
        address impl,
        Config memory cfg,
        address l2OutputOracleProxy,
        address _spoofedAdmin,
        bool executeUpgradeCall
    ) public {
        // require that the verifier gateway is deployed
        require(
            address(cfg.verifierGateway).code.length > 0,
            "OPSuccinctL2OutputOracleUpgrader: verifier gateway not deployed"
        );

        // If we are spoofing the admin (used in testing), start prank.
        if (_spoofedAdmin != address(0)) vm.startPrank(_spoofedAdmin);

        if (executeUpgradeCall) {
            Proxy(payable(l2OutputOracleProxy)).upgradeTo(impl);
        } else {
            // Raw calldata for an upgrade call by a multisig.
            bytes memory multisigCalldata = abi.encodeWithSelector(Proxy.upgradeTo.selector, impl);
            console.log("Upgrade calldata:");
            console.logBytes(multisigCalldata);

            // Raw calldata for an upgrade call with initialization parameters.
            bytes memory initializationParams = abi.encodeWithSelector(
                OPSuccinctL2OutputOracle.upgradeWithInitParams.selector,
                cfg.chainId,
                cfg.aggregationVkey,
                cfg.rangeVkeyCommitment,
                cfg.verifierGateway,
                cfg.rollupConfigHash
            );

            console.log("Update contract parameter calldata:");
            console.logBytes(initializationParams);
        }
    }

    // Read the config from the json file.
    function readJson(string memory filepath) public view returns (Config memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/", filepath);
        string memory json = vm.readFile(path);
        bytes memory data = vm.parseJson(json);
        return abi.decode(data, (Config));
    }

    // This script updates the rollup config hash and the block number in the config.
    function updateRollupConfig() public {
        // If ENV_FILE is set, pass it to the fetch-rollup-config binary.
        string memory envFile = vm.envOr("ENV_FILE", string(".env"));

        // Build the fetch-rollup-config binary. Use the quiet flag to suppress build output.
        string[] memory inputs = new string[](6);
        inputs[0] = "cargo";
        inputs[1] = "build";
        inputs[2] = "--bin";
        inputs[3] = "fetch-rollup-config";
        inputs[4] = "--release";
        inputs[5] = "--quiet";
        vm.ffi(inputs);

        // Run the fetch-rollup-config binary which updates the rollup config hash and the block number in the config.
        // Use the quiet flag to suppress build output.
        string[] memory inputs2 = new string[](9);
        inputs2[0] = "cargo";
        inputs2[1] = "run";
        inputs2[2] = "--bin";
        inputs2[3] = "fetch-rollup-config";
        inputs2[4] = "--release";
        inputs2[5] = "--quiet";
        inputs2[6] = "--";
        inputs2[7] = "--env-file";
        inputs2[8] = envFile;

        vm.ffi(inputs2);
    }
}
