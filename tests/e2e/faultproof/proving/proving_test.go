package faultproof

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestMain(m *testing.M) {
	presets.DoMain(m,
		opspresets.WithSuccinctFPProposerFastFinality(&sysgo.DefaultSingleChainInteropSystemIDs{}),
		presets.WithSafeDBEnabled(),
	)
}

func TestFaultProofProposer_WaitsForFirstGameDefenderWins(gt *testing.T) {
	waitForDefenderWinsAtIndex(gt, 0, 5*time.Minute)
}

func TestFaultProofProposer_WaitsForTenthGameDefenderWins(gt *testing.T) {
	waitForDefenderWinsAtIndex(gt, 9, 30*time.Minute)
}

func waitForDefenderWinsAtIndex(gt *testing.T, index int, timeout time.Duration) {
	t := devtest.SerialT(gt)
	sys := presets.NewMinimalWithProposer(t)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), timeout)
	defer cancel()

	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	logger.Info("Dispute Game Factory Address:", "address", dgfAddr.Hex())
	dgf, err := utils.NewDgfClient(sys.L1EL.EthClient(), dgfAddr)
	require.NoError(err, "failed to create Dispute Game Factory client")

	utils.WaitForGameCount(ctx, t, dgf, uint64(index+1))

	game, err := dgf.GameAtIndex(ctx, uint64(index))
	require.NoError(err, "failed to get game from factory")

	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
	require.NoError(err, "failed to create Fault Dispute Game client")

	utils.WaitForDefenderWins(ctx, t, fdg)
	t.Logger().Info("Dispute game defender wins", "gameIndex", index)
}
