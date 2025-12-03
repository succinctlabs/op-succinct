package presets

import (
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-service/eth"
)

// ValidityConfig holds configuration for validity proposer tests.
type ValidityConfig struct {
	SubmissionInterval uint64
	RangeProofInterval uint64
}

// DefaultValidityConfig returns the default configuration.
func DefaultValidityConfig() ValidityConfig {
	return ValidityConfig{
		SubmissionInterval: 10,
		RangeProofInterval: 10,
	}
}

// WithSuccinctValidityProposer creates a validity proposer with custom configuration.
func WithSuccinctValidityProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs, cfg ValidityConfig) stack.CommonOption {
	return withSuccinctPreset(dest, func(opt *stack.CombinedOption[*sysgo.Orchestrator], ids sysgo.DefaultSingleChainInteropSystemIDs, l2ChainID eth.ChainID) {
		opt.Add(sysgo.WithSuperDeploySP1MockVerifier(ids.L1EL, l2ChainID))
		opt.Add(sysgo.WithSuperDeployOpSuccinctL2OutputOracle(ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL, sysgo.WithL2OOStartingBlockNumber(1)))
		opt.Add(sysgo.WithSuperSuccinctValidityProposer(ids.L2AProposer, ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL,
			sysgo.WithVPSubmissionInterval(cfg.SubmissionInterval),
			sysgo.WithVPRangeProofInterval(cfg.RangeProofInterval),
			sysgo.WithVPMockMode(true)))
	})
}

// WithDefaultSuccinctValidityProposer creates a validity proposer with default configuration.
// This maintains backward compatibility with existing tests.
func WithDefaultSuccinctValidityProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctValidityProposer(dest, DefaultValidityConfig())
}

// NewValiditySystem creates a new validity test system with custom configuration.
// This allows per-test configuration instead of relying on TestMain.
func NewValiditySystem(t devtest.T, cfg ValidityConfig) *presets.MinimalWithProposer {
	var ids sysgo.DefaultSingleChainInteropSystemIDs
	return NewSystem(t, WithSuccinctValidityProposer(&ids, cfg))
}
