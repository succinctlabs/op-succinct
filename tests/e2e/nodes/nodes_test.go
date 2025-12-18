package nodes

import (
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestNodes_RunOnly starts L1/L2 nodes without any proposer and runs until shutdown.
// Useful for local development and debugging - outputs RPC endpoints to tests/.env
func TestNodes_RunOnly(gt *testing.T) {
	t := devtest.SerialT(gt)

	chain := opspresets.DefaultL2ChainConfig()
	chain.EnvFilePath = ".env"

	var ids sysgo.DefaultSingleChainInteropSystemIDs
	opt := opspresets.WithSuccinctNodes(&ids, chain)
	sys := opspresets.NewSystemNodesOnly(t, opt)

	t.Logger().Info("L1/L2 nodes are running. RPC endpoints written to tests/.env")
	t.Logger().Info("Press Ctrl+C to stop...")

	utils.RunUntilShutdown(30*time.Second, func() error {
		l2Unsafe := sys.L2EL.BlockRefByLabel(eth.Unsafe)
		t.Logger().Info("L2 block", "unsafe", l2Unsafe.Number)
		return nil
	})
}
