// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

// Libraries
import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Duration} from "src/dispute/lib/Types.sol";

// Interfaces
import {IDisputeGameFactory} from "interfaces/dispute/IDisputeGameFactory.sol";
import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";
import {IAnchorStateRegistry} from "interfaces/dispute/IAnchorStateRegistry.sol";

// Contracts
import {OPSuccinctFaultDisputeGame} from "../../src/fp/OPSuccinctFaultDisputeGame.sol";
import {AccessManager} from "src/fp/AccessManager.sol";
import {AccessManagerFactory} from "./deterministic-deployment/AccessManagerFactory.sol";
import {Create3Deployer} from "./deterministic-deployment/Create3Deployer.sol";

contract DeployDeterministically is Script {
    struct ExpectedAddresses {
        address create3Deployer;
        address accessManagerFactory;
        address accessManager;
        address disputeGame;
    }

    struct AccessManagerParams {
        uint256 fallbackTimeout;
        address factory;
        bool permissionlessMode;
        address[] proposerAddresses;
        address[] challengerAddresses;
        bytes32 salt;
    }

    struct DisputeGameParams {
        uint64 maxChallengeDuration;
        uint64 maxProveDuration;
        address factory;
        address verifierAddress;
        bytes32 rollupConfigHash;
        bytes32 aggregationVkey;
        bytes32 rangeVkeyCommitment;
        uint256 challengerBondWei;
        address registry;
        bytes32 salt;
    }

    event ExpectedAddressesSet(ExpectedAddresses expected);
    event AccessManagerParamsSet(AccessManagerParams params);
    event DisputeGameParamsSet(DisputeGameParams params);

    error AddressTaken(address addr);
    error AddressMismatch(address expected, address actual);
    error AddressNotExpected(address expected, address actual);
    error IncorrectOwner(address expected, address actual);

    Create3Deployer internal create3Deployer;

    function run() public {
        // Default proposer and challenger addresses
        address[] memory defaultProposers_ = new address[](1);
        defaultProposers_[0] = 0x9D17db7073Ea468AD6B96C34a4D04f1745eC7DdE;
        address[] memory defaultChallengers_ = new address[](1);
        defaultChallengers_[0] = 0x5e6450D449A90ECd9D07A01B2c97ADC6C8771A41;

        // Load expected addresses from env
        ExpectedAddresses memory expected_ = ExpectedAddresses({
            create3Deployer: vm.envOr("CREATE3_DEPLOYER", address(0x0)),
            accessManagerFactory: vm.envOr("ACCESS_MANAGER_FACTORY", address(0x0)),
            accessManager: vm.envOr("ACCESS_MANAGER", address(0x0)),
            disputeGame: vm.envOr("DISPUTE_GAME", address(0x0))
        });
        emit ExpectedAddressesSet(expected_);

        // Load create3 deployer salt from env
        bytes32 create3Salt_ = vm.envOr("CREATE3_SALT", bytes32(hex"ce12"));

        // Load access manager params from env
        AccessManagerParams memory managerParams_ = AccessManagerParams({
            fallbackTimeout: vm.envOr("FALLBACK_TIMEOUT", uint256(1209600)),
            factory: vm.envOr("FACTORY", address(0x57C45d82D1a995F1e135B8D7EDc0a6BB5211cfAA)),
            permissionlessMode: vm.envOr("PERMISSIONLESS_MODE", false),
            proposerAddresses: vm.envOr("PROPOSER_ADDRESSES", ",", defaultProposers_),
            challengerAddresses: vm.envOr("CHALLENGER_ADDRESSES", ",", defaultChallengers_),
            salt: vm.envOr("ACCESS_MANAGER_SALT", bytes32(hex"ce11"))
        });
        emit AccessManagerParamsSet(managerParams_);

        // Load dispute game params from env
        DisputeGameParams memory gameParams_ = DisputeGameParams({
            maxChallengeDuration: uint64(vm.envOr("MAX_CHALLENGE_DURATION", uint256(302400))),
            maxProveDuration: uint64(vm.envOr("MAX_PROVE_DURATION", uint256(86400))),
            factory: vm.envOr("FACTORY", address(0x57C45d82D1a995F1e135B8D7EDc0a6BB5211cfAA)),
            verifierAddress: vm.envOr("SP1_VERIFIER", address(0x3B6041173B80E77f038f3F2C0f9744f04837185e)),
            rollupConfigHash: vm.envOr(
                "ROLLUP_CONFIG_HASH", bytes32(0x0a8ce4334536ad2360bc97a487be5d25cc2f2d82dc7dc5c677dcd2d5bf8a1abc)
            ),
            aggregationVkey: vm.envOr(
                "AGGREGATION_VKEY", bytes32(0x00b121c37fbdfaf7a60941b02452b64f98e40f8a34a269c110598cf18237f738)
            ),
            rangeVkeyCommitment: vm.envOr(
                "RANGE_VKEY", bytes32(0x6ce13b162434b1c614e56b763ba0c7491145a09d6c0c945c47c5e99c7408d44a)
            ),
            challengerBondWei: vm.envOr("CHALLENGER_BOND_WEI", uint256(1e15)),
            registry: vm.envOr("REGISTRY", address(0xD73BA8168A61F3E917F0930D5C0401aA47e269D6)),
            salt: vm.envOr("DISPUTE_GAME_SALT", bytes32(hex"ce10"))
        });
        emit DisputeGameParamsSet(gameParams_);

        // Deterministically deploy contracts & optionally verify their addresses
        vm.startBroadcast();
        newCreate3Deployer(create3Salt_);
        if (expected_.create3Deployer != address(0) && address(create3Deployer) != expected_.create3Deployer) {
            revert AddressNotExpected(expected_.create3Deployer, address(create3Deployer));
        }
        (address amFactory_, AccessManager accessManager_) = newAccessManager(managerParams_);
        if (expected_.accessManagerFactory != address(0) && amFactory_ != expected_.accessManagerFactory) {
            revert AddressNotExpected(expected_.accessManagerFactory, amFactory_);
        }
        if (expected_.accessManager != address(0) && address(accessManager_) != expected_.accessManager) {
            revert AddressNotExpected(expected_.accessManager, address(accessManager_));
        }
        address game_ = newOpSuccinctDisputeGame(gameParams_, accessManager_);
        if (expected_.disputeGame != address(0) && game_ != expected_.disputeGame) {
            revert AddressNotExpected(expected_.disputeGame, game_);
        }
        vm.stopBroadcast();

        console.log("Deterministic deployment completed successfully");
    }

    function newCreate3Deployer(bytes32 _salt) internal {
        // Compute the deterministic address
        address address_ =
            vm.computeCreate2Address(_salt, keccak256(abi.encodePacked(type(Create3Deployer).creationCode)));
        console.log("Predicted Create3Deployer address:", address_);

        // Check if address is already taken
        if (address_.code.length > 0) revert AddressTaken(address_);

        // Deterministically deploy the Create3Deployer
        create3Deployer = new Create3Deployer{salt: _salt}();
        console.log("Create3Deployer deployed at:", address(create3Deployer));
        if (address(create3Deployer) != address_) {
            revert AddressMismatch(address_, address(create3Deployer));
        }
    }

    function newAccessManager(AccessManagerParams memory _params)
        internal
        returns (address amFactory_, AccessManager accessManager_)
    {
        // Compute the deterministic address of the AccessManagerFactory
        address amFactoryAddress_ = vm.computeCreate2Address(
            _params.salt, keccak256(abi.encodePacked(type(AccessManagerFactory).creationCode))
        );
        console.log("Predicted AccessManagerFactory address:", amFactoryAddress_);

        // Check if address is already taken
        if (amFactoryAddress_.code.length > 0) revert AddressTaken(amFactoryAddress_);

        // Deterministically deploy the access manager factory
        amFactory_ = address(new AccessManagerFactory{salt: _params.salt}());
        if (amFactoryAddress_ != amFactory_) revert AddressMismatch(amFactoryAddress_, address(amFactory_));
        console.log("Access manager factory deployed at:", amFactory_);

        // Compute the deterministic address
        address address_ = vm.computeCreate2Address(
            _params.salt,
            keccak256(
                abi.encodePacked(
                    type(AccessManager).creationCode,
                    abi.encode(_params.fallbackTimeout, IDisputeGameFactory(_params.factory))
                )
            ),
            amFactoryAddress_
        );
        console.log("Predicted access manager address:", address_);

        // Check if address is already taken
        if (address_.code.length > 0) revert AddressTaken(address_);

        // Deterministically deploy the access manager
        accessManager_ = AccessManagerFactory(amFactory_)
            .createAccessManager(
                _params.fallbackTimeout, IDisputeGameFactory(_params.factory), msg.sender, _params.salt
            );
        if (address(accessManager_) != address_) revert AddressMismatch(address_, address(accessManager_));
        console.log("Access manager:", address(accessManager_));

        // Verify owner
        address owner_ = accessManager_.owner();
        console.log("Access Manager owner:", owner_);
        if (owner_ != msg.sender) {
            revert IncorrectOwner(msg.sender, owner_);
        }

        if (_params.permissionlessMode) {
            // Set to permissionless games (anyone can propose and challenge).
            accessManager_.setProposer(address(0), true);
            accessManager_.setChallenger(address(0), true);
            console.log("Access Manager configured for permissionless mode");
        } else {
            // Set proposers.
            for (uint256 i = 0; i < _params.proposerAddresses.length; i++) {
                if (_params.proposerAddresses[i] != address(0)) {
                    accessManager_.setProposer(_params.proposerAddresses[i], true);
                    console.log("Added proposer:", _params.proposerAddresses[i]);
                }
            }
            // Set challengers.
            for (uint256 i = 0; i < _params.challengerAddresses.length; i++) {
                if (_params.challengerAddresses[i] != address(0)) {
                    accessManager_.setChallenger(_params.challengerAddresses[i], true);
                    console.log("Added challenger:", _params.challengerAddresses[i]);
                }
            }
            console.log("Access Manager configured for permissioned mode");
        }
    }

    function newOpSuccinctDisputeGame(DisputeGameParams memory _params, AccessManager _accessManager)
        internal
        returns (address game_)
    {
        // Compute the deterministic address
        address address_ = create3Deployer.addressOf(_params.salt);
        console.log("Predicted dispute game address:", address_);

        // Check if address is already taken
        if (address_.code.length > 0) revert AddressTaken(address_);

        // Deterministically deploy the dispute game
        game_ = create3Deployer.create3(
            _params.salt,
            abi.encodePacked(
                type(OPSuccinctFaultDisputeGame).creationCode,
                abi.encode(
                    Duration.wrap(_params.maxChallengeDuration),
                    Duration.wrap(_params.maxProveDuration),
                    IDisputeGameFactory(_params.factory),
                    ISP1Verifier(_params.verifierAddress),
                    _params.rollupConfigHash,
                    _params.aggregationVkey,
                    _params.rangeVkeyCommitment,
                    _params.challengerBondWei,
                    IAnchorStateRegistry(_params.registry),
                    _accessManager
                )
            )
        );
        if (game_ != address_) revert AddressMismatch(address_, game_);
        console.log("Dispute game deployed at:", game_);
    }
}
