// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Claim, GameType, Hash, OutputRoot, Duration} from "src/dispute/lib/Types.sol";
import {LibString} from "@solady/utils/LibString.sol";

// Interfaces
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";
import {ISuperchainConfig} from "interfaces/L1/ISuperchainConfig.sol";
import {IOptimismPortal2} from "interfaces/L1/IOptimismPortal2.sol";

// Contracts
import {AnchorStateRegistry} from "src/dispute/AnchorStateRegistry.sol";
import {AccessManager} from "../../src/fp/AccessManager.sol";
import {SuperchainConfig} from "src/L1/SuperchainConfig.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {OPSuccinctFaultDisputeGame} from "../../src/fp/OPSuccinctFaultDisputeGame.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

// Utils
import {MockOptimismPortal2} from "../../utils/MockOptimismPortal2.sol";

contract DeployOPSuccinctFDG is Script {
    struct FaultDisputeGameConfig {
        bytes32 aggregationVkey;
        address[] challengerAddresses;
        uint256 challengerBondWei;
        uint256 disputeGameFinalityDelaySeconds;
        uint256 fallbackTimeoutFpSecs;
        uint32 gameType;
        uint256 initialBondWei;
        uint256 maxChallengeDuration;
        uint256 maxProveDuration;
        bool permissionlessMode;
        address[] proposerAddresses;
        bytes32 rangeVkeyCommitment;
        bytes32 rollupConfigHash;
        uint256 startingL2BlockNumber;
        bytes32 startingRoot;
        bool useSp1MockVerifier;
        address verifierAddress;
    }

    function run() public {
        vm.startBroadcast();

        // Load configuration
        FaultDisputeGameConfig memory config = loadConfig();
        
        // Deploy contracts
        deployContracts(config);
        
        vm.stopBroadcast();
    }
    
    function loadConfig() internal returns (FaultDisputeGameConfig memory config) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/opsuccinctfdgconfig.json");
        
        if (vm.exists(path)) {
            console.log("Reading config from opsuccinctfdgconfig.json");
            string memory json = vm.readFile(path);
            
            // Parse JSON fields
            config.aggregationVkey = vm.parseJsonBytes32(json, ".aggregationVkey");
            config.challengerBondWei = vm.parseJsonUint(json, ".challengerBondWei");
            config.disputeGameFinalityDelaySeconds = vm.parseJsonUint(json, ".disputeGameFinalityDelaySeconds");
            config.fallbackTimeoutFpSecs = vm.parseJsonUint(json, ".fallbackTimeoutFpSecs");
            config.gameType = uint32(vm.parseJsonUint(json, ".gameType"));
            config.initialBondWei = vm.parseJsonUint(json, ".initialBondWei");
            config.maxChallengeDuration = vm.parseJsonUint(json, ".maxChallengeDuration");
            config.maxProveDuration = vm.parseJsonUint(json, ".maxProveDuration");
            config.permissionlessMode = vm.parseJsonBool(json, ".permissionlessMode");
            config.rangeVkeyCommitment = vm.parseJsonBytes32(json, ".rangeVkeyCommitment");
            config.rollupConfigHash = vm.parseJsonBytes32(json, ".rollupConfigHash");
            config.startingL2BlockNumber = vm.parseJsonUint(json, ".startingL2BlockNumber");
            config.startingRoot = vm.parseJsonBytes32(json, ".startingRoot");
            config.useSp1MockVerifier = vm.parseJsonBool(json, ".useSp1MockVerifier");
            config.verifierAddress = vm.parseJsonAddress(json, ".verifierAddress");
            
            // Parse arrays
            config.proposerAddresses = vm.parseJsonAddressArray(json, ".proposerAddresses");
            config.challengerAddresses = vm.parseJsonAddressArray(json, ".challengerAddresses");
        } else {
            console.log("No config file found, reading from environment variables");
            config = loadConfigFromEnv();
        }
    }
    
    function loadConfigFromEnv() internal returns (FaultDisputeGameConfig memory config) {
        config.gameType = uint32(vm.envUint("GAME_TYPE"));
        config.disputeGameFinalityDelaySeconds = vm.envUint("DISPUTE_GAME_FINALITY_DELAY_SECONDS");
        config.maxChallengeDuration = vm.envUint("MAX_CHALLENGE_DURATION");
        config.maxProveDuration = vm.envUint("MAX_PROVE_DURATION");
        config.startingL2BlockNumber = vm.envUint("STARTING_L2_BLOCK_NUMBER");
        config.startingRoot = vm.envBytes32("STARTING_ROOT");
        config.fallbackTimeoutFpSecs = vm.envOr("FALLBACK_TIMEOUT_FP_SECS", uint256(1209600));
        config.challengerBondWei = vm.envOr("CHALLENGER_BOND_WEI", uint256(0.001 ether));
        config.initialBondWei = vm.envOr("INITIAL_BOND_WEI", uint256(0.001 ether));
        config.useSp1MockVerifier = vm.envOr("USE_SP1_MOCK_VERIFIER", false);
        config.permissionlessMode = vm.envOr("PERMISSIONLESS_MODE", false);
        
        if (!config.useSp1MockVerifier) {
            config.verifierAddress = vm.envAddress("VERIFIER_ADDRESS");
            config.rollupConfigHash = vm.envBytes32("ROLLUP_CONFIG_HASH");
            config.aggregationVkey = vm.envBytes32("AGGREGATION_VKEY");
            config.rangeVkeyCommitment = vm.envBytes32("RANGE_VKEY_COMMITMENT");
        }
        
        // Parse addresses from comma-separated strings
        if (!config.permissionlessMode) {
            string memory proposersStr = vm.envOr("PROPOSER_ADDRESSES", string(""));
            string memory challengersStr = vm.envOr("CHALLENGER_ADDRESSES", string(""));
            
            if (bytes(proposersStr).length > 0) {
                string[] memory proposers = LibString.split(proposersStr, ",");
                config.proposerAddresses = new address[](proposers.length);
                for (uint256 i = 0; i < proposers.length; i++) {
                    config.proposerAddresses[i] = vm.parseAddress(proposers[i]);
                }
            }
            
            if (bytes(challengersStr).length > 0) {
                string[] memory challengers = LibString.split(challengersStr, ",");
                config.challengerAddresses = new address[](challengers.length);
                for (uint256 i = 0; i < challengers.length; i++) {
                    config.challengerAddresses[i] = vm.parseAddress(challengers[i]);
                }
            }
        }
    }
    
    function deployContracts(FaultDisputeGameConfig memory config) internal {

        // Deploy factory proxy.
        ERC1967Proxy factoryProxy = new ERC1967Proxy(
            address(new DisputeGameFactory()),
            abi.encodeWithSelector(DisputeGameFactory.initialize.selector, msg.sender)
        );
        DisputeGameFactory factory = DisputeGameFactory(address(factoryProxy));

        GameType gameType = GameType.wrap(config.gameType);

        // Use provided OptimismPortal2 address if given, otherwise deploy MockOptimismPortal2.
        address payable portalAddress;
        if (vm.envOr("OPTIMISM_PORTAL2_ADDRESS", address(0)) != address(0)) {
            portalAddress = payable(vm.envAddress("OPTIMISM_PORTAL2_ADDRESS"));
            console.log("Using existing OptimismPortal2:", portalAddress);
        } else {
            MockOptimismPortal2 portal =
                new MockOptimismPortal2(gameType, config.disputeGameFinalityDelaySeconds);
            portalAddress = payable(address(portal));
            console.log("Deployed MockOptimismPortal2:", portalAddress);
        }

        OutputRoot memory startingAnchorRoot = OutputRoot({
            root: Hash.wrap(config.startingRoot),
            l2BlockNumber: config.startingL2BlockNumber
        });

        // Deploy the anchor state registry proxy.
        ERC1967Proxy registryProxy = new ERC1967Proxy(
            address(new AnchorStateRegistry()),
            abi.encodeCall(
                AnchorStateRegistry.initialize,
                (
                    ISuperchainConfig(address(new SuperchainConfig())),
                    IDisputeGameFactory(address(factory)),
                    IOptimismPortal2(portalAddress),
                    startingAnchorRoot
                )
            )
        );

        AnchorStateRegistry registry = AnchorStateRegistry(address(registryProxy));
        console.log("Anchor state registry:", address(registry));

        // Deploy the access manager contract.
        AccessManager accessManager = new AccessManager(config.fallbackTimeoutFpSecs);
        console.log("Access manager:", address(accessManager));
        console.log("Permissionless fallback timeout (seconds):", config.fallbackTimeoutFpSecs);

        // Configure access control based on config.
        if (config.permissionlessMode) {
            // Set to permissionless games (anyone can propose and challenge).
            accessManager.setProposer(address(0), true);
            accessManager.setChallenger(address(0), true);
            console.log("Access Manager configured for permissionless mode");
        } else {
            // Set proposers.
            for (uint256 i = 0; i < config.proposerAddresses.length; i++) {
                if (config.proposerAddresses[i] != address(0)) {
                    accessManager.setProposer(config.proposerAddresses[i], true);
                    console.log("Added proposer:", config.proposerAddresses[i]);
                }
            }

            // Set challengers.
            for (uint256 i = 0; i < config.challengerAddresses.length; i++) {
                if (config.challengerAddresses[i] != address(0)) {
                    accessManager.setChallenger(config.challengerAddresses[i], true);
                    console.log("Added challenger:", config.challengerAddresses[i]);
                }
            }
        }

        // Config values for verifier.
        address sp1VerifierAddress;
        bytes32 rollupConfigHash = config.rollupConfigHash;
        bytes32 aggregationVkey = config.aggregationVkey;
        bytes32 rangeVkeyCommitment = config.rangeVkeyCommitment;

        // Get or deploy SP1 verifier based on configuration.
        if (config.useSp1MockVerifier) {
            // Deploy mock verifier for testing.
            SP1MockVerifier sp1Verifier = new SP1MockVerifier();
            sp1VerifierAddress = address(sp1Verifier);
            console.log("Using SP1 Mock Verifier:", address(sp1Verifier));

            // Use zero values for mock verifier if not already set.
            if (rollupConfigHash == bytes32(0)) rollupConfigHash = bytes32(0);
            if (aggregationVkey == bytes32(0)) aggregationVkey = bytes32(0);
            if (rangeVkeyCommitment == bytes32(0)) rangeVkeyCommitment = bytes32(0);
        } else {
            // Use provided verifier address for production.
            sp1VerifierAddress = config.verifierAddress;
            console.log("Using SP1 Verifier Gateway:", sp1VerifierAddress);
        }

        // Deploy game implementation (split to avoid stack too deep)
        OPSuccinctFaultDisputeGame gameImpl = deployGameImplementation(
            config,
            factory,
            sp1VerifierAddress,
            rollupConfigHash,
            aggregationVkey,
            rangeVkeyCommitment,
            registry,
            accessManager
        );

        // Set initial bond and implementation in factory.
        factory.setInitBond(gameType, config.initialBondWei);
        factory.setImplementation(gameType, IDisputeGame(address(gameImpl)));

        // Log deployed addresses.
        console.log("Factory Proxy:", address(factoryProxy));
        console.log("Game Implementation:", address(gameImpl));
        console.log("SP1 Verifier:", sp1VerifierAddress);
    }

    function deployGameImplementation(
        FaultDisputeGameConfig memory config,
        DisputeGameFactory factory,
        address sp1VerifierAddress,
        bytes32 rollupConfigHash,
        bytes32 aggregationVkey,
        bytes32 rangeVkeyCommitment,
        AnchorStateRegistry registry,
        AccessManager accessManager
    ) internal returns (OPSuccinctFaultDisputeGame) {
        return new OPSuccinctFaultDisputeGame(
            Duration.wrap(uint64(config.maxChallengeDuration)),
            Duration.wrap(uint64(config.maxProveDuration)),
            IDisputeGameFactory(address(factory)),
            ISP1Verifier(sp1VerifierAddress),
            rollupConfigHash,
            aggregationVkey,
            rangeVkeyCommitment,
            config.challengerBondWei,
            IAnchorStateRegistry(address(registry)),
            accessManager
        );
    }
}