// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package bindings

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// OPSuccinctL2OutputOracleInitParams is an auto generated low-level Go binding around an user-defined struct.
type OPSuccinctL2OutputOracleInitParams struct {
	Challenger                common.Address
	Proposer                  common.Address
	Owner                     common.Address
	FinalizationPeriodSeconds *big.Int
	L2BlockTime               *big.Int
	AggregationVkey           [32]byte
	RangeVkeyCommitment       [32]byte
	RollupConfigHash          [32]byte
	StartingOutputRoot        [32]byte
	StartingBlockNumber       *big.Int
	StartingTimestamp         *big.Int
	SubmissionInterval        *big.Int
	Verifier                  common.Address
}

// TypesOutputProposal is an auto generated low-level Go binding around an user-defined struct.
type TypesOutputProposal struct {
	OutputRoot    [32]byte
	Timestamp     *big.Int
	L2BlockNumber *big.Int
}

// OPSuccinctL2OutputOracleMetaData contains all meta data concerning the OPSuccinctL2OutputOracle contract.
var OPSuccinctL2OutputOracleMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"CHALLENGER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"FINALIZATION_PERIOD_SECONDS\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"L2_BLOCK_TIME\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"PROPOSER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"SUBMISSION_INTERVAL\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"addProposer\",\"inputs\":[{\"name\":\"_proposer\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"aggregationVkey\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"approvedProposers\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challenger\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkpointBlockHash\",\"inputs\":[{\"name\":\"_blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"computeL2Timestamp\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"deleteL2Outputs\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"disableOptimisticMode\",\"inputs\":[{\"name\":\"_finalizationPeriodSeconds\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"enableOptimisticMode\",\"inputs\":[{\"name\":\"_finalizationPeriodSeconds\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"finalizationPeriodSeconds\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2Output\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputIndexAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"historicBlockHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_initParams\",\"type\":\"tuple\",\"internalType\":\"structOPSuccinctL2OutputOracle.InitParams\",\"components\":[{\"name\":\"challenger\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"proposer\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"finalizationPeriodSeconds\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"l2BlockTime\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"startingOutputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"startingBlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"startingTimestamp\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"submissionInterval\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"verifier\",\"type\":\"address\",\"internalType\":\"address\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"initializerVersion\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"l2BlockTime\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"optimisticMode\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"proposeL2Output\",\"inputs\":[{\"name\":\"_outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l1BlockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_l1BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"proposeL2Output\",\"inputs\":[{\"name\":\"_outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l1BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"proposer\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rangeVkeyCommitment\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"removeProposer\",\"inputs\":[{\"name\":\"_proposer\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rollupConfigHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"submissionInterval\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateAggregationVkey\",\"inputs\":[{\"name\":\"_aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRangeVkeyCommitment\",\"inputs\":[{\"name\":\"_rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRollupConfigHash\",\"inputs\":[{\"name\":\"_rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateSubmissionInterval\",\"inputs\":[{\"name\":\"_submissionInterval\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateVerifier\",\"inputs\":[{\"name\":\"_verifier\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifier\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"version\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\",\"internalType\":\"string\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"AggregationVkeyUpdated\",\"inputs\":[{\"name\":\"oldAggregationVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newAggregationVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OptimisticModeToggled\",\"inputs\":[{\"name\":\"enabled\",\"type\":\"bool\",\"indexed\":true,\"internalType\":\"bool\"},{\"name\":\"finalizationPeriodSeconds\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputProposed\",\"inputs\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"l2OutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l1Timestamp\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputsDeleted\",\"inputs\":[{\"name\":\"prevNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"newNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"ProposerUpdated\",\"inputs\":[{\"name\":\"proposer\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"added\",\"type\":\"bool\",\"indexed\":false,\"internalType\":\"bool\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RangeVkeyCommitmentUpdated\",\"inputs\":[{\"name\":\"oldRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"RollupConfigHashUpdated\",\"inputs\":[{\"name\":\"oldRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"SubmissionIntervalUpdated\",\"inputs\":[{\"name\":\"oldSubmissionInterval\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"},{\"name\":\"newSubmissionInterval\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"VerifierUpdated\",\"inputs\":[{\"name\":\"oldVerifier\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newVerifier\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"L1BlockHashNotAvailable\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L1BlockHashNotCheckpointed\",\"inputs\":[]}]",
	Bin: "0x60e060405234801561001057600080fd5b506002608052600060a081905260c05261002861002d565b6100ed565b600054610100900460ff16156100995760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156100eb576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b60805160a05160c0516124016200011d6000396000610b4101526000610b1801526000610aef01526124016000f3fe6080604052600436106102715760003560e01c806389c44cbb1161014f578063bffa7f0f116100c1578063d46512761161007a578063d465127614610737578063db1470f514610767578063dcec334814610787578063e1a41bcf1461079c578063f2fde38b146107b2578063f4daa291146107d257600080fd5b8063bffa7f0f1461068d578063c32e4e3e146106ab578063c4cb03ec146106c1578063ce5db8d6146106e1578063cf8e5cf0146106f7578063d1de856c1461071757600080fd5b80639ad84880116101135780639ad848801461059a578063a196b525146105ad578063a25ae557146105da578063a8e4fb901461062d578063b03cd4181461064d578063bc91ce331461066d57600080fd5b806389c44cbb146105115780638da5cb5b1461053157806393991af31461055157806397fc007c146105675780639aaab6481461058757600080fd5b8063534db0e2116101e85780636b4d98dd116101ac5780636b4d98dd1461046a5780636d9a1c8b1461048857806370872aa51461049e5780637f006420146104b45780637f01ea68146104d457806388786272146104fb57600080fd5b8063534db0e2146103d457806354fd4d50146103f457806360caf7a01461041657806369f16eec146104405780636abcf5631461045557600080fd5b80632b7ac3f31161023a5780632b7ac3f3146103125780632c6979611461034a578063336c9e811461036a5780634599c7881461038a5780634ab309ac1461039f578063529933df146103bf57600080fd5b80622134cc1461027657806309d632d31461029a5780631bdd450c146102bc5780631e856800146102dc5780632b31841e146102fc575b600080fd5b34801561028257600080fd5b506005545b6040519081526020015b60405180910390f35b3480156102a657600080fd5b506102ba6102b5366004611dba565b6107e7565b005b3480156102c857600080fd5b506102ba6102d7366004611ddc565b610870565b3480156102e857600080fd5b506102ba6102f7366004611ddc565b6108ce565b34801561030857600080fd5b50610287600a5481565b34801561031e57600080fd5b50600b54610332906001600160a01b031681565b6040516001600160a01b039091168152602001610291565b34801561035657600080fd5b506102ba610365366004611ddc565b610900565b34801561037657600080fd5b506102ba610385366004611ddc565b610992565b34801561039657600080fd5b506102876109fd565b3480156103ab57600080fd5b506102ba6103ba366004611ddc565b610a5a565b3480156103cb57600080fd5b50600454610287565b3480156103e057600080fd5b50600654610332906001600160a01b031681565b34801561040057600080fd5b50610409610ae8565b6040516102919190611e51565b34801561042257600080fd5b506010546104309060ff1681565b6040519015158152602001610291565b34801561044c57600080fd5b50610287610b8b565b34801561046157600080fd5b50600354610287565b34801561047657600080fd5b506006546001600160a01b0316610332565b34801561049457600080fd5b50610287600c5481565b3480156104aa57600080fd5b5061028760015481565b3480156104c057600080fd5b506102876104cf366004611ddc565b610b9d565b3480156104e057600080fd5b506104e9600281565b60405160ff9091168152602001610291565b34801561050757600080fd5b5061028760025481565b34801561051d57600080fd5b506102ba61052c366004611ddc565b610d3b565b34801561053d57600080fd5b50600d54610332906001600160a01b031681565b34801561055d57600080fd5b5061028760055481565b34801561057357600080fd5b506102ba610582366004611dba565b610f40565b6102ba610595366004611e64565b610fc6565b6102ba6105a8366004611f07565b61128d565b3480156105b957600080fd5b506102876105c8366004611ddc565b600f6020526000908152604090205481565b3480156105e657600080fd5b506105fa6105f5366004611ddc565b611627565b60408051825181526020808401516001600160801b03908116918301919091529282015190921690820152606001610291565b34801561063957600080fd5b50600754610332906001600160a01b031681565b34801561065957600080fd5b506102ba610668366004611dba565b6116a5565b34801561067957600080fd5b506102ba610688366004611ddc565b611726565b34801561069957600080fd5b506007546001600160a01b0316610332565b3480156106b757600080fd5b5061028760095481565b3480156106cd57600080fd5b506102ba6106dc366004611ddc565b611784565b3480156106ed57600080fd5b5061028760085481565b34801561070357600080fd5b506105fa610712366004611ddc565b6117e2565b34801561072357600080fd5b50610287610732366004611ddc565b61181a565b34801561074357600080fd5b50610430610752366004611dba565b600e6020526000908152604090205460ff1681565b34801561077357600080fd5b506102ba610782366004611fb9565b61184a565b34801561079357600080fd5b50610287611bf8565b3480156107a857600080fd5b5061028760045481565b3480156107be57600080fd5b506102ba6107cd366004611dba565b611c0f565b3480156107de57600080fd5b50600854610287565b600d546001600160a01b0316331461081a5760405162461bcd60e51b81526004016108119061207c565b60405180910390fd5b6001600160a01b0381166000818152600e60209081526040808320805460ff19169055519182527f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a91015b60405180910390a250565b600d546001600160a01b0316331461089a5760405162461bcd60e51b81526004016108119061207c565b600c546040518291907f5d9ebe9f09b0810b3546b30781ba9a51092b37dd6abada4b830ce54a41ac6a4b90600090a3600c55565b8040806108ee576040516321301a1960e21b815260040160405180910390fd5b6000918252600f602052604090912055565b600d546001600160a01b0316331461092a5760405162461bcd60e51b81526004016108119061207c565b60105460ff161561094d5760405162461bcd60e51b8152600401610811906120c3565b60088190556010805460ff191660019081179091556040518281527f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a90602001610865565b600d546001600160a01b031633146109bc5760405162461bcd60e51b81526004016108119061207c565b60045460408051918252602082018390527fc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b2910160405180910390a1600455565b60035460009015610a515760038054610a1890600190612123565b81548110610a2857610a2861213a565b6000918252602090912060029091020160010154600160801b90046001600160801b0316919050565b6001545b905090565b600d546001600160a01b03163314610a845760405162461bcd60e51b81526004016108119061207c565b60105460ff16610aa65760405162461bcd60e51b815260040161081190612150565b60088190556010805460ff191690556040518181526000907f1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a90602001610865565b6060610b137f0000000000000000000000000000000000000000000000000000000000000000611c95565b610b3c7f0000000000000000000000000000000000000000000000000000000000000000611c95565b610b657f0000000000000000000000000000000000000000000000000000000000000000611c95565b604051602001610b779392919061219e565b604051602081830303815290604052905090565b600354600090610a5590600190612123565b6000610ba76109fd565b821115610c2d5760405162461bcd60e51b815260206004820152604860248201527f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560448201527f7420666f72206120626c6f636b207468617420686173206e6f74206265656e206064820152671c1c9bdc1bdcd95960c21b608482015260a401610811565b600354610cb15760405162461bcd60e51b815260206004820152604660248201527f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560448201527f74206173206e6f206f7574707574732068617665206265656e2070726f706f736064820152651959081e595d60d21b608482015260a401610811565b6003546000905b80821015610d345760006002610cce83856121f8565b610cd89190612226565b90508460038281548110610cee57610cee61213a565b6000918252602090912060029091020160010154600160801b90046001600160801b03161015610d2a57610d238160016121f8565b9250610d2e565b8091505b50610cb8565b5092915050565b6006546001600160a01b03163314610dbb5760405162461bcd60e51b815260206004820152603e60248201527f4c324f75747075744f7261636c653a206f6e6c7920746865206368616c6c656e60448201527f67657220616464726573732063616e2064656c657465206f75747075747300006064820152608401610811565b6003548110610e3e5760405162461bcd60e51b815260206004820152604360248201527f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560448201527f747075747320616674657220746865206c6174657374206f757470757420696e6064820152620c8caf60eb1b608482015260a401610811565b60085460038281548110610e5457610e5461213a565b6000918252602090912060016002909202010154610e7b906001600160801b031642612123565b10610efd5760405162461bcd60e51b815260206004820152604660248201527f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560448201527f74707574732074686174206861766520616c7265616479206265656e2066696e606482015265185b1a5e995960d21b608482015260a401610811565b6000610f0860035490565b90508160035581817f4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b660405160405180910390a35050565b600d546001600160a01b03163314610f6a5760405162461bcd60e51b81526004016108119061207c565b600b546040516001600160a01b038084169216907f0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc90600090a3600b80546001600160a01b0319166001600160a01b0392909216919091179055565b60105460ff16610fe85760405162461bcd60e51b815260040161081190612150565b336000908152600e602052604090205460ff1680611030575060008052600e6020527fe710864318d4a32f37d6ce54cb3fadbef648dd12d8dbdf53973564d56b7f881c5460ff165b61104c5760405162461bcd60e51b81526004016108119061223a565b611054611bf8565b83146110d95760405162461bcd60e51b815260206004820152604860248201527f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360448201527f7420626520657175616c20746f206e65787420657870656374656420626c6f636064820152673590373ab6b132b960c11b608482015260a401610811565b426110e38461181a565b106111005760405162461bcd60e51b815260040161081190612297565b8361111d5760405162461bcd60e51b8152600401610811906122ed565b81156111ab57818140146111ab5760405162461bcd60e51b815260206004820152604960248201527f4c324f75747075744f7261636c653a20626c6f636b206861736820646f65732060448201527f6e6f74206d6174636820746865206861736820617420746865206578706563746064820152681959081a195a59da1d60ba1b608482015260a401610811565b826111b560035490565b857fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2426040516111e791815260200190565b60405180910390a45050604080516060810182529283526001600160801b034281166020850190815292811691840191825260038054600181018255600091909152935160029094027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810194909455915190518216600160801b029116177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c90910155565b60105460ff16156112b05760405162461bcd60e51b8152600401610811906120c3565b336000908152600e602052604090205460ff16806112f8575060008052600e6020527fe710864318d4a32f37d6ce54cb3fadbef648dd12d8dbdf53973564d56b7f881c5460ff165b6113145760405162461bcd60e51b81526004016108119061223a565b61131c611bf8565b8310156113b75760405162461bcd60e51b815260206004820152605860248201527f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360448201527f742062652067726561746572207468616e206f7220657175616c20746f206e6560648201527f787420657870656374656420626c6f636b206e756d6265720000000000000000608482015260a401610811565b426113c18461181a565b106113de5760405162461bcd60e51b815260040161081190612297565b836113fb5760405162461bcd60e51b8152600401610811906122ed565b6000828152600f60205260409020548061142857604051630455475360e31b815260040160405180910390fd5b60006040518060c001604052808381526020016003611445610b8b565b815481106114555761145561213a565b60009182526020918290206002909102015482528181018990526040808301899052600c54606080850191909152600a54608094850152600b5460095483518751818701529487015185850152928601518483015290850151838501529284015160a08084019190915284015160c08301529293506001600160a01b03909116916341493c609160e001604051602081830303815290604052866040518463ffffffff1660e01b815260040161150d9392919061234a565b60006040518083038186803b15801561152557600080fd5b505afa158015611539573d6000803e3d6000fd5b505050508461154760035490565b877fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e24260405161157991815260200190565b60405180910390a45050604080516060810182529485526001600160801b034281166020870190815294811691860191825260038054600181018255600091909152955160029096027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810196909655935190518416600160801b029316929092177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c909301929092555050565b6040805160608101825260008082526020820181905291810191909152600382815481106116575761165761213a565b600091825260209182902060408051606081018252600290930290910180548352600101546001600160801b0380821694840194909452600160801b90049092169181019190915292915050565b600d546001600160a01b031633146116cf5760405162461bcd60e51b81526004016108119061207c565b6001600160a01b0381166000818152600e6020908152604091829020805460ff1916600190811790915591519182527f5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a9101610865565b600d546001600160a01b031633146117505760405162461bcd60e51b81526004016108119061207c565b600a546040518291907fbf8cab6317796bfa97fea82b6d27c9542a08fa0821813cf2a57e7cff7fdc815690600090a3600a55565b600d546001600160a01b031633146117ae5760405162461bcd60e51b81526004016108119061207c565b6009546040518291907f390b73b2b067afcef04d30b573e4590c6e565519e370927dd777ca0ce8a55db090600090a3600955565b6040805160608101825260008082526020820181905291810191909152600361180a83610b9d565b815481106116575761165761213a565b60006005546001548361182d9190612123565b611837919061237f565b60025461184491906121f8565b92915050565b600054600290610100900460ff1615801561186c575060005460ff8083169116105b6118cf5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610811565b6000805461ffff191660ff83161761010017905561016082015161195b5760405162461bcd60e51b815260206004820152603a60248201527f4c324f75747075744f7261636c653a207375626d697373696f6e20696e74657260448201527f76616c206d7573742062652067726561746572207468616e20300000000000006064820152608401610811565b60008260800151116119cc5760405162461bcd60e51b815260206004820152603460248201527f4c324f75747075744f7261636c653a204c3220626c6f636b2074696d65206d75604482015273073742062652067726561746572207468616e20360641b6064820152608401610811565b428261014001511115611a555760405162461bcd60e51b8152602060048201526044602482018190527f4c324f75747075744f7261636c653a207374617274696e67204c322074696d65908201527f7374616d70206d757374206265206c657373207468616e2063757272656e742060648201526374696d6560e01b608482015260a401610811565b6101608201516004556080820151600555600354600003611b2c57604080516060810182526101008401518152610140840180516001600160801b03908116602084019081526101208701805183169585019586526003805460018181018355600092909252955160029687027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810191909155925196518416600160801b0296909316959095177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c909101559251909255905190555b8151600680546001600160a01b03199081166001600160a01b0393841617909155606084015160085560208085015183166000908152600e82526040808220805460ff1916600117905560a087015160095560c0870151600a55610180870151600b8054861691871691909117905560e0870151600c5580870151600d8054909516951694909417909255815461ff001916909155905160ff831681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498910160405180910390a15050565b6000600454611c056109fd565b610a5591906121f8565b600d546001600160a01b03163314611c395760405162461bcd60e51b81526004016108119061207c565b600d546040516001600160a01b038084169216907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a3600d80546001600160a01b0319166001600160a01b0392909216919091179055565b606081600003611cbc5750506040805180820190915260018152600360fc1b602082015290565b8160005b8115611ce65780611cd08161239e565b9150611cdf9050600a83612226565b9150611cc0565b60008167ffffffffffffffff811115611d0157611d01611e96565b6040519080825280601f01601f191660200182016040528015611d2b576020820181803683370190505b5090505b8415611d9657611d40600183612123565b9150611d4d600a866123b7565b611d589060306121f8565b60f81b818381518110611d6d57611d6d61213a565b60200101906001600160f81b031916908160001a905350611d8f600a86612226565b9450611d2f565b949350505050565b80356001600160a01b0381168114611db557600080fd5b919050565b600060208284031215611dcc57600080fd5b611dd582611d9e565b9392505050565b600060208284031215611dee57600080fd5b5035919050565b60005b83811015611e10578181015183820152602001611df8565b83811115611e1f576000848401525b50505050565b60008151808452611e3d816020860160208601611df5565b601f01601f19169290920160200192915050565b602081526000611dd56020830184611e25565b60008060008060808587031215611e7a57600080fd5b5050823594602084013594506040840135936060013592509050565b634e487b7160e01b600052604160045260246000fd5b6040516101a0810167ffffffffffffffff81118282101715611ed057611ed0611e96565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715611eff57611eff611e96565b604052919050565b60008060008060808587031215611f1d57600080fd5b84359350602080860135935060408601359250606086013567ffffffffffffffff80821115611f4b57600080fd5b818801915088601f830112611f5f57600080fd5b813581811115611f7157611f71611e96565b611f83601f8201601f19168501611ed6565b91508082528984828501011115611f9957600080fd5b808484018584013760008482840101525080935050505092959194509250565b60006101a08284031215611fcc57600080fd5b611fd4611eac565b611fdd83611d9e565b8152611feb60208401611d9e565b6020820152611ffc60408401611d9e565b6040820152606083013560608201526080830135608082015260a083013560a082015260c083013560c082015260e083013560e0820152610100808401358183015250610120808401358183015250610140808401358183015250610160808401358183015250610180612071818501611d9e565b908201529392505050565b60208082526027908201527f4c324f75747075744f7261636c653a2063616c6c6572206973206e6f74207468604082015266329037bbb732b960c91b606082015260800190565b6020808252602a908201527f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f6465206040820152691a5cc8195b98589b195960b21b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b6000828210156121355761213561210d565b500390565b634e487b7160e01b600052603260045260246000fd5b6020808252602e908201527f4c324f75747075744f7261636c653a206f7074696d6973746963206d6f64652060408201526d1a5cc81b9bdd08195b98589b195960921b606082015260800190565b600084516121b0818460208901611df5565b8083019050601760f91b80825285516121d0816001850160208a01611df5565b600192019182015283516121eb816002840160208801611df5565b0160020195945050505050565b6000821982111561220b5761220b61210d565b500190565b634e487b7160e01b600052601260045260246000fd5b60008261223557612235612210565b500490565b6020808252603f908201527f4c324f75747075744f7261636c653a206f6e6c7920617070726f76656420707260408201527f6f706f736572732063616e2070726f706f7365206e6577206f75747075747300606082015260800190565b60208082526036908201527f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60408201527532206f757470757420696e207468652066757475726560501b606082015260800190565b6020808252603a908201527f4c324f75747075744f7261636c653a204c32206f75747075742070726f706f7360408201527f616c2063616e6e6f7420626520746865207a65726f2068617368000000000000606082015260800190565b8381526060602082015260006123636060830185611e25565b82810360408401526123758185611e25565b9695505050505050565b60008160001904831182151516156123995761239961210d565b500290565b6000600182016123b0576123b061210d565b5060010190565b6000826123c6576123c6612210565b50069056fea2646970667358221220508c013c5d0421b92d259564fb51699bda00b9aa1482864349b367c6d7fe6e5264736f6c634300080f0033",
}

