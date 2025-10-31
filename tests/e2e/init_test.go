package e2e

import (
	"fmt"
	"os"
	"path/filepath"
	"testing"

	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-deployer/pkg/deployer/artifacts"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/dsl/contract"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/intentbuilder"
	"github.com/ethereum-optimism/optimism/op-service/txintent/bindings"
)

const DefaultL1ID = 900
const DefaultL2ID = 901

func TestMain(m *testing.M) {
	stack := WithSVProposer(&sysgo.DefaultMinimalSystemIDs{})
	presets.DoMain(m, stack)
}

func TestMinimal(gt *testing.T) {
	t := devtest.SerialT(gt)
	require := t.Require()
	sys := presets.NewMinimal(t)
	sys.DisputeGameFactory()

	client := sys.L1EL.Escape().EthClient()
	addr := sys.L2Networks()[0].Escape().Deployment().DisputeGameFactoryProxyAddr()
	dgf := bindings.NewBindings[bindings.DisputeGameFactory](bindings.WithClient(client), bindings.WithTest(t), bindings.WithTo(addr))

	blocknumber := contract.Read(dgf.GameCount())
	t.Logf("Latest L2 Block Number from DisputeGameFactory: %s", blocknumber.String())

	require.Equal(blocknumber.Int64(), int64(0))
}

func WithSVProposer(dest *sysgo.DefaultMinimalSystemIDs) stack.CommonOption {
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

	opt.Add(sysgo.WithL2ELNode(ids.L2EL))
	opt.Add(sysgo.WithL2CLNode(ids.L2CL, ids.L1CL, ids.L1EL, ids.L2EL, sysgo.L2CLSequencer()))

	opt.Add(sysgo.WithBatcher(ids.L2Batcher, ids.L1EL, ids.L2CL, ids.L2EL))
	opt.Add(sysgo.WithTestSequencer(ids.TestSequencer, ids.L1CL, ids.L2CL, ids.L1EL, ids.L2EL))
	opt.Add(sysgo.WithSuperSVProposer(ids.L2CL, ids.L1CL, ids.L1EL, ids.L2EL, ids.L2Proposer))

	opt.Add(sysgo.WithFaucets([]stack.L1ELNodeID{ids.L1EL}, []stack.L2ELNodeID{ids.L2EL}))

	opt.Add(sysgo.WithL2MetricsDashboard())

	opt.Add(stack.Finally(func(orch *sysgo.Orchestrator) {
		*dest = ids
	}))

	return stack.MakeCommon(opt)
}
