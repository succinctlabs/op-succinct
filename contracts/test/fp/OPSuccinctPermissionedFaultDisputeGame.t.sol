// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Libraries
import {Claim, Duration, GameStatus, GameType} from "src/dispute/lib/Types.sol";
import {BadAuth} from "src/dispute/lib/Errors.sol";

// Contracts
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {OPSuccinctPermissionedFaultDisputeGame} from "src/fp/OPSuccinctPermissionedFaultDisputeGame.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

// Interfaces
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

contract OPSuccinctPermissionedFaultDisputeGameTest is Test {
    DisputeGameFactory factory;
    ERC1967Proxy factoryProxy;

    OPSuccinctPermissionedFaultDisputeGame gameImpl;
    OPSuccinctPermissionedFaultDisputeGame game;

    address proposer = address(0x123);
    address challenger = address(0x456);
    address unauthorized_address = address(0x789);

    // Fixed parameters
    GameType gameType = GameType.wrap(42);
    Duration maxChallengeDuration = Duration.wrap(12 hours);
    Duration maxProveDuration = Duration.wrap(3 days);
    Claim rootClaim = Claim.wrap(keccak256("rootClaim"));

    function setUp() public {
        // Deploy the implementation contract for DisputeGameFactory
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy a proxy pointing to the factory implementation
        factoryProxy = new ERC1967Proxy(
            address(factoryImpl), abi.encodeWithSelector(DisputeGameFactory.initialize.selector, address(this))
        );

        // Cast the proxy to the factory contract
        factory = DisputeGameFactory(address(factoryProxy));

        // Create a mock verifier
        SP1MockVerifier sp1Verifier = new SP1MockVerifier();

        // Parameters for the OPSuccinctFaultDisputeGame
        bytes32 rollupConfigHash = bytes32(0);
        bytes32 aggregationVkey = bytes32(0);
        bytes32 rangeVkeyCommitment = bytes32(0);
        uint256 genesisL2BlockNumber = 0;
        bytes32 genesisL2OutputRoot = keccak256("genesis");
        uint256 proofReward = 1 ether;

        // Deploy the reference implementation of OPSuccinctPermissionedFaultDisputeGame
        gameImpl = new OPSuccinctPermissionedFaultDisputeGame(
            maxChallengeDuration,
            maxProveDuration,
            IDisputeGameFactory(address(factory)),
            ISP1Verifier(address(sp1Verifier)),
            rollupConfigHash,
            aggregationVkey,
            rangeVkeyCommitment,
            genesisL2BlockNumber,
            genesisL2OutputRoot,
            proofReward,
            challenger,
            proposer
        );

        // Set the init bond on the factory for our specific GameType
        factory.setInitBond(gameType, 1 ether);

        // Register our reference implementation under the specified gameType
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        vm.startPrank(proposer, proposer); // The second argument sets tx.origin to the proposer
        vm.deal(proposer, 1 ether); // extra funds for testing
        game = OPSuccinctPermissionedFaultDisputeGame(
            address(
                factory.create{value: 1 ether}(
                    gameType,
                    rootClaim,
                    // encode l2BlockNumber = 1000, parentIndex = uint32.max
                    abi.encodePacked(uint256(1000), type(uint32).max)
                )
            )
        );

        vm.stopPrank();
    }

    // =========================================
    // Test: Only proposer can propose (testing initialize)
    // =========================================
    function testOnlyProposerCanPropose() public {
        vm.startPrank(challenger);
        vm.deal(challenger, 1 ether);
        vm.expectRevert(BadAuth.selector);
        game = OPSuccinctPermissionedFaultDisputeGame(
            address(
                factory.create{value: 1 ether}(
                    gameType,
                    rootClaim,
                    // encode l2BlockNumber = 1000, parentIndex = uint32.max
                    abi.encodePacked(uint256(1000), type(uint32).max)
                )
            )
        );
    }

    // =========================================
    // Test: Only challenger can challenge
    // =========================================
    function testOnlyChallengerCanChallenge() public {
        vm.startPrank(proposer);
        vm.deal(proposer, 1 ether);
        vm.expectRevert(BadAuth.selector);
        game.challenge{value: 1 ether}();
        vm.stopPrank();
    }

    // =========================================
    // Test: Only proposer can prove
    // =========================================
    function testOnlyProposerCanProve() public {
        vm.startPrank(challenger);
        vm.deal(challenger, 1 ether);
        vm.expectRevert(BadAuth.selector);
        game.prove(bytes(""));
        vm.stopPrank();
    }

    // =========================================
    // Test: Only proposer and challenger can resolve
    // =========================================
    function testOnlyProposerAndChallengerCanResolve() public {
        vm.startPrank(unauthorized_address);
        vm.expectRevert(BadAuth.selector);
        game.resolve();
        vm.stopPrank();
    }
}
