package longrunning

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestValidityProposer_LongRunning runs indefinitely, logging progress without failing.
func TestValidityProposer_LongRunning(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys, l2oo := setupValiditySystem(t)

	utils.RunUntilShutdown(10*time.Second, func() error {
		return checkValidityLag(t, sys, l2oo, false)
	})
}
