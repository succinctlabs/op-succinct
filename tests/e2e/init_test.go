package e2e

import (
	"os"
	"testing"

	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-deployer/pkg/deployer/artifacts"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-e2e/e2eutils/intentbuilder"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/ethclient"
	opbind "github.com/succinctlabs/op-succinct/bindings"
)

const DefaultL1ID = 900
const DefaultL2ID = 901

func TestMain(m *testing.M) {
	stack := WithSVProposer(&sysgo.DefaultMinimalSystemIDs{})
	presets.DoMain(m, stack)
}

func TestMinimal(gt *testing.T) {
	t := devtest.SerialT(gt)
	sys := presets.NewMinimal(t)
	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	rpc := sys.L2CL.Escape().UserRPC()
	client, _ := ethclient.Dial(rpc)
	l2oo, _ := opbind.NewOPSuccinctL2OutputOracleCaller(l2ooAddr, client)
	opts := &bind.CallOpts{}
	latestBlockNumber, _ := l2oo.LatestBlockNumber(opts)
	t.Logger().Info("Latest L2 Block Number from L2 Output Oracle:", "blockNumber", latestBlockNumber.Uint64())
}

func WithSVProposer(dest *sysgo.DefaultMinimalSystemIDs) stack.CommonOption {
	ids := sysgo.NewDefaultMinimalSystemIDs(eth.ChainIDFromUInt64(DefaultL1ID), eth.ChainIDFromUInt64(DefaultL2ID))

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
				builder.WithL1ContractsLocator(artifacts.MustNewFileLocator(artifactsPath))
				builder.WithL2ContractsLocator(artifacts.MustNewFileLocator(artifactsPath))
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

	opt.Add(sysgo.WithFaucets([]stack.L1ELNodeID{ids.L1EL}, []stack.L2ELNodeID{ids.L2EL}))

	opt.Add(sysgo.WithL2MetricsDashboard())

	opt.Add(sysgo.WithDeployOpSuccinctL2OutputOracle(ids.L1CL, ids.L1EL, ids.L2CL, ids.L2EL))

	opt.Add(sysgo.WithSVProposer(ids.L2Proposer, ids.L1CL, ids.L1EL, ids.L2CL, ids.L2EL))

	opt.Add(stack.Finally(func(orch *sysgo.Orchestrator) {
		*dest = ids
	}))

	return stack.MakeCommon(opt)
}