// OPSuccinctL2OutputOracleABI is the input ABI used to generate the binding from.
// Deprecated: Use OPSuccinctL2OutputOracleMetaData.ABI instead.
var OPSuccinctL2OutputOracleABI = OPSuccinctL2OutputOracleMetaData.ABI

// OPSuccinctL2OutputOracleBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use OPSuccinctL2OutputOracleMetaData.Bin instead.
var OPSuccinctL2OutputOracleBin = OPSuccinctL2OutputOracleMetaData.Bin

// DeployOPSuccinctL2OutputOracle deploys a new Ethereum contract, binding an instance of OPSuccinctL2OutputOracle to it.
func DeployOPSuccinctL2OutputOracle(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *OPSuccinctL2OutputOracle, error) {
	parsed, err := OPSuccinctL2OutputOracleMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(OPSuccinctL2OutputOracleBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &OPSuccinctL2OutputOracle{OPSuccinctL2OutputOracleCaller: OPSuccinctL2OutputOracleCaller{contract: contract}, OPSuccinctL2OutputOracleTransactor: OPSuccinctL2OutputOracleTransactor{contract: contract}, OPSuccinctL2OutputOracleFilterer: OPSuccinctL2OutputOracleFilterer{contract: contract}}, nil
}

// OPSuccinctL2OutputOracle is an auto generated Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracle struct {
	OPSuccinctL2OutputOracleCaller     // Read-only binding to the contract
	OPSuccinctL2OutputOracleTransactor // Write-only binding to the contract
	OPSuccinctL2OutputOracleFilterer   // Log filterer for contract events
}

