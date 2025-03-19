// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {GameType, Claim} from "src/dispute/lib/Types.sol";
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {OPSuccinctDisputeGame} from "../../src/validity/OPSuccinctDisputeGame.sol";

import {OPSuccinctL2OutputOracle} from "../../src/validity/OPSuccinctL2OutputOracle.sol";

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {console} from "forge-std/console.sol";

contract ValidityTest is Script {
    // The owner of the L2OO.
    address OWNER = 0x788c45CafaB3ea427b9079889BE43D7d3cd7500C;



    function configureSetup() internal returns (DisputeGameFactory factory) {
        OPSuccinctL2OutputOracle l2OutputOracleProxy = OPSuccinctL2OutputOracle(
            vm.envAddress("L2OO_ADDRESS")
        );

        // Initialize the dispute game based on the existing L2OO_ADDRESS.
        OPSuccinctDisputeGame game = new OPSuccinctDisputeGame(
            address(l2OutputOracleProxy)
        );

        // Deploy the factory implementation
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy factory proxy
        ERC1967Proxy factoryProxy = new ERC1967Proxy(
            address(factoryImpl),
            abi.encodeWithSelector(
                DisputeGameFactory.initialize.selector,
                OWNER
            )
        );

        // Cast the factory proxy to the factory contract
        DisputeGameFactory gameFactory = DisputeGameFactory(
            address(factoryProxy)
        );

        // Set the init bond and implementation for the game type
        // NOTE(fakedev9999): GameType 6 is the game type for the OP_SUCCINCT proof system.
        // See https://github.com/ethereum-optimism/optimism/blob/6d7f3bcf1e3a80749a5d70f224e35b49dbd3bb3c/packages/contracts-bedrock/src/dispute/lib/Types.sol#L63-L64
        // Will be updated to GameTypes.OP_SUCCINCT once we upgrade to a new version of the Optimism contracts.
        GameType gameType = GameType.wrap(uint32(6));
        gameFactory.setImplementation(gameType, IDisputeGame(address(game)));

        return gameFactory;
    }

    function attemptCreateGame(
        uint8 gameType,
        bytes32 outputRoot,
        bytes memory extraData,
        DisputeGameFactory factory
    ) internal {
        // Try to create the game and log any revert
        console.log("Creating game");
        try
            factory.create{value: 0}(
                GameType.wrap(gameType),
                Claim.wrap(outputRoot),
                extraData
            )
        returns (IDisputeGame gameAddress) {
            console.log("Game created");
            console.logAddress(address(gameAddress));
        } catch Error(string memory reason) {
            console.log("Error creating game 1");
            console.log(reason);
        } catch (bytes memory returnData) {
            console.log("Error creating game 2");
            console.logBytes(returnData);
        }
    }

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(OWNER);

        bytes memory extraDataExample = abi.encodePacked(
            uint256(15000000),
            uint256(15000000),
            address(0x788c45CafaB3ea427b9079889BE43D7d3cd7500C),
            bytes("")
        );

        console.log("Extra data example");
        console.logBytes(extraDataExample);

        // // Load private key from env
        // uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        // address dgfAddress = vm.envAddress("DGF_ADDRESS");

        // vm.createSelectFork(vm.envString("L1_RPC"));

        // Setup values from the error log
        uint8 gameType = 6;
        bytes32 outputRoot = 0x2ca794b84ab6af41b2a39a40ef918e1b667a3c6a5f6f398e49475ee81bcb4863;
        bytes
            memory extraData = hex"000000000000000000000000000000000000000000000000000000000015d9c70000000000000000000000000000000000000000000000000000000000790fe7788c45cafab3ea427b9079889be43d7d3cd7500c";

        // DisputeGameFactory factory = DisputeGameFactory(dgfAddress);

        DisputeGameFactory factory = configureSetup();

        attemptCreateGame(gameType, outputRoot, extraData, factory);

        vm.stopBroadcast();
    }
}