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
	ABI: "[{\"type\":\"constructor\",\"inputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"CHALLENGER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"FINALIZATION_PERIOD_SECONDS\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"L2_BLOCK_TIME\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"PROPOSER\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"SUBMISSION_INTERVAL\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"aggregationVkey\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"challenger\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"checkpointBlockHash\",\"inputs\":[{\"name\":\"_blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"computeL2Timestamp\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"deleteL2Outputs\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"finalizationPeriodSeconds\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2Output\",\"inputs\":[{\"name\":\"_l2OutputIndex\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structTypes.OutputProposal\",\"components\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"timestamp\",\"type\":\"uint128\",\"internalType\":\"uint128\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint128\",\"internalType\":\"uint128\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"getL2OutputIndexAfter\",\"inputs\":[{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"historicBlockHashes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_submissionInterval\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l2BlockTime\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_startingBlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_startingTimestamp\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_proposer\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_challenger\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_finalizationPeriodSeconds\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_initParams\",\"type\":\"tuple\",\"internalType\":\"structOPSuccinctL2OutputOracle.InitParams\",\"components\":[{\"name\":\"aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"startingOutputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"owner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"l2BlockTime\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"nextOutputIndex\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"proposeL2Output\",\"inputs\":[{\"name\":\"_outputRoot\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_l2BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_l1BlockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"_proof\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"outputs\":[],\"stateMutability\":\"payable\"},{\"type\":\"function\",\"name\":\"proposer\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rangeVkeyCommitment\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"rollupConfigHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingBlockNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"startingTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"submissionInterval\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"_newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateAggregationVKey\",\"inputs\":[{\"name\":\"_aggregationVKey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRangeVkeyCommitment\",\"inputs\":[{\"name\":\"_rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateRollupConfigHash\",\"inputs\":[{\"name\":\"_rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updateVerifierGateway\",\"inputs\":[{\"name\":\"_verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"upgradeWithInitParams\",\"inputs\":[{\"name\":\"_aggregationVkey\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_rangeVkeyCommitment\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"_verifierGateway\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rollupConfigHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"verifierGateway\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractSP1VerifierGateway\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"version\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"string\",\"internalType\":\"string\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputProposed\",\"inputs\":[{\"name\":\"outputRoot\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"l2OutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l2BlockNumber\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"l1Timestamp\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OutputsDeleted\",\"inputs\":[{\"name\":\"prevNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"},{\"name\":\"newNextOutputIndex\",\"type\":\"uint256\",\"indexed\":true,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedAggregationVKey\",\"inputs\":[{\"name\":\"oldVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newVkey\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedRangeVkeyCommitment\",\"inputs\":[{\"name\":\"oldRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRangeVkeyCommitment\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedRollupConfigHash\",\"inputs\":[{\"name\":\"oldRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"},{\"name\":\"newRollupConfigHash\",\"type\":\"bytes32\",\"indexed\":true,\"internalType\":\"bytes32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"UpdatedVerifierGateway\",\"inputs\":[{\"name\":\"oldVerifierGateway\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newVerifierGateway\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"CallerIsNotOwner\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"CallerNotAuthorized\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"CannotDeleteFinalizedOutputs\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidOutputRoot\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InvalidParameter\",\"inputs\":[{\"name\":\"paramName\",\"type\":\"string\",\"internalType\":\"string\"}]},{\"type\":\"error\",\"name\":\"L1BlockHashNotAvailable\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L1BlockHashNotCheckpointed\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L2BlockNumberTooHigh\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L2BlockNumberTooLow\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L2BlockProposedInFuture\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"L2OutputIndexOutOfBounds\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"NoOutputsProposed\",\"inputs\":[]}]",
	Bin: "0x608060405234801561001057600080fd5b5061001961001e565b6100de565b600054610100900460ff161561008a5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156100dc576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b61165a806100ed6000396000f3fe6080604052600436106102195760003560e01c806389c44cbb11610123578063c32e4e3e116100ab578063e01e2c971161006f578063e01e2c971461060c578063e1a41bcf1461062c578063f2fde38b14610642578063f4daa29114610662578063fb3c491c1461067757600080fd5b8063c32e4e3e1461058b578063ce5db8d6146105a1578063cf8e5cf0146105b7578063d1de856c146105d7578063dcec3348146105f757600080fd5b8063a196b525116100f2578063a196b525146104ad578063a25ae557146104da578063a8e4fb901461052d578063bc91ce331461054d578063bffa7f0f1461056d57600080fd5b806389c44cbb146104445780638da5cb5b1461046457806393991af3146104845780639ad848801461049a57600080fd5b806354fd4d50116101a65780636d9a1c8b116101755780636d9a1c8b146103c257806370872aa5146103d85780637f006420146103ee57806380fdb3e11461040e578063887862721461042e57600080fd5b806354fd4d501461033c57806369f16eec1461037a5780636abcf5631461038f5780636b4d98dd146103a457600080fd5b80632b31841e116101ed5780632b31841e146102a45780634418db5e146102ba5780634599c788146102da578063529933df146102ef578063534db0e21461030457600080fd5b80622134cc1461021e578063166140b2146102425780631bdd450c146102645780631e85680014610284575b600080fd5b34801561022a57600080fd5b506005545b6040519081526020015b60405180910390f35b34801561024e57600080fd5b5061026261025d3660046112f8565b610697565b005b34801561027057600080fd5b5061026261027f3660046113c9565b610980565b34801561029057600080fd5b5061026261029f3660046113c9565b6109b7565b3480156102b057600080fd5b5061022f600a5481565b3480156102c657600080fd5b506102626102d53660046113e2565b6109e9565b3480156102e657600080fd5b5061022f610a1d565b3480156102fb57600080fd5b5060045461022f565b34801561031057600080fd5b50600654610324906001600160a01b031681565b6040516001600160a01b039091168152602001610239565b34801561034857600080fd5b5061036d604051806040016040528060058152602001640322e302e360dc1b81525081565b6040516102399190611451565b34801561038657600080fd5b5061022f610a7a565b34801561039b57600080fd5b5060035461022f565b3480156103b057600080fd5b506006546001600160a01b0316610324565b3480156103ce57600080fd5b5061022f600d5481565b3480156103e457600080fd5b5061022f60015481565b3480156103fa57600080fd5b5061022f6104093660046113c9565b610a8c565b34801561041a57600080fd5b506102626104293660046113c9565b610b66565b34801561043a57600080fd5b5061022f60025481565b34801561045057600080fd5b5061026261045f3660046113c9565b610b9a565b34801561047057600080fd5b50600c54610324906001600160a01b031681565b34801561049057600080fd5b5061022f60055481565b6102626104a8366004611464565b610c85565b3480156104b957600080fd5b5061022f6104c83660046113c9565b600e6020526000908152604090205481565b3480156104e657600080fd5b506104fa6104f53660046113c9565b610f62565b60408051825181526020808401516001600160801b03908116918301919091529282015190921690820152606001610239565b34801561053957600080fd5b50600754610324906001600160a01b031681565b34801561055957600080fd5b506102626105683660046113c9565b610fe0565b34801561057957600080fd5b506007546001600160a01b0316610324565b34801561059757600080fd5b5061022f60095481565b3480156105ad57600080fd5b5061022f60085481565b3480156105c357600080fd5b506104fa6105d23660046113c9565b611014565b3480156105e357600080fd5b5061022f6105f23660046113c9565b61104c565b34801561060357600080fd5b5061022f61107c565b34801561061857600080fd5b50610262610627366004611516565b611093565b34801561063857600080fd5b5061022f60045481565b34801561064e57600080fd5b5061026261065d3660046113e2565b6110e8565b34801561066e57600080fd5b5060085461022f565b34801561068357600080fd5b50600b54610324906001600160a01b031681565b600054600290610100900460ff161580156106b9575060005460ff8083169116105b6107215760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805461ffff191660ff83161761010017815589900361077a576040516305519d6f60e51b81526020600482015260126024820152711cdd589b5a5cdcda5bdb925b9d195c9d985b60721b6044820152606401610718565b876000036107b9576040516305519d6f60e51b815260206004820152600b60248201526a6c32426c6f636b54696d6560a81b6044820152606401610718565b428611156107fe576040516305519d6f60e51b815260206004820152601160248201527007374617274696e6754696d657374616d7607c1b6044820152606401610718565b60048990556005889055600780546001600160a01b038088166001600160a01b031992831617909255600680549287169290911691909117905560088390556003546000036108f557604080516060808201835284015181526001600160801b03808916602083019081528a82169383019384526003805460018181018355600092909252935160029485027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810191909155915194518316600160801b0294909216939093177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c90930192909255908890558690555b6109028260800151611118565b815161090d90611174565b61091a82602001516111a8565b61092782604001516111dc565b6109348260a00151611238565b6000805461ff001916905560405160ff821681527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a1505050505050505050565b600c546001600160a01b031633146109ab57604051636db2465f60e01b815260040160405180910390fd5b6109b481611238565b50565b8040806109d7576040516321301a1960e21b815260040160405180910390fd5b6000918252600e602052604090912055565b600c546001600160a01b03163314610a1457604051636db2465f60e01b815260040160405180910390fd5b6109b4816111dc565b60035460009015610a715760038054610a3890600190611569565b81548110610a4857610a48611580565b6000918252602090912060029091020160010154600160801b90046001600160801b0316919050565b6001545b905090565b600354600090610a7590600190611569565b6000610a96610a1d565b821115610ab65760405163df4acadb60e01b815260040160405180910390fd5b600354600003610adc57604051600162294f6560e11b0319815260040160405180910390fd5b6003546000905b80821015610b5f5760006002610af98385611596565b610b0391906115ae565b90508460038281548110610b1957610b19611580565b6000918252602090912060029091020160010154600160801b90046001600160801b03161015610b5557610b4e816001611596565b9250610b59565b8091505b50610ae3565b5092915050565b600c546001600160a01b03163314610b9157604051636db2465f60e01b815260040160405180910390fd5b6109b481611174565b6006546001600160a01b03163314610bc55760405163c183bcef60e01b815260040160405180910390fd5b6003548110610be7576040516336968cd360e11b815260040160405180910390fd5b60085460038281548110610bfd57610bfd611580565b6000918252602090912060016002909202010154610c24906001600160801b031642611569565b10610c425760405163329e0cff60e01b815260040160405180910390fd5b6000610c4d60035490565b90508160035581817f4ee37ac2c786ec85e87592d3c5c8a1dd66f8496dda3f125d9ea8ca5f657629b660405160405180910390a35050565b6007546001600160a01b03163314801590610caa57506007546001600160a01b031615155b15610cc85760405163c183bcef60e01b815260040160405180910390fd5b610cd061107c565b831015610cf05760405163478860f560e01b815260040160405180910390fd5b42610cfa8461104c565b10610d18576040516321a13c5b60e01b815260040160405180910390fd5b83610d36576040516315ae1f5d60e31b815260040160405180910390fd5b6000828152600e602052604090205480610d6357604051630455475360e31b815260040160405180910390fd5b60006040518060c001604052808381526020016003610d80610a7a565b81548110610d9057610d90611580565b60009182526020918290206002909102015482528181018990526040808301899052600d54606080850191909152600a54608094850152600b5460095483518751818701529487015185850152928601518483015290850151838501529284015160a08084019190915284015160c08301529293506001600160a01b03909116916341493c609160e001604051602081830303815290604052866040518463ffffffff1660e01b8152600401610e48939291906115d0565b60006040518083038186803b158015610e6057600080fd5b505afa158015610e74573d6000803e3d6000fd5b5050505084610e8260035490565b877fa7aaf2512769da4e444e3de247be2564225c2e7a8f74cfe528e46e17d24868e242604051610eb491815260200190565b60405180910390a45050604080516060810182529485526001600160801b034281166020870190815294811691860191825260038054600181018255600091909152955160029096027fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b810196909655935190518416600160801b029316929092177fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85c909301929092555050565b604080516060810182526000808252602082018190529181019190915260038281548110610f9257610f92611580565b600091825260209182902060408051606081018252600290930290910180548352600101546001600160801b0380821694840194909452600160801b90049092169181019190915292915050565b600c546001600160a01b0316331461100b57604051636db2465f60e01b815260040160405180910390fd5b6109b4816111a8565b6040805160608101825260008082526020820181905291810191909152600361103c83610a8c565b81548110610f9257610f92611580565b60006005546001548361105f9190611569565b6110699190611605565b6002546110769190611596565b92915050565b6000600454611089610a1d565b610a759190611596565b600c546001600160a01b031633146110be57604051636db2465f60e01b815260040160405180910390fd5b6110c784611174565b6110d0836111a8565b6110d9826111dc565b6110e281611238565b50505050565b600c546001600160a01b0316331461111357604051636db2465f60e01b815260040160405180910390fd5b6109b4815b600c546040516001600160a01b038084169216907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a3600c80546001600160a01b0319166001600160a01b0392909216919091179055565b6009546040518291907fb81f9c41933b730a90fba96ab14541de7cab774f762ea0c183054947bc49aee790600090a3600955565b600a546040518291907f1035606f0606905acdf851342466a5b64406fa798b7440235cd5811cea2850fd90600090a3600a55565b600b546040516001600160a01b038084169216907f1379941631ff0ed9178ab16ab67a2e5db3aeada7f87e518f761e79c8e38377e390600090a3600b80546001600160a01b0319166001600160a01b0392909216919091179055565b600d546040518291907fda2f5f014ada26cff39a0f2a9dc6fa4fca1581376fc91ec09506c8fb8657bc3590600090a3600d55565b80356001600160a01b038116811461128357600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b60405160c0810167ffffffffffffffff811182821017156112c1576112c1611288565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156112f0576112f0611288565b604052919050565b600080600080600080600080888a036101a081121561131657600080fd5b8935985060208a0135975060408a0135965060608a0135955061133b60808b0161126c565b945061134960a08b0161126c565b935060c08a810135935060df198201121561136357600080fd5b5061136c61129e565b60e08a013581526101008a0135602082015261138b6101208b0161126c565b60408201526101408a013560608201526113a86101608b0161126c565b60808201526101808a013560a0820152809150509295985092959890939650565b6000602082840312156113db57600080fd5b5035919050565b6000602082840312156113f457600080fd5b6113fd8261126c565b9392505050565b6000815180845260005b8181101561142a5760208185018101518683018201520161140e565b8181111561143c576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006113fd6020830184611404565b6000806000806080858703121561147a57600080fd5b84359350602080860135935060408601359250606086013567ffffffffffffffff808211156114a857600080fd5b818801915088601f8301126114bc57600080fd5b8135818111156114ce576114ce611288565b6114e0601f8201601f191685016112c7565b915080825289848285010111156114f657600080fd5b808484018584013760008482840101525080935050505092959194509250565b6000806000806080858703121561152c57600080fd5b84359350602085013592506115436040860161126c565b9396929550929360600135925050565b634e487b7160e01b600052601160045260246000fd5b60008282101561157b5761157b611553565b500390565b634e487b7160e01b600052603260045260246000fd5b600082198211156115a9576115a9611553565b500190565b6000826115cb57634e487b7160e01b600052601260045260246000fd5b500490565b8381526060602082015260006115e96060830185611404565b82810360408401526115fb8185611404565b9695505050505050565b600081600019048311821515161561161f5761161f611553565b50029056fea2646970667358221220f30851f3d5704c7b1da984fe50d8db744df2aebe4047ca487fb43121c6301a4764736f6c634300080f0033",
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

// Initialize is a paid mutator transaction binding the contract method 0x166140b2.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) Initialize(opts *bind.TransactOpts, _submissionInterval *big.Int, _l2BlockTime *big.Int, _startingBlockNumber *big.Int, _startingTimestamp *big.Int, _proposer common.Address, _challenger common.Address, _finalizationPeriodSeconds *big.Int, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "initialize", _submissionInterval, _l2BlockTime, _startingBlockNumber, _startingTimestamp, _proposer, _challenger, _finalizationPeriodSeconds, _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0x166140b2.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) Initialize(_submissionInterval *big.Int, _l2BlockTime *big.Int, _startingBlockNumber *big.Int, _startingTimestamp *big.Int, _proposer common.Address, _challenger common.Address, _finalizationPeriodSeconds *big.Int, _initParams OPSuccinctL2OutputOracleInitParams) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.Initialize(&_OPSuccinctL2OutputOracle.TransactOpts, _submissionInterval, _l2BlockTime, _startingBlockNumber, _startingTimestamp, _proposer, _challenger, _finalizationPeriodSeconds, _initParams)
}

