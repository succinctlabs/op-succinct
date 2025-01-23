// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Testing
import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Libraries
import {Claim, Clock, Duration, GameStatus, GameType, Hash, OutputRoot, Position, Timestamp} from "src/dispute/lib/Types.sol";
import {ClockNotExpired} from "src/dispute/lib/Errors.sol";

// Contracts
import {AnchorStateRegistry} from "src/dispute/AnchorStateRegistry.sol";
import {OPSuccinctFaultDisputeGame} from "src/op/OPSuccinctFaultDisputeGame.sol";
import {OPSuccinctDisputeGameFactory} from "src/op/OPSuccinctDisputeGameFactory.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

// Interfaces
import {IAnchorStateRegistry} from "src/dispute/interfaces/IAnchorStateRegistry.sol";
import {IDisputeGame} from "src/dispute/interfaces/IDisputeGame.sol";
import {IDisputeGameFactory} from "src/dispute/interfaces/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

// Mock SuperchainConfig
contract MockSuperchainConfig {
    address public guardian = address(0x789);

    function setGuardian(address _guardian) external {
        guardian = _guardian;
    }
}

contract OPSuccinctDisputeGameTest is Test {
    // Event definition matching the one in the game contract
    event Resolved(GameStatus indexed status);

    OPSuccinctDisputeGameFactory factory;
    ERC1967Proxy factoryProxy;

    OPSuccinctFaultDisputeGame gameImpl;
    OPSuccinctFaultDisputeGame game;

    AnchorStateRegistry registry;
    ERC1967Proxy registryProxy;

    address proposer = address(0x123);
    address challenger = address(0x456);

    GameType gameType = GameType.wrap(6);
    Claim absolutePrestate = Claim.wrap(keccak256("absolutePrestate"));
    Duration maxClockDuration = Duration.wrap(7 days);
    uint256 l2ChainId = 10;
    AnchorStateRegistry.StartingAnchorRoot[] startingAnchorRoots;
    Hash startingAnchorRoot = Hash.wrap(keccak256("startingAnchorRoot"));
    Claim rootClaim = Claim.wrap(keccak256("rootClaim"));

    // Extra data is L2 block number bigger
    bytes extraData = abi.encode(uint256(1234567891));

    Hash gameUUID =
        Hash.wrap(keccak256(abi.encode(gameType, rootClaim, extraData)));

    function setUp() public {
        // Deploy the implementation contract
        OPSuccinctDisputeGameFactory implementation = new OPSuccinctDisputeGameFactory();

        // Deploy a proxy pointing to the implementation contract
        factoryProxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeWithSelector(
                OPSuccinctDisputeGameFactory.initialize.selector,
                this
            )
        );

        // Cast the factory proxy to the factory contract
        factory = OPSuccinctDisputeGameFactory(address(factoryProxy));

        // Deploy the registry implementation contract
        AnchorStateRegistry registryImpl = new AnchorStateRegistry(
            IDisputeGameFactory(address(factory))
        );

        // Prepare starting anchor roots
        startingAnchorRoots.push(
            AnchorStateRegistry.StartingAnchorRoot({
                gameType: gameType,
                outputRoot: OutputRoot({
                    l2BlockNumber: 1234567890,
                    root: startingAnchorRoot
                })
            })
        );

        // Deploy a mock superchain config
        MockSuperchainConfig mockSuperchainConfig = new MockSuperchainConfig();

        // Deploy a proxy pointing to the registry implementation contract
        registryProxy = new ERC1967Proxy(
            address(registryImpl),
            abi.encodeWithSelector(
                AnchorStateRegistry.initialize.selector,
                startingAnchorRoots,
                mockSuperchainConfig
            )
        );

        // Cast the registry proxy to the registry contract
        registry = AnchorStateRegistry(address(registryProxy));

        SP1MockVerifier sp1Verifier = new SP1MockVerifier();
        bytes32 aggregationVkey = bytes32(0);

        gameImpl = new OPSuccinctFaultDisputeGame(
            gameType,
            absolutePrestate,
            maxClockDuration,
            IAnchorStateRegistry(address(registry)),
            l2ChainId,
            ISP1Verifier(address(sp1Verifier)),
            aggregationVkey
        );

        // Set the initial bond
        factory.setInitBond(gameType, 1 ether);

        // Set the implementation
        factory.setImplementation(
            GameType.wrap(6),
            IDisputeGame(address(gameImpl))
        );

        // Create a new dispute game
        vm.prank(proposer);
        vm.deal(proposer, 1 ether);
        game = OPSuccinctFaultDisputeGame(
            address(
                factory.create{value: 1 ether}(
                    GameType.wrap(6),
                    rootClaim,
                    extraData
                )
            )
        );
        vm.stopPrank();
    }

    function testInitialization() public {
        vm.deal(address(game), 1 ether);
        // Test the initialization of the factory
        assertEq(address(factory.owner()), address(this));
        assertEq(address(factory.gameImpls(gameType)), address(gameImpl));
        assertEq(factory.gameCount(), 1);
        (
            GameType gameType_,
            Timestamp timestamp_,
            IDisputeGame proxy_
        ) = factory.gameAtIndex(0);
        assertEq(address(game), address(proxy_));

        // Test the initialization of the game
        assertEq(game.gameType().raw(), gameType.raw());
        assertEq(game.rootClaim().raw(), rootClaim.raw());
        assertEq(game.maxClockDuration().raw(), maxClockDuration.raw());
        assertEq(address(game.anchorStateRegistry()), address(registry));
        assertEq(game.l2ChainId(), l2ChainId);
        assertEq(game.l2BlockNumber(), 1234567891);
        assertEq(game.startingBlockNumber(), 1234567890);
        assertEq(game.startingRootHash().raw(), startingAnchorRoot.raw());
        assertEq(address(game).balance, 1 ether);

        // Test the claim data
        (
            uint32 parentIndex,
            address counteredBy,
            address claimant,
            uint128 bond,
            Claim claim,
            Clock clock
        ) = game.claimData();
        assertEq(counteredBy, address(0));
        assertEq(claimant, proposer);
        assertEq(bond, 1 ether);
        assertEq(claim.raw(), rootClaim.raw());

        // Test the anchor state registry
        assertEq(registry.superchainConfig().guardian(), address(0x789));
        (Hash root, uint256 rootBlockNumber) = registry.anchors(gameType);
        assertEq(root.raw(), startingAnchorRoot.raw());
    }

    function testResolve() public {
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));

        vm.expectRevert(ClockNotExpired.selector);
        game.resolve();

        // Set the clock to expire
        (
            uint32 parentIndex,
            address counteredBy,
            address claimant,
            uint128 bond,
            Claim claim,
            Clock clock
        ) = game.claimData();
        vm.warp(clock.raw() + 7 days);

        vm.expectEmit(true, false, false, false, address(game));
        emit Resolved(GameStatus.DEFENDER_WINS);
        game.resolve();

        assertEq(address(game).balance, 0);
        assertEq(proposer.balance, 1 ether);
        assertEq(challenger.balance, 0);
    }

    function testResolveChallengerWins() public {
        assertEq(uint8(game.status()), uint8(GameStatus.IN_PROGRESS));

        vm.expectRevert(ClockNotExpired.selector);
        game.resolve();

        vm.prank(challenger);
        vm.expectEmit(true, false, false, false, address(game));
        emit Resolved(GameStatus.CHALLENGER_WINS);
        game.resolveWithProof(bytes(""), bytes(""));

        assertEq(address(game).balance, 0);
        assertEq(proposer.balance, 0);
        assertEq(challenger.balance, 1 ether);
    }
}
