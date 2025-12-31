package presets

import (
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/dsl"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/stack/match"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/succinctlabs/op-succinct/utils"
)

// FaultProofConfig holds configuration for fault proof proposer tests.
type FaultProofConfig struct {
	// Contract deployment configuration
	MaxChallengeDuration         uint64
	MaxProveDuration             uint64
	DisputeGameFinalityDelaySecs uint64

	// Proposer configuration
	ProposalIntervalInBlocks uint64
	FetchInterval            uint64
	RangeSplitCount          uint64
	MaxConcurrentRangeProofs uint64
	FastFinalityMode         bool
	FastFinalityProvingLimit uint64
	// Timeout is the proving timeout in seconds.
	// If nil, the default (4 hours / 14400s) is used.
	Timeout     *uint64
	EnvFilePath string

	// AggProofMode selects the SP1 verifier backend ("plonk" or "groth16").
	// Only applies for network proving (i.e. when utils.UseNetworkProver() is true).
	// If nil/empty, defaults to "plonk".
	AggProofMode *string
}

// DefaultFaultProofConfig returns the default configuration for fast tests.
func DefaultFaultProofConfig() FaultProofConfig {
	return FaultProofConfig{
		MaxChallengeDuration:         10, // Low for tests (vs 1 hour production)
		MaxProveDuration:             10, // Low for tests (vs 12 hours production)
		DisputeGameFinalityDelaySecs: 30, // Low for tests (vs 7 days production)
		ProposalIntervalInBlocks:     10,
		FetchInterval:                1,
		RangeSplitCount:              1,
		MaxConcurrentRangeProofs:     1,
		FastFinalityMode:             false,
		FastFinalityProvingLimit:     1,
	}
}

// FastFinalityFaultProofConfig returns configuration with fast finality mode enabled.
func FastFinalityFaultProofConfig() FaultProofConfig {
	cfg := DefaultFaultProofConfig()
	cfg.FastFinalityMode = true
	return cfg
}

// LongRunningFaultProofConfig returns configuration optimized for long-running progress tests.
// If NETWORK_PRIVATE_KEY is set, uses larger intervals tuned for network proving.
func LongRunningFaultProofConfig() FaultProofConfig {
	cfg := DefaultFaultProofConfig()
	cfg.MaxChallengeDuration = 1800 // =30m

	timeout := uint64(900) // =15m
	cfg.Timeout = &timeout

	if utils.UseNetworkProver() {
		cfg.ProposalIntervalInBlocks = 240
	} else {
		cfg.ProposalIntervalInBlocks = 120
	}
	return cfg
}

// LongRunningFastFinalityFaultProofConfig returns configuration for long-running fast finality tests.
// If NETWORK_PRIVATE_KEY is set, uses larger intervals tuned for network proving.
func LongRunningFastFinalityFaultProofConfig() FaultProofConfig {
	cfg := LongRunningFaultProofConfig()
	cfg.FastFinalityMode = true
	cfg.FastFinalityProvingLimit = 8
	return cfg
}

// ProposerOptions returns the proposer options for this configuration.
func (c FaultProofConfig) ProposerOptions() []sysgo.FaultProofProposerOption {
	opts := []sysgo.FaultProofProposerOption{
		sysgo.WithFPProposalIntervalInBlocks(c.ProposalIntervalInBlocks),
		sysgo.WithFPFetchInterval(c.FetchInterval),
		sysgo.WithFPRangeSplitCount(c.RangeSplitCount),
		sysgo.WithFPMaxConcurrentRangeProofs(c.MaxConcurrentRangeProofs),
		sysgo.WithFPFastFinalityMode(c.FastFinalityMode),
		sysgo.WithFPFastFinalityProvingLimit(c.FastFinalityProvingLimit),
	}
	if c.Timeout != nil {
		opts = append(opts, sysgo.WithFPTimeout(*c.Timeout))
	}
	if c.EnvFilePath != "" {
		opts = append(opts, sysgo.WithFPWriteEnvFile(c.EnvFilePath))
	}
	return opts
}