// OPSuccinctL2OutputOracleCaller is an auto generated read-only Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracleCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// OPSuccinctL2OutputOracleTransactor is an auto generated write-only Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracleTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// OPSuccinctL2OutputOracleFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type OPSuccinctL2OutputOracleFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// OPSuccinctL2OutputOracleSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type OPSuccinctL2OutputOracleSession struct {
	Contract     *OPSuccinctL2OutputOracle // Generic contract binding to set the session for
	CallOpts     bind.CallOpts             // Call options to use throughout this session
	TransactOpts bind.TransactOpts         // Transaction auth options to use throughout this session
}

// OPSuccinctL2OutputOracleCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type OPSuccinctL2OutputOracleCallerSession struct {
	Contract *OPSuccinctL2OutputOracleCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                   // Call options to use throughout this session
}

// OPSuccinctL2OutputOracleTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type OPSuccinctL2OutputOracleTransactorSession struct {
	Contract     *OPSuccinctL2OutputOracleTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                   // Transaction auth options to use throughout this session
}

// OPSuccinctL2OutputOracleRaw is an auto generated low-level Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracleRaw struct {
	Contract *OPSuccinctL2OutputOracle // Generic contract binding to access the raw methods on
}

// OPSuccinctL2OutputOracleCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracleCallerRaw struct {
	Contract *OPSuccinctL2OutputOracleCaller // Generic read-only contract binding to access the raw methods on
}

// OPSuccinctL2OutputOracleTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type OPSuccinctL2OutputOracleTransactorRaw struct {
	Contract *OPSuccinctL2OutputOracleTransactor // Generic write-only contract binding to access the raw methods on
}

// NewOPSuccinctL2OutputOracle creates a new instance of OPSuccinctL2OutputOracle, bound to a specific deployed contract.
func NewOPSuccinctL2OutputOracle(address common.Address, backend bind.ContractBackend) (*OPSuccinctL2OutputOracle, error) {
	contract, err := bindOPSuccinctL2OutputOracle(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracle{OPSuccinctL2OutputOracleCaller: OPSuccinctL2OutputOracleCaller{contract: contract}, OPSuccinctL2OutputOracleTransactor: OPSuccinctL2OutputOracleTransactor{contract: contract}, OPSuccinctL2OutputOracleFilterer: OPSuccinctL2OutputOracleFilterer{contract: contract}}, nil
}

// NewOPSuccinctL2OutputOracleCaller creates a new read-only instance of OPSuccinctL2OutputOracle, bound to a specific deployed contract.
func NewOPSuccinctL2OutputOracleCaller(address common.Address, caller bind.ContractCaller) (*OPSuccinctL2OutputOracleCaller, error) {
	contract, err := bindOPSuccinctL2OutputOracle(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleCaller{contract: contract}, nil
}

// NewOPSuccinctL2OutputOracleTransactor creates a new write-only instance of OPSuccinctL2OutputOracle, bound to a specific deployed contract.
func NewOPSuccinctL2OutputOracleTransactor(address common.Address, transactor bind.ContractTransactor) (*OPSuccinctL2OutputOracleTransactor, error) {
	contract, err := bindOPSuccinctL2OutputOracle(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleTransactor{contract: contract}, nil
}

// NewOPSuccinctL2OutputOracleFilterer creates a new log filterer instance of OPSuccinctL2OutputOracle, bound to a specific deployed contract.
func NewOPSuccinctL2OutputOracleFilterer(address common.Address, filterer bind.ContractFilterer) (*OPSuccinctL2OutputOracleFilterer, error) {
	contract, err := bindOPSuccinctL2OutputOracle(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleFilterer{contract: contract}, nil
}

// bindOPSuccinctL2OutputOracle binds a generic wrapper to an already deployed contract.
func bindOPSuccinctL2OutputOracle(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := OPSuccinctL2OutputOracleMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _OPSuccinctL2OutputOracle.Contract.OPSuccinctL2OutputOracleCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.OPSuccinctL2OutputOracleTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.OPSuccinctL2OutputOracleTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _OPSuccinctL2OutputOracle.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.contract.Transact(opts, method, params...)
}

// CHALLENGER is a free data retrieval call binding the contract method 0x6b4d98dd.
//
// Solidity: function CHALLENGER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) CHALLENGER(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "CHALLENGER")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// CHALLENGER is a free data retrieval call binding the contract method 0x6b4d98dd.
//
// Solidity: function CHALLENGER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) CHALLENGER() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.CHALLENGER(&_OPSuccinctL2OutputOracle.CallOpts)
}

// CHALLENGER is a free data retrieval call binding the contract method 0x6b4d98dd.
//
// Solidity: function CHALLENGER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) CHALLENGER() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.CHALLENGER(&_OPSuccinctL2OutputOracle.CallOpts)
}

// FINALIZATIONPERIODSECONDS is a free data retrieval call binding the contract method 0xf4daa291.
//
// Solidity: function FINALIZATION_PERIOD_SECONDS() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) FINALIZATIONPERIODSECONDS(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "FINALIZATION_PERIOD_SECONDS")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// FINALIZATIONPERIODSECONDS is a free data retrieval call binding the contract method 0xf4daa291.
//
// Solidity: function FINALIZATION_PERIOD_SECONDS() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) FINALIZATIONPERIODSECONDS() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.FINALIZATIONPERIODSECONDS(&_OPSuccinctL2OutputOracle.CallOpts)
}

// FINALIZATIONPERIODSECONDS is a free data retrieval call binding the contract method 0xf4daa291.
//
// Solidity: function FINALIZATION_PERIOD_SECONDS() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) FINALIZATIONPERIODSECONDS() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.FINALIZATIONPERIODSECONDS(&_OPSuccinctL2OutputOracle.CallOpts)
}

// L2BLOCKTIME is a free data retrieval call binding the contract method 0x002134cc.
//
// Solidity: function L2_BLOCK_TIME() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) L2BLOCKTIME(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "L2_BLOCK_TIME")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// L2BLOCKTIME is a free data retrieval call binding the contract method 0x002134cc.
//
// Solidity: function L2_BLOCK_TIME() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) L2BLOCKTIME() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.L2BLOCKTIME(&_OPSuccinctL2OutputOracle.CallOpts)
}

// L2BLOCKTIME is a free data retrieval call binding the contract method 0x002134cc.
//
// Solidity: function L2_BLOCK_TIME() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) L2BLOCKTIME() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.L2BLOCKTIME(&_OPSuccinctL2OutputOracle.CallOpts)
}

// PROPOSER is a free data retrieval call binding the contract method 0xbffa7f0f.
//
// Solidity: function PROPOSER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) PROPOSER(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "PROPOSER")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PROPOSER is a free data retrieval call binding the contract method 0xbffa7f0f.
//
// Solidity: function PROPOSER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) PROPOSER() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.PROPOSER(&_OPSuccinctL2OutputOracle.CallOpts)
}

// PROPOSER is a free data retrieval call binding the contract method 0xbffa7f0f.
//
// Solidity: function PROPOSER() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) PROPOSER() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.PROPOSER(&_OPSuccinctL2OutputOracle.CallOpts)
}

// SUBMISSIONINTERVAL is a free data retrieval call binding the contract method 0x529933df.
//
// Solidity: function SUBMISSION_INTERVAL() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) SUBMISSIONINTERVAL(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "SUBMISSION_INTERVAL")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// SUBMISSIONINTERVAL is a free data retrieval call binding the contract method 0x529933df.
//
// Solidity: function SUBMISSION_INTERVAL() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) SUBMISSIONINTERVAL() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.SUBMISSIONINTERVAL(&_OPSuccinctL2OutputOracle.CallOpts)
}

// SUBMISSIONINTERVAL is a free data retrieval call binding the contract method 0x529933df.
//
// Solidity: function SUBMISSION_INTERVAL() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) SUBMISSIONINTERVAL() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.SUBMISSIONINTERVAL(&_OPSuccinctL2OutputOracle.CallOpts)
}

// AggregationVkey is a free data retrieval call binding the contract method 0xc32e4e3e.
//
// Solidity: function aggregationVkey() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) AggregationVkey(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "aggregationVkey")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// AggregationVkey is a free data retrieval call binding the contract method 0xc32e4e3e.
//
// Solidity: function aggregationVkey() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) AggregationVkey() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.AggregationVkey(&_OPSuccinctL2OutputOracle.CallOpts)
}

