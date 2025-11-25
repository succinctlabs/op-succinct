package utils

import (
	"context"
	"fmt"
	"math/big"
	"strings"

	"github.com/ethereum-optimism/optimism/op-service/apis"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/rpc"
	opsbind "github.com/succinctlabs/op-succinct/bindings"
)

// contractClient is a generic Ethereum contract client.
type contractClient struct {
	client apis.EthClient
	addr   common.Address
	abi    abi.ABI
}

// L2OOClient is a client for interacting with the SuccinctL2OutputOracle contract.
type L2OOClient struct {
	contractClient
}

// NewL2OOClient creates a new L2OOClient.
func NewL2OOClient(client apis.EthClient, l2ooAddr common.Address) (*L2OOClient, error) {
	parsedABI, err := abi.JSON(strings.NewReader(opsbind.OPSuccinctL2OutputOracleMetaData.ABI))
	if err != nil {
		return nil, fmt.Errorf("parse L2OO ABI: %w", err)
	}

	return &L2OOClient{
		contractClient: contractClient{
			client: client,
			addr:   l2ooAddr,
			abi:    parsedABI,
		},
	}, nil
}

// LatestBlockNumber fetches the latest L2 block number from the contract.
func (l2oo *L2OOClient) LatestBlockNumber(ctx context.Context) (uint64, error) {
	return call(ctx, l2oo.contractClient, "latestBlockNumber", asUint64)
}

// NextBlockNumber fetches the next L2 block number to be submitted by the proposer.
func (l2oo *L2OOClient) NextBlockNumber(ctx context.Context) (uint64, error) {
	return call(ctx, l2oo.contractClient, "nextBlockNumber", asUint64)
}

// DgfClient is a client for interacting with the DisputeGameFactory contract.
type DgfClient struct {
	contractClient
}

// NewDgfClient creates a new DgfClient.
func NewDgfClient(client apis.EthClient, dfgAddr common.Address) (*DgfClient, error) {
	parsedABI, err := abi.JSON(strings.NewReader(opsbind.DisputeGameFactoryMetaData.ABI))
	if err != nil {
		return nil, fmt.Errorf("parse Dispute Game Factory ABI: %w", err)
	}

	return &DgfClient{
		contractClient: contractClient{
			client: client,
			addr:   dfgAddr,
			abi:    parsedABI,
		},
	}, nil
}

// GameCount fetches the number of dispute games created.
func (dfg *DgfClient) GameCount(ctx context.Context) (uint64, error) {
	return call(ctx, dfg.contractClient, "gameCount", asUint64)
}

// call is a generic function to call a contract method and convert the output.
func call[R any](ctx context.Context, c contractClient, method string, convert func(string, []any) (R, error)) (R, error) {
	var zero R

	data, err := c.abi.Pack(method)
	if err != nil {
		return zero, fmt.Errorf("pack %s call: %w", method, err)
	}

	callMsg := ethereum.CallMsg{
		To:   &c.addr,
		Data: data,
	}

	raw, err := c.client.Call(ctx, callMsg, rpc.LatestBlockNumber)
	if err != nil {
		return zero, fmt.Errorf("call %s: %w", method, err)
	}

	outs, err := c.abi.Unpack(method, raw)
	if err != nil {
		return zero, fmt.Errorf("unpack %s: %w", method, err)
	}

	return convert(method, outs)
}

// asUint64 converts the output of a contract call to uint64.
func asUint64(method string, outputs []any) (uint64, error) {
	if len(outputs) != 1 {
		return 0, fmt.Errorf("unexpected number of outputs from %s", method)
	}

	value, ok := outputs[0].(*big.Int)
	if !ok {
		return 0, fmt.Errorf("unexpected output type from %s: %T", method, outputs[0])
	}
	return value.Uint64(), nil
}
