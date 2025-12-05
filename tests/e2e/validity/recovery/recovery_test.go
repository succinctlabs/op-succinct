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

func TestValidityProposer_RestartRecovery_Basic(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	runRecoveryTest(gt, cfg, 1, 1, 20*time.Minute)
}

func TestValidityProposer_RestartRecovery_ThreeSubmissions(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	runRecoveryTest(gt, cfg, 1, 3, 20*time.Minute)
}

func TestValidityProposer_RestartRecovery_RangeSplit(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	cfg.SubmissionInterval = 20
	cfg.RangeProofInterval = 5
	runRecoveryTest(gt, cfg, 1, 1, 20*time.Minute)
}

func TestValidityProposer_RestartRecovery_MultipleRestarts(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	runRecoveryTest(gt, cfg, 3, 1, 20*time.Minute)
}

func runRecoveryTest(gt *testing.T, cfg opspresets.ValidityConfig, restartCount, expectedSubmissions int, timeout time.Duration) {
	t := devtest.ParallelT(gt)
	sys := opspresets.NewValiditySystem(t, cfg)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), timeout)
	defer cancel()

	logger.Info("Running validity recovery test",
		"restartCount", restartCount,
		"expectedSubmissions", expectedSubmissions)

	// Perform restart cycles
	// The proposer makes incremental progress between each restart cycle
	for i := 1; i <= restartCount; i++ {
		utils.WaitForRangeProofProgress(ctx, t, sys.DatabaseURL(), i)

		countBefore, err := utils.CountRangeProofRequests(ctx, sys.DatabaseURL())
		require.NoError(err, "failed to count range proofs before stop")
		logger.Info("Stopping proposer", "restart", i, "rangeProofRequests", countBefore)

		sys.StopProposer()
		time.Sleep(5 * time.Second) // Wait for lock to expire

		countAfterStop, err := utils.CountRangeProofRequests(ctx, sys.DatabaseURL())
		require.NoError(err, "failed to count range proofs after stop")
		require.Equal(countBefore, countAfterStop, "data should persist after stop")

		sys.StartProposer()
		logger.Info("Proposer restarted", "restart", i)
	}

	// Wait for submissions to complete
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
