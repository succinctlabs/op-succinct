// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Libraries
import {
    Claim,
    Clock,
    Duration,
    GameStatus,
    GameType,
    Hash,
    OutputRoot,
    Position,
    Timestamp
} from "src/dispute/lib/Types.sol";
import {ClockNotExpired, IncorrectBondAmount} from "src/dispute/lib/Errors.sol";
import {AggregationOutputs} from "src/lib/Types.sol";

// Contracts
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {OPSuccinctFaultDisputeGame} from "src/fp/OPSuccinctFaultDisputeGame.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

// Interfaces
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

contract OPSuccinctFaultDisputeGameTest is Test {
    // Event definition matching the one in the game contract
    event Resolved(GameStatus indexed status);

    DisputeGameFactory factory;
    ERC1967Proxy factoryProxy;

    OPSuccinctFaultDisputeGame gameImpl;
    OPSuccinctFaultDisputeGame parentGame;
    OPSuccinctFaultDisputeGame game;

    address proposer = address(0x123);
    address challenger = address(0x456);

    GameType gameType = GameType.wrap(42);
    Duration maxChallengeDuration = Duration.wrap(7 days);
    Duration maxProveDuration = Duration.wrap(1 days);
    uint256 l2ChainId = 10;
    Claim rootClaim = Claim.wrap(keccak256("rootClaim1"));

    // Extra data must be the L2 block number bigger than the starting anchor root's block number
    uint256 l2BlockNumber = 1234567891;
    uint32 parentIndex = 0;

    function setUp() public {
        // Deploy the implementation contract
        DisputeGameFactory factoryImpl = new DisputeGameFactory();

        // Deploy a proxy pointing to the implementation contract
        factoryProxy =
            new ERC1967Proxy(address(factoryImpl), abi.encodeWithSelector(DisputeGameFactory.initialize.selector, this));

        // Cast the factory proxy to the factory contract
        factory = DisputeGameFactory(address(factoryProxy));

        SP1MockVerifier sp1Verifier = new SP1MockVerifier();
        bytes32 rollupConfigHash = bytes32(0);
        bytes32 aggregationVkey = bytes32(0);
        bytes32 rangeVkeyCommitment = bytes32(0);

        gameImpl = new OPSuccinctFaultDisputeGame(
            maxChallengeDuration,
            maxProveDuration,
            IDisputeGameFactory(address(factory)),
            l2ChainId,
            ISP1Verifier(address(sp1Verifier)),
            rollupConfigHash,
            aggregationVkey,
            rangeVkeyCommitment
        );

        // Set the initial bond
        factory.setInitBond(gameType, 1 ether);

        // Set the implementation
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        vm.startPrank(proposer);
        vm.deal(proposer, 1 ether);
        // Create the very first game
        parentGame = OPSuccinctFaultDisputeGame(
            address(
                factory.create{value: 1 ether}(
                    gameType, Claim.wrap(keccak256("rootClaim")), abi.encodePacked(uint256(1234567890), uint32(0))
                )
            )
        );
        parentGame.resolve();

        // Create the second game
        game = OPSuccinctFaultDisputeGame(
            address(factory.create{value: 1 ether}(gameType, rootClaim, abi.encodePacked(l2BlockNumber, parentIndex)))
        );
        vm.stopPrank();
    }

    function testInitialization() public {
        // Test the initialization of the factory
        assertEq(address(factory.owner()), address(this));
        assertEq(address(factory.gameImpls(gameType)), address(gameImpl));
        assertEq(factory.gameCount(), 2);
        (,, IDisputeGame proxy_) = factory.gameAtIndex(1);
        assertEq(address(game), address(proxy_));

        // Test the initialization of the second game
        assertEq(game.gameType().raw(), gameType.raw());
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.maxChallengeDuration().raw(), maxChallengeDuration.raw());
        assertEq(game.maxProveDuration().raw(), maxProveDuration.raw());
        assertEq(address(game.disputeGameFactory()), address(factory));
        assertEq(game.l2ChainId(), l2ChainId);
        assertEq(game.l2BlockNumber(), l2BlockNumber);
        assertEq(game.startingBlockNumber(), 1234567890);
        assertEq(game.startingRootHash().raw(), keccak256("rootClaim"));
        assertEq(address(game).balance, 1 ether);

        // // Test the claim data
        (, address counteredBy, address claimant, Claim claim,) = game.claimData();
        assertEq(counteredBy, address(0));
        assertEq(claimant, proposer);
        assertEq(address(game).balance, 1 ether);
        assertEq(claim.raw(), rootClaim.raw());
    }

    function testResolve() public {
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));

        vm.expectRevert(ClockNotExpired.selector);
        game.resolve();

        // Set the clock to expire
        (,,,, Clock clock) = game.claimData();
        vm.warp(clock.raw() + 7 days);

        vm.expectEmit(true, false, false, false, address(game));
        emit Resolved(GameStatus.DEFENDER_WINS);
        game.resolve();

        assertEq(address(game).balance, 0);
        assertEq(proposer.balance, 1 ether);
        assertEq(challenger.balance, 0);
    }

    function testProve() public {
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));

        vm.expectRevert(ClockNotExpired.selector);
        game.resolve();

        assertEq(address(game).balance, 1 ether);

        vm.startPrank(challenger);
        vm.deal(challenger, 1 ether);

        vm.expectRevert(IncorrectBondAmount.selector);
        game.challenge{value: 0.5 ether}();

        game.challenge{value: 1 ether}();
        vm.stopPrank();

        vm.startPrank(proposer);
        game.prove(
            abi.encode(
                AggregationOutputs({
                    l1Head: Hash.unwrap(game.l1Head()),
                    l2PreRoot: Hash.unwrap(game.startingRootHash()),
                    claimRoot: rootClaim.raw(),
                    claimBlockNum: l2BlockNumber,
                    rollupConfigHash: bytes32(0),
                    rangeVkeyCommitment: bytes32(0)
                })
            ),
            bytes("")
        );
        vm.stopPrank();

        (, address counteredBy,,,) = game.claimData();
        assertEq(counteredBy, challenger);

        assertEq(address(game).balance, 0);
        assertEq(proposer.balance, 2 ether);
        assertEq(challenger.balance, 0);
    }
}
