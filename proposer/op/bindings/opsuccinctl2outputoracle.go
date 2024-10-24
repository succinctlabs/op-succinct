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
	ChainId             *big.Int
	AggregationVkey     [32]byte
	RangeVkeyCommitment [32]byte
	VerifierGateway     common.Address
	StartingOutputRoot  [32]byte
	Owner               common.Address
	RollupConfigHash    [32]byte
}

// TypesOutputProposal is an auto generated low-level Go binding around an user-defined struct.
type TypesOutputProposal struct {
	OutputRoot    [32]byte
	Timestamp     *big.Int
	L2BlockNumber *big.Int
}

// OPSuccinctL2OutputOracleMetaData contains all meta data concerning the OPSuccinctL2OutputOracle contract.
var OPSuccinctL2OutputOracleMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"constructor\",\"inputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"CHALLENGER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"FINALIZATION_PERIOD_SECONDS\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"L2_BLOCK_TIME\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"PROPOSER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"SUBMISSION_INTERVAL\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregationVkey\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"chainId\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challenger\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkpointBlockHash\",\"inputs\":[{\"name\":\"_blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"computeL2Timestamp\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"deleteL2Outputs\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"finalizationPeriodSeconds\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2Output\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputIndexAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"historicBlockHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_submissionInterval\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l2BlockTime\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_startingBlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_startingTimestamp\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_proposer\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_challenger\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_finalizationPeriodSeconds\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_initParams\",\"type\":\"tuple\",\"internalType\":\"structOPSuccinctL2OutputOracle.InitParams\",\"components\":[{\"name\":\"chainId\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"startingOutputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"l2BlockTime\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"proposeL2Output\",\"inputs\":[{\"name\":\"_outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l1BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"proposer\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rangeVkeyCommitment\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rollupConfigHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"submissionInterval\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"_newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateAggregationVKey\",\"inputs\":[{\"name\":\"_aggregationVKey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRangeVkeyCommitment\",\"inputs\":[{\"name\":\"_rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRollupConfigHash\",\"inputs\":[{\"name\":\"_rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateVerifierGateway\",\"inputs\":[{\"name\":\"_verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"upgradeWithInitParams\",\"inputs\":[{\"name\":\"_chainId\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifierGateway\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractSP1VerifierGateway\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"version\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\",\"internalType\":\"string\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputProposed\",\"inputs\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"l2OutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l1Timestamp\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputsDeleted\",\"inputs\":[{\"name\":\"prevNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"newNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedAggregationVKey\",\"inputs\":[{\"name\":\"oldVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedRangeVkeyCommitment\",\"inputs\":[{\"name\":\"oldRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedRollupConfigHash\",\"inputs\":[{\"name\":\"oldRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedVerifierGateway\",\"inputs\":[{\"name\":\"oldVerifierGateway\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newVerifierGateway\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"BlockHashMismatch\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"BlockHashTooFarInPast\",\"inputs\":[]}]",
	Bin: "0x608060405234801561001057600080fd5b5061001961001e565b6100de565b600054610100900460ff161561008a5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156100dc576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b611cc3806100ed6000396000f3fe6080604052600436106102245760003560e01c80638da5cb5b11610123578063c32e4e3e116100ab578063e1a41bcf1161006f578063e1a41bcf1461062d578063e5c325c614610643578063f2fde38b14610663578063f4daa29114610683578063fb3c491c1461069857600080fd5b8063c32e4e3e146105ac578063ce5db8d6146105c2578063cf8e5cf0146105d8578063d1de856c146105f8578063dcec33481461061857600080fd5b8063a196b525116100f2578063a196b525146104ce578063a25ae557146104fb578063a8e4fb901461054e578063bc91ce331461056e578063bffa7f0f1461058e57600080fd5b80638da5cb5b1461046f57806393991af31461048f5780639a8a0592146104a55780639ad84880146104bb57600080fd5b806369f16eec116101b15780637ad01652116101755780637ad01652146103d95780637f006420146103f957806380fdb3e114610419578063887862721461043957806389c44cbb1461044f57600080fd5b806369f16eec146103655780636abcf5631461037a5780636b4d98dd1461038f5780636d9a1c8b146103ad57806370872aa5146103c357600080fd5b80634418db5e116101f85780634418db5e146102a55780634599c788146102c5578063529933df146102da578063534db0e2146102ef57806354fd4d501461032757600080fd5b80622134cc146102295780631bdd450c1461024d5780631e8568001461026f5780632b31841e1461028f575b600080fd5b34801561023557600080fd5b506005545b6040519081526020015b60405180910390f35b34801561025957600080fd5b5061026d610268366004611878565b6106b8565b005b34801561027b57600080fd5b5061026d61028a366004611878565b6106f7565b34801561029b57600080fd5b5061023a600b5481565b3480156102b157600080fd5b5061026d6102c03660046118ad565b61076f565b3480156102d157600080fd5b5061023a6107a2565b3480156102e657600080fd5b5060045461023a565b3480156102fb57600080fd5b5060065461030f906001600160a01b031681565b6040516001600160a01b039091168152602001610244565b34801561033357600080fd5b50610358604051806040016040528060058152602001640322e302e360dc1b81525081565b604051610244919061191c565b34801561037157600080fd5b5061023a6107ff565b34801561038657600080fd5b5060035461023a565b34801561039b57600080fd5b506006546001600160a01b031661030f565b3480156103b957600080fd5b5061023a600e5481565b3480156103cf57600080fd5b5061023a60015481565b3480156103e557600080fd5b5061026d6103f436600461192f565b610811565b34801561040557600080fd5b5061023a610414366004611878565b61086b565b34801561042557600080fd5b5061026d610434366004611878565b610a09565b34801561044557600080fd5b5061023a60025481565b34801561045b57600080fd5b5061026d61046a366004611878565b610a3c565b34801561047b57600080fd5b50600d5461030f906001600160a01b031681565b34801561049b57600080fd5b5061023a60055481565b3480156104b157600080fd5b5061023a60095481565b61026d6104c93660046119e6565b610c41565b3480156104da57600080fd5b5061023a6104e9366004611878565b600f6020526000908152604090205481565b34801561050757600080fd5b5061051b610516366004611878565b611239565b60408051825181526020808401516001600160801b03908116918301919091529282015190921690820152606001610244565b34801561055a57600080fd5b5060075461030f906001600160a01b031681565b34801561057a57600080fd5b5061026d610589366004611878565b6112b7565b34801561059a57600080fd5b506007546001600160a01b031661030f565b3480156105b857600080fd5b5061023a600a5481565b3480156105ce57600080fd5b5061023a60085481565b3480156105e457600080fd5b5061051b6105f3366004611878565b6112ea565b34801561060457600080fd5b5061023a610613366004611878565b611322565b34801561062457600080fd5b5061023a611352565b34801561063957600080fd5b5061023a60045481565b34801561064f57600080fd5b5061026d61065e366004611a98565b611369565b34801561066f57600080fd5b5061026d61067e3660046118ad565b6116f1565b34801561068f57600080fd5b5060085461023a565b3480156106a457600080fd5b50600c5461030f906001600160a01b031681565b600d546001600160a01b031633146106eb5760405162461bcd60e51b81526004016106e290611b75565b60405180910390fd5b6106f481611724565b50565b80408061075d5760405162461bcd60e51b815260206004820152602e60248201527f4c324f75747075744f7261636c653a20626c6f636b206861736820746f6f206660448201526d185c881a5b881d1a19481c185cdd60921b60648201526084016106e2565b6000918252600f602052604090912055565b600d546001600160a01b031633146107995760405162461bcd60e51b81526004016106e290611b75565b6106f481611758565b600354600090156107f657600380546107bd90600190611bd2565b815481106107cd576107cd611be9565b6000918252602090912060029091020160010154600160801b90046001600160801b0316919050565b6001545b905090565b6003546000906107fa90600190611bd2565b600d546001600160a01b0316331461083b5760405162461bcd60e51b81526004016106e290611b75565b6009859055610849846117b4565b610852836117e8565b61085b82611758565b61086481611724565b5050505050565b60006108756107a2565b8211156108fb5760405162461bcd60e51b815260206004820152604860248201527f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560448201527f7420666f72206120626c6f636b207468617420686173206e6f74206265656e206064820152671c1c9bdc1bdcd95960c21b608482015260a4016106e2565b60035461097f5760405162461bcd60e51b815260206004820152604660248201527f4c324f75747075744f7261636c653a2063616e6e6f7420676574206f7574707560448201527f74206173206e6f206f7574707574732068617665206265656e2070726f706f736064820152651959081e595d60d21b608482015260a4016106e2565b6003546000905b80821015610a02576000600261099c8385611bff565b6109a69190611c17565b905084600382815481106109bc576109bc611be9565b6000918252602090912060029091020160010154600160801b90046001600160801b031610156109f8576109f1816001611bff565b92506109fc565b8091505b50610986565b5092915050565b600d546001600160a01b03163314610a335760405162461bcd60e51b81526004016106e290611b75565b6106f4816117b4565b6006546001600160a01b03163314610abc5760405162461bcd60e51b815260206004820152603e60248201527f4c324f75747075744f7261636c653a206f6e6c7920746865206368616c6c656e60448201527f67657220616464726573732063616e2064656c657465206f757470757473000060648201526084016106e2565b6003548110610b3f5760405162461bcd60e51b815260206004820152604360248201527f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560448201527f747075747320616674657220746865206c6174657374206f757470757420696e6064820152620c8caf60eb1b608482015260a4016106e2565b60085460038281548110610b5557610b55611be9565b6000918252602090912060016002909202010154610b7c906001600160801b031642611bd2565b10610bfe5760405162461bcd60e51b815260206004820152604660248201527f4c324f75747075744f7261636c653a2063616e6e6f742064656c657465206f7560448201527f74707574732074686174206861766520616c7265616479206265656e2066696e606482015265185b1a5e995960d21b608482015260a4016106e2565b6000610c0960035490565b90508160035581817f4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b660405160405180910390a35050565b6007546001600160a01b0316331480610c6357506007546001600160a01b0316155b610cdf5760405162461bcd60e51b815260206004820152604160248201527f4c324f75747075744f7261636c653a206f6e6c79207468652070726f706f736560448201527f7220616464726573732063616e2070726f706f7365206e6577206f75747075746064820152607360f81b608482015260a4016106e2565b610ce7611352565b831015610d825760405162461bcd60e51b815260206004820152605860248201527f4c324f75747075744f7261636c653a20626c6f636b206e756d626572206d757360448201527f742062652067726561746572207468616e206f7220657175616c20746f206e6560648201527f787420657870656374656420626c6f636b206e756d6265720000000000000000608482015260a4016106e2565b42610d8c84611322565b10610df85760405162461bcd60e51b815260206004820152603660248201527f4c324f75747075744f7261636c653a2063616e6e6f742070726f706f7365204c60448201527532206f757470757420696e207468652066757475726560501b60648201526084016106e2565b83610e6b5760405162461bcd60e51b815260206004820152603a60248201527f4c324f75747075744f7261636c653a204c32206f75747075742070726f706f7360448201527f616c2063616e6e6f7420626520746865207a65726f206861736800000000000060648201526084016106e2565b600a54610ef05760405162461bcd60e51b815260206004820152604760248201527f4c324f75747075744f7261636c653a206167677265676174696f6e20766b657960448201527f206d75737420626520736574206265666f72652070726f706f73696e6720616e606482015266081bdd5d1c1d5d60ca1b608482015260a4016106e2565b600b54610f7a5760405162461bcd60e51b815260206004820152604c60248201527f4c324f75747075744f7261636c653a2072616e676520766b657920636f6d6d6960448201527f746d656e74206d75737420626520736574206265666f72652070726f706f736960648201526b1b99c8185b881bdd5d1c1d5d60a21b608482015260a4016106e2565b6000828152600f6020526040902054806110085760405162461bcd60e51b815260206004820152604360248201527f4c324f75747075744f7261636c653a2070726f706f73656420626c6f636b206860448201527f61736820616e64206e756d62657220617265206e6f7420636865636b706f696e6064820152621d195960ea1b608482015260a4016106e2565b60006040518060e0016040528083815260200160036110256107ff565b8154811061103557611035611be9565b90600052602060002090600202016000015481526020018781526020018681526020016009548152602001600e548152602001600b548152509050600c60009054906101000a90046001600160a01b03166001600160a01b03166341493c60600a54836040516020016110f19190600060e082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015260c083015160c083015292915050565b604051602081830303815290604052866040518463ffffffff1660e01b815260040161111f93929190611c39565b60006040518083038186803b15801561113757600080fd5b505afa15801561114b573d6000803e3d6000fd5b505050508461115960035490565b877fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e24260405161118b91815260200190565b60405180910390a45050604080516060810182529485526001600160801b034281166020870190815294811691860191825260038054600181018255600091909152955160029096027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810196909655935190518416600160801b029316929092177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c909301929092555050565b60408051606081018252600080825260208201819052918101919091526003828154811061126957611269611be9565b600091825260209182902060408051606081018252600290930290910180548352600101546001600160801b0380821694840194909452600160801b90049092169181019190915292915050565b600d546001600160a01b031633146112e15760405162461bcd60e51b81526004016106e290611b75565b6106f4816117e8565b604080516060810182526000808252602082018190529181019190915260036113128361086b565b8154811061126957611269611be9565b6000600554600154836113359190611bd2565b61133f9190611c6e565b60025461134c9190611bff565b92915050565b600060045461135f6107a2565b6107fa9190611bff565b600054600290610100900460ff1615801561138b575060005460ff8083169116105b6113ee5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016106e2565b6000805461ffff191660ff831617610100179055886114755760405162461bcd60e51b815260206004820152603a60248201527f4c324f75747075744f7261636c653a207375626d697373696f6e20696e74657260448201527f76616c206d7573742062652067726561746572207468616e203000000000000060648201526084016106e2565b600088116114e25760405162461bcd60e51b815260206004820152603460248201527f4c324f75747075744f7261636c653a204c3220626c6f636b2074696d65206d75604482015273073742062652067726561746572207468616e20360641b60648201526084016106e2565b428611156115665760405162461bcd60e51b8152602060048201526044602482018190527f4c324f75747075744f7261636c653a207374617274696e67204c322074696d65908201527f7374616d70206d757374206265206c657373207468616e2063757272656e742060648201526374696d6560e01b608482015260a4016106e2565b60048990556005889055600780546001600160a01b038088166001600160a01b0319928316179092556006805492871692909116919091179055600883905560035460000361165e5760408051606081018252608084015181526001600160801b03808916602083019081528a82169383019384526003805460018181018355600092909252935160029485027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810191909155915194518316600160801b0294909216939093177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c90930192909255908890558690555b815160095560a08201516116719061181c565b61167e82602001516117b4565b61168b82604001516117e8565b6116988260600151611758565b6116a58260c00151611724565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050505050505050565b600d546001600160a01b0316331461171b5760405162461bcd60e51b81526004016106e290611b75565b6106f48161181c565b600e546040518291907fda2f5f014ada26cff39a0f2a9dc6fa4fca1581376fc91ec09506c8fb8657bc3590600090a3600e55565b600c546040516001600160a01b038084169216907f1379941631ff0ed9178ab16ab67a2e5db3aeada7f87e518f761e79c8e38377e390600090a3600c80546001600160a01b0319166001600160a01b0392909216919091179055565b600a546040518291907fb81f9c41933b730a90fba96ab14541de7cab774f762ea0c183054947bc49aee790600090a3600a55565b600b546040518291907f1035606f0606905acdf851342466a5b64406fa798b7440235cd5811cea2850fd90600090a3600b55565b600d546040516001600160a01b038084169216907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a3600d80546001600160a01b0319166001600160a01b0392909216919091179055565b60006020828403121561188a57600080fd5b5035919050565b80356001600160a01b03811681146118a857600080fd5b919050565b6000602082840312156118bf57600080fd5b6118c882611891565b9392505050565b6000815180845260005b818110156118f5576020818501810151868301820152016118d9565b81811115611907576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006118c860208301846118cf565b600080600080600060a0868803121561194757600080fd5b85359450602086013593506040860135925061196560608701611891565b949793965091946080013592915050565b634e487b7160e01b600052604160045260246000fd5b60405160e0810167ffffffffffffffff811182821017156119af576119af611976565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156119de576119de611976565b604052919050565b600080600080608085870312156119fc57600080fd5b84359350602080860135935060408601359250606086013567ffffffffffffffff80821115611a2a57600080fd5b818801915088601f830112611a3e57600080fd5b813581811115611a5057611a50611976565b611a62601f8201601f191685016119b5565b91508082528984828501011115611a7857600080fd5b808484018584013760008482840101525080935050505092959194509250565b600080600080600080600080888a036101c0811215611ab657600080fd5b8935985060208a0135975060408a0135965060608a01359550611adb60808b01611891565b9450611ae960a08b01611891565b935060c08a0135925060e060df1982011215611b0457600080fd5b50611b0d61198c565b60e08a013581526101008a013560208201526101208a01356040820152611b376101408b01611891565b60608201526101608a01356080820152611b546101808b01611891565b60a08201526101a08a013560c0820152809150509295985092959890939650565b60208082526027908201527f4c324f75747075744f7261636c653a2063616c6c6572206973206e6f74207468604082015266329037bbb732b960c91b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600082821015611be457611be4611bbc565b500390565b634e487b7160e01b600052603260045260246000fd5b60008219821115611c1257611c12611bbc565b500190565b600082611c3457634e487b7160e01b600052601260045260246000fd5b500490565b838152606060208201526000611c5260608301856118cf565b8281036040840152611c6481856118cf565b9695505050505050565b6000816000190483118215151615611c8857611c88611bbc565b50029056fea264697066735822122095becb439f8a60395066f2ccf9013b634385327fbfa84daa0103642bd16da7f864736f6c634300080f0033",
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

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) ChainId(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "chainId")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ChainId() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.ChainId(&_OPSuccinctL2OutputOracle.CallOpts)
}

