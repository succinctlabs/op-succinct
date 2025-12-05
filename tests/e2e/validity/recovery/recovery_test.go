package recovery

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestValidityProposer_RestartRecovery_Basic tests basic restart recovery with default config.
func TestValidityProposer_RestartRecovery_Basic(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	runRecoveryTest(gt, cfg, 1, 20*time.Minute)
}

// TestValidityProposer_RestartRecovery_ThreeSubmissions tests that proposer continues working
// after restart by verifying three submissions complete.
func TestValidityProposer_RestartRecovery_ThreeSubmissions(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	runRecoveryTest(gt, cfg, 3, 30*time.Minute)
}

// TestValidityProposer_RestartRecovery_RangeSplit tests restart recovery with multiple range proofs per submission.
func TestValidityProposer_RestartRecovery_RangeSplit(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	cfg.SubmissionInterval = 20
	cfg.RangeProofInterval = 5
	runRecoveryTest(gt, cfg, 1, 20*time.Minute)
}

func runRecoveryTest(gt *testing.T, cfg opspresets.ValidityConfig, expectedSubmissions int, timeout time.Duration) {
	t := devtest.ParallelT(gt)
	sys := opspresets.NewValiditySystem(t, cfg)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), timeout)
	defer cancel()

	logger.Info("Running validity recovery test",
		"submissionInterval", cfg.SubmissionInterval,
		"rangeProofInterval", cfg.RangeProofInterval,
		"expectedSubmissions", expectedSubmissions)

	// Wait for at least 1 range proof request to be created (partial progress)
	logger.Info("Waiting for partial progress before restart...")
	utils.WaitForRangeProofProgress(ctx, t, sys.DatabaseURL(), 1)

	// Check how many range proofs exist before stopping
	countBefore, err := utils.CountRangeProofRequests(ctx, sys.DatabaseURL())
	require.NoError(err, "failed to count range proofs before stop")
	logger.Info("Stopping proposer", "rangeProofRequests", countBefore)

	// Stop the proposer mid-proving
	sys.StopProposer()
	logger.Info("Proposer stopped")

	// Brief pause to ensure lock expires (LoopInterval + buffer)
	time.Sleep(5 * time.Second)

	// Verify data persisted before restart
	countAfterStop, err := utils.CountRangeProofRequests(ctx, sys.DatabaseURL())
	require.NoError(err, "failed to count range proofs after stop")
	require.Equal(countBefore, countAfterStop, "range proof data should persist after proposer stop")
	logger.Info("Data persistence verified", "count", countAfterStop)

	// Restart the proposer
	sys.StartProposer()
	logger.Info("Proposer restarted")

	// Wait for expected submissions to complete
	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	l2oo, err := utils.NewL2OOClient(sys.L1EL.EthClient(), l2ooAddr)
	require.NoError(err, "failed to create L2OO client")

	expectedOutputBlock := cfg.ExpectedOutputBlock(expectedSubmissions)
	logger.Info("Waiting for output after restart", "expectedBlock", expectedOutputBlock, "submissions", expectedSubmissions)

	utils.WaitForLatestBlockNumber(ctx, t, l2oo, expectedOutputBlock)

	// Verify output
	outputProposal, err := l2oo.GetL2OutputAfter(ctx, expectedOutputBlock)
	require.NoError(err, "failed to get output proposal from L2OO")
	require.Equal(expectedOutputBlock, outputProposal.L2BlockNumber, "L2 block number mismatch")

	expectedOutput, err := sys.L2EL.Escape().L2EthClient().OutputV0AtBlockNumber(ctx, outputProposal.L2BlockNumber)
	require.NoError(err, "failed to get expected output from L2")
	require.Equal(eth.OutputRoot(expectedOutput), outputProposal.OutputRoot, "output root mismatch")

	logger.Info("Output verified after restart", "block", outputProposal.L2BlockNumber)

	// Verify range proofs
	expectedCount := cfg.ExpectedRangeCount(outputProposal.L2BlockNumber)
	utils.VerifyRangeProofsWithExpected(ctx, t, sys.DatabaseURL(), cfg.StartingBlock, outputProposal.L2BlockNumber, expectedCount)

	logger.Info("Recovery test completed successfully",
		"submissions", expectedSubmissions,
		"rangeProofs", expectedCount)
}
