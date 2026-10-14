//go:build agglayer

package agglayer

import (
	"context"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/stretchr/testify/require"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestValidityProposer_ExternalAggregation(gt *testing.T) {
	// Reuse one local chain and proposer for all three serial RPC scenarios.
	for _, name := range []string{"NETWORK_PRIVATE_KEY", "DATABASE_URL", "L1_RPC", "L1_BEACON_RPC", "L2_RPC", "L2_NODE_RPC"} {
		_, set := os.LookupEnv(name)
		require.False(gt, set, "unset %s so Sysgo owns the test services and mock identity", name)
	}
	gt.Setenv("OP_SUCCINCT_MOCK", "true")
	gt.Setenv("AGG_PROOF_MODE", "compressed")
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	require.NoError(gt, err)
	gt.Setenv("GRPC_ADDRESS", listener.Addr().String())
	require.NoError(gt, listener.Close())

	cfg := opspresets.DefaultValidityConfig()
	capacity := uint64(4)
	cfg.MaxConcurrentProofRequests = &capacity
	cfg.MaxConcurrentWitnessGen = &capacity
	cfg.EnvFilePath = filepath.Join(gt.TempDir(), "proposer.env")
	t := devtest.SerialT(gt)
	sys := opspresets.NewValiditySystem(t, cfg, opspresets.DefaultL2ChainConfig())

	ctx, cancel := context.WithTimeout(t.Ctx(), 10*time.Minute)
	defer cancel()
	// Use the generated Rust client and keep the scenario definitions in one place.
	cmd := exec.CommandContext(ctx, "cargo", "test", "--release", "-p", "op-succinct-validity",
		"--features", "agglayer", "--test", "external_aggregation",
		"--", "--ignored", "--test-threads=1", "--nocapture")
	cmd.Dir = utils.RepoRoot()
	cmd.Env = append(os.Environ(), "AGGLAYER_TEST_ENV_FILE="+cfg.EnvFilePath)
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	require.NoError(gt, cmd.Run())

	// External aggregation must not restore the proposer's automatic L1 submission loop.
	latest, err := sys.L2OOClient(t).LatestBlockNumber(t.Ctx())
	require.NoError(gt, err)
	require.Equal(gt, cfg.StartingBlock, latest)
}
