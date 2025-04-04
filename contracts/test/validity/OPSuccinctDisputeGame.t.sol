// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Libraries
import {Claim, GameStatus, GameType, Hash, OutputRoot, Timestamp} from "src/dispute/lib/Types.sol";
import {
    BadAuth,
    AlreadyInitialized,
    GameNotInProgress,
    NoCreditToClaim,
    GameNotFinalized
} from "src/dispute/lib/Errors.sol";
import {Utils} from "../helpers/Utils.sol";

// Contracts
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {OPSuccinctDisputeGame} from "src/validity/OPSuccinctDisputeGame.sol";
import {OPSuccinctL2OutputOracle} from "src/validity/OPSuccinctL2OutputOracle.sol";
import {AnchorStateRegistry} from "src/dispute/AnchorStateRegistry.sol";
import {SuperchainConfig} from "src/L1/SuperchainConfig.sol";
import {Proxy} from "@optimism/src/universal/Proxy.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";
import {AccessManager} from "src/lib/AccessManager.sol";

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

    AnchorStateRegistry anchorStateRegistry;
    OPSuccinctL2OutputOracle l2OutputOracle;

    address proposer = address(0x123);
    address challenger = address(0x456);
    address prover = address(0x789);

    MockOptimismPortal2 portal;

    uint256 disputeGameFinalityDelaySeconds = 1000;

    // Fixed parameters.
    GameType gameType = GameType.wrap(6);
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

        // Create a mock portal
        portal = new MockOptimismPortal2(gameType, disputeGameFinalityDelaySeconds);

        // Create an anchor state registry.
        SuperchainConfig superchainConfig = new SuperchainConfig();
        OutputRoot memory startingAnchorRoot = OutputRoot({root: Hash.wrap(keccak256("genesis")), l2BlockNumber: 0});

        ERC1967Proxy proxy = new ERC1967Proxy(
            address(new AnchorStateRegistry()),
            abi.encodeCall(
                AnchorStateRegistry.initialize,
                (
                    ISuperchainConfig(address(superchainConfig)),
                    IDisputeGameFactory(address(factory)),
                    IOptimismPortal2(payable(address(portal))),
                    startingAnchorRoot
                )
            )
        );
        anchorStateRegistry = AnchorStateRegistry(address(proxy));

        // Create a mock verifier.
        SP1MockVerifier sp1Verifier = new SP1MockVerifier();

        // Deploy L2OutputOracle.
        OPSuccinctL2OutputOracle.InitParams memory initParams = OPSuccinctL2OutputOracle.InitParams({
            verifier: address(sp1Verifier),
            aggregationVkey: bytes32(0),
            rangeVkeyCommitment: bytes32(0),
            startingOutputRoot: startingAnchorRoot.root.raw(),
            rollupConfigHash: bytes32(0),
            proposer: address(0), // Should be permissionless when using game creation from the factory or else, the check in `proposeL2Output` will fail.
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

        // Deploy the access manager with the proposer as the only allowed proposer.
        AccessManager accessManager = new AccessManager();
        accessManager.setProposer(proposer, true);

        // Deploy the implementation of OPSuccinctDisputeGame.
        gameImpl = new OPSuccinctDisputeGame(address(l2OutputOracle), accessManager);

        // Set the init bond on the factory for the OPSuccinctDG specific GameType.
        // factory.setInitBond(gameType,  ether);

        // Register our reference implementation under the specified gameType.
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        // Create a game
        vm.startPrank(proposer);
        vm.deal(proposer, 2 ether); // extra funds for testing.

        // Warp time forward to ensure the game is created after the respectedGameTypeUpdatedAt timestamp.
        vm.warp(block.timestamp + 4001);

        // Roll forward to the block we want to checkpoint
        vm.roll(l1BlockNumber + 1);

        // Checkpoint the L1 block hash that we'll use
        l2OutputOracle.checkpointBlockHash(l1BlockNumber);

        bytes memory proof = bytes("");
        game = OPSuccinctDisputeGame(
            address(factory.create(gameType, rootClaim, abi.encodePacked(l2BlockNumber, l1BlockNumber, prover, proof)))
        );

        vm.stopPrank();
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
        assertEq(game.proverAddress(), prover);
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
        vm.expectRevert("L2OutputOracle: block number must be greater than or equal to next expected block number");
        game.initialize();
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
        vm.expectRevert(BadAuth.selector);
        factory.create(
            gameType,
            Claim.wrap(keccak256("new-claim")),
            abi.encodePacked(l2BlockNumber + 1000, newL1BlockNumber, prover, proof)
        );

        vm.stopPrank();
    }
}
