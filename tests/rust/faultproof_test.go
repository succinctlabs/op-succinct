package rust

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestFaultProof runs the existing Rust E2E against local Sysgo nodes.
func TestFaultProof(gt *testing.T) {
	t := devtest.SerialT(gt)
	target := os.Getenv("FP_RUST_TEST_TARGET")
	da := os.Getenv("FP_RUST_TEST_DA")
	t.Require().NotEmpty(target, "FP_RUST_TEST_TARGET is required")
	t.Require().NotEmpty(da, "FP_RUST_TEST_DA is required")

	configDir := gt.TempDir()
	chain := opspresets.DefaultL2ChainConfig()
	chain.EnvFilePath = filepath.Join(configDir, "nodes.env")
	chain.L1ConfigDir = filepath.Join(configDir, "L1")
	var ids sysgo.DefaultSingleChainInteropSystemIDs
	sys := opspresets.NewSystemNodesOnly(t, opspresets.WithSuccinctNodes(&ids, chain))

	// Rust anchors 500 blocks behind finalized so lifecycle tests have historical L2 outputs.
	t.Require().Eventually(func() bool {
		ctx, cancel := context.WithTimeout(t.Ctx(), 5*time.Second)
		defer cancel()
		head, err := sys.L2EL.Escape().L2EthClient().L2BlockRefByLabel(ctx, eth.Finalized)
		return err == nil && head.Number >= 500
	}, 15*time.Minute, time.Second, "waiting for 500 finalized L2 blocks")

	nodeEnv, err := os.ReadFile(chain.EnvFilePath)
	t.Require().NoError(err, "read local node RPCs")
	cmd := exec.CommandContext(t.Ctx(), "just", "fp-integration-tests-rpc", target, da)
	cmd.Dir = utils.RepoRoot()
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Env = append(os.Environ(), strings.Split(strings.TrimSpace(string(nodeEnv)), "\n")...)
	cmd.Env = append(cmd.Env,
		"L1_CONFIG_DIR="+chain.L1ConfigDir,
		"L2_CONFIG_DIR="+filepath.Join(configDir, "L2"),
	)
	t.Require().NoError(cmd.Run(), "Rust fault-proof E2E failed")
}