// AggregationVkey is a free data retrieval call binding the contract method 0xc32e4e3e.
//
// Solidity: function aggregationVkey() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) AggregationVkey() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.AggregationVkey(&_OPSuccinctL2OutputOracle.CallOpts)
}

// ApprovedProposers is a free data retrieval call binding the contract method 0xd4651276.
//
// Solidity: function approvedProposers(address ) view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) ApprovedProposers(opts *bind.CallOpts, arg0 common.Address) (bool, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "approvedProposers", arg0)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// ApprovedProposers is a free data retrieval call binding the contract method 0xd4651276.
//
// Solidity: function approvedProposers(address ) view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ApprovedProposers(arg0 common.Address) (bool, error) {
	return _OPSuccinctL2OutputOracle.Contract.ApprovedProposers(&_OPSuccinctL2OutputOracle.CallOpts, arg0)
}

// ApprovedProposers is a free data retrieval call binding the contract method 0xd4651276.
//
// Solidity: function approvedProposers(address ) view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) ApprovedProposers(arg0 common.Address) (bool, error) {
	return _OPSuccinctL2OutputOracle.Contract.ApprovedProposers(&_OPSuccinctL2OutputOracle.CallOpts, arg0)
}

// Challenger is a free data retrieval call binding the contract method 0x534db0e2.
//
// Solidity: function challenger() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) Challenger(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "challenger")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Challenger is a free data retrieval call binding the contract method 0x534db0e2.
//
// Solidity: function challenger() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Challenger() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Challenger(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Challenger is a free data retrieval call binding the contract method 0x534db0e2.
//
// Solidity: function challenger() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) Challenger() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Challenger(&_OPSuccinctL2OutputOracle.CallOpts)
}

// ComputeL2Timestamp is a free data retrieval call binding the contract method 0xd1de856c.
//
// Solidity: function computeL2Timestamp(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) ComputeL2Timestamp(opts *bind.CallOpts, _l2BlockNumber *big.Int) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "computeL2Timestamp", _l2BlockNumber)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// ComputeL2Timestamp is a free data retrieval call binding the contract method 0xd1de856c.
//
// Solidity: function computeL2Timestamp(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ComputeL2Timestamp(_l2BlockNumber *big.Int) (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.ComputeL2Timestamp(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// ComputeL2Timestamp is a free data retrieval call binding the contract method 0xd1de856c.
//
// Solidity: function computeL2Timestamp(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) ComputeL2Timestamp(_l2BlockNumber *big.Int) (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.ComputeL2Timestamp(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// FinalizationPeriodSeconds is a free data retrieval call binding the contract method 0xce5db8d6.
//
// Solidity: function finalizationPeriodSeconds() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) FinalizationPeriodSeconds(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "finalizationPeriodSeconds")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// FinalizationPeriodSeconds is a free data retrieval call binding the contract method 0xce5db8d6.
//
// Solidity: function finalizationPeriodSeconds() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) FinalizationPeriodSeconds() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.FinalizationPeriodSeconds(&_OPSuccinctL2OutputOracle.CallOpts)
}

// FinalizationPeriodSeconds is a free data retrieval call binding the contract method 0xce5db8d6.
//
// Solidity: function finalizationPeriodSeconds() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) FinalizationPeriodSeconds() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.FinalizationPeriodSeconds(&_OPSuccinctL2OutputOracle.CallOpts)
}

// GetL2Output is a free data retrieval call binding the contract method 0xa25ae557.
//
// Solidity: function getL2Output(uint256 _l2OutputIndex) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) GetL2Output(opts *bind.CallOpts, _l2OutputIndex *big.Int) (TypesOutputProposal, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "getL2Output", _l2OutputIndex)

	if err != nil {
		return *new(TypesOutputProposal), err
	}

	out0 := *abi.ConvertType(out[0], new(TypesOutputProposal)).(*TypesOutputProposal)

	return out0, err

}

// GetL2Output is a free data retrieval call binding the contract method 0xa25ae557.
//
// Solidity: function getL2Output(uint256 _l2OutputIndex) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) GetL2Output(_l2OutputIndex *big.Int) (TypesOutputProposal, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2Output(&_OPSuccinctL2OutputOracle.CallOpts, _l2OutputIndex)
}

// GetL2Output is a free data retrieval call binding the contract method 0xa25ae557.
//
// Solidity: function getL2Output(uint256 _l2OutputIndex) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) GetL2Output(_l2OutputIndex *big.Int) (TypesOutputProposal, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2Output(&_OPSuccinctL2OutputOracle.CallOpts, _l2OutputIndex)
}

// GetL2OutputAfter is a free data retrieval call binding the contract method 0xcf8e5cf0.
//
// Solidity: function getL2OutputAfter(uint256 _l2BlockNumber) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) GetL2OutputAfter(opts *bind.CallOpts, _l2BlockNumber *big.Int) (TypesOutputProposal, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "getL2OutputAfter", _l2BlockNumber)

	if err != nil {
		return *new(TypesOutputProposal), err
	}

	out0 := *abi.ConvertType(out[0], new(TypesOutputProposal)).(*TypesOutputProposal)

	return out0, err

}

// GetL2OutputAfter is a free data retrieval call binding the contract method 0xcf8e5cf0.
//
// Solidity: function getL2OutputAfter(uint256 _l2BlockNumber) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) GetL2OutputAfter(_l2BlockNumber *big.Int) (TypesOutputProposal, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2OutputAfter(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// GetL2OutputAfter is a free data retrieval call binding the contract method 0xcf8e5cf0.
//
// Solidity: function getL2OutputAfter(uint256 _l2BlockNumber) view returns((bytes32,uint128,uint128))
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) GetL2OutputAfter(_l2BlockNumber *big.Int) (TypesOutputProposal, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2OutputAfter(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// GetL2OutputIndexAfter is a free data retrieval call binding the contract method 0x7f006420.
//
// Solidity: function getL2OutputIndexAfter(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) GetL2OutputIndexAfter(opts *bind.CallOpts, _l2BlockNumber *big.Int) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "getL2OutputIndexAfter", _l2BlockNumber)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// GetL2OutputIndexAfter is a free data retrieval call binding the contract method 0x7f006420.
//
// Solidity: function getL2OutputIndexAfter(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) GetL2OutputIndexAfter(_l2BlockNumber *big.Int) (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2OutputIndexAfter(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// GetL2OutputIndexAfter is a free data retrieval call binding the contract method 0x7f006420.
//
// Solidity: function getL2OutputIndexAfter(uint256 _l2BlockNumber) view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) GetL2OutputIndexAfter(_l2BlockNumber *big.Int) (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.GetL2OutputIndexAfter(&_OPSuccinctL2OutputOracle.CallOpts, _l2BlockNumber)
}

// HistoricBlockHashes is a free data retrieval call binding the contract method 0xa196b525.
//
// Solidity: function historicBlockHashes(uint256 ) view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) HistoricBlockHashes(opts *bind.CallOpts, arg0 *big.Int) ([32]byte, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "historicBlockHashes", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// HistoricBlockHashes is a free data retrieval call binding the contract method 0xa196b525.
//
// Solidity: function historicBlockHashes(uint256 ) view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) HistoricBlockHashes(arg0 *big.Int) ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.HistoricBlockHashes(&_OPSuccinctL2OutputOracle.CallOpts, arg0)
}

// HistoricBlockHashes is a free data retrieval call binding the contract method 0xa196b525.
//
// Solidity: function historicBlockHashes(uint256 ) view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) HistoricBlockHashes(arg0 *big.Int) ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.HistoricBlockHashes(&_OPSuccinctL2OutputOracle.CallOpts, arg0)
}

// InitializerVersion is a free data retrieval call binding the contract method 0x7f01ea68.
//
// Solidity: function initializerVersion() view returns(uint8)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) InitializerVersion(opts *bind.CallOpts) (uint8, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "initializerVersion")

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// InitializerVersion is a free data retrieval call binding the contract method 0x7f01ea68.
//
// Solidity: function initializerVersion() view returns(uint8)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) InitializerVersion() (uint8, error) {
	return _OPSuccinctL2OutputOracle.Contract.InitializerVersion(&_OPSuccinctL2OutputOracle.CallOpts)
}

// InitializerVersion is a free data retrieval call binding the contract method 0x7f01ea68.
//
// Solidity: function initializerVersion() view returns(uint8)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) InitializerVersion() (uint8, error) {
	return _OPSuccinctL2OutputOracle.Contract.InitializerVersion(&_OPSuccinctL2OutputOracle.CallOpts)
}

// L2BlockTime is a free data retrieval call binding the contract method 0x93991af3.
//
// Solidity: function l2BlockTime() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) L2BlockTime(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "l2BlockTime")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// L2BlockTime is a free data retrieval call binding the contract method 0x93991af3.
//
// Solidity: function l2BlockTime() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) L2BlockTime() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.L2BlockTime(&_OPSuccinctL2OutputOracle.CallOpts)
}

// L2BlockTime is a free data retrieval call binding the contract method 0x93991af3.
//
// Solidity: function l2BlockTime() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) L2BlockTime() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.L2BlockTime(&_OPSuccinctL2OutputOracle.CallOpts)
}

// LatestBlockNumber is a free data retrieval call binding the contract method 0x4599c788.
//
// Solidity: function latestBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) LatestBlockNumber(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "latestBlockNumber")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LatestBlockNumber is a free data retrieval call binding the contract method 0x4599c788.
//
// Solidity: function latestBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) LatestBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.LatestBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// LatestBlockNumber is a free data retrieval call binding the contract method 0x4599c788.
//
// Solidity: function latestBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) LatestBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.LatestBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// LatestOutputIndex is a free data retrieval call binding the contract method 0x69f16eec.
//
// Solidity: function latestOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) LatestOutputIndex(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "latestOutputIndex")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LatestOutputIndex is a free data retrieval call binding the contract method 0x69f16eec.
//
// Solidity: function latestOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) LatestOutputIndex() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.LatestOutputIndex(&_OPSuccinctL2OutputOracle.CallOpts)
}

// LatestOutputIndex is a free data retrieval call binding the contract method 0x69f16eec.
//
// Solidity: function latestOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) LatestOutputIndex() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.LatestOutputIndex(&_OPSuccinctL2OutputOracle.CallOpts)
}

