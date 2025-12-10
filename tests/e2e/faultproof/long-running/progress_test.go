package longrunning

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// MaxProposerLag is the maximum allowed lag between L2 finalized block and the latest game's L2 block.
const MaxProposerLag uint64 = 100

// TestFaultProofProposer_Progress runs until shutdown and fails if lag exceeds threshold.
func TestFaultProofProposer_Progress(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys, dgf := setupFaultProofSystem(t)

	err := utils.RunUntilShutdown(10*time.Second, func() error {
		return checkFaultProofLag(t, sys, dgf, true)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

func setupFaultProofSystem(t devtest.T) (*opspresets.FaultProofSystem, *utils.DgfClient) {
	cfg := opspresets.DefaultFaultProofConfig()
	cfg.ProposalIntervalInBlocks = 50
	cfg.EnvFilePath = "../../../.env.faultproof"
	sys := opspresets.NewFaultProofSystem(t, cfg)
	t.Log("=== Stack is running ===")
	return sys, sys.DgfClient(t)
}

func checkFaultProofLag(t devtest.T, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient, failOnLag bool) error {
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

	if failOnLag && lag > MaxProposerLag {
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
