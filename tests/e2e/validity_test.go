package e2e

import (
	"context"
	"math/big"
	"strings"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/rpc"
	"github.com/stretchr/testify/require"
	opsbind "github.com/succinctlabs/op-succinct/bindings"
	opspresets "github.com/succinctlabs/op-succinct/presets"
)

func TestMain(m *testing.M) {
	presets.DoMain(m,
		presets.WithTimeTravel(),
		opspresets.WithSuccinctValidityProposer(&sysgo.DefaultMinimalSystemIDs{}),
	)
}

func TestValidityProposer_L2OODeployedAndUp(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys := presets.NewMinimal(t)

	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	t.Logger().Info("L2 Output Oracle Address:", "address", l2ooAddr.Hex())

	ctx := context.Background()
	ethClient := sys.L1EL.EthClient()

	parsedABI, err := abi.JSON(strings.NewReader(opsbind.OPSuccinctL2OutputOracleMetaData.ABI))
	require.NoError(t, err, "failed to parse OPSuccinctL2OutputOracle ABI")

	data, err := parsedABI.Pack("latestBlockNumber")
	require.NoError(t, err, "failed to pack latestBlockNumber call")

	callMsg := ethereum.CallMsg{
		To:   &l2ooAddr,
		Data: data,
	}

	raw, err := ethClient.Call(ctx, callMsg, rpc.LatestBlockNumber)
	require.NoError(t, err, "eth_call to L2OO failed")

	outs, err := parsedABI.Unpack("latestBlockNumber", raw)
	require.NoError(t, err, "failed to unpack latestBlockNumber result")
	require.Len(t, outs, 1)

	latestBlock := outs[0].(*big.Int)
	t.Logger().Info("Latest L2 block number from L2OO", "block", latestBlock.Uint64())
	require.Equal(t, uint64(0), latestBlock.Uint64(), "expected latest L2 block number to be 0")
}
