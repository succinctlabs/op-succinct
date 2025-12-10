package longrunning

import (
	"fmt"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// MaxProposerLag is the maximum allowed lag between L2 finalized block and L2OO latest block.
const MaxProposerLag uint64 = 100

// TestValidityProposer_LongRunning runs indefinitely, logging progress without failing.
func TestValidityProposer_LongRunning(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys, l2oo := setupValiditySystem(t)

	utils.RunUntilShutdown(10*time.Second, func() error {
		return checkValidityLag(t, sys, l2oo, false)
	})
}

// TestValidityProposer_Progress runs until shutdown and fails if lag exceeds threshold.
func TestValidityProposer_Progress(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys, l2oo := setupValiditySystem(t)

	err := utils.RunUntilShutdown(10*time.Second, func() error {
		return checkValidityLag(t, sys, l2oo, true)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

func setupValiditySystem(t devtest.T) (*opspresets.ValiditySystem, *utils.L2OOClient) {
	cfg := opspresets.DefaultValidityConfig()
	cfg.EnvFilePath = "../../../.env.validity"
	sys := opspresets.NewValiditySystem(t, cfg)
	t.Log("=== Stack is running ===")
	return sys, sys.L2OOClient(t)
}

func checkValidityLag(t devtest.T, sys *opspresets.ValiditySystem, l2oo *utils.L2OOClient, failOnLag bool) error {
	l2Finalized := sys.L2EL.BlockRefByLabel(eth.Finalized)
	l2ooBlock, _ := l2oo.LatestBlockNumber(t.Ctx())

	var lag uint64
	if l2Finalized.Number > l2ooBlock {
		lag = l2Finalized.Number - l2ooBlock
	}
	t.Logf("L2 Finalized: %d | L2 Latest Block: %d | Lag: %d", l2Finalized.Number, l2ooBlock, lag)

	if failOnLag && lag > MaxProposerLag {
		return fmt.Errorf("lag %d exceeds max %d", lag, MaxProposerLag)
	}
	return nil
}
