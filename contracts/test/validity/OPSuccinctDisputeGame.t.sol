// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Libraries
import {Claim, GameStatus, GameType, GameTypes, Hash, OutputRoot, Timestamp} from "src/dispute/lib/Types.sol";
import {AlreadyInitialized, GameNotInProgress, NoCreditToClaim, GameNotFinalized} from "src/dispute/lib/Errors.sol";
import {Utils} from "../helpers/Utils.sol";

// Contracts
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {OPSuccinctDisputeGame} from "src/validity/OPSuccinctDisputeGame.sol";
import {OPSuccinctL2OutputOracle} from "src/validity/OPSuccinctL2OutputOracle.sol";
import {AnchorStateRegistry} from "src/dispute/AnchorStateRegistry.sol";
import {SuperchainConfig} from "src/L1/SuperchainConfig.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";

// Interfaces
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISuperchainConfig} from "interfaces/L1/ISuperchainConfig.sol";
import {IOptimismPortal2} from "interfaces/L1/IOptimismPortal2.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";

// Utils
import {MockOptimismPortal2} from "../../utils/MockOptimismPortal2.sol";

contract OPSuccinctDisputeGameTest is Test, Utils {
    // Event definitions matching those in OPSuccinctDisputeGame.
    event Resolved(GameStatus indexed status);

    DisputeGameFactory factory;
    ERC1967Proxy factoryProxy;

    OPSuccinctDisputeGame gameImpl;
    OPSuccinctDisputeGame game;

    OPSuccinctL2OutputOracle l2OutputOracle;

    address proposer = address(0x123);

    // Fixed parameters.
    GameType gameType = GameTypes.OP_SUCCINCT;
    Claim rootClaim = Claim.wrap(keccak256("rootClaim"));

    // Game creation parameters.
    uint256 l2BlockNumber = 2000;
    uint256 l1BlockNumber = 1000;

    function setUp() public {
        // Deploy the implementation contract for DisputeGameFactory.
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy a proxy pointing to the factory implementation.
        factoryProxy = new ERC1967Proxy(
            address(factoryImpl), abi.encodeWithSelector(DisputeGameFactory.initialize.selector, address(this))
        );

        // Cast the proxy to the factory contract.
        factory = DisputeGameFactory(address(factoryProxy));

        // Deploy L2OutputOracle using Utils helper functions.
        (l2OutputOracle,) = deployL2OutputOracleWithStandardParams(proposer, address(0), address(this));

        // Deploy the implementation of OPSuccinctDisputeGame.
        gameImpl = new OPSuccinctDisputeGame(address(l2OutputOracle));

        // Register our reference implementation under the specified gameType.
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        // Create a game
        vm.startBroadcast(proposer);

        // Warp time forward to ensure the game is created after the respectedGameTypeUpdatedAt timestamp.
        warpRollAndCheckpoint(l2OutputOracle, 4001, l1BlockNumber);

        bytes memory proof = bytes("");
        game = OPSuccinctDisputeGame(
            address(
                factory.create(gameType, rootClaim, abi.encodePacked(l2BlockNumber, l1BlockNumber, proposer, l2OutputOracle.DEFAULT_CONFIG_NAME(), proof))
            )
        );

        vm.stopBroadcast();
    }

    // =========================================
    // Test: Basic initialization checks
    // =========================================
    function testInitialization() public view {
        // Test that the factory is correctly initialized.
        assertEq(address(factory.owner()), address(this));
        assertEq(address(factory.gameImpls(gameType)), address(gameImpl));
        assertEq(factory.gameCount(), 1);

        // Check that the game matches the 'gameAtIndex(0)'.
        (,, IDisputeGame proxy_) = factory.gameAtIndex(0);
        assertEq(address(game), address(proxy_));

        // Check the game fields.
        assertEq(game.gameType().raw(), gameType.raw());
        assertEq(game.gameCreator(), proposer);
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.l2BlockNumber(), l2BlockNumber);
        assertEq(game.l1BlockNumber(), l1BlockNumber);
        assertEq(game.proverAddress(), proposer);
        assertEq(game.configName(), l2OutputOracle.DEFAULT_CONFIG_NAME());
        assertEq(keccak256(game.proof()), keccak256(bytes("")));
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
    }

    // =========================================
    // Test: Cannot resolve game twice
    // =========================================
    function testCannotResolveTwice() public {
        vm.expectRevert(GameNotInProgress.selector);
        game.resolve();
    }

    // =========================================
    // Test: Cannot re-initialize game
    // =========================================
    function testCannotReInitializeGame() public {
        vm.startBroadcast(proposer);
        vm.expectRevert("L2OutputOracle: block number must be greater than or equal to next expected block number");
        game.initialize();
        vm.stopBroadcast();
    }

    // =========================================
    // Test: Cannot create game without permission
    // =========================================
    function testCannotCreateGameWithoutPermission() public {
        address maliciousProposer = address(0x1234);

        vm.startPrank(maliciousProposer);
        vm.deal(maliciousProposer, 1 ether);

        // Warp forward to the block we want to propose and checkpoint
        uint256 newL1BlockNumber = l1BlockNumber + 500;
        warpRollAndCheckpoint(l2OutputOracle, 2000, newL1BlockNumber);

        bytes memory proof = bytes("");
        bytes32 configName = l2OutputOracle.DEFAULT_CONFIG_NAME();
        vm.expectRevert("L2OutputOracle: only approved proposers can propose new outputs");
        factory.create(
            gameType,
            Claim.wrap(keccak256("new-claim")),
            abi.encodePacked(l2BlockNumber + 1000, newL1BlockNumber, maliciousProposer, configName, proof)
        );

        vm.stopPrank();
    }

    // =========================================
    // Test: Real Proof
    // =========================================
    function testRealProof() public {
        uint256 checkpointedL1BlockNum = 8577428;
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);

        proposer = 0x4b713049Fc139df09A20F55f5b76c08184135DF8;

        factory = DisputeGameFactory(0x329326d97759e4802a8c583517Ba0435b9906091);

        // Example proof data for a real proof for Phala Testnet. Tx: https://sepolia.etherscan.io/tx/0xeb3ccf9d86b5495da24df4ecfbb02b03404ef3a72de4fc29326c996be4c10005
        rootClaim = Claim.wrap(0xea25931ad9d3b9fc0c12864219ee1b10e9e9dd1237bbb066b2ee69bbbc0e036c);
        bytes memory extraData = bytes(
            hex"000000000000000000000000000000000000000000000000000000000021c195000000000000000000000000000000000000000000000000000000000082e1944b713049fc139df09a20f55f5b76c08184135df8572f01d824885a118d5d21c74542f263b131d2897955c62a721594f1d7c3b2e2a4594c591be3a79f6de27adfd2967726ce24d7e3eaead80b2c49e72363086100569dbf5c28ec37ff682ded52c427f51306fc846a3b584b9d48705ea04f1749fdb0dd61e91ce1563d1a2d59375dfd6d668f04c4fcd477a13b4d0f5f759e2c1bc180426e94236cb48e2ae1f22683903b0dace873e31db391b89f4ca8400c4f3cc3c94406f9093badcdb48f964a79787734e5a30999f4e0df495ac740844421779ac975eab107a347746da798d4f6f5d88b2998c51f68bc9ab575f7e97c8ea27aaa95f08d840b80ad90e76e59fa4098db4e2bfe302854c99f4cd53c47a9744f7fa9604b8c822b323243c1df4a173df607db192e5d478ff5c31042fe9790d28337b3fd342ed5"
        );

        vm.startBroadcast(proposer);
        factory.create(gameType, rootClaim, extraData);
        vm.stopBroadcast();
    }
}