// NextBlockNumber is a free data retrieval call binding the contract method 0xdcec3348.
//
// Solidity: function nextBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) NextBlockNumber(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "nextBlockNumber")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// NextBlockNumber is a free data retrieval call binding the contract method 0xdcec3348.
//
// Solidity: function nextBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) NextBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.NextBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// NextBlockNumber is a free data retrieval call binding the contract method 0xdcec3348.
//
// Solidity: function nextBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) NextBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.NextBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// NextOutputIndex is a free data retrieval call binding the contract method 0x6abcf563.
//
// Solidity: function nextOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) NextOutputIndex(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "nextOutputIndex")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// NextOutputIndex is a free data retrieval call binding the contract method 0x6abcf563.
//
// Solidity: function nextOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) NextOutputIndex() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.NextOutputIndex(&_OPSuccinctL2OutputOracle.CallOpts)
}

// NextOutputIndex is a free data retrieval call binding the contract method 0x6abcf563.
//
// Solidity: function nextOutputIndex() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) NextOutputIndex() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.NextOutputIndex(&_OPSuccinctL2OutputOracle.CallOpts)
}

// OptimisticMode is a free data retrieval call binding the contract method 0x60caf7a0.
//
// Solidity: function optimisticMode() view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) OptimisticMode(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "optimisticMode")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// OptimisticMode is a free data retrieval call binding the contract method 0x60caf7a0.
//
// Solidity: function optimisticMode() view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) OptimisticMode() (bool, error) {
	return _OPSuccinctL2OutputOracle.Contract.OptimisticMode(&_OPSuccinctL2OutputOracle.CallOpts)
}

// OptimisticMode is a free data retrieval call binding the contract method 0x60caf7a0.
//
// Solidity: function optimisticMode() view returns(bool)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) OptimisticMode() (bool, error) {
	return _OPSuccinctL2OutputOracle.Contract.OptimisticMode(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Owner() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Owner(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) Owner() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Owner(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Proposer is a free data retrieval call binding the contract method 0xa8e4fb90.
//
// Solidity: function proposer() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) Proposer(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "proposer")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Proposer is a free data retrieval call binding the contract method 0xa8e4fb90.
//
// Solidity: function proposer() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Proposer() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Proposer(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Proposer is a free data retrieval call binding the contract method 0xa8e4fb90.
//
// Solidity: function proposer() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) Proposer() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Proposer(&_OPSuccinctL2OutputOracle.CallOpts)
}

// RangeVkeyCommitment is a free data retrieval call binding the contract method 0x2b31841e.
//
// Solidity: function rangeVkeyCommitment() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) RangeVkeyCommitment(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "rangeVkeyCommitment")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// RangeVkeyCommitment is a free data retrieval call binding the contract method 0x2b31841e.
//
// Solidity: function rangeVkeyCommitment() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) RangeVkeyCommitment() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.RangeVkeyCommitment(&_OPSuccinctL2OutputOracle.CallOpts)
}

// RangeVkeyCommitment is a free data retrieval call binding the contract method 0x2b31841e.
//
// Solidity: function rangeVkeyCommitment() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) RangeVkeyCommitment() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.RangeVkeyCommitment(&_OPSuccinctL2OutputOracle.CallOpts)
}

// RollupConfigHash is a free data retrieval call binding the contract method 0x6d9a1c8b.
//
// Solidity: function rollupConfigHash() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) RollupConfigHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "rollupConfigHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// RollupConfigHash is a free data retrieval call binding the contract method 0x6d9a1c8b.
//
// Solidity: function rollupConfigHash() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) RollupConfigHash() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.RollupConfigHash(&_OPSuccinctL2OutputOracle.CallOpts)
}

// RollupConfigHash is a free data retrieval call binding the contract method 0x6d9a1c8b.
//
// Solidity: function rollupConfigHash() view returns(bytes32)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) RollupConfigHash() ([32]byte, error) {
	return _OPSuccinctL2OutputOracle.Contract.RollupConfigHash(&_OPSuccinctL2OutputOracle.CallOpts)
}

// StartingBlockNumber is a free data retrieval call binding the contract method 0x70872aa5.
//
// Solidity: function startingBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) StartingBlockNumber(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "startingBlockNumber")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// StartingBlockNumber is a free data retrieval call binding the contract method 0x70872aa5.
//
// Solidity: function startingBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) StartingBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.StartingBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// StartingBlockNumber is a free data retrieval call binding the contract method 0x70872aa5.
//
// Solidity: function startingBlockNumber() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) StartingBlockNumber() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.StartingBlockNumber(&_OPSuccinctL2OutputOracle.CallOpts)
}

// StartingTimestamp is a free data retrieval call binding the contract method 0x88786272.
//
// Solidity: function startingTimestamp() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) StartingTimestamp(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "startingTimestamp")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// StartingTimestamp is a free data retrieval call binding the contract method 0x88786272.
//
// Solidity: function startingTimestamp() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) StartingTimestamp() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.StartingTimestamp(&_OPSuccinctL2OutputOracle.CallOpts)
}

// StartingTimestamp is a free data retrieval call binding the contract method 0x88786272.
//
// Solidity: function startingTimestamp() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) StartingTimestamp() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.StartingTimestamp(&_OPSuccinctL2OutputOracle.CallOpts)
}

// SubmissionInterval is a free data retrieval call binding the contract method 0xe1a41bcf.
//
// Solidity: function submissionInterval() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) SubmissionInterval(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "submissionInterval")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// SubmissionInterval is a free data retrieval call binding the contract method 0xe1a41bcf.
//
// Solidity: function submissionInterval() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) SubmissionInterval() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.SubmissionInterval(&_OPSuccinctL2OutputOracle.CallOpts)
}

// SubmissionInterval is a free data retrieval call binding the contract method 0xe1a41bcf.
//
// Solidity: function submissionInterval() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) SubmissionInterval() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.SubmissionInterval(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Verifier is a free data retrieval call binding the contract method 0x2b7ac3f3.
//
// Solidity: function verifier() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) Verifier(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "verifier")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Verifier is a free data retrieval call binding the contract method 0x2b7ac3f3.
//
// Solidity: function verifier() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Verifier() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Verifier(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Verifier is a free data retrieval call binding the contract method 0x2b7ac3f3.
//
// Solidity: function verifier() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) Verifier() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.Verifier(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(string)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) Version(opts *bind.CallOpts) (string, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "version")

	if err != nil {
		return *new(string), err
	}

	out0 := *abi.ConvertType(out[0], new(string)).(*string)

	return out0, err

}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(string)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Version() (string, error) {
	return _OPSuccinctL2OutputOracle.Contract.Version(&_OPSuccinctL2OutputOracle.CallOpts)
}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(string)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) Version() (string, error) {
	return _OPSuccinctL2OutputOracle.Contract.Version(&_OPSuccinctL2OutputOracle.CallOpts)
}

// AddProposer is a paid mutator transaction binding the contract method 0xb03cd418.
//
// Solidity: function addProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) AddProposer(opts *bind.TransactOpts, _proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "addProposer", _proposer)
}

// AddProposer is a paid mutator transaction binding the contract method 0xb03cd418.
//
// Solidity: function addProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) AddProposer(_proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.AddProposer(&_OPSuccinctL2OutputOracle.TransactOpts, _proposer)
}

// AddProposer is a paid mutator transaction binding the contract method 0xb03cd418.
//
// Solidity: function addProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) AddProposer(_proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.AddProposer(&_OPSuccinctL2OutputOracle.TransactOpts, _proposer)
}

// CheckpointBlockHash is a paid mutator transaction binding the contract method 0x1e856800.
//
// Solidity: function checkpointBlockHash(uint256 _blockNumber) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) CheckpointBlockHash(opts *bind.TransactOpts, _blockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "checkpointBlockHash", _blockNumber)
}

// CheckpointBlockHash is a paid mutator transaction binding the contract method 0x1e856800.
//
// Solidity: function checkpointBlockHash(uint256 _blockNumber) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) CheckpointBlockHash(_blockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.CheckpointBlockHash(&_OPSuccinctL2OutputOracle.TransactOpts, _blockNumber)
}

// CheckpointBlockHash is a paid mutator transaction binding the contract method 0x1e856800.
//
// Solidity: function checkpointBlockHash(uint256 _blockNumber) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) CheckpointBlockHash(_blockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.CheckpointBlockHash(&_OPSuccinctL2OutputOracle.TransactOpts, _blockNumber)
}

// DeleteL2Outputs is a paid mutator transaction binding the contract method 0x89c44cbb.
//
// Solidity: function deleteL2Outputs(uint256 _l2OutputIndex) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) DeleteL2Outputs(opts *bind.TransactOpts, _l2OutputIndex *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "deleteL2Outputs", _l2OutputIndex)
}

// DeleteL2Outputs is a paid mutator transaction binding the contract method 0x89c44cbb.
//
// Solidity: function deleteL2Outputs(uint256 _l2OutputIndex) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) DeleteL2Outputs(_l2OutputIndex *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.DeleteL2Outputs(&_OPSuccinctL2OutputOracle.TransactOpts, _l2OutputIndex)
}

// DeleteL2Outputs is a paid mutator transaction binding the contract method 0x89c44cbb.
//
// Solidity: function deleteL2Outputs(uint256 _l2OutputIndex) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) DeleteL2Outputs(_l2OutputIndex *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.DeleteL2Outputs(&_OPSuccinctL2OutputOracle.TransactOpts, _l2OutputIndex)
}

// DisableOptimisticMode is a paid mutator transaction binding the contract method 0x4ab309ac.
//
// Solidity: function disableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) DisableOptimisticMode(opts *bind.TransactOpts, _finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "disableOptimisticMode", _finalizationPeriodSeconds)
}

// DisableOptimisticMode is a paid mutator transaction binding the contract method 0x4ab309ac.
//
// Solidity: function disableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) DisableOptimisticMode(_finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.DisableOptimisticMode(&_OPSuccinctL2OutputOracle.TransactOpts, _finalizationPeriodSeconds)
}

// DisableOptimisticMode is a paid mutator transaction binding the contract method 0x4ab309ac.
//
// Solidity: function disableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) DisableOptimisticMode(_finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.DisableOptimisticMode(&_OPSuccinctL2OutputOracle.TransactOpts, _finalizationPeriodSeconds)
}

// EnableOptimisticMode is a paid mutator transaction binding the contract method 0x2c697961.
//
// Solidity: function enableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) EnableOptimisticMode(opts *bind.TransactOpts, _finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "enableOptimisticMode", _finalizationPeriodSeconds)
}

// EnableOptimisticMode is a paid mutator transaction binding the contract method 0x2c697961.
//
// Solidity: function enableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) EnableOptimisticMode(_finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.EnableOptimisticMode(&_OPSuccinctL2OutputOracle.TransactOpts, _finalizationPeriodSeconds)
}

// EnableOptimisticMode is a paid mutator transaction binding the contract method 0x2c697961.
//
// Solidity: function enableOptimisticMode(uint256 _finalizationPeriodSeconds) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) EnableOptimisticMode(_finalizationPeriodSeconds *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.EnableOptimisticMode(&_OPSuccinctL2OutputOracle.TransactOpts, _finalizationPeriodSeconds)
}