// WithSuccinctFPProposer creates a fault proof proposer with custom configuration.
func WithSuccinctFPProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs, cfg FaultProofConfig, chain L2ChainConfig) stack.CommonOption {
	// Set batcher's MaxBlocksPerSpanBatch to match the proposal interval
	maxBlocksPerSpanBatch := int(cfg.ProposalIntervalInBlocks)
	return withSuccinctPreset(dest, chain, maxBlocksPerSpanBatch, cfg.AggProofMode, func(opt *stack.CombinedOption[*sysgo.Orchestrator], ids sysgo.DefaultSingleChainInteropSystemIDs, l2ChainID eth.ChainID) {
		if !utils.UseNetworkProver() {
			opt.Add(sysgo.WithSuperDeploySP1MockVerifier(ids.L1EL, l2ChainID))
		}
		opt.Add(sysgo.WithSuperDeployOPSuccinctFaultDisputeGame(ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL,
			sysgo.WithFdgL2StartingBlockNumber(1),
			sysgo.WithFdgMaxChallengeDuration(cfg.MaxChallengeDuration),
			sysgo.WithFdgMaxProveDuration(cfg.MaxProveDuration),
			sysgo.WithFdgDisputeGameFinalityDelaySecs(cfg.DisputeGameFinalityDelaySecs)))
		opt.Add(sysgo.WithSuperSuccinctFaultProofProposer(ids.L2AProposer, ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL, cfg.ProposerOptions()...))
	})
}

// WithDefaultSuccinctFPProposer creates a fault proof proposer with default configuration.
func WithDefaultSuccinctFPProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPProposer(dest, DefaultFaultProofConfig(), DefaultL2ChainConfig())
}

// WithSuccinctFPProposerFastFinality creates a fault proof proposer optimized for fast finality.
func WithSuccinctFPProposerFastFinality(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPProposer(dest, FastFinalityFaultProofConfig(), DefaultL2ChainConfig())
}

// WithSuccinctFPProposerAndChallenger creates both a fault proof proposer and challenger with custom configuration.
func WithSuccinctFPProposerAndChallenger(dest *sysgo.DefaultSingleChainInteropSystemIDs, proposerCfg FaultProofConfig, challengerCfg ChallengerConfig, chain L2ChainConfig) stack.CommonOption {
	return stack.MakeCommon(stack.Combine(
		WithSuccinctFPProposer(dest, proposerCfg, chain),
		WithSuccinctFPChallenger(dest, challengerCfg),
	))
}

// WithDefaultSuccinctFPProposerAndChallenger creates both a fault proof proposer and challenger with default configuration.
func WithDefaultSuccinctFPProposerAndChallenger(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPProposerAndChallenger(dest, DefaultFaultProofConfig(), DefaultChallengerConfig(), DefaultL2ChainConfig())
}

// FaultProofSystem wraps MinimalWithProposer and provides access to faultproof-specific features.
type FaultProofSystem struct {
	*presets.MinimalWithProposer
	proposer sysgo.FaultProofProposer
}

// DgfClient creates a DisputeGameFactory client for the faultproof system.
func (s *FaultProofSystem) DgfClient(t devtest.T) *utils.DgfClient {
	dgfAddr := s.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	dgf, err := utils.NewDgfClient(s.L1EL.EthClient(), dgfAddr)
	t.Require().NoError(err, "failed to create DGF client")
	return dgf
}

// StopProposer stops the faultproof proposer (for restart testing).
func (s *FaultProofSystem) StopProposer() {
	s.proposer.Stop()
}

// StartProposer starts the faultproof proposer (for restart testing).
func (s *FaultProofSystem) StartProposer() {
	s.proposer.Start()
}

// NewFaultProofSystem creates a new fault proof test system with custom configuration.
func NewFaultProofSystem(t devtest.T, cfg FaultProofConfig, chain L2ChainConfig) *FaultProofSystem {
	var ids sysgo.DefaultSingleChainInteropSystemIDs
	sys, prop := newSystemWithProposer(t, WithSuccinctFPProposer(&ids, cfg, chain), &ids)

	fp, ok := prop.(sysgo.FaultProofProposer)
	t.Require().True(ok, "proposer must implement FaultProofProposer")

	return &FaultProofSystem{
		MinimalWithProposer: sys,
		proposer:            fp,
	}
}

// ChallengerConfig holds configuration for fault proof challenger tests.
type ChallengerConfig struct {
	// FetchInterval is the polling interval in seconds for the challenger.
	FetchInterval uint64

	// MaliciousChallengePercentage is the percentage of valid games to challenge
	// maliciously for testing defense mechanisms. Set to 0.0 for honest mode (default).
	MaliciousChallengePercentage float64

	// EnvFilePath is an optional path to write the challenger environment variables.
	EnvFilePath string
}