// ChainId is a free data retrieval call binding the contract method 0x9a8a0592.
//
// Solidity: function chainId() view returns(uint256)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) ChainId() (*big.Int, error) {
	return _OPSuccinctL2OutputOracle.Contract.ChainId(&_OPSuccinctL2OutputOracle.CallOpts)
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

// VerifierGateway is a free data retrieval call binding the contract method 0xfb3c491c.
//
// Solidity: function verifierGateway() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCaller) VerifierGateway(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _OPSuccinctL2OutputOracle.contract.Call(opts, &out, "verifierGateway")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// VerifierGateway is a free data retrieval call binding the contract method 0xfb3c491c.
//
// Solidity: function verifierGateway() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) VerifierGateway() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.VerifierGateway(&_OPSuccinctL2OutputOracle.CallOpts)
}

// VerifierGateway is a free data retrieval call binding the contract method 0xfb3c491c.
//
// Solidity: function verifierGateway() view returns(address)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleCallerSession) VerifierGateway() (common.Address, error) {
	return _OPSuccinctL2OutputOracle.Contract.VerifierGateway(&_OPSuccinctL2OutputOracle.CallOpts)
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

// Initialize is a paid mutator transaction binding the contract method 0xe5c325c6.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (uint256,bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) Initialize(opts *bind.TransactOpts, _submissionInterval *big.Int, _l2BlockTime *big.Int, _startingBlockNumber *big.Int, _startingTimestamp *big.Int, _proposer common.Address, _challenger common.Address, _finalizationPeriodSeconds *big.Int, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "initialize", _submissionInterval, _l2BlockTime, _startingBlockNumber, _startingTimestamp, _proposer, _challenger, _finalizationPeriodSeconds, _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0xe5c325c6.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (uint256,bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Initialize(_submissionInterval *big.Int, _l2BlockTime *big.Int, _startingBlockNumber *big.Int, _startingTimestamp *big.Int, _proposer common.Address, _challenger common.Address, _finalizationPeriodSeconds *big.Int, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.Initialize(&_OPSuccinctL2OutputOracle.TransactOpts, _submissionInterval, _l2BlockTime, _startingBlockNumber, _startingTimestamp, _proposer, _challenger, _finalizationPeriodSeconds, _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0xe5c325c6.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (uint256,bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) Initialize(_submissionInterval *big.Int, _l2BlockTime *big.Int, _startingBlockNumber *big.Int, _startingTimestamp *big.Int, _proposer common.Address, _challenger common.Address, _finalizationPeriodSeconds *big.Int, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.Initialize(&_OPSuccinctL2OutputOracle.TransactOpts, _submissionInterval, _l2BlockTime, _startingBlockNumber, _startingTimestamp, _proposer, _challenger, _finalizationPeriodSeconds, _initParams)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) ProposeL2Output(opts *bind.TransactOpts, _outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "proposeL2Output", _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) ProposeL2Output(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// ProposeL2Output is a paid mutator transaction binding the contract method 0x9ad84880.
//
// Solidity: function proposeL2Output(bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes _proof) payable returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) ProposeL2Output(_outputRoot [32]byte, _l2BlockNumber *big.Int, _l1BlockNumber *big.Int, _proof []byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.ProposeL2Output(&_OPSuccinctL2OutputOracle.TransactOpts, _outputRoot, _l2BlockNumber, _l1BlockNumber, _proof)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _newOwner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) TransferOwnership(opts *bind.TransactOpts, _newOwner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "transferOwnership", _newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _newOwner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) TransferOwnership(_newOwner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.TransferOwnership(&_OPSuccinctL2OutputOracle.TransactOpts, _newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _newOwner) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) TransferOwnership(_newOwner common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.TransferOwnership(&_OPSuccinctL2OutputOracle.TransactOpts, _newOwner)
}