// Initialize is a paid mutator transaction binding the contract method 0x166140b2.
//
// Solidity: function initialize(uint256 _submissionInterval, uint256 _l2BlockTime, uint256 _startingBlockNumber, uint256 _startingTimestamp, address _proposer, address _challenger, uint256 _finalizationPeriodSeconds, (bytes32,bytes32,address,bytes32,address,bytes32) _initParams) returns()
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

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0xe01e2c97.
//
// Solidity: function upgradeWithInitParams(bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactor) UpgradeWithInitParams(opts *bind.TransactOpts, _aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.contract.Transact(opts, "upgradeWithInitParams", _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
}

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0xe01e2c97.
//
// Solidity: function upgradeWithInitParams(bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleSession) UpgradeWithInitParams(_aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpgradeWithInitParams(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
}

// UpgradeWithInitParams is a paid mutator transaction binding the contract method 0xe01e2c97.
//
// Solidity: function upgradeWithInitParams(bytes32 _aggregationVkey, bytes32 _rangeVkeyCommitment, address _verifierGateway, bytes32 _rollupConfigHash) returns()
func (_OPSuccinctL2OutputOracle *OPSuccinctL2OutputOracleTransactorSession) UpgradeWithInitParams(_aggregationVkey [32]byte, _rangeVkeyCommitment [32]byte, _verifierGateway common.Address, _rollupConfigHash [32]byte) (*types.Transaction, error) {
	return _OPSuccinctL2OutputOracle.Contract.UpgradeWithInitParams(&_OPSuccinctL2OutputOracle.TransactOpts, _aggregationVkey, _rangeVkeyCommitment, _verifierGateway, _rollupConfigHash)
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
