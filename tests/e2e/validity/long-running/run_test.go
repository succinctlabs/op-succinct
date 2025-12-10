package longrunning

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestLongRunningValidityProposer keeps the validity proposer stack running indefinitely.
// Use this for development and debugging - submit transactions, inspect state, etc.
//
// Run with: just long-running validity
// Stop with: Ctrl+C
func TestLongRunningValidityProposer(gt *testing.T) {
	t := devtest.SerialT(gt)

	cfg := opspresets.DefaultValidityConfig()
	cfg.EnvFilePath = "../../../.env.validity"
	sys := opspresets.NewValiditySystem(t, cfg)

	t.Log("=== Stack is running ===")

	// Create L2OO client to monitor proposer progress
	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	l2oo, err := utils.NewL2OOClient(sys.L1EL.EthClient(), l2ooAddr)
	t.Require().NoError(err, "failed to create L2OO client")

	// Log progress periodically until interrupted
	ticker := time.NewTicker(10 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-t.Ctx().Done():
			t.Log("Shutting down")
			return
		case <-ticker.C:
			l2Block := sys.L2EL.BlockRefByLabel(eth.Unsafe)
			l2ooBlock, _ := l2oo.LatestBlockNumber(t.Ctx())
			t.Logf("L2 block: %d | L2OO latest: %d", l2Block.Number, l2ooBlock)
		}
	}
}