// UpdateAggregationVKey is a paid mutator transaction binding the contract method 0x80fdb3e1.
//
// Solidity: function updateAggregationVKey(bytes32 _aggregationVKey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateAggregationVKey(opts *bind.TransactOpts, _aggregationVKey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateAggregationVKey", _aggregationVKey)
}

// UpdateAggregationVKey is a paid mutator transaction binding the contract method 0x80fdb3e1.
//
// Solidity: function updateAggregationVKey(bytes32 _aggregationVKey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateAggregationVKey(_aggregationVKey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateAggregationVKey(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVKey)
}

// UpdateAggregationVKey is a paid mutator transaction binding the contract method 0x80fdb3e1.
//
// Solidity: function updateAggregationVKey(bytes32 _aggregationVKey) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateAggregationVKey(_aggregationVKey [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateAggregationVKey(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVKey)
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

// UpdateVerifierGateway is a paid mutator transaction binding the contract method 0x4418db5e.
//
// Solidity: function updateVerifierGateway(address _verifierGateway) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpdateVerifierGateway(opts *bind.TransactOpts, _verifierGateway common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "updateVerifierGateway", _verifierGateway)
}

// UpdateVerifierGateway is a paid mutator transaction binding the contract method 0x4418db5e.
//
// Solidity: function updateVerifierGateway(address _verifierGateway) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpdateVerifierGateway(_verifierGateway common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateVerifierGateway(&_OPSuccinctL2OutputOracle.TransactOpts, _verifierGateway)
}

