package longrunning

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestFaultProofProposer_LongRunning runs indefinitely, logging progress without failing.
func TestFaultProofProposer_LongRunning(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys, dgf := setupFaultProofSystem(t)

	utils.RunUntilShutdown(10*time.Second, func() error {
		return checkFaultProofLag(t, sys, dgf, false)
	})
}
