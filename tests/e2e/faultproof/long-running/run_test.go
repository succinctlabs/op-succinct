package longrunning

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestLongRunningFaultProofProposer keeps the fault proof proposer stack running indefinitely.
// Use this for development and debugging - submit transactions, inspect state, etc.
//
// Run with: just long-running faultproof
// Stop with: Ctrl+C
func TestLongRunningFaultProofProposer(gt *testing.T) {
	t := devtest.SerialT(gt)

	cfg := opspresets.DefaultFaultProofConfig()
	cfg.EnvFilePath = "../../../.env.faultproof"
	sys := opspresets.NewFaultProofSystem(t, cfg)

	t.Log("=== Stack is running ===")

	// Create DGF client to monitor proposer progress
	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	dgf, err := utils.NewDgfClient(sys.L1EL.EthClient(), dgfAddr)
	t.Require().NoError(err, "failed to create DGF client")

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
			gameCount, _ := dgf.GameCount(t.Ctx())
			t.Logf("L2 block: %d | Games: %d", l2Block.Number, gameCount)
		}
	}
}
