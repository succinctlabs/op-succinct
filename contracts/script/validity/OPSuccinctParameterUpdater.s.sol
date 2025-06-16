// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {OPSuccinctL2OutputOracle} from "../../src/validity/OPSuccinctL2OutputOracle.sol";
import {Utils} from "../../test/helpers/Utils.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {console} from "forge-std/console.sol";

// This script is used to update the parameters of the OPSuccinctL2OutputOracle contract.
// If the parameters in the contract don't match the parameters in the config file, the script will update the parameters.
// If executeUpgradeCall is false, the script will only log the calldata for the parameter update calls.
contract OPSuccinctParameterUpdater is Script, Utils {
    function run() public {
        vm.startBroadcast();

        Config memory cfg = readJson("opsuccinctl2ooconfig.json");

        address l2OutputOracleProxy = vm.envAddress("L2OO_ADDRESS");
        bool executeUpgradeCall = vm.envOr("EXECUTE_UPGRADE_CALL", true);

        OPSuccinctL2OutputOracle oracleImpl = OPSuccinctL2OutputOracle(l2OutputOracleProxy);

        if (executeUpgradeCall) {
            oracleImpl.updateOpSuccinctConfig(
                oracleImpl.DEFAULT_CONFIG_NAME(),
                cfg.rollupConfigHash,
                cfg.aggregationVkey,
                cfg.rangeVkeyCommitment,
                cfg.verifier
            );
        } else {
            bytes memory configUpdateCalldata = abi.encodeWithSelector(
                OPSuccinctL2OutputOracle.updateOpSuccinctConfig.selector,
                oracleImpl.DEFAULT_CONFIG_NAME(),
                cfg.rollupConfigHash,
                cfg.aggregationVkey,
                cfg.rangeVkeyCommitment,
                cfg.verifier
            );
            console.log("The calldata for upgrading the OP Succinct configuration is:");
            console.logBytes(configUpdateCalldata);
        }

        if (cfg.submissionInterval != oracleImpl.submissionInterval()) {
            if (executeUpgradeCall) {
                oracleImpl.updateSubmissionInterval(cfg.submissionInterval);
            } else {
                bytes memory submissionIntervalCalldata = abi.encodeWithSelector(
                    OPSuccinctL2OutputOracle.updateSubmissionInterval.selector, cfg.submissionInterval
                );
                console.log("The calldata for upgrading the submissionInterval is:");
                console.logBytes(submissionIntervalCalldata);
            }
        }

        vm.stopBroadcast();
    }
}