// Initialize is a paid mutator transaction binding the contract method 0xdb1470f5.
//
// Solidity: function initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) Initialize(opts *bind.TransactOpts, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "initialize", _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0xdb1470f5.
//
// Solidity: function initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Initialize(_initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.Initialize(&_OPSuccinctL2OutputOracle.TransactOpts, _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0xdb1470f5.
//
// Solidity: function initialize((address,address,address,uint256,uint256,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint256,address) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) Initialize(_initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.Initialize(&_OPSuccinctL2OutputOracle.TransactOpts, _initParams)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9aaab648.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, bytes32 _l1BlockHash, uint256 _l1BlockNumber) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) ProposeL2Output(opts *bind.TransactOpts, _outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockHash [32]byte, _l1BlockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "proposeL2Output", _outputRoot, _l2BlockNumber, _l1BlockHash, _l1BlockNumber)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9aaab648.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, bytes32 _l1BlockHash, uint256 _l1BlockNumber) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ProposeL2Output(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockHash [32]byte, _l1BlockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockHash, _l1BlockNumber)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9aaab648.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, bytes32 _l1BlockHash, uint256 _l1BlockNumber) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) ProposeL2Output(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockHash [32]byte, _l1BlockNumber *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockHash, _l1BlockNumber)
}

// ProposeL2Output0 is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) ProposeL2Output0(opts *bind.TransactOpts, _outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "proposeL2Output0", _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// ProposeL2Output0 is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ProposeL2Output0(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output0(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// ProposeL2Output0 is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) ProposeL2Output0(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output0(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// RemoveProposer is a paid mutator transaction binding the contract method 0x09d632d3.
//
// Solidity: function removeProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) RemoveProposer(opts *bind.TransactOpts, _proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "removeProposer", _proposer)
}

// RemoveProposer is a paid mutator transaction binding the contract method 0x09d632d3.
//
// Solidity: function removeProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) RemoveProposer(_proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.RemoveProposer(&_OPSuccinctL2OutputOracle.TransactOpts, _proposer)
}

// RemoveProposer is a paid mutator transaction binding the contract method 0x09d632d3.
//
// Solidity: function removeProposer(address _proposer) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) RemoveProposer(_proposer common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.RemoveProposer(&_OPSuccinctL2OutputOracle.TransactOpts, _proposer)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _owner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) TransferOwnership(opts *bind.TransactOpts, _owner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "transferOwnership", _owner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _owner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) TransferOwnership(_owner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.TransferOwnership(&_OPSuccinctL2OutputOracle.TransactOpts, _owner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _owner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) TransferOwnership(_owner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.TransferOwnership(&_OPSuccinctL2OutputOracle.TransactOpts, _owner)
}

// UpdateAggregationVkey is a paid mutator transaction binding the contract method 0xc4cb03ec.
//
// Solidity: function updateAggregationVkey(bytes32 _aggregationVkey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateAggregationVkey(opts *bind.TransactOpts, _aggregationVkey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateAggregationVkey", _aggregationVkey)
}

// UpdateAggregationVkey is a paid mutator transaction binding the contract method 0xc4cb03ec.
//
// Solidity: function updateAggregationVkey(bytes32 _aggregationVkey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateAggregationVkey(_aggregationVkey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateAggregationVkey(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVkey)
}

// UpdateAggregationVkey is a paid mutator transaction binding the contract method 0xc4cb03ec.
//
// Solidity: function updateAggregationVkey(bytes32 _aggregationVkey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateAggregationVkey(_aggregationVkey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateAggregationVkey(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVkey)
}

// UpdateRangeVkeyCommitment is a paid mutator transaction binding the contract method 0xbc91ce33.
//
// Solidity: function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateRangeVkeyCommitment(opts *bind.TransactOpts, _rangeVkeyCommitment [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateRangeVkeyCommitment", _rangeVkeyCommitment)
}

// UpdateRangeVkeyCommitment is a paid mutator transaction binding the contract method 0xbc91ce33.
//
// Solidity: function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateRangeVkeyCommitment(_rangeVkeyCommitment [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateRangeVkeyCommitment(&_OPSuccinctL2OutputOracle.TransactOpts, _rangeVkeyCommitment)
}

// UpdateRangeVkeyCommitment is a paid mutator transaction binding the contract method 0xbc91ce33.
//
// Solidity: function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateRangeVkeyCommitment(_rangeVkeyCommitment [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateRangeVkeyCommitment(&_OPSuccinctL2OutputOracle.TransactOpts, _rangeVkeyCommitment)
}

// UpdateRollupConfigHash is a paid mutator transaction binding the contract method 0x1bdd450c.
//
// Solidity: function updateRollupConfigHash(bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateRollupConfigHash(opts *bind.TransactOpts, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateRollupConfigHash", _rollupConfigHash)
}

// UpdateRollupConfigHash is a paid mutator transaction binding the contract method 0x1bdd450c.
//
// Solidity: function updateRollupConfigHash(bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateRollupConfigHash(_rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateRollupConfigHash(&_OPSuccinctL2OutputOracle.TransactOpts, _rollupConfigHash)
}

// UpdateRollupConfigHash is a paid mutator transaction binding the contract method 0x1bdd450c.
//
// Solidity: function updateRollupConfigHash(bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateRollupConfigHash(_rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateRollupConfigHash(&_OPSuccinctL2OutputOracle.TransactOpts, _rollupConfigHash)
}

// UpdateSubmissionInterval is a paid mutator transaction binding the contract method 0x336c9e81.
//
// Solidity: function updateSubmissionInterval(uint256 _submissionInterval) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateSubmissionInterval(opts *bind.TransactOpts, _submissionInterval *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateSubmissionInterval", _submissionInterval)
}

// UpdateSubmissionInterval is a paid mutator transaction binding the contract method 0x336c9e81.
//
// Solidity: function updateSubmissionInterval(uint256 _submissionInterval) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateSubmissionInterval(_submissionInterval *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateSubmissionInterval(&_OPSuccinctL2OutputOracle.TransactOpts, _submissionInterval)
}

// UpdateSubmissionInterval is a paid mutator transaction binding the contract method 0x336c9e81.
//
// Solidity: function updateSubmissionInterval(uint256 _submissionInterval) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateSubmissionInterval(_submissionInterval *big.Int) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateSubmissionInterval(&_OPSuccinctL2OutputOracle.TransactOpts, _submissionInterval)
}

// UpdateVerifier is a paid mutator transaction binding the contract method 0x97fc007c.
//
// Solidity: function updateVerifier(address _verifier) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateVerifier(opts *bind.TransactOpts, _verifier common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateVerifier", _verifier)
}

// UpdateVerifier is a paid mutator transaction binding the contract method 0x97fc007c.
//
// Solidity: function updateVerifier(address _verifier) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateVerifier(_verifier common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateVerifier(&_OPSuccinctL2OutputOracle.TransactOpts, _verifier)
}

// UpdateVerifier is a paid mutator transaction binding the contract method 0x97fc007c.
//
// Solidity: function updateVerifier(address _verifier) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateVerifier(_verifier common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateVerifier(&_OPSuccinctL2OutputOracle.TransactOpts, _verifier)
}

// OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator is returned from FilterAggregationVkeyUpdated and is used to iterate over the raw logs and unpacked data for AggregationVkeyUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleAggregationVkeyUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleAggregationVkeyUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleAggregationVkeyUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleAggregationVkeyUpdated represents a AggregationVkeyUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleAggregationVkeyUpdated struct {
	OldAggregationVkey [32]byte
	NewAggregationVkey [32]byte
	Raw                types.Log // Blockchain specific contextual infos
}

// FilterAggregationVkeyUpdated is a free log retrieval operation binding the contract event 0x390b73b2b067afcef04d30b573e4590c6e565519e370927dd777ca0ce8a55db0.
//
// Solidity: event AggregationVkeyUpdated(bytes32 indexed oldAggregationVkey, bytes32 indexed newAggregationVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterAggregationVkeyUpdated(opts *bind.FilterOpts, oldAggregationVkey [][32]byte, newAggregationVkey [][32]byte) (*OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator, error) {

	var oldAggregationVkeyRule []interface{}
	for _, oldAggregationVkeyItem := range oldAggregationVkey {
		oldAggregationVkeyRule = append(oldAggregationVkeyRule, oldAggregationVkeyItem)
	}
	var newAggregationVkeyRule []interface{}
	for _, newAggregationVkeyItem := range newAggregationVkey {
		newAggregationVkeyRule = append(newAggregationVkeyRule, newAggregationVkeyItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "AggregationVkeyUpdated", oldAggregationVkeyRule, newAggregationVkeyRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleAggregationVkeyUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "AggregationVkeyUpdated", logs: logs, sub: sub}, nil
}

