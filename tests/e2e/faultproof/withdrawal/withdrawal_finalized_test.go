package withdrawal

import (
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
)

// TestFaultProofProposer_WithdrawalFinalized verifies the full withdrawal lifecycle:
// L2 withdrawal initiation → game creation → proof submission → game resolution → finalization
func TestFaultProofProposer_WithdrawalFinalized(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.DefaultFPProposerConfig()
	sys := opspresets.NewFaultProofSystem(t, cfg, opspresets.DefaultL2ChainConfig())
	logger := t.Logger()

	bridge := sys.StandardBridge()

	withdrawAmount := eth.OneHundredthEther

	// Use pre-funded L2 account to initiate withdrawal early (no deposit delay).
	// This ensures the withdrawal happens at a low L2 block number, close to what
	// games already cover, avoiding the race condition where L2 advances far ahead.
	l2User := sys.FunderL2.NewFundedEOA(withdrawAmount.Mul(2)) // Extra for gas
	initialL2Balance := l2User.GetBalance()
	logger.Info("L2 user funded", "balance", initialL2Balance)

	// L1 user for proving and finalizing (needs L1 ETH for gas)
	l1User := sys.FunderL1.NewFundedEOA(eth.Ether(1))
	initialL1Balance := l1User.GetBalance()

	// Log respected game type for debugging
	logger.Info("Using respected game type", "gameType", bridge.RespectedGameType())

	// Phase 1: Initiate withdrawal on L2 (happens early, at low block number)
	logger.Info("Phase 1: Initiating withdrawal on L2")
	withdrawal := bridge.InitiateWithdrawal(withdrawAmount, l2User)
	logger.Info("Withdrawal initiated", "blockHash", withdrawal.InitiateBlockHash())

	// Verify L2 balance after initiation
	expectedL2Balance := initialL2Balance.Sub(withdrawAmount).Sub(withdrawal.InitiateGasCost())
	l2User.VerifyBalanceExact(expectedL2Balance)

	// Phase 2: Prove withdrawal on L1
	// forGamePublished will wait for a game covering the withdrawal block.
	logger.Info("Phase 2: Proving withdrawal on L1")
	withdrawal.Prove(l1User)
	expectedL1Balance := initialL1Balance.Sub(withdrawal.ProveGasCost())
	l1User.VerifyBalanceExact(expectedL1Balance)

	// Phase 3: Advance time and wait for game resolution
	logger.Info("Phase 3: Waiting for dispute game resolution")
	sys.AdvanceTime(bridge.GameResolutionDelay())
	withdrawal.WaitForDisputeGameResolved()
	logger.Info("Dispute game resolved with DefenderWins")

	// Phase 4: Advance time for finalization delays
	sys.AdvanceTime(max(bridge.WithdrawalDelay()-bridge.GameResolutionDelay(), bridge.DisputeGameFinalityDelay()))

	// Phase 5: Finalize withdrawal on L1
	logger.Info("Phase 5: Finalizing withdrawal on L1")
	logger.Info("Attempting to finalize", "proofMaturity", bridge.WithdrawalDelay(),
		"gameResolutionDelay", bridge.GameResolutionDelay(),
		"gameFinalityDelay", bridge.DisputeGameFinalityDelay())
	withdrawal.Finalize(l1User)
	expectedL1Balance = expectedL1Balance.Sub(withdrawal.FinalizeGasCost()).Add(withdrawAmount)
	l1User.VerifyBalanceExact(expectedL1Balance)

	logger.Info("Withdrawal finalization test completed successfully")
}
