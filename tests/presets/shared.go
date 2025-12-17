package presets

import (
	"os"

	bss "github.com/ethereum-optimism/optimism/op-batcher/batcher"
	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-deployer/pkg/deployer/artifacts"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/dsl"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/shim"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/stack/match"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/intentbuilder"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/succinctlabs/op-succinct/utils"
)

const (
	DefaultL1ID = 900
	DefaultL2ID = 901
)

type succinctConfigurator func(*stack.CombinedOption[*sysgo.Orchestrator], sysgo.DefaultSingleChainInteropSystemIDs, eth.ChainID)

func withSuccinctPreset(dest *sysgo.DefaultSingleChainInteropSystemIDs, l2BlockTime uint64, maxBlocksPerSpanBatch int, configure succinctConfigurator) stack.CommonOption {
	// Ensure OP Succinct artifact symlinks exist before deployer loads artifacts
	if root := utils.RepoRoot(); root != "" {
		if err := utils.SymlinkSuccinctArtifacts(root); err != nil {
			panic("failed to symlink Succinct artifacts: " + err.Error())
		}
	}

	l1ChainID := eth.ChainIDFromUInt64(DefaultL1ID)
	l2ChainID := eth.ChainIDFromUInt64(DefaultL2ID)
	ids := sysgo.NewDefaultSingleChainInteropSystemIDs(l1ChainID, l2ChainID)

	opt := stack.Combine[*sysgo.Orchestrator]()
	opt.Add(stack.BeforeDeploy(func(o *sysgo.Orchestrator) {
		o.P().Logger().Info("Setting up")
	}))

	opt.Add(sysgo.WithMnemonicKeys(devkeys.TestMnemonic))

	artifactsPath := os.Getenv("OP_DEPLOYER_ARTIFACTS")
	if artifactsPath == "" {
		panic("OP_DEPLOYER_ARTIFACTS is not set")
	}

	opt.Add(sysgo.WithDeployer(),
		sysgo.WithDeployerPipelineOption(
			sysgo.WithDeployerCacheDir(artifactsPath),
		),
		sysgo.WithDeployerOptions(
			func(_ devtest.P, _ devkeys.Keys, builder intentbuilder.Builder) {
				builder.WithGlobalOverride("l2BlockTime", l2BlockTime)
				builder.WithL1ContractsLocator(artifacts.MustNewFileLocator(artifactsPath))
				builder.WithL2ContractsLocator(artifacts.MustNewFileLocator(artifactsPath))
			},
			sysgo.WithSP1ProverMode("plonk"),
			sysgo.WithCommons(ids.L1.ChainID()),
			sysgo.WithPrefundedL2(ids.L1.ChainID(), ids.L2A.ChainID()),
		),
	)
	// Configure batcher to accumulate more blocks before submitting.
	// MaxBlocksPerSpanBatch: max L2 blocks per span batch (matches proposer interval)
	// MaxChannelDuration: max L1 blocks before forcing submission
	//   - L1 block time is 6s in test environment (see sysgo/deployer.go)
	const l1BlockTime = uint64(6)
	l2TimeSeconds := uint64(maxBlocksPerSpanBatch) * l2BlockTime
	maxChannelDuration := (l2TimeSeconds + l1BlockTime - 1) / l1BlockTime
	opt.Add(sysgo.WithBatcherOption(func(id stack.L2BatcherID, cfg *bss.CLIConfig) {
		cfg.MaxBlocksPerSpanBatch = maxBlocksPerSpanBatch
		cfg.MaxChannelDuration = maxChannelDuration
	}))

	opt.Add(sysgo.WithL1Nodes(ids.L1EL, ids.L1CL))

	opt.Add(sysgo.WithSupervisor(ids.Supervisor, ids.Cluster, ids.L1EL))
	opt.Add(sysgo.WithL2ELNode(ids.L2AEL, sysgo.L2ELWithSupervisor(ids.Supervisor)))
	opt.Add(sysgo.WithL2CLNode(ids.L2ACL, ids.L1CL, ids.L1EL, ids.L2AEL, sysgo.L2CLSequencer(), sysgo.L2CLIndexing()))
	opt.Add(sysgo.WithManagedBySupervisor(ids.L2ACL, ids.Supervisor))

	opt.Add(sysgo.WithBatcher(ids.L2ABatcher, ids.L1EL, ids.L2ACL, ids.L2AEL))
	opt.Add(sysgo.WithTestSequencer(ids.TestSequencer, ids.L1CL, ids.L2ACL, ids.L1EL, ids.L2AEL))
	opt.Add(sysgo.WithFaucets([]stack.L1ELNodeID{ids.L1EL}, []stack.L2ELNodeID{ids.L2AEL}))

	configure(&opt, ids, l2ChainID)

	opt.Add(sysgo.WithL2MetricsDashboard())
	opt.Add(stack.Finally(func(orch *sysgo.Orchestrator) {
		*dest = ids
	}))

	return stack.MakeCommon(opt)
}

// NewSystem creates a new test system with the given stack option.
// This is a unified function for creating both validity and fault proof test systems.
func NewSystem(t devtest.T, opt stack.CommonOption) *presets.MinimalWithProposer {
	sys, _ := newSystemWithProposer(t, opt, nil)
	return sys
}

// newSystemWithProposer creates a new test system and optionally returns the L2Prop backend.
// If ids is provided, it retrieves the proposer from the orchestrator.
func newSystemWithProposer(t devtest.T, opt stack.CommonOption, ids *sysgo.DefaultSingleChainInteropSystemIDs) (*presets.MinimalWithProposer, sysgo.L2Prop) {
	p := devtest.NewP(t.Ctx(), t.Logger(), func(now bool) {
		t.Errorf("test failed")
		if now {
			t.FailNow()
		}
	}, func() {
		t.SkipNow()
	})
	t.Cleanup(p.Close)

	combinedOpt := stack.Combine(opt, presets.WithSafeDBEnabled())

	orch := sysgo.NewOrchestrator(p, stack.SystemHook(combinedOpt))
	var orchIntf stack.Orchestrator = orch
	stack.ApplyOptionLifecycle(combinedOpt, orchIntf)

	system := shim.NewSystem(t)
	orch.Hydrate(system)

	minimal := presets.MinimalFromSystem(t, system, orch)
	l2 := system.L2Network(match.Assume(t, match.L2ChainA))
	proposer := l2.L2Proposer(match.Assume(t, match.FirstL2Proposer))

	sys := &presets.MinimalWithProposer{
		Minimal:    *minimal,
		L2Proposer: dsl.NewL2Proposer(proposer),
	}

	var prop sysgo.L2Prop
	if ids != nil {
		var ok bool
		prop, ok = orch.GetProposer(ids.L2AProposer)
		t.Require().True(ok, "proposer not found")
	}

	return sys, prop
}

// MaxProposerLag returns the maximum allowed lag between L2 finalized and proposer submissions.
// Network proving uses larger intervals, so allows more lag.
func MaxProposerLag() uint64 {
	if utils.UseNetworkProver() {
		return 600 // ~20m at 2s block time
	}
	return 300 // ~10m at 2s block time
}
