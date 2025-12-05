package recovery

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestFaultProofProposer_RestartRecovery_Optimistic(gt *testing.T) {
	cfg := opspresets.DefaultFaultProofConfig()
	cfg.ProposalIntervalInBlocks = 40
	runRecoveryTest(gt, cfg, 20*time.Minute)
}

func TestFaultProofProposer_RestartRecovery_FastFinalityBasic(gt *testing.T) {
	cfg := opspresets.FastFinalityFaultProofConfig()
	cfg.ProposalIntervalInBlocks = 40
	runRecoveryTest(gt, cfg, 20*time.Minute)
}

func TestFaultProofProposer_RestartRecovery_FastFinalityRangeSplit(gt *testing.T) {
	cfg := opspresets.FastFinalityFaultProofConfig()
	cfg.ProposalIntervalInBlocks = 40
	cfg.RangeSplitCount = 4
	cfg.MaxConcurrentRangeProofs = 4
	runRecoveryTest(gt, cfg, 20*time.Minute)
}

func runRecoveryTest(gt *testing.T, cfg opspresets.FaultProofConfig, timeout time.Duration) {
	t := devtest.ParallelT(gt)
	sys := opspresets.NewFaultProofSystem(t, cfg)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), timeout)
	defer cancel()

	logger.Info("Running recovery test", "fastFinalityMode", cfg.FastFinalityMode)

	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	logger.Info("Dispute Game Factory Address", "address", dgfAddr.Hex())
	dgf, err := utils.NewDgfClient(sys.L1EL.EthClient(), dgfAddr)
	require.NoError(err, "failed to create Dispute Game Factory client")

	// Wait for at least 1 game to be created (partial progress)
	logger.Info("Waiting for first game to be created...")
	utils.WaitForGameCount(ctx, t, dgf, 1)

	// Get game count before stopping
	gameCountBefore, err := dgf.GameCount(ctx)
	require.NoError(err, "failed to get game count before stop")
	logger.Info("Stopping proposer", "gameCount", gameCountBefore)

	// Stop the proposer mid-operation
	sys.StopProposer()
	logger.Info("Proposer stopped")

	// Restart the proposer
	sys.StartProposer()
	logger.Info("Proposer restarted")

	// Get the first game and wait for it to be resolved
	game, err := dgf.GameAtIndex(ctx, 0)
	require.NoError(err, "failed to get game from factory")

	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
	require.NoError(err, "failed to create Fault Dispute Game client")

	logger.Info("Waiting for game to be resolved after restart...")
	utils.WaitForDefenderWins(ctx, t, fdg)
	logger.Info("Game resolved after restart - DefenderWins", "fastFinalityMode", cfg.FastFinalityMode)
}
