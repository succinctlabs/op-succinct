package e2e

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/stretchr/testify/require"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestMain(m *testing.M) {
	presets.DoMain(m,
		opspresets.WithSuccinctFaultProofProposer(&sysgo.DefaultSingleChainInteropSystemIDs{}),
		presets.WithSafeDBEnabled(),
	)
}

func TestFaultProofProposer_L2DgfDeployedAndUp(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys := presets.NewMinimalWithProposer(t)

	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	t.Logger().Info("Dispute Game Factory Address:", "address", dgfAddr.Hex())

	dgf, err := utils.NewDgfClient(sys.L1EL.EthClient(), dgfAddr)
	require.NoError(t, err, "failed to create DGF client")

	gameCount, err := dgf.GameCount(t.Ctx())
	require.NoError(t, err, "failed to get game count from DGF")
	t.Logger().Info("Dispute Game Count:", "count", gameCount)
	require.Equal(t, uint64(0), gameCount, "expected zero dispute games initially")
}

func TestFaultProofProposer_DetectsFirstGameCreated(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys := presets.NewMinimalWithProposer(t)

	ctx, cancel := context.WithTimeout(t.Ctx(), 5*time.Minute)
	defer cancel()

	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	t.Logger().Info("Dispute Game Factory Address:", "address", dgfAddr.Hex())
	dgf, err := utils.NewDgfClient(sys.L1EL.EthClient(), dgfAddr)

	require.NoError(t, err, "failed to create DGF client")

	for {
		gameCount, err := dgf.GameCount(t.Ctx())
		require.NoError(t, err, "failed to get game count from DGF")

		if gameCount >= 1 {
			break
		}

		select {
		case <-ctx.Done():
			t.Errorf("timeout waiting for dispute game to be created")
		case <-time.After(time.Second):
		}
	}
}
