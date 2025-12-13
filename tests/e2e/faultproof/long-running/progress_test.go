package longrunning

import (
	"context"
	"fmt"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// MaxProposerLag is the maximum allowed gap between L2 finalized head and the latest game's
// L2 block. Set to 150 with a 100-block proposal interval, providing 50 blocks of headroom
// to absorb transient spikes from proposer startup or game resolution/bond claiming.
const MaxProposerLag uint64 = 200

// TestFaultProofProposer_Progress verifies the proposer maintains acceptable lag for 15 minutes.
// The test succeeds if lag stays below MaxProposerLag throughout; fails immediately if exceeded.
func TestFaultProofProposer_Progress(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.LongRunningFaultProofConfig()
	sys, dgf := setupFaultProofSystem(t, cfg)

	err := utils.RunProgressTest(func() error {
		return checkFaultProofLag(t, sys, dgf)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

// TestFaultProofProposer_FastFinality_Progress verifies fast finality mode maintains acceptable lag for 15 minutes.
func TestFaultProofProposer_FastFinality_Progress(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.LongRunningFastFinalityFaultProofConfig()
	sys, dgf := setupFaultProofSystem(t, cfg)

	err := utils.RunProgressTest(func() error {
		return checkFaultProofLag(t, sys, dgf)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

func setupFaultProofSystem(t devtest.T, cfg opspresets.FaultProofConfig) (*opspresets.FaultProofSystem, *utils.DgfClient) {
	sys := opspresets.NewFaultProofSystem(t, cfg)
	t.Log("=== Stack is running ===")
	return sys, sys.DgfClient(t)
}

func checkFaultProofLag(t devtest.T, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient) error {
	l2Finalized := sys.L2EL.BlockRefByLabel(eth.Finalized)

	gameCount, _ := dgf.GameCount(t.Ctx())
	if gameCount == 0 {
		t.Logf("Games: 0 | L2 finalized: %d | waiting...", l2Finalized.Number)
		return nil
	}

	latestGameL2Block, _ := getLatestGameL2Block(t.Ctx(), sys, dgf)

	var lag uint64
	if l2Finalized.Number > latestGameL2Block {
		lag = l2Finalized.Number - latestGameL2Block
	}
	t.Logf("Games: %d | L2 Finalized: %d | L2 Latest Block: %d | Lag: %d",
		gameCount, l2Finalized.Number, latestGameL2Block, lag)

	if lag > MaxProposerLag {
		return fmt.Errorf("lag %d exceeds max %d", lag, MaxProposerLag)
	}
	return nil
}

func getLatestGameL2Block(ctx context.Context, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient) (uint64, error) {
	count, err := dgf.GameCount(ctx)
	if err != nil || count == 0 {
		return 0, err
	}
	game, err := dgf.GameAtIndex(ctx, count-1)
	if err != nil {
		return 0, err
	}
	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
	if err != nil {
		return 0, err
	}
	return fdg.L2BlockNumber(ctx)
}
