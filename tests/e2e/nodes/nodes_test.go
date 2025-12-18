package nodes

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
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

	utils.RunUntilShutdown(30*time.Second, func() error {
		l2Unsafe := sys.L2EL.BlockRefByLabel(eth.Unsafe)
		t.Logger().Info("L2 block", "unsafe", l2Unsafe.Number)
		return nil
	})
}

// TestBaseNodes_RunOnly starts L1/L2 nodes with Base mainnet parameters (no proposer, no load).
// - 2s block time
// - 375M gas limit (matches Base mainnet)
func TestBaseNodes_RunOnly(gt *testing.T) {
	t := devtest.SerialT(gt)

	chain := opspresets.BaseMainnetL2ChainConfig()
	chain.EnvFilePath = ".env"

	var ids sysgo.DefaultSingleChainInteropSystemIDs
	opt := opspresets.WithSuccinctNodes(&ids, chain)
	sys := opspresets.NewSystemNodesOnly(t, opt)

	utils.RunUntilShutdown(30*time.Second, func() error {
		l2Unsafe := sys.L2EL.BlockRefByLabel(eth.Unsafe)
		t.Logger().Info("L2 block", "unsafe", l2Unsafe.Number)
		return nil
	})
}

// TestBaseNodesWithLoad_RunOnly starts L1/L2 nodes with Base mainnet parameters and ERC20 load.
// - 2s block time
// - 375M gas limit (matches Base mainnet)
// - ERC20 transfers filling blocks
func TestBaseNodesWithLoad_RunOnly(gt *testing.T) {
	t := devtest.SerialT(gt)

	chain := opspresets.BaseMainnetL2ChainConfig()
	chain.EnvFilePath = ".env"

	var ids sysgo.DefaultSingleChainInteropSystemIDs
	opt := opspresets.WithSuccinctNodes(&ids, chain)
	sys := opspresets.NewSystemNodesOnly(t, opt)

	// Start load generator with Base mainnet config (1000 accounts for ~17% block fill)
	loadGen := startLoadGenerator(t, sys, utils.BaseMainnetERC20LoadConfig())
	t.Cleanup(loadGen.Stop)

	utils.RunUntilShutdown(30*time.Second, func() error {
		l2Unsafe := sys.L2EL.BlockRefByLabel(eth.Unsafe)
		t.Logger().Info("L2 block", "unsafe", l2Unsafe.Number)
		return nil
	})
}

// startLoadGenerator creates and starts an ERC20 load generator.
func startLoadGenerator(t devtest.T, sys *presets.Minimal, cfg utils.ERC20LoadConfig) *utils.ERC20Load {
	client := utils.NewL2ClientAdapter(sys.L2EL.Escape().L2EthClient())

	keys, err := devkeys.NewMnemonicDevKeys(devkeys.TestMnemonic)
	t.Require().NoError(err, "failed to create devkeys")

	// Use funder account index (matches sysgo/deployer.go)
	funderKey, err := keys.Secret(devkeys.UserKey(10_000))
	t.Require().NoError(err, "failed to get funder key")

	loadGen, err := utils.NewERC20Load(client, funderKey, cfg, t.Logger())
	t.Require().NoError(err, "failed to create load generator")

	err = loadGen.Start(context.Background())
	t.Require().NoError(err, "failed to start load generator")

	return loadGen
}
