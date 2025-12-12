package longrunning

import (
	"fmt"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// MaxProposerLag is the maximum allowed gap between L2 finalized head and the L2OO's
// latest submitted block.
const MaxProposerLag uint64 = 200

// TestValidityProposer_Progress verifies the proposer maintains acceptable lag for 15 minutes.
// The test succeeds if lag stays below MaxProposerLag throughout; fails immediately if exceeded.
func TestValidityProposer_Progress(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.LongRunningValidityConfig()
	sys, l2oo := setupValiditySystem(t, cfg)

	err := utils.RunProgressTest(func() error {
		return checkValidityLag(t, sys, l2oo)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

func setupValiditySystem(t devtest.T, cfg opspresets.ValidityConfig) (*opspresets.ValiditySystem, *utils.L2OOClient) {
	sys := opspresets.NewValiditySystem(t, cfg)
	t.Log("=== Stack is running ===")
	return sys, sys.L2OOClient(t)
}

func checkValidityLag(t devtest.T, sys *opspresets.ValiditySystem, l2oo *utils.L2OOClient) error {
	l2Finalized := sys.L2EL.BlockRefByLabel(eth.Finalized)
	l2ooBlock, _ := l2oo.LatestBlockNumber(t.Ctx())

	var lag uint64
	if l2Finalized.Number > l2ooBlock {
		lag = l2Finalized.Number - l2ooBlock
	}
	t.Logf("L2 Finalized: %d | L2 Latest Block: %d | Lag: %d", l2Finalized.Number, l2ooBlock, lag)

	if lag > MaxProposerLag {
		return fmt.Errorf("lag %d exceeds max %d", lag, MaxProposerLag)
	}
	return nil
}
