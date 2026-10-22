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

// TestRustE2E runs existing Rust commands against local Sysgo nodes.
func TestRustE2E(gt *testing.T) {
	t := devtest.SerialT(gt)
	var args []string
	minFinalized := uint64(6)
	switch mode := os.Getenv("RUST_E2E_MODE"); mode {
	case "fault-proof":
		target := os.Getenv("FP_RUST_TEST_TARGET")
		da := os.Getenv("FP_RUST_TEST_DA")
		t.Require().NotEmpty(target, "FP_RUST_TEST_TARGET is required")
		t.Require().NotEmpty(da, "FP_RUST_TEST_DA is required")
		args = []string{"fp-integration-tests-rpc", target, da}
		// Rust lifecycle tests anchor 500 blocks behind finalized.
		minFinalized = 500
	case "workspace":
		args = []string{"tests-rpc"}
	case "cost-estimator":
		args = []string{"cost-estimator-rpc"}
	case "cycle-count":
		t.Require().NotEmpty(os.Getenv("CYCLE_COUNT_BASE_DIR"), "CYCLE_COUNT_BASE_DIR is required")
	default:
		gt.Fatalf("unsupported RUST_E2E_MODE %q", mode)
	}

	configDir := gt.TempDir()
	chain := opspresets.DefaultL2ChainConfig()
	chain.EnvFilePath = filepath.Join(configDir, "nodes.env")
	chain.L1ConfigDir = filepath.Join(configDir, "L1")
	var ids sysgo.DefaultSingleChainInteropSystemIDs
	sys := opspresets.NewSystemNodesOnly(t, opspresets.WithSuccinctNodes(&ids, chain))

	t.Require().Eventually(func() bool {
		ctx, cancel := context.WithTimeout(t.Ctx(), 5*time.Second)
		defer cancel()
		head, err := sys.L2EL.Escape().L2EthClient().L2BlockRefByLabel(ctx, eth.Finalized)
		return err == nil && head.Number >= minFinalized
	}, 15*time.Minute, time.Second, "waiting for finalized L2 history")

	nodeEnv, err := os.ReadFile(chain.EnvFilePath)
	t.Require().NoError(err, "read local node RPCs")
	env := append(os.Environ(), strings.Split(strings.TrimSpace(string(nodeEnv)), "\n")...)
	env = append(env,
		"L1_CONFIG_DIR="+chain.L1ConfigDir,
		"L2_CONFIG_DIR="+filepath.Join(configDir, "L2"),
	)
	if os.Getenv("RUST_E2E_MODE") == "cycle-count" {
		runCycleCountDiff(t.Ctx(), gt, env)
		return
	}
	cmd := exec.CommandContext(t.Ctx(), "just", args...)
	cmd.Dir = utils.RepoRoot()
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Env = env
	t.Require().NoError(cmd.Run(), "Rust E2E command failed")
}
