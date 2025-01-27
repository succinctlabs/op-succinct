// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {AnchorStateRegistry} from "src/dispute/AnchorStateRegistry.sol";
import {OPSuccinctFaultDisputeGame} from "src/fp/OPSuccinctFaultDisputeGame.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";
import {Claim, GameType, Hash, OutputRoot, Duration} from "src/dispute/lib/Types.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {IAnchorStateRegistry} from "src/dispute/interfaces/IAnchorStateRegistry.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

contract MockSuperchainConfig {
    address public guardian = address(0x789);

    function setGuardian(address _guardian) external {
        guardian = _guardian;
    }
}

contract DeployOPSuccinctDG is Script {
    function run() public {
        vm.startBroadcast();

        // Deploy the factory implementation
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy factory proxy
        ERC1967Proxy factoryProxy = new ERC1967Proxy(
            address(factoryImpl), abi.encodeWithSelector(DisputeGameFactory.initialize.selector, msg.sender)
        );
        DisputeGameFactory factory = DisputeGameFactory(address(factoryProxy));

        // Deploy registry implementation
        AnchorStateRegistry registryImpl = new AnchorStateRegistry(IDisputeGameFactory(address(factory)));

        // Setup starting anchor roots
        GameType gameType = GameType.wrap(42);
        Hash startingAnchorRoot = Hash.wrap(0xfff891c4fc262317af4e9a7bffedafc53c78e9e2aa83741bf15dc62afd932ffb);
        AnchorStateRegistry.StartingAnchorRoot[] memory startingAnchorRoots =
            new AnchorStateRegistry.StartingAnchorRoot[](1);
        startingAnchorRoots[0] = AnchorStateRegistry.StartingAnchorRoot({
            gameType: gameType,
            outputRoot: OutputRoot({l2BlockNumber: 22859590, root: startingAnchorRoot})
        });

        // Deploy mock superchain config
        MockSuperchainConfig mockSuperchainConfig = new MockSuperchainConfig();

        // Deploy registry proxy
        ERC1967Proxy registryProxy = new ERC1967Proxy(
            address(registryImpl),
            abi.encodeWithSelector(AnchorStateRegistry.initialize.selector, startingAnchorRoots, mockSuperchainConfig)
        );

        // Deploy SP1 mock verifier
        SP1MockVerifier sp1Verifier = new SP1MockVerifier();
        bytes32 aggregationVkey = bytes32(0);

        // Deploy game implementation
        Claim absolutePrestate = Claim.wrap(keccak256("absolutePrestate"));
        uint64 maxClockDuration = uint64(vm.envUint("MAX_CLOCK_DURATION"));
        uint256 l2ChainId = vm.envUint("L2_CHAIN_ID");

        OPSuccinctFaultDisputeGame gameImpl = new OPSuccinctFaultDisputeGame(
            absolutePrestate,
            Duration.wrap(maxClockDuration),
            IAnchorStateRegistry(address(registryProxy)),
            l2ChainId,
            ISP1Verifier(address(sp1Verifier)),
            aggregationVkey
        );

        // Set initial bond and implementation in factory
        factory.setInitBond(gameType, 0.01 ether);
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        vm.stopBroadcast();

        // Log deployed addresses
        console.log("Factory Proxy:", address(factoryProxy));
        console.log("Registry Proxy:", address(registryProxy));
        console.log("Game Implementation:", address(gameImpl));
        console.log("SP1 Verifier:", address(sp1Verifier));
    }
}
