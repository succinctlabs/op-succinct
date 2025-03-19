// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {GameType, Claim} from "src/dispute/lib/Types.sol";
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {console} from "forge-std/console.sol";

contract ValidityTest is Script {
    function run() public {
        // Load private key from env
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address dgfAddress = vm.envAddress("DGF_ADDRESS");

        // Setup values from the error log
        uint8 gameType = 6;
        bytes32 outputRoot = 0x2ca794b84ab6af41b2a39a40ef918e1b667a3c6a5f6f398e49475ee81bcb4863;
        bytes memory extraData = hex"000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000015d9c700000000000000000000000000000000000000000000000000000000790fe700000000000000000000000000000000000000000000000000000000000000008000000000000000000000000078c45cfab3ea427b90798697e43d7d3cd7500c00000000000000000000000000000000000000000000000000000000000000000";

        vm.startBroadcast(deployerPrivateKey);

        DisputeGameFactory factory = DisputeGameFactory(dgfAddress);

        // Try to create the game and log any revert
        try factory.create{value: 0}(GameType.wrap(gameType), Claim.wrap(outputRoot), extraData) returns (IDisputeGame gameAddress) {
            console.logAddress(address(gameAddress));
        } catch Error(string memory reason) {
            console.log(reason);
        } catch (bytes memory returnData) {
            console.logBytes(returnData);
        }

        vm.stopBroadcast();
    }
} 