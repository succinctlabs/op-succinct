// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {GameType, Duration} from "src/dispute/lib/Types.sol";

// Interfaces
import {IDisputeGame} from "interfaces/dispute/IDisputeGame.sol";
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";

// Contracts
import {AccessManager} from "../../src/fp/AccessManager.sol";
import {DisputeGameFactory} from "src/dispute/DisputeGameFactory.sol";
import {OPSuccinctFaultDisputeGame} from "../../src/fp/OPSuccinctFaultDisputeGame.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";
import {Transactor} from "@optimism/src/periphery/Transactor.sol";

// Utils
import {Utils} from "../../test/helpers/Utils.sol";
import {SP1Verifier as SP1VerifierPlonk} from "../../lib/sp1-contracts/contracts/src/v5.0.0/SP1VerifierPlonk.sol";
import {SP1Verifier as SP1VerifierGroth16} from "../../lib/sp1-contracts/contracts/src/v5.0.0/SP1VerifierGroth16.sol";
import {SP1VerifierGateway} from "../../lib/sp1-contracts/contracts/src/SP1VerifierGateway.sol";

contract DeployOPSuccinctLite is Script, Utils {
    function run()
        public
        returns (
            address gameImplementation,
            address sp1Verifier,
            address accessManager
        )
    {
        vm.startBroadcast();

        // Load configuration from JSON file (priority)
        FDGConfig memory config = readFDGJson("opsuccinctfdgconfig.json");

        // Read required contract addresses from environment variables (not in JSON)
        address factoryAddress = vm.envAddress("FACTORY_ADDRESS");
        address registryAddress = vm.envAddress("ANCHOR_STATE_REGISTRY");

        // Step 4: Deploy AccessManager and configure it
        AccessManager accessManagerContract = deployAccessManager(
            config.fallbackTimeoutFpSecs,
            factoryAddress,
            config.permissionlessMode,
            config.proposerAddresses,
            config.challengerAddresses
        );

        // Step 5: Deploy or get SP1 verifier
        SP1Config memory sp1Config = deploySP1Verifier(
            config.useSp1MockVerifier,
            config.rollupConfigHash,
            config.aggregationVkey,
            config.rangeVkeyCommitment
        );

        // Step 6: Deploy OPSuccinctFaultDisputeGame implementation
        OPSuccinctFaultDisputeGame gameImpl = deployGameImplementation(
            config.maxChallengeDuration,
            config.maxProveDuration,
            DisputeGameFactory(factoryAddress),
            sp1Config,
            IAnchorStateRegistry(registryAddress),
            accessManagerContract,
            config.challengerBondWei
        );

        console.log("Game implementation deployed at:", address(gameImpl));

        // Step 7: Configure factory with initial bond and game implementation
        configureFactory(factoryAddress, config.gameType, config.initialBondWei, address(gameImpl));



        vm.stopBroadcast();

        return (
            address(gameImpl),
            sp1Config.verifierAddress,
            address(accessManagerContract)
        );
    }

    function configureFactory(
        address factoryAddress,
        uint32 gameTypeValue,
        uint256 initialBondWei,
        address gameImplAddress
    ) internal {
        // Note: Factory owner is a Transactor contract, we need to use CALL through it
        address transactorAddress = vm.envAddress("TRANSACTOR");
        
        Transactor transactor = Transactor(transactorAddress);
        GameType gameType = GameType.wrap(gameTypeValue);
        
        // Call setInitBond through Transactor's CALL
        // CALL executes Factory code with msg.sender = Transactor address
        bytes memory setInitBondData = abi.encodeWithSelector(
            DisputeGameFactory.setInitBond.selector,
            gameType,
            initialBondWei
        );
        
        // Note: Transactor.CALL will revert if the call fails, so we use try-catch
        // to get better error messages
        try transactor.CALL(factoryAddress, setInitBondData, 0) returns (bool success1, bytes memory) {
            require(success1, "Transactor.CALL returned false for setInitBond");
        } catch Error(string memory reason) {
            revert(string.concat("Failed to set initial bond via Transactor: ", reason));
        } catch (bytes memory) {
            revert("Failed to set initial bond via Transactor: low-level call reverted");
        }
        
        // Call setImplementation through Transactor's CALL
        // Use explicit signature for overloaded function (setImplementation has multiple overloads)
        bytes memory setImplementationData = abi.encodeWithSignature(
            "setImplementation(uint32,address)",
            GameType.unwrap(gameType),
            gameImplAddress
        );
        
        try transactor.CALL(factoryAddress, setImplementationData, 0) returns (bool success2, bytes memory) {
            require(success2, "Transactor.CALL returned false for setImplementation");
        } catch Error(string memory reason) {
            revert(string.concat("Failed to set implementation via Transactor: ", reason));
        } catch (bytes memory) {
            revert("Failed to set implementation via Transactor: low-level call reverted");
        }

        console.log("Factory configured with game type:", uint256(gameTypeValue));
    }

    function deployAccessManager(
        uint256 fallbackTimeoutFpSecs,
        address factoryAddress,
        bool permissionlessMode,
        address[] memory proposerAddresses,
        address[] memory challengerAddresses
    ) internal returns (AccessManager) {
        // Deploy the access manager contract
        AccessManager accessManager = new AccessManager(
            fallbackTimeoutFpSecs,
            IDisputeGameFactory(factoryAddress)
        );
        console.log("Access manager deployed at:", address(accessManager));
        console.log("Permissionless fallback timeout (seconds):", fallbackTimeoutFpSecs);

        // Configure access control based on mode
        if (permissionlessMode) {
            // Set to permissionless games (anyone can propose and challenge)
            accessManager.setProposer(address(0), true);
            accessManager.setChallenger(address(0), true);
            console.log("Access Manager configured for permissionless mode");
        } else {
            // Set proposers from JSON config
            for (uint256 i = 0; i < proposerAddresses.length; i++) {
                if (proposerAddresses[i] != address(0)) {
                    accessManager.setProposer(proposerAddresses[i], true);
                    console.log("Added proposer:", proposerAddresses[i]);
                }
            }

            // Set challengers from JSON config
            for (uint256 i = 0; i < challengerAddresses.length; i++) {
                if (challengerAddresses[i] != address(0)) {
                    accessManager.setChallenger(challengerAddresses[i], true);
                    console.log("Added challenger:", challengerAddresses[i]);
                }
            }
        }

        return accessManager;
    }

    function deploySP1Verifier(
        bool useSp1MockVerifier,
        bytes32 rollupConfigHash,
        bytes32 aggregationVkey,
        bytes32 rangeVkeyCommitment
    ) internal returns (SP1Config memory) {
        SP1Config memory sp1Config;
        sp1Config.rollupConfigHash = rollupConfigHash;
        sp1Config.aggregationVkey = aggregationVkey;
        sp1Config.rangeVkeyCommitment = rangeVkeyCommitment;

        if (useSp1MockVerifier) {
            // Deploy mock verifier for testing
            SP1MockVerifier sp1Verifier = new SP1MockVerifier();
            sp1Config.verifierAddress = address(sp1Verifier);
            console.log("Using SP1 Mock Verifier:", address(sp1Verifier));
        } else {
            SP1VerifierPlonk sp1VerifierPlonk = new SP1VerifierPlonk();
            SP1VerifierGroth16 sp1VerifierGroth16 = new SP1VerifierGroth16();
            // Deploy gateway with current transaction sender as owner
            SP1VerifierGateway sp1VerifierGateway = new SP1VerifierGateway(tx.origin);
            sp1VerifierGateway.addRoute(address(sp1VerifierPlonk));
            sp1VerifierGateway.addRoute(address(sp1VerifierGroth16));
            sp1Config.verifierAddress = address(sp1VerifierGateway);
            console.log("Using SP1 Verifier Gateway:", address(sp1VerifierGateway));
        }

        return sp1Config;
    }

    function deployGameImplementation(
        uint256 maxChallengeDuration,
        uint256 maxProveDuration,
        DisputeGameFactory factory,
        SP1Config memory sp1Config,
        IAnchorStateRegistry registry,
        AccessManager accessManager,
        uint256 challengerBondWei
    ) internal returns (OPSuccinctFaultDisputeGame) {
        return new OPSuccinctFaultDisputeGame(
            Duration.wrap(uint64(maxChallengeDuration)),
            Duration.wrap(uint64(maxProveDuration)),
            IDisputeGameFactory(address(factory)),
            ISP1Verifier(sp1Config.verifierAddress),
            sp1Config.rollupConfigHash,
            sp1Config.aggregationVkey,
            sp1Config.rangeVkeyCommitment,
            challengerBondWei,
            registry,
            accessManager
        );
    }
}
