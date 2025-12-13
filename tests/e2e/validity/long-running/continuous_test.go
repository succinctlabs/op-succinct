package longrunning

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestValidityProposer_LongRunning runs indefinitely, logging progress without failing.
func TestValidityProposer_LongRunning(gt *testing.T) {
	t := devtest.SerialT(gt)
	cfg := opspresets.LongRunningValidityConfig()
	cfg.EnvFilePath = "../../../.env.validity"
	sys, l2oo := setupValiditySystem(t, cfg)

	utils.RunUntilShutdown(60*time.Second, func() error {
		return checkValidityLag(t, sys, l2oo)
	})
}