// DefaultChallengerConfig returns the default challenger configuration.
func DefaultChallengerConfig() ChallengerConfig {
	return ChallengerConfig{
		FetchInterval:                1,
		MaliciousChallengePercentage: 0.0,
	}
}

// WithSuccinctFPChallenger creates a fault proof challenger with custom configuration.
// Use with WithSuccinctFPProposer to create a complete system with both proposer and challenger.
func WithSuccinctFPChallenger(ids *sysgo.DefaultSingleChainInteropSystemIDs, cfg ChallengerConfig) stack.CommonOption {
	return stack.MakeCommon(stack.Finally(func(orch *sysgo.Orchestrator) {
		challengerOpts := []sysgo.FaultProofChallengerOption{
			sysgo.WithFPChallengerFetchInterval(cfg.FetchInterval),
		}
		if cfg.MaliciousChallengePercentage > 0 {
			challengerOpts = append(challengerOpts, sysgo.WithFPChallengerMaliciousChallengePercentage(cfg.MaliciousChallengePercentage))
		}
		if cfg.EnvFilePath != "" {
			challengerOpts = append(challengerOpts, sysgo.WithFPChallengerWriteEnvFile(cfg.EnvFilePath))
		}
		sysgo.WithSuccinctFaultProofChallengerPostDeploy(orch, ids.L2ChallengerA, ids.L1EL, ids.L2AEL, challengerOpts...)
	}))
}

// WithDefaultSuccinctFPChallenger creates a fault proof challenger with default configuration.
func WithDefaultSuccinctFPChallenger(ids *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPChallenger(ids, DefaultChallengerConfig())
}

// FaultProofSystemWithChallenger wraps FaultProofSystem and adds challenger access.
type FaultProofSystemWithChallenger struct {
	*FaultProofSystem
	challenger sysgo.FaultProofChallenger
}

// StopChallenger stops the faultproof challenger (for restart testing).
func (s *FaultProofSystemWithChallenger) StopChallenger() {
	s.challenger.Stop()
}

// StartChallenger starts the faultproof challenger (for restart testing).
func (s *FaultProofSystemWithChallenger) StartChallenger() {
	s.challenger.Start()
}

// NewFaultProofSystemWithChallenger creates a new fault proof test system with both proposer and challenger.
// Both components can be started/stopped independently for testing scenarios.
func NewFaultProofSystemWithChallenger(t devtest.T, proposerCfg FaultProofConfig, challengerCfg ChallengerConfig) *FaultProofSystemWithChallenger {
	var ids sysgo.DefaultSingleChainInteropSystemIDs

	// Combine proposer and challenger options
	combinedOpt := WithSuccinctFPProposerAndChallenger(&ids, proposerCfg, challengerCfg, DefaultL2ChainConfig())

	minimal, orch, system := newSystemCore(t, combinedOpt)

	l2 := system.L2Network(match.Assume(t, match.L2ChainA))
	proposer := l2.L2Proposer(match.Assume(t, match.FirstL2Proposer))

	sys := &presets.MinimalWithProposer{
		Minimal:    *minimal,
		L2Proposer: dsl.NewL2Proposer(proposer),
	}

	prop, ok := orch.GetProposer(ids.L2AProposer)
	t.Require().True(ok, "proposer not found")

	fp, ok := prop.(sysgo.FaultProofProposer)
	t.Require().True(ok, "proposer must implement FaultProofProposer")

	chall, ok := orch.GetChallenger(ids.L2ChallengerA)
	t.Require().True(ok, "challenger not found")

	fc, ok := chall.(sysgo.FaultProofChallenger)
	t.Require().True(ok, "challenger must implement FaultProofChallenger")

	return &FaultProofSystemWithChallenger{
		FaultProofSystem: &FaultProofSystem{
			MinimalWithProposer: sys,
			proposer:            fp,
		},
		challenger: fc,
	}
}

// NewDefaultFaultProofSystemWithChallenger creates a fault proof system with both proposer and challenger using default configs.
func NewDefaultFaultProofSystemWithChallenger(t devtest.T) *FaultProofSystemWithChallenger {
	return NewFaultProofSystemWithChallenger(t, DefaultFaultProofConfig(), DefaultChallengerConfig())
}
