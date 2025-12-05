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

func TestValidityProposer_RestartRecovery(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.DefaultValidityConfig()
	sys := opspresets.NewValiditySystem(t, cfg)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), 20*time.Minute)
	defer cancel()

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

	// Brief pause to ensure clean shutdown
	time.Sleep(5 * time.Second)

  // Verify data persisted before restart
  countAfterStop, err := utils.CountRangeProofRequests(ctx, sys.DatabaseURL())
  require.NoError(err, "failed to count range proofs after stop")
  require.Equal(countBefore, countAfterStop, "range proof data should persist after proposer stop")
  logger.Info("Data persistence verified", "count", countAfterStop)

	// Restart the proposer
	sys.StartProposer()
	logger.Info("Proposer restarted")

	// Wait for submission to complete
	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	l2oo, err := utils.NewL2OOClient(sys.L1EL.EthClient(), l2ooAddr)
	require.NoError(err, "failed to create L2OO client")

	expectedOutputBlock := cfg.ExpectedOutputBlock(1)
	logger.Info("Waiting for output after restart", "expectedBlock", expectedOutputBlock)

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
}