// UpdateVerifierGateway is a paid mutator transaction binding the contract method 0x4418db5e.
//
// Solidity: function updateVerifierGateway(address _verifierGateway) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpdateVerifierGateway(_verifierGateway common.Address) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpdateVerifierGateway(&_OPSuccinctL2OutputOracle.TransactOpts, _verifierGateway)
}

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0x7ad01652.
//
// Solidity: function upgradeWithInitParams(uint256 _chainId, bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpgradeWithInitParams(opts *bind.TransactOpts, _chainId *big.Int, _aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "upgradeWithInitParams", _chainId, _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
}

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0x7ad01652.
//
// Solidity: function upgradeWithInitParams(uint256 _chainId, bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpgradeWithInitParams(_chainId *big.Int, _aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpgradeWithInitParams(&_OPSuccinctL2OutputOracle.TransactOpts, _chainId, _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
}

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0x7ad01652.
//
// Solidity: function upgradeWithInitParams(uint256 _chainId, bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpgradeWithInitParams(_chainId *big.Int, _aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpgradeWithInitParams(&_OPSuccinctL2OutputOracle.TransactOpts, _chainId, _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
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

// OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator is returned from FilterUpdatedAggregationVKey and is used to iterate over the raw logs and unpacked data for UpdatedAggregationVKey events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator struct {
	Event *OPSuccinctL2OutputOracleUpdatedAggregationVKey // Event containing the contract specifics and raw log

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
func (it *OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleUpdatedAggregationVKey)
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
		it.Event = new(OPSuccinctL2OutputOracleUpdatedAggregationVKey)
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
func (it *OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleUpdatedAggregationVKey represents a UpdatedAggregationVKey event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedAggregationVKey struct {
	OldVkey [32]byte
	NewVkey [32]byte
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterUpdatedAggregationVKey is a free log retrieval operation binding the contract event 0xb81f9c41933b730a90fba96ab14541de7cab774f762ea0c183054947bc49aee7.
//
// Solidity: event UpdatedAggregationVKey(bytes32 indexed oldVkey, bytes32 indexed newVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterUpdatedAggregationVKey(opts *bind.FilterOpts, oldVkey [][32]byte, newVkey [][32]byte) (*OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator, error) {

	var oldVkeyRule []interface{}
	for _, oldVkeyItem := range oldVkey {
		oldVkeyRule = append(oldVkeyRule, oldVkeyItem)
	}
	var newVkeyRule []interface{}
	for _, newVkeyItem := range newVkey {
		newVkeyRule = append(newVkeyRule, newVkeyItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "UpdatedAggregationVKey", oldVkeyRule, newVkeyRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleUpdatedAggregationVKeyIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "UpdatedAggregationVKey", logs: logs, sub: sub}, nil
}

// WatchUpdatedAggregationVKey is a free log subscription operation binding the contract event 0xb81f9c41933b730a90fba96ab14541de7cab774f762ea0c183054947bc49aee7.
//
// Solidity: event UpdatedAggregationVKey(bytes32 indexed oldVkey, bytes32 indexed newVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchUpdatedAggregationVKey(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleUpdatedAggregationVKey, oldVkey [][32]byte, newVkey [][32]byte) (event.Subscription, error) {

	var oldVkeyRule []interface{}
	for _, oldVkeyItem := range oldVkey {
		oldVkeyRule = append(oldVkeyRule, oldVkeyItem)
	}
	var newVkeyRule []interface{}
	for _, newVkeyItem := range newVkey {
		newVkeyRule = append(newVkeyRule, newVkeyItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "UpdatedAggregationVKey", oldVkeyRule, newVkeyRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleUpdatedAggregationVKey)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedAggregationVKey", log); err != nil {
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

// ParseUpdatedAggregationVKey is a log parse operation binding the contract event 0xb81f9c41933b730a90fba96ab14541de7cab774f762ea0c183054947bc49aee7.
//
// Solidity: event UpdatedAggregationVKey(bytes32 indexed oldVkey, bytes32 indexed newVkey)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseUpdatedAggregationVKey(log types.Log) (*OPSuccinctL2OutputOracleUpdatedAggregationVKey, error) {
	event := new(OPSuccinctL2OutputOracleUpdatedAggregationVKey)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedAggregationVKey", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator is returned from FilterUpdatedRangeVkeyCommitment and is used to iterate over the raw logs and unpacked data for UpdatedRangeVkeyCommitment events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator struct {
	Event *OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment // Event containing the contract specifics and raw log

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
func (it *OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment)
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
		it.Event = new(OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment)
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
func (it *OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment represents a UpdatedRangeVkeyCommitment event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment struct {
	OldRangeVkeyCommitment [32]byte
	NewRangeVkeyCommitment [32]byte
	Raw                    types.Log // Blockchain specific contextual infos
}

// FilterUpdatedRangeVkeyCommitment is a free log retrieval operation binding the contract event 0x1035606f0606905acdf851342466a5b64406fa798b7440235cd5811cea2850fd.
//
// Solidity: event UpdatedRangeVkeyCommitment(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterUpdatedRangeVkeyCommitment(opts *bind.FilterOpts, oldRangeVkeyCommitment [][32]byte, newRangeVkeyCommitment [][32]byte) (*OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator, error) {

	var oldRangeVkeyCommitmentRule []interface{}
	for _, oldRangeVkeyCommitmentItem := range oldRangeVkeyCommitment {
		oldRangeVkeyCommitmentRule = append(oldRangeVkeyCommitmentRule, oldRangeVkeyCommitmentItem)
	}
	var newRangeVkeyCommitmentRule []interface{}
	for _, newRangeVkeyCommitmentItem := range newRangeVkeyCommitment {
		newRangeVkeyCommitmentRule = append(newRangeVkeyCommitmentRule, newRangeVkeyCommitmentItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "UpdatedRangeVkeyCommitment", oldRangeVkeyCommitmentRule, newRangeVkeyCommitmentRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitmentIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "UpdatedRangeVkeyCommitment", logs: logs, sub: sub}, nil
}

// WatchUpdatedRangeVkeyCommitment is a free log subscription operation binding the contract event 0x1035606f0606905acdf851342466a5b64406fa798b7440235cd5811cea2850fd.
//
// Solidity: event UpdatedRangeVkeyCommitment(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchUpdatedRangeVkeyCommitment(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment, oldRangeVkeyCommitment [][32]byte, newRangeVkeyCommitment [][32]byte) (event.Subscription, error) {

	var oldRangeVkeyCommitmentRule []interface{}
	for _, oldRangeVkeyCommitmentItem := range oldRangeVkeyCommitment {
		oldRangeVkeyCommitmentRule = append(oldRangeVkeyCommitmentRule, oldRangeVkeyCommitmentItem)
	}
	var newRangeVkeyCommitmentRule []interface{}
	for _, newRangeVkeyCommitmentItem := range newRangeVkeyCommitment {
		newRangeVkeyCommitmentRule = append(newRangeVkeyCommitmentRule, newRangeVkeyCommitmentItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "UpdatedRangeVkeyCommitment", oldRangeVkeyCommitmentRule, newRangeVkeyCommitmentRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedRangeVkeyCommitment", log); err != nil {
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

// ParseUpdatedRangeVkeyCommitment is a log parse operation binding the contract event 0x1035606f0606905acdf851342466a5b64406fa798b7440235cd5811cea2850fd.
//
// Solidity: event UpdatedRangeVkeyCommitment(bytes32 indexed oldRangeVkeyCommitment, bytes32 indexed newRangeVkeyCommitment)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseUpdatedRangeVkeyCommitment(log types.Log) (*OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment, error) {
	event := new(OPSuccinctL2OutputOracleUpdatedRangeVkeyCommitment)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedRangeVkeyCommitment", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator is returned from FilterUpdatedRollupConfigHash and is used to iterate over the raw logs and unpacked data for UpdatedRollupConfigHash events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator struct {
	Event *OPSuccinctL2OutputOracleUpdatedRollupConfigHash // Event containing the contract specifics and raw log

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
func (it *OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleUpdatedRollupConfigHash)
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
		it.Event = new(OPSuccinctL2OutputOracleUpdatedRollupConfigHash)
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
func (it *OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleUpdatedRollupConfigHash represents a UpdatedRollupConfigHash event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedRollupConfigHash struct {
	OldRollupConfigHash [32]byte
	NewRollupConfigHash [32]byte
	Raw                 types.Log // Blockchain specific contextual infos
}

// FilterUpdatedRollupConfigHash is a free log retrieval operation binding the contract event 0xda2f5f014ada26cff39a0f2a9dc6fa4fca1581376fc91ec09506c8fb8657bc35.
//
// Solidity: event UpdatedRollupConfigHash(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterUpdatedRollupConfigHash(opts *bind.FilterOpts, oldRollupConfigHash [][32]byte, newRollupConfigHash [][32]byte) (*OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator, error) {

	var oldRollupConfigHashRule []interface{}
	for _, oldRollupConfigHashItem := range oldRollupConfigHash {
		oldRollupConfigHashRule = append(oldRollupConfigHashRule, oldRollupConfigHashItem)
	}
	var newRollupConfigHashRule []interface{}
	for _, newRollupConfigHashItem := range newRollupConfigHash {
		newRollupConfigHashRule = append(newRollupConfigHashRule, newRollupConfigHashItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "UpdatedRollupConfigHash", oldRollupConfigHashRule, newRollupConfigHashRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleUpdatedRollupConfigHashIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "UpdatedRollupConfigHash", logs: logs, sub: sub}, nil
}

// WatchUpdatedRollupConfigHash is a free log subscription operation binding the contract event 0xda2f5f014ada26cff39a0f2a9dc6fa4fca1581376fc91ec09506c8fb8657bc35.
//
// Solidity: event UpdatedRollupConfigHash(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchUpdatedRollupConfigHash(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleUpdatedRollupConfigHash, oldRollupConfigHash [][32]byte, newRollupConfigHash [][32]byte) (event.Subscription, error) {

	var oldRollupConfigHashRule []interface{}
	for _, oldRollupConfigHashItem := range oldRollupConfigHash {
		oldRollupConfigHashRule = append(oldRollupConfigHashRule, oldRollupConfigHashItem)
	}
	var newRollupConfigHashRule []interface{}
	for _, newRollupConfigHashItem := range newRollupConfigHash {
		newRollupConfigHashRule = append(newRollupConfigHashRule, newRollupConfigHashItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "UpdatedRollupConfigHash", oldRollupConfigHashRule, newRollupConfigHashRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleUpdatedRollupConfigHash)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedRollupConfigHash", log); err != nil {
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

// ParseUpdatedRollupConfigHash is a log parse operation binding the contract event 0xda2f5f014ada26cff39a0f2a9dc6fa4fca1581376fc91ec09506c8fb8657bc35.
//
// Solidity: event UpdatedRollupConfigHash(bytes32 indexed oldRollupConfigHash, bytes32 indexed newRollupConfigHash)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseUpdatedRollupConfigHash(log types.Log) (*OPSuccinctL2OutputOracleUpdatedRollupConfigHash, error) {
	event := new(OPSuccinctL2OutputOracleUpdatedRollupConfigHash)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedRollupConfigHash", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator is returned from FilterUpdatedVerifierGateway and is used to iterate over the raw logs and unpacked data for UpdatedVerifierGateway events raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator struct {
	Event *OPSuccinctL2OutputOracleUpdatedVerifierGateway // Event containing the contract specifics and raw log

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
func (it *OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(OPSuccinctL2OutputOracleUpdatedVerifierGateway)
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
		it.Event = new(OPSuccinctL2OutputOracleUpdatedVerifierGateway)
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
func (it *OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// OPSuccinctL2OutputOracleUpdatedVerifierGateway represents a UpdatedVerifierGateway event raised by the OPSuccinctL2OutputOracle contract.
type OPSuccinctL2OutputOracleUpdatedVerifierGateway struct {
	OldVerifierGateway common.Address
	NewVerifierGateway common.Address
	Raw                types.Log // Blockchain specific contextual infos
}

// FilterUpdatedVerifierGateway is a free log retrieval operation binding the contract event 0x1379941631ff0ed9178ab16ab67a2e5db3aeada7f87e518f761e79c8e38377e3.
//
// Solidity: event UpdatedVerifierGateway(address indexed oldVerifierGateway, address indexed newVerifierGateway)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) FilterUpdatedVerifierGateway(opts *bind.FilterOpts, oldVerifierGateway []common.Address, newVerifierGateway []common.Address) (*OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator, error) {

	var oldVerifierGatewayRule []interface{}
	for _, oldVerifierGatewayItem := range oldVerifierGateway {
		oldVerifierGatewayRule = append(oldVerifierGatewayRule, oldVerifierGatewayItem)
	}
	var newVerifierGatewayRule []interface{}
	for _, newVerifierGatewayItem := range newVerifierGateway {
		newVerifierGatewayRule = append(newVerifierGatewayRule, newVerifierGatewayItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.FilterLogs(opts, "UpdatedVerifierGateway", oldVerifierGatewayRule, newVerifierGatewayRule)
	if err != nil {
		return nil, err
	}
	return &OPSuccinctL2OutputOracleUpdatedVerifierGatewayIterator{contract: _OPSuccinctL2OutputOracle.contract, event: "UpdatedVerifierGateway", logs: logs, sub: sub}, nil
}

// WatchUpdatedVerifierGateway is a free log subscription operation binding the contract event 0x1379941631ff0ed9178ab16ab67a2e5db3aeada7f87e518f761e79c8e38377e3.
//
// Solidity: event UpdatedVerifierGateway(address indexed oldVerifierGateway, address indexed newVerifierGateway)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) WatchUpdatedVerifierGateway(opts *bind.WatchOpts, sink chan<- *OPSuccinctL2OutputOracleUpdatedVerifierGateway, oldVerifierGateway []common.Address, newVerifierGateway []common.Address) (event.Subscription, error) {

	var oldVerifierGatewayRule []interface{}
	for _, oldVerifierGatewayItem := range oldVerifierGateway {
		oldVerifierGatewayRule = append(oldVerifierGatewayRule, oldVerifierGatewayItem)
	}
	var newVerifierGatewayRule []interface{}
	for _, newVerifierGatewayItem := range newVerifierGateway {
		newVerifierGatewayRule = append(newVerifierGatewayRule, newVerifierGatewayItem)
	}

	logs, sub, err := _OPSuccinctL2OutputOracle.contract.WatchLogs(opts, "UpdatedVerifierGateway", oldVerifierGatewayRule, newVerifierGatewayRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(OPSuccinctL2OutputOracleUpdatedVerifierGateway)
				if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedVerifierGateway", log); err != nil {
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

// ParseUpdatedVerifierGateway is a log parse operation binding the contract event 0x1379941631ff0ed9178ab16ab67a2e5db3aeada7f87e518f761e79c8e38377e3.
//
// Solidity: event UpdatedVerifierGateway(address indexed oldVerifierGateway, address indexed newVerifierGateway)
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleFilterer) ParseUpdatedVerifierGateway(log types.Log) (*OPSuccinctL2OutputOracleUpdatedVerifierGateway, error) {
	event := new(OPSuccinctL2OutputOracleUpdatedVerifierGateway)
	if err := _OPSuccinctL2OutputOracle.contract.UnpackLog(event, "UpdatedVerifierGateway", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
