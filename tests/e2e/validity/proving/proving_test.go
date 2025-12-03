package proving

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestValidityProposer_SingleSubmission(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	waitForOutputAndVerify(gt, 1, 10*time.Minute, cfg)
}

func TestValidityProposer_ThreeSubmissions(gt *testing.T) {
	cfg := opspresets.DefaultValidityConfig()
	waitForOutputAndVerify(gt, 3, 30*time.Minute, cfg)
}

func waitForOutputAndVerify(gt *testing.T, submissionCount int, timeout time.Duration, cfg opspresets.ValidityConfig) {
	t := devtest.SerialT(gt)
	sys := opspresets.NewValiditySystem(t, cfg)
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), timeout)
	defer cancel()

	l2ooAddr := sys.L2Chain.Escape().Deployment().OPSuccinctL2OutputOracleAddr()
	logger.Info("L2 Output Oracle Address", "address", l2ooAddr.Hex())

	l2oo, err := utils.NewL2OOClient(sys.L1EL.EthClient(), l2ooAddr)
	require.NoError(err, "failed to create L2OO client")

	// Starting block is 1, submission interval is 10
	targetBlockNumber := uint64(submissionCount*int(cfg.SubmissionInterval) + 1)
	utils.WaitForLatestBlockNumber(ctx, t, l2oo, targetBlockNumber)

	// Verify the output root matches expected L2 state
	outputProposal, err := l2oo.GetL2OutputAfter(ctx, targetBlockNumber)
	require.NoError(err, "failed to get output proposal from L2OO")

	expectedOutput, err := sys.L2EL.Escape().L2EthClient().OutputV0AtBlockNumber(ctx, outputProposal.L2BlockNumber)
	require.NoError(err, "failed to get expected output from L2")

	require.Equal(eth.OutputRoot(expectedOutput), outputProposal.OutputRoot, "output root mismatch")
	logger.Info("Output verified", "submissions", submissionCount, "l2BlockNumber", outputProposal.L2BlockNumber)
}