// WatchAggregationVkeyUpdated is a free log subscription operation binding the contract event 0x390b73b2b067afcef04d30b573e4590c6e565519e370927dd777ca0ce8a55db0.
//
// Solidity: event AggregationVkeyUpdated(bytes32 indexed oldAggregationVkey, bytes32 indexed newAggregationVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchAggregationVkeyUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleAggregationVkeyUpdated, oldAggregationVkey [][32]byte, newAggregationVkey [][32]byte) (event.Subscription, error) {

	var oldAggregationVkeyRule []interface{}
	for _, oldAggregationVkeyItem := range oldAggregationVkey {
		oldAggregationVkeyRule = append(oldAggregationVkeyRule, oldAggregationVkeyItem)
	}
	var newAggregationVkeyRule []interface{}
	for _, newAggregationVkeyItem := range newAggregationVkey {
		newAggregationVkeyRule = append(newAggregationVkeyRule, newAggregationVkeyItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "AggregationVkeyUpdated", oldAggregationVkeyRule, newAggregationVkeyRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleAggregationVkeyUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "AggregationVkeyUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseAggregationVkeyUpdated is a log parse operation binding the contract event 0x390b73b2b067afcef04d30b573e4590c6e565519e370927dd777ca0ce8a55db0.
//
// Solidity: event AggregationVkeyUpdated(bytes32 indexed oldAggregationVkey, bytes32 indexed newAggregationVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseAggregationVkeyUpdated(log types.Log) (*OPSuccinctL2OutputOracleAggregationVkeyUpdated, error) {
	event := new(OPSuccinctL2OutputOracleAggregationVkeyUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "AggregationVkeyUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleInitializedIterator struct {
	Event *OPSuccinctL2OutputOracleInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleInitialized represents a Initialized event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterInitialized(opts *bind.FilterOpts) (*OPSuccinctL2OutputOracleInitializedIterator, error) {

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleInitializedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleInitialized) (event.Subscription, error) {

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleInitialized)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseInitialized(log types.Log) (*OPSuccinctL2OutputOracleInitialized, error) {
	event := new(OPSuccinctL2OutputOracleInitialized)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleOptimisticModeToggledIterator is returned from FilterOptimisticModeToggled and is used to iterate over the raw logs and unpacked data for OptimisticModeToggled events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOptimisticModeToggledIterator struct {
	Event *OPSuccinctL2OutputOracleOptimisticModeToggled // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleOptimisticModeToggledIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleOptimisticModeToggled)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleOptimisticModeToggled)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleOptimisticModeToggledIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleOptimisticModeToggledIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleOptimisticModeToggled represents a OptimisticModeToggled event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOptimisticModeToggled struct {
	Enabled                   bool
	FinalizationPeriodSeconds *big.Int
	Raw                       types.Log // Blockchain specific contextual infos
}

// FilterOptimisticModeToggled is a free log retrieval operation binding the contract event 0x1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a.
//
// Solidity: event OptimisticModeToggled(bool indexed enabled, uint256 finalizationPeriodSeconds)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterOptimisticModeToggled(opts *bind.FilterOpts, enabled []bool) (*OPSuccinctL2OutputOracleOptimisticModeToggledIterator, error) {

	var enabledRule []interface{}
	for _, enabledItem := range enabled {
		enabledRule = append(enabledRule, enabledItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "OptimisticModeToggled", enabledRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleOptimisticModeToggledIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "OptimisticModeToggled", logs: logs, sub: sub}, nil
}

// WatchOptimisticModeToggled is a free log subscription operation binding the contract event 0x1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a.
//
// Solidity: event OptimisticModeToggled(bool indexed enabled, uint256 finalizationPeriodSeconds)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchOptimisticModeToggled(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleOptimisticModeToggled, enabled []bool) (event.Subscription, error) {

	var enabledRule []interface{}
	for _, enabledItem := range enabled {
		enabledRule = append(enabledRule, enabledItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "OptimisticModeToggled", enabledRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleOptimisticModeToggled)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OptimisticModeToggled", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOptimisticModeToggled is a log parse operation binding the contract event 0x1f5c872f1ea93c57e43112ea449ee19ef5754488b87627b4c52456b0e5a4109a.
//
// Solidity: event OptimisticModeToggled(bool indexed enabled, uint256 finalizationPeriodSeconds)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseOptimisticModeToggled(log types.Log) (*OPSuccinctL2OutputOracleOptimisticModeToggled, error) {
	event := new(OPSuccinctL2OutputOracleOptimisticModeToggled)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OptimisticModeToggled", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleOutputProposedIterator is returned from FilterOutputProposed and is used to iterate over the raw logs and unpacked data for OutputProposed events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOutputProposedIterator struct {
	Event *OPSuccinctL2OutputOracleOutputProposed // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleOutputProposedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleOutputProposed)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleOutputProposed)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleOutputProposedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleOutputProposedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleOutputProposed represents a OutputProposed event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOutputProposed struct {
	OutputRoot    [32]byte
	L2OutputIndex *big.Int
	L2BlockNumber *big.Int
	L1Timestamp   *big.Int
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOutputProposed is a free log retrieval operation binding the contract event 0xa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2.
//
// Solidity: event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterOutputProposed(opts *bind.FilterOpts, outputRoot [][32]byte, l2OutputIndex []*big.Int, l2BlockNumber []*big.Int) (*OPSuccinctL2OutputOracleOutputProposedIterator, error) {

	var outputRootRule []interface{}
	for _, outputRootItem := range outputRoot {
		outputRootRule = append(outputRootRule, outputRootItem)
	}
	var l2OutputIndexRule []interface{}
	for _, l2OutputIndexItem := range l2OutputIndex {
		l2OutputIndexRule = append(l2OutputIndexRule, l2OutputIndexItem)
	}
	var l2BlockNumberRule []interface{}
	for _, l2BlockNumberItem := range l2BlockNumber {
		l2BlockNumberRule = append(l2BlockNumberRule, l2BlockNumberItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "OutputProposed", outputRootRule, l2OutputIndexRule, l2BlockNumberRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleOutputProposedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "OutputProposed", logs: logs, sub: sub}, nil
}

// WatchOutputProposed is a free log subscription operation binding the contract event 0xa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2.
//
// Solidity: event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchOutputProposed(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleOutputProposed, outputRoot [][32]byte, l2OutputIndex []*big.Int, l2BlockNumber []*big.Int) (event.Subscription, error) {

	var outputRootRule []interface{}
	for _, outputRootItem := range outputRoot {
		outputRootRule = append(outputRootRule, outputRootItem)
	}
	var l2OutputIndexRule []interface{}
	for _, l2OutputIndexItem := range l2OutputIndex {
		l2OutputIndexRule = append(l2OutputIndexRule, l2OutputIndexItem)
	}
	var l2BlockNumberRule []interface{}
	for _, l2BlockNumberItem := range l2BlockNumber {
		l2BlockNumberRule = append(l2BlockNumberRule, l2BlockNumberItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "OutputProposed", outputRootRule, l2OutputIndexRule, l2BlockNumberRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleOutputProposed)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OutputProposed", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOutputProposed is a log parse operation binding the contract event 0xa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e2.
//
// Solidity: event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber, uint256 l1Timestamp)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseOutputProposed(log types.Log) (*OPSuccinctL2OutputOracleOutputProposed, error) {
	event := new(OPSuccinctL2OutputOracleOutputProposed)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OutputProposed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleOutputsDeletedIterator is returned from FilterOutputsDeleted and is used to iterate over the raw logs and unpacked data for OutputsDeleted events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOutputsDeletedIterator struct {
	Event *OPSuccinctL2OutputOracleOutputsDeleted // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleOutputsDeletedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleOutputsDeleted)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleOutputsDeleted)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleOutputsDeletedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleOutputsDeletedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleOutputsDeleted represents a OutputsDeleted event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOutputsDeleted struct {
	PrevNextOutputIndex *big.Int
	NewNextOutputIndex  *big.Int
	Raw                 types.Log // Blockchain specific contextual infos
}

// FilterOutputsDeleted is a free log retrieval operation binding the contract event 0x4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b6.
//
// Solidity: event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterOutputsDeleted(opts *bind.FilterOpts, prevNextOutputIndex []*big.Int, newNextOutputIndex []*big.Int) (*OPSuccinctL2OutputOracleOutputsDeletedIterator, error) {

	var prevNextOutputIndexRule []interface{}
	for _, prevNextOutputIndexItem := range prevNextOutputIndex {
		prevNextOutputIndexRule = append(prevNextOutputIndexRule, prevNextOutputIndexItem)
	}
	var newNextOutputIndexRule []interface{}
	for _, newNextOutputIndexItem := range newNextOutputIndex {
		newNextOutputIndexRule = append(newNextOutputIndexRule, newNextOutputIndexItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "OutputsDeleted", prevNextOutputIndexRule, newNextOutputIndexRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleOutputsDeletedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "OutputsDeleted", logs: logs, sub: sub}, nil
}

// WatchOutputsDeleted is a free log subscription operation binding the contract event 0x4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b6.
//
// Solidity: event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchOutputsDeleted(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleOutputsDeleted, prevNextOutputIndex []*big.Int, newNextOutputIndex []*big.Int) (event.Subscription, error) {

	var prevNextOutputIndexRule []interface{}
	for _, prevNextOutputIndexItem := range prevNextOutputIndex {
		prevNextOutputIndexRule = append(prevNextOutputIndexRule, prevNextOutputIndexItem)
	}
	var newNextOutputIndexRule []interface{}
	for _, newNextOutputIndexItem := range newNextOutputIndex {
		newNextOutputIndexRule = append(newNextOutputIndexRule, newNextOutputIndexItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "OutputsDeleted", prevNextOutputIndexRule, newNextOutputIndexRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleOutputsDeleted)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OutputsDeleted", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOutputsDeleted is a log parse operation binding the contract event 0x4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b6.
//
// Solidity: event OutputsDeleted(uint256 indexed prevNextOutputIndex, uint256 indexed newNextOutputIndex)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseOutputsDeleted(log types.Log) (*OPSuccinctL2OutputOracleOutputsDeleted, error) {
	event := new(OPSuccinctL2OutputOracleOutputsDeleted)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OutputsDeleted", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOwnershipTransferredIterator struct {
	Event *OPSuccinctL2OutputOracleOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleOwnershipTransferred represents a OwnershipTransferred event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*OPSuccinctL2OutputOracleOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleOwnershipTransferredIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleOwnershipTransferred)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseOwnershipTransferred(log types.Log) (*OPSuccinctL2OutputOracleOwnershipTransferred, error) {
	event := new(OPSuccinctL2OutputOracleOwnershipTransferred)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleProposerUpdatedIterator is returned from FilterProposerUpdated and is used to iterate over the raw logs and unpacked data for ProposerUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleProposerUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleProposerUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleProposerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleProposerUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleProposerUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleProposerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleProposerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleProposerUpdated represents a ProposerUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleProposerUpdated struct {
	Proposer common.Address
	Added    bool
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterProposerUpdated is a free log retrieval operation binding the contract event 0x5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a.
//
// Solidity: event ProposerUpdated(address indexed proposer, bool added)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterProposerUpdated(opts *bind.FilterOpts, proposer []common.Address) (*OPSuccinctL2OutputOracleProposerUpdatedIterator, error) {

	var proposerRule []interface{}
	for _, proposerItem := range proposer {
		proposerRule = append(proposerRule, proposerItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "ProposerUpdated", proposerRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleProposerUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "ProposerUpdated", logs: logs, sub: sub}, nil
}

// WatchProposerUpdated is a free log subscription operation binding the contract event 0x5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a.
//
// Solidity: event ProposerUpdated(address indexed proposer, bool added)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchProposerUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleProposerUpdated, proposer []common.Address) (event.Subscription, error) {

	var proposerRule []interface{}
	for _, proposerItem := range proposer {
		proposerRule = append(proposerRule, proposerItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "ProposerUpdated", proposerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleProposerUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "ProposerUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseProposerUpdated is a log parse operation binding the contract event 0x5df38d395edc15b669d646569bd015513395070b5b4deb8a16300abb060d1b5a.
//
// Solidity: event ProposerUpdated(address indexed proposer, bool added)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseProposerUpdated(log types.Log) (*OPSuccinctL2OutputOracleProposerUpdated, error) {
	event := new(OPSuccinctL2OutputOracleProposerUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "ProposerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator is returned from FilterRangeVkeyCommitmentUpdated and is used to iterate over the raw logs and unpacked data for RangeVkeyCommitmentUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated represents a RangeVkeyCommitmentUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated struct {
	OldRangeVkeyCommitment [32]byte
	NewRangeVkeyCommitment [32]byte
	Raw                    types.Log // Blockchain specific contextual infos
}

// FilterRangeVkeyCommitmentUpdated is a free log retrieval operation binding the contract event 0xbf8cab6317796bfa97fea82b6d27c9542a08fa0821813cf2a57e7cff7fdc8156.
//
// Solidity: event RangeVkeyCommitmentUpdated(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterRangeVkeyCommitmentUpdated(opts *bind.FilterOpts, oldRangeVkeyCommitment [][32]byte, newRangeVkeyCommitment [][32]byte) (*OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator, error) {

	var oldRangeVkeyCommitmentRule []interface{}
	for _, oldRangeVkeyCommitmentItem := range oldRangeVkeyCommitment {
		oldRangeVkeyCommitmentRule = append(oldRangeVkeyCommitmentRule, oldRangeVkeyCommitmentItem)
	}
	var newRangeVkeyCommitmentRule []interface{}
	for _, newRangeVkeyCommitmentItem := range newRangeVkeyCommitment {
		newRangeVkeyCommitmentRule = append(newRangeVkeyCommitmentRule, newRangeVkeyCommitmentItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "RangeVkeyCommitmentUpdated", oldRangeVkeyCommitmentRule, newRangeVkeyCommitmentRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "RangeVkeyCommitmentUpdated", logs: logs, sub: sub}, nil
}

// WatchRangeVkeyCommitmentUpdated is a free log subscription operation binding the contract event 0xbf8cab6317796bfa97fea82b6d27c9542a08fa0821813cf2a57e7cff7fdc8156.
//
// Solidity: event RangeVkeyCommitmentUpdated(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchRangeVkeyCommitmentUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated, oldRangeVkeyCommitment [][32]byte, newRangeVkeyCommitment [][32]byte) (event.Subscription, error) {

	var oldRangeVkeyCommitmentRule []interface{}
	for _, oldRangeVkeyCommitmentItem := range oldRangeVkeyCommitment {
		oldRangeVkeyCommitmentRule = append(oldRangeVkeyCommitmentRule, oldRangeVkeyCommitmentItem)
	}
	var newRangeVkeyCommitmentRule []interface{}
	for _, newRangeVkeyCommitmentItem := range newRangeVkeyCommitment {
		newRangeVkeyCommitmentRule = append(newRangeVkeyCommitmentRule, newRangeVkeyCommitmentItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "RangeVkeyCommitmentUpdated", oldRangeVkeyCommitmentRule, newRangeVkeyCommitmentRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "RangeVkeyCommitmentUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRangeVkeyCommitmentUpdated is a log parse operation binding the contract event 0xbf8cab6317796bfa97fea82b6d27c9542a08fa0821813cf2a57e7cff7fdc8156.
//
// Solidity: event RangeVkeyCommitmentUpdated(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseRangeVkeyCommitmentUpdated(log types.Log) (*OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated, error) {
	event := new(OPSuccinctL2OutputOracleRangeVkeyCommitmentUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "RangeVkeyCommitmentUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator is returned from FilterRollupConfigHashUpdated and is used to iterate over the raw logs and unpacked data for RollupConfigHashUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleRollupConfigHashUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleRollupConfigHashUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleRollupConfigHashUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleRollupConfigHashUpdated represents a RollupConfigHashUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleRollupConfigHashUpdated struct {
	OldRollupConfigHash [32]byte
	NewRollupConfigHash [32]byte
	Raw                 types.Log // Blockchain specific contextual infos
}

// FilterRollupConfigHashUpdated is a free log retrieval operation binding the contract event 0x5d9ebe9f09b0810b3546b30781ba9a51092b37dd6abada4b830ce54a41ac6a4b.
//
// Solidity: event RollupConfigHashUpdated(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterRollupConfigHashUpdated(opts *bind.FilterOpts, oldRollupConfigHash [][32]byte, newRollupConfigHash [][32]byte) (*OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator, error) {

	var oldRollupConfigHashRule []interface{}
	for _, oldRollupConfigHashItem := range oldRollupConfigHash {
		oldRollupConfigHashRule = append(oldRollupConfigHashRule, oldRollupConfigHashItem)
	}
	var newRollupConfigHashRule []interface{}
	for _, newRollupConfigHashItem := range newRollupConfigHash {
		newRollupConfigHashRule = append(newRollupConfigHashRule, newRollupConfigHashItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "RollupConfigHashUpdated", oldRollupConfigHashRule, newRollupConfigHashRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleRollupConfigHashUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "RollupConfigHashUpdated", logs: logs, sub: sub}, nil
}

// WatchRollupConfigHashUpdated is a free log subscription operation binding the contract event 0x5d9ebe9f09b0810b3546b30781ba9a51092b37dd6abada4b830ce54a41ac6a4b.
//
// Solidity: event RollupConfigHashUpdated(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchRollupConfigHashUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleRollupConfigHashUpdated, oldRollupConfigHash [][32]byte, newRollupConfigHash [][32]byte) (event.Subscription, error) {

	var oldRollupConfigHashRule []interface{}
	for _, oldRollupConfigHashItem := range oldRollupConfigHash {
		oldRollupConfigHashRule = append(oldRollupConfigHashRule, oldRollupConfigHashItem)
	}
	var newRollupConfigHashRule []interface{}
	for _, newRollupConfigHashItem := range newRollupConfigHash {
		newRollupConfigHashRule = append(newRollupConfigHashRule, newRollupConfigHashItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "RollupConfigHashUpdated", oldRollupConfigHashRule, newRollupConfigHashRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleRollupConfigHashUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "RollupConfigHashUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRollupConfigHashUpdated is a log parse operation binding the contract event 0x5d9ebe9f09b0810b3546b30781ba9a51092b37dd6abada4b830ce54a41ac6a4b.
//
// Solidity: event RollupConfigHashUpdated(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseRollupConfigHashUpdated(log types.Log) (*OPSuccinctL2OutputOracleRollupConfigHashUpdated, error) {
	event := new(OPSuccinctL2OutputOracleRollupConfigHashUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "RollupConfigHashUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator is returned from FilterSubmissionIntervalUpdated and is used to iterate over the raw logs and unpacked data for SubmissionIntervalUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleSubmissionIntervalUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleSubmissionIntervalUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleSubmissionIntervalUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleSubmissionIntervalUpdated represents a SubmissionIntervalUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleSubmissionIntervalUpdated struct {
	OldSubmissionInterval *big.Int
	NewSubmissionInterval *big.Int
	Raw                   types.Log // Blockchain specific contextual infos
}

// FilterSubmissionIntervalUpdated is a free log retrieval operation binding the contract event 0xc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b2.
//
// Solidity: event SubmissionIntervalUpdated(uint256 oldSubmissionInterval, uint256 newSubmissionInterval)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterSubmissionIntervalUpdated(opts *bind.FilterOpts) (*OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator, error) {

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "SubmissionIntervalUpdated")
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleSubmissionIntervalUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "SubmissionIntervalUpdated", logs: logs, sub: sub}, nil
}

// WatchSubmissionIntervalUpdated is a free log subscription operation binding the contract event 0xc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b2.
//
// Solidity: event SubmissionIntervalUpdated(uint256 oldSubmissionInterval, uint256 newSubmissionInterval)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchSubmissionIntervalUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleSubmissionIntervalUpdated) (event.Subscription, error) {

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "SubmissionIntervalUpdated")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleSubmissionIntervalUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "SubmissionIntervalUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseSubmissionIntervalUpdated is a log parse operation binding the contract event 0xc1bf9abfb57ea01ed9ecb4f45e9cefa7ba44b2e6778c3ce7281409999f1af1b2.
//
// Solidity: event SubmissionIntervalUpdated(uint256 oldSubmissionInterval, uint256 newSubmissionInterval)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseSubmissionIntervalUpdated(log types.Log) (*OPSuccinctL2OutputOracleSubmissionIntervalUpdated, error) {
	event := new(OPSuccinctL2OutputOracleSubmissionIntervalUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "SubmissionIntervalUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleVerifierUpdatedIterator is returned from FilterVerifierUpdated and is used to iterate over the raw logs and unpacked data for VerifierUpdated events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleVerifierUpdatedIterator struct {
	Event *OPSuccinctL2OutputOracleVerifierUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *OPSuccinctL2OutputOracleVerifierUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleVerifierUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(OPSuccinctL2OutputOracleVerifierUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *OPSuccinctL2OutputOracleVerifierUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleVerifierUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleVerifierUpdated represents a VerifierUpdated event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleVerifierUpdated struct {
	OldVerifier common.Address
	NewVerifier common.Address
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterVerifierUpdated is a free log retrieval operation binding the contract event 0x0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc.
//
// Solidity: event VerifierUpdated(address indexed oldVerifier, address indexed newVerifier)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterVerifierUpdated(opts *bind.FilterOpts, oldVerifier []common.Address, newVerifier []common.Address) (*OPSuccinctL2OutputOracleVerifierUpdatedIterator, error) {

	var oldVerifierRule []interface{}
	for _, oldVerifierItem := range oldVerifier {
		oldVerifierRule = append(oldVerifierRule, oldVerifierItem)
	}
	var newVerifierRule []interface{}
	for _, newVerifierItem := range newVerifier {
		newVerifierRule = append(newVerifierRule, newVerifierItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "VerifierUpdated", oldVerifierRule, newVerifierRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleVerifierUpdatedIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "VerifierUpdated", logs: logs, sub: sub}, nil
}

// WatchVerifierUpdated is a free log subscription operation binding the contract event 0x0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc.
//
// Solidity: event VerifierUpdated(address indexed oldVerifier, address indexed newVerifier)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchVerifierUpdated(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleVerifierUpdated, oldVerifier []common.Address, newVerifier []common.Address) (event.Subscription, error) {

	var oldVerifierRule []interface{}
	for _, oldVerifierItem := range oldVerifier {
		oldVerifierRule = append(oldVerifierRule, oldVerifierItem)
	}
	var newVerifierRule []interface{}
	for _, newVerifierItem := range newVerifier {
		newVerifierRule = append(newVerifierRule, newVerifierItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "VerifierUpdated", oldVerifierRule, newVerifierRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleVerifierUpdated)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "VerifierUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseVerifierUpdated is a log parse operation binding the contract event 0x0243549a92b2412f7a3caf7a2e56d65b8821b91345363faa5f57195384065fcc.
//
// Solidity: event VerifierUpdated(address indexed oldVerifier, address indexed newVerifier)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseVerifierUpdated(log types.Log) (*OPSuccinctL2OutputOracleVerifierUpdated, error) {
	event := new(OPSuccinctL2OutputOracleVerifierUpdated)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "VerifierUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
