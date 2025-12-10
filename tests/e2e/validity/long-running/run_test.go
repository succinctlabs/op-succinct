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

	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	l2oo, err := utils.NewL2OOClient(sys.L1EL.EthClient(), l2ooAddr)
	t.Require().NoError(err, "failed to create L2OO client")

	utils.RunUntilShutdown(10*time.Second, func() {
		l2Block := sys.L2EL.BlockRefByLabel(eth.Unsafe)
		l2ooBlock, _ := l2oo.LatestBlockNumber(t.Ctx())
		t.Logf("L2 block: %d | L2OO latest: %d", l2Block.Number, l2ooBlock)
	})
}
