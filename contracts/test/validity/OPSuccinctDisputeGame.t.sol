// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import "forge-std/console.sol";
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
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

// Interfaces
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISuperchainConfig} from "interfaces/L1/ISuperchainConfig.sol";
import {IOptimismPortal2} from "interfaces/L1/IOptimismPortal2.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";

// Utils
import {MockOptimismPortal2} from "../../utils/MockOptimismPortal2.sol";

contract OPSuccinctDisputeGameTest is Test {
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

        // Create a mock verifier.
        SP1MockVerifier sp1Verifier = new SP1MockVerifier();

        // Deploy L2OutputOracle.
        OPSuccinctL2OutputOracle.InitParams memory initParams = OPSuccinctL2OutputOracle.InitParams({
            verifier: address(sp1Verifier),
            aggregationVkey: bytes32(0),
            rangeVkeyCommitment: bytes32(0),
            startingOutputRoot: bytes32(0),
            rollupConfigHash: bytes32(0),
            proposer: address(proposer), // Should be permissionless when using game creation from the factory or else, the check in `proposeL2Output` will fail.
            challenger: address(0),
            owner: address(this),
            finalizationPeriodSeconds: 1000 seconds,
            l2BlockTime: 2 seconds,
            startingBlockNumber: 0,
            startingTimestamp: block.timestamp,
            submissionInterval: 1000 seconds
        });
        bytes memory initializationParams =
            abi.encodeWithSelector(OPSuccinctL2OutputOracle.initialize.selector, initParams);

        Proxy l2OutputOracleProxy = new Proxy(address(this));
        l2OutputOracleProxy.upgradeToAndCall(address(new OPSuccinctL2OutputOracle()), initializationParams);

        l2OutputOracle = OPSuccinctL2OutputOracle(address(l2OutputOracleProxy));

        // Deploy the implementation of OPSuccinctDisputeGame.
        gameImpl = new OPSuccinctDisputeGame(address(l2OutputOracle));

        // Register our reference implementation under the specified gameType.
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        // Create a game
        vm.startBroadcast(proposer);

        // Warp time forward to ensure the game is created after the respectedGameTypeUpdatedAt timestamp.
        vm.warp(block.timestamp + 4001);

        // Roll forward to the block we want to checkpoint
        vm.roll(l1BlockNumber + 1);

        // Checkpoint the L1 block hash that we'll use
        l2OutputOracle.checkpointBlockHash(l1BlockNumber);

        bytes memory proof = bytes("");
        game = OPSuccinctDisputeGame(
            address(
                factory.create(gameType, rootClaim, abi.encodePacked(l2BlockNumber, l1BlockNumber, proposer, proof))
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

        // Warp forward to the block we want to propose
        vm.warp(block.timestamp + 2000);

        // Roll forward to the block we want to checkpoint
        uint256 newL1BlockNumber = l1BlockNumber + 500;
        vm.roll(newL1BlockNumber + 1);

        // Checkpoint the L1 block hash that we'll use
        l2OutputOracle.checkpointBlockHash(newL1BlockNumber);

        bytes memory proof = bytes("");
        vm.expectRevert("L2OutputOracle: only approved proposers can propose new outputs");
        factory.create(
            gameType,
            Claim.wrap(keccak256("new-claim")),
            abi.encodePacked(l2BlockNumber + 1000, newL1BlockNumber, maliciousProposer, proof)
        );

        vm.stopPrank();
    }

    // =========================================
    // Test: Can add and verify proposer permissions
    // =========================================
    function testCanAddProposerPermissions() public {
        address newProposer = address(0x5678);
        
        // Initially not approved
        assertFalse(l2OutputOracle.approvedProposers(newProposer));
        
        // Add new proposer to approved list
        l2OutputOracle.addProposer(newProposer);
        
        // Now should be approved
        assertTrue(l2OutputOracle.approvedProposers(newProposer));
        
        // Remove proposer
        l2OutputOracle.removeProposer(newProposer);
        
        // Should no longer be approved
        assertFalse(l2OutputOracle.approvedProposers(newProposer));
    }

    // =========================================
    // Test: Permissionless mode allows anyone
    // =========================================
    function testPermissionlessModeAllowsAnyone() public {
        address randomUser = address(0x9999);

        // Enable permissionless mode by adding address(0) to approved proposers
        l2OutputOracle.addProposer(address(0));
        assertTrue(l2OutputOracle.approvedProposers(address(0)));

        // Need more significant time warp to ensure future timestamp check passes
        vm.warp(block.timestamp + 10000);

        // Roll forward and checkpoint block
        uint256 newL1BlockNumber = l1BlockNumber + 600;
        vm.roll(newL1BlockNumber + 1);
        l2OutputOracle.checkpointBlockHash(newL1BlockNumber);

        vm.startPrank(randomUser);
        vm.deal(randomUser, 1 ether);

        bytes memory proof = bytes("");
        
        // Should succeed in permissionless mode
        IDisputeGame newGame = factory.create(
            gameType,
            Claim.wrap(keccak256("permissionless-claim")),
            abi.encodePacked(l2BlockNumber + 1500, newL1BlockNumber, randomUser, proof)
        );

        // Verify game was created successfully
        assertEq(uint8(newGame.status()), uint8(GameStatus.DEFENDER_WINS));

        vm.stopPrank();
    }

    // =========================================
    // Test: Remove proposer permission
    // =========================================
    function testRemoveProposerPermission() public {
        address tempProposer = address(0x7777);
        
        // First add the proposer
        l2OutputOracle.addProposer(tempProposer);
        assertTrue(l2OutputOracle.approvedProposers(tempProposer));

        // Remove the proposer
        l2OutputOracle.removeProposer(tempProposer);
        assertFalse(l2OutputOracle.approvedProposers(tempProposer));

        vm.startPrank(tempProposer);
        vm.deal(tempProposer, 1 ether);

        // Warp forward to ensure we can propose
        vm.warp(block.timestamp + 2000);

        // Roll forward and checkpoint block
        uint256 newL1BlockNumber = l1BlockNumber + 700;
        vm.roll(newL1BlockNumber + 1);
        
        // Switch to owner to checkpoint
        vm.stopPrank();
        l2OutputOracle.checkpointBlockHash(newL1BlockNumber);
        vm.startPrank(tempProposer);

        bytes memory proof = bytes("");
        
        // Should fail since proposer was removed
        vm.expectRevert("L2OutputOracle: only approved proposers can propose new outputs");
        factory.create(
            gameType,
            Claim.wrap(keccak256("removed-proposer-claim")),
            abi.encodePacked(l2BlockNumber + 2000, newL1BlockNumber, tempProposer, proof)
        );

        vm.stopPrank();
    }

    // =========================================
    // Test: Only owner can manage proposer permissions
    // =========================================
    function testOnlyOwnerCanManageProposerPermissions() public {
        address unauthorizedUser = address(0x8888);
        address testProposer = address(0x9090);

        vm.startPrank(unauthorizedUser);

        // Non-owner should not be able to add proposers
        vm.expectRevert("L2OutputOracle: caller is not the owner");
        l2OutputOracle.addProposer(testProposer);

        // Non-owner should not be able to remove proposers  
        vm.expectRevert("L2OutputOracle: caller is not the owner");
        l2OutputOracle.removeProposer(proposer);

        vm.stopPrank();

        // Owner should be able to add proposers
        l2OutputOracle.addProposer(testProposer);
        assertTrue(l2OutputOracle.approvedProposers(testProposer));

        // Owner should be able to remove proposers
        l2OutputOracle.removeProposer(testProposer);
        assertFalse(l2OutputOracle.approvedProposers(testProposer));
    }

    // =========================================
    // Test: tx.origin vs msg.sender behavior
    // =========================================
    function testTxOriginPermissionCheck() public {
        // This test demonstrates that the permission check uses tx.origin
        // which allows the game contract (msg.sender) to call proposeL2Output
        // as long as the transaction originator (tx.origin) is authorized

        // The current test setup already demonstrates this:
        // - tx.origin = proposer (authorized)  
        // - msg.sender = newly created game contract (not in approved proposers)
        // - Permission check passes because it uses tx.origin

        assertTrue(l2OutputOracle.approvedProposers(proposer));
        
        // Verify the game was created successfully in setUp
        assertEq(uint8(game.status()), uint8(GameStatus.DEFENDER_WINS));
        assertEq(game.gameCreator(), proposer);
    }

    // =========================================
    // Test: Real Proof
    // =========================================
    function testRealProof() public {
        uint256 checkpointedL1BlockNum = 8093968;
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);

        proposer = 0x9193a78157957F3E03beE50A3E6a51F0f1669E23;

        factory = DisputeGameFactory(0x62985aeB77b55aDAfAA21cCE41a7D8765D6B9507);

        // Example proof data for a real proof for Phala Testnet. Tx: https://sepolia.etherscan.io/tx/0xeb3ccf9d86b5495da24df4ecfbb02b03404ef3a72de4fc29326c996be4c10005
        rootClaim = Claim.wrap(0x80d3ec53fbda02abff3780477d29c5c7a51647bc1b4a0a296817c66d1426229d);
        bytes memory extraData = bytes(
            hex"000000000000000000000000000000000000000000000000000000000018ce4400000000000000000000000000000000000000000000000000000000007b81109193a78157957f3e03bee50a3e6a51f0f1669e2311b6a09d0f21aea5d178d355dc1799f78c1e82237dea7c175c01b1935588352d42d88ad92e9e7bc2ab4032b3137b71668be972c49b4e8a3797cb6aef1dc502ee5a79d92413408e8d4ca8a843156edfe2daa113bb48d541ef645953a8fc48a531fb033f5e253952f4174024cbd8f56b6c6a45de761c27bfb7518eb94837efc3f5f83728f628268cc82c127a83691b22b614ca8eeeaee49a9aa68f6cdb4a02078f968985282f2321941993b79cf426a012518bb89ac1bba3543cab9d11cf0698605924257a0909309b0a5e8f7bd5d5b1c63d35dfce67ee19abe6cce8a0911ce518af84edd322b2d48b71d146a319a502163e41e012c8872644f8737324fe91675a3d7f0dbb"
        );

        vm.startBroadcast(proposer);
        factory.create(gameType, rootClaim, extraData);
        vm.stopBroadcast();
    }
}
