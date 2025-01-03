// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console} from "forge-std/Test.sol";
import {Utils} from "./helpers/Utils.sol";
import {OPSuccinctL2OutputOracle} from "../src/OPSuccinctL2OutputOracle.sol";
import {OPSuccinctDisputeGame} from "../src/OPSuccinctDisputeGame.sol";
import {IDisputeGame} from "@optimism/src/dispute/interfaces/IDisputeGame.sol";
import { LibCWIA } from "@solady/utils/legacy/LibCWIA.sol";

contract OPSuccinctL2OutputOracleTest is Test, Utils {
    using LibCWIA for address;

    // Example proof data for the BoB testnet. Tx: https://sepolia.etherscan.io/tx/0x3910121f57c2e81ac98f5154eba7a2845f7ed27caf57a73e516ca606ad9d9aab
    uint256 checkpointedL1BlockNum = 6931062;
    bytes32 claimedOutputRoot = 0xf5ef905ba2c0e598c2f5274177700f3dfe37f66db15e8957e63d0732b0e611b8;
    uint256 claimedL2BlockNum = 3677705;
    bytes proof =
        hex"91ff06f303ed1bf4b5dbf52b2dd7201cb9675afd59200464ef55cff01d113ca54d96b52c2689d0a64c90eb674d1cb9119e4f4fde54d9414d056112df7bf01066b86ee5e410d4d6a93c26c287e1c010bf03fcc0ebfaa6ae294650bba1bf177271c96911771624e73cf6192e3f1a5ac0bd7943f5921df5c22e1c2661a40c33a40b70e9f8d6164ab1e3e1abd666c19aae2012ec389a295e9ce148f781a81363685da83b32390785840f77691e93d734863d283a05497f8a8621dd1dc5e410b6bef0ed9ce53422a8b41ebdbc7e82202fafa1dd5a0fcc458932f76390f9d1f1fbf4134cf68dec06bf5b5b1c0cde47bd89198a52e7b92c634da6dadcf59efa6b78d51273e3316d";

    // The owner of the L2OO.
    address PROPOSER = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;

    OPSuccinctL2OutputOracle l2oo;
    OPSuccinctDisputeGame game;

    function setUp() public {
        // Note: L1_RPC should be a valid Sepolia RPC.
        vm.createSelectFork(vm.envString("L1_RPC"), checkpointedL1BlockNum + 1);
    }

    // Test the DisputeGame contract.
    function testOPSuccinctDisputeGame() public {
        l2oo = new OPSuccinctL2OutputOracle();
        game = new OPSuccinctDisputeGame(address(l2oo));

        l2oo.initializeParams(
            OPSuccinctL2OutputOracle.InitParams({
                challenger: address(0),
                proposer: PROPOSER,
                owner: PROPOSER,
                finalizationPeriodSeconds: 0,
                l2BlockTime: 2,
                aggregationVkey: 0x002d397eaa6f2bd3a873f2b996a6d486eb20774092e68a75471e287084180c13,
                rangeVkeyCommitment: 0x3237870c3fe7a735661b52f641bd41c85a886c916a962526533c8c9d17dc0831,
                rollupConfigHash: 0x8dad3aa88de72762859feb6781d937efa8f39c8b681a51443b0abfde108fbbcd,
                startingOutputRoot: bytes32(0),
                startingBlockNumber: 3677700,
                startingTimestamp: 1729707888,
                submissionInterval: 5,
                verifier: 0x3B6041173B80E77f038f3F2C0f9744f04837185e
            })
        );

        l2oo.checkpointBlockHash(checkpointedL1BlockNum);
        vm.prank(PROPOSER);

        IDisputeGame proxy = IDisputeGame(address(game).clone(
            abi.encodePacked(
                msg.sender,
                claimedOutputRoot,
                bytes32(0), // TODO: This should be parentHash
                abi.encode(claimedL2BlockNum, checkpointedL1BlockNum, proof)
            )
        ));

        proxy.initialize{ value: 10 }();
    }
}
