// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {OPSuccinctL2OutputOracle} from "../src/validity/OPSuccinctL2OutputOracle.sol";
import {OPSuccinctDisputeGame} from "../src/validity/OPSuccinctDisputeGame.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {Utils} from "../test/helpers/Utils.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {console} from "forge-std/console.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {GameType} from "src/dispute/lib/Types.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";

contract OPSuccinctDFGDeployer is Script, Utils {
    function run() public returns (address) {
        vm.startBroadcast();

        OPSuccinctL2OutputOracle l2OutputOracleProxy = OPSuccinctL2OutputOracle(vm.envAddress("L2OO_ADDRESS"));

        l2OutputOracleProxy.addProposer(address(0));

        // Initialize the dispute game based on the existing L2OO_ADDRESS.
        OPSuccinctDisputeGame game = new OPSuccinctDisputeGame(address(l2OutputOracleProxy));

        // Deploy the factory implementation
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy factory proxy
        ERC1967Proxy factoryProxy = new ERC1967Proxy(
            address(factoryImpl), abi.encodeWithSelector(DisputeGameFactory.initialize.selector, msg.sender)
        );

        // Cast the factory proxy to the factory contract
        DisputeGameFactory gameFactory = DisputeGameFactory(address(factoryProxy));

        // Set the init bond and implementation for the game type
        GameType gameType = GameType.wrap(uint32(6));
        gameFactory.setInitBond(gameType, 0.01 ether);
        gameFactory.setImplementation(gameType, IDisputeGame(address(game)));

        vm.stopBroadcast();

        return address(gameFactory);
    }
}
