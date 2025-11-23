package e2e

import (
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	opspresets "github.com/succinctlabs/op-succinct/presets"
)

func TestMain(m *testing.M) {
	presets.DoMain(m,
		opspresets.WithSuccinctFaultProofProposer(&sysgo.DefaultSingleChainInteropSystemIDs{}),
		presets.WithSafeDBEnabled(),
	)
}
