package rust

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/succinctlabs/op-succinct/utils"
)

// runCycleCountDiff keeps both branch executions within one Sysgo node lifecycle.
func runCycleCountDiff(ctx context.Context, t *testing.T, env []string) {
	t.Helper()
	root := utils.RepoRoot()
	base, err := filepath.Abs(os.Getenv("CYCLE_COUNT_BASE_DIR"))
	require.NoError(t, err)
	require.NotEqual(t, root, base, "the base must be a separate checkout")

	// Compile the same test driver against each branch's host, witness code and ELF.
	// A separate temporary target preserves the base checkout's existing tests.
	driver, err := os.ReadFile(filepath.Join(root, "scripts/prove/tests/cycle_count_diff.rs"))
	require.NoError(t, err)
	writeCycleCountTempFile(t, filepath.Join(base, "scripts/prove/tests/sysgo_cycle_count.rs"), driver)
	for _, dir := range []string{root, base} {
		// The existing Rust driver loads dotenv, while local RPCs arrive through env.
		envFile := filepath.Join(dir, ".env")
		if _, err := os.Stat(envFile); errors.Is(err, os.ErrNotExist) {
			writeCycleCountTempFile(t, envFile, nil)
		} else {
			require.NoError(t, err)
		}
	}

	run := func(dir, target, name string, extraEnv ...string) {
		t.Helper()
		cmd := exec.CommandContext(ctx, "cargo", "test", "-p", "op-succinct-prove",
			"--release", "--test", target, name, "--", "--exact", "--nocapture")
		cmd.Dir = dir
		cmd.Env = append(append([]string{}, env...), extraEnv...)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		require.NoError(t, cmd.Run(), "cycle count command failed in %s", dir)
	}

	run(root, "cycle_count_diff", "test_cycle_count_diff", "NEW_BRANCH=true")
	for _, name := range []string{"new_cycle_stats.json", "cycle_count_heads.json"} {
		data, err := os.ReadFile(filepath.Join(root, "scripts/prove", name))
		require.NoError(t, err)
		writeCycleCountTempFile(t, filepath.Join(base, "scripts/prove", name), data)
	}
	run(base, "sysgo_cycle_count", "test_cycle_count_diff", "NEW_BRANCH=false")
	oldStats, err := os.ReadFile(filepath.Join(base, "scripts/prove/old_cycle_stats.json"))
	require.NoError(t, err)
	require.NoError(t, os.WriteFile(filepath.Join(root, "scripts/prove/old_cycle_stats.json"), oldStats, 0600))
	run(root, "cycle_count_diff", "test_post_to_github")
}

func writeCycleCountTempFile(t *testing.T, path string, data []byte) {
	t.Helper()
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	require.NoError(t, err, "temporary benchmark file must not replace an existing file: %s", path)
	t.Cleanup(func() { require.NoError(t, os.Remove(path)) })
	_, err = file.Write(data)
	closeErr := file.Close()
	require.NoError(t, err)
	require.NoError(t, closeErr)
}
