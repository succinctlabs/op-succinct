package e2e

import (
	"fmt"
	"os"
	"path/filepath"
	"testing"

	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-deployer/pkg/deployer/artifacts"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/intentbuilder"
)

const DefaultL1ID = 900
const DefaultL2ID = 901

func TestMain(m *testing.M) {
	stack := WithDefault(&sysgo.DefaultMinimalSystemIDs{})
	presets.DoMain(m, stack)
}

func WithDefault(dest *sysgo.DefaultMinimalSystemIDs) stack.CommonOption {
	ids := sysgo.NewDefaultMinimalSystemIDs(sysgo.DefaultL1ID, sysgo.DefaultL2AID)

	opt := stack.Combine[*sysgo.Orchestrator]()
	opt.Add(stack.BeforeDeploy(func(o *sysgo.Orchestrator) {
		o.P().Logger().Info("Setting up")
	}))

	opt.Add(sysgo.WithMnemonicKeys(devkeys.TestMnemonic))

	artifactsPath := os.Getenv("OP_DEPLOYER_ARTIFACTS")
	fmt.Println("Using artifacts path:", artifactsPath)
	if artifactsPath == "" {
		panic("OP_DEPLOYER_ARTIFACTS is not set")
	}

	opt.Add(sysgo.WithDeployer(),
		sysgo.WithDeployerPipelineOption(
			sysgo.WithDeployerCacheDir(artifactsPath),
		),
		sysgo.WithDeployerOptions(
			func(_ devtest.P, _ devkeys.Keys, builder intentbuilder.Builder) {
				builder.WithL1ContractsLocator(artifacts.MustNewFileLocator(filepath.Join(artifactsPath, "src/forge-artifacts")))
				builder.WithL2ContractsLocator(artifacts.MustNewFileLocator(filepath.Join(artifactsPath, "src/forge-artifacts")))
			},
			sysgo.WithCommons(ids.L1.ChainID()),
			sysgo.WithPrefundedL2(ids.L1.ChainID(), ids.L2.ChainID()),
		),
	)

	opt.Add(sysgo.WithL1Nodes(ids.L1EL, ids.L1CL))
	opt.Add(sysgo.WithOpGeth(ids.L2EL))
	opt.Add(sysgo.WithOpNode(ids.L2CL, ids.L1CL, ids.L1EL, ids.L2EL,
		sysgo.L2CLOptionFn(func(p devtest.P, id stack.L2CLNodeID, cfg *sysgo.L2CLConfig) {
			cfg.IsSequencer = true
		})))
	opt.Add(sysgo.WithSuperSVProposer(ids.L2CL, ids.L1CL, ids.L1EL, ids.L2EL, ids.L2Proposer))

	opt.Add(stack.Finally(func(orch *sysgo.Orchestrator) {
		*dest = ids
	}))

	return stack.MakeCommon(opt)
}
