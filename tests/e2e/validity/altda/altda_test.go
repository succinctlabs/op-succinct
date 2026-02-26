// Package altda tests validity proof generation with AltDA (Alternative Data Availability) mode.
// Batch data is posted off-chain to a DA server; only Keccak256 commitments are submitted to L1.
// The validity proposer generates range proofs using the AltDA ELF, which fetches batch data
// from the DA server via ALTDA_SERVER_URL instead of reading raw calldata from L1.
package altda

import (
	"context"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

func TestValidityProposer_AltDA_SingleSubmission(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.DefaultValidityConfig()
	sys := opspresets.NewValiditySystem(t, cfg, opspresets.DefaultL2ChainConfig(), opspresets.WithAltDA())
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), utils.ShortTimeout())
	defer cancel()

	l2oo := sys.L2OOClient(t)
	expectedOutputBlock := cfg.ExpectedOutputBlock(1)
	logger.Info("Waiting for AltDA output", "expectedBlock", expectedOutputBlock)

	utils.WaitForLatestBlockNumber(ctx, t, l2oo, expectedOutputBlock)

	outputProposal, err := l2oo.GetL2OutputAfter(ctx, expectedOutputBlock)
	require.NoError(err, "failed to get output proposal from L2OO")

	require.Equal(expectedOutputBlock, outputProposal.L2BlockNumber, "L2 block number mismatch")

	err = utils.VerifyOutputRoot(ctx, sys.L2EL.Escape().L2EthClient(), outputProposal.L2BlockNumber, outputProposal.OutputRoot)
	require.NoError(err, "output root verification failed")

	logger.Info("AltDA output verified", "block", outputProposal.L2BlockNumber)

	expectedCount := cfg.ExpectedRangeCount(outputProposal.L2BlockNumber)
	utils.VerifyRangeProofsWithExpected(ctx, t, sys.DatabaseURL(), cfg.StartingBlock, outputProposal.L2BlockNumber, expectedCount)
}
