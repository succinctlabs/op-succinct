// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {console2} from "forge-std/console2.sol";
import {GameType} from "src/dispute/lib/Types.sol";

// Interfaces
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {IOptimismPortal2} from "interfaces/L1/IOptimismPortal2.sol";
import {IMulticall3} from "forge-std/interfaces/IMulticall3.sol";

// Contracts
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {GnosisSafe} from "@safe-contracts/GnosisSafe.sol";

// Utils
import {Utils} from "../../test/helpers/Utils.sol";
import {Enum} from "@safe-contracts/common/Enum.sol";

/// @notice Configures deployment of OpSuccinct FDG using Safe
contract ConfigureDeploymentSafe is Script, Utils {
    error MissingSignatures();

    struct EnvConfig {
        uint32 gameType;
        uint256 initialBondWei;
        address factoryProxy;
        address gameImplementation;
        address optimismPortal2;
        address safe;
        address sender;
        bytes signatures;
    }

    function readEnv() internal view returns (EnvConfig memory) {
        return EnvConfig(
            uint32(vm.envOr("GAME_TYPE", uint256(42))),
            vm.envUint("INITIAL_BOND"),
            vm.envAddress("FACTORY"),
            vm.envAddress("GAME_IMPL"),
            vm.envOr("PORTAL", address(0)), // skip portal configuration if env not present
            vm.envAddress("SAFE"),
            vm.envAddress("SENDER"),
            vm.envOr("SIG", bytes(hex"00"))
        );
    }

    function buildSafeTx(EnvConfig memory config) internal pure returns (bytes memory) {
        GameType gameType = GameType.wrap(config.gameType);

        // Determine number of operations
        uint8 opNumber;
        if (config.optimismPortal2 == address(0)) {
            opNumber = 2;
        } else {
            opNumber = 3;
        }

        // Build IMulticall3 calldata
        IMulticall3.Call3[] memory calls = new IMulticall3.Call3[](opNumber);
        calls[0] = IMulticall3.Call3(
            config.factoryProxy,
            false,
            abi.encodeWithSelector(DisputeGameFactory.setInitBond.selector, gameType, config.initialBondWei)
        );
        calls[1] = IMulticall3.Call3(
            config.factoryProxy,
            false,
            abi.encodeWithSelector(
                DisputeGameFactory.setImplementation.selector, gameType, IDisputeGame(config.gameImplementation)
            )
        );
        if (config.optimismPortal2 != address(0)) {
            calls[2] = IMulticall3.Call3(
                config.optimismPortal2,
                false,
                abi.encodeWithSelector(IOptimismPortal2.setRespectedGameType.selector, gameType)
            );
        }

        return abi.encodeWithSelector(IMulticall3.aggregate3.selector, calls);
    }

    function getTransactionHash() public view returns (bytes32) {
        EnvConfig memory config = readEnv();

        // Build tx
        bytes memory calls = buildSafeTx(config);

        // Build tx hash
        GnosisSafe safe = GnosisSafe(payable(config.safe));
        bytes32 txHash = safe.getTransactionHash(
            MULTICALL3_ADDRESS,
            0, // value
            calls,
            Enum.Operation(1), // delegate call
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            config.sender, // refundReceiver
            safe.nonce()
        );
        console.log("Transaction hash for Safe: ");
        console.logBytes32(txHash);

        return txHash;
    }

    function execTransaction() public {
        EnvConfig memory config = readEnv();
        if (config.signatures.length == 0) {
            revert MissingSignatures();
        }

        // Build tx
        bytes memory calls = buildSafeTx(config);

        // Exec tx
        GnosisSafe safe = GnosisSafe(payable(config.safe));
        vm.startBroadcast();
        safe.execTransaction(
            MULTICALL3_ADDRESS,
            0, // value
            calls,
            Enum.Operation(1), // delegate call
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(config.sender), // refundReceiver
            config.signatures
        );
        vm.stopBroadcast();
        console.log("Transaction executed with Safe");
    }
}
