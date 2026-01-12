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

	initialL1Balance := eth.Ether(1)
	depositAmount := eth.OneTenthEther
	withdrawAmount := eth.OneHundredthEther

	// Use same key for both chains (upstream pattern)
	l1User := sys.FunderL1.NewFundedEOA(initialL1Balance)
	l2User := l1User.AsEL(sys.L2EL)

	// Fund L2 user via deposit
	logger.Info("Depositing to L2")
	deposit := bridge.Deposit(depositAmount, l1User)
	expectedL1UserBalance := initialL1Balance.Sub(depositAmount).Sub(deposit.GasCost())
	l1User.VerifyBalanceExact(expectedL1UserBalance)
	l2User.VerifyBalanceExact(depositAmount)

	// Log respected game type for debugging
	logger.Info("Using respected game type", "gameType", bridge.RespectedGameType())

	// Phase 1: Initiate withdrawal on L2
	logger.Info("Phase 1: Initiating withdrawal on L2")
	withdrawal := bridge.InitiateWithdrawal(withdrawAmount, l2User)
	logger.Info("Withdrawal initiated", "blockHash", withdrawal.InitiateBlockHash())

	// Verify L2 balance after initiation
	expectedL2UserBalance := depositAmount.Sub(withdrawAmount).Sub(withdrawal.InitiateGasCost())
	l2User.VerifyBalanceExact(expectedL2UserBalance)

	// Phase 2: Prove withdrawal on L1
	logger.Info("Phase 2: Proving withdrawal on L1")
	withdrawal.Prove(l1User)
	expectedL1UserBalance = expectedL1UserBalance.Sub(withdrawal.ProveGasCost())
	l1User.VerifyBalanceExact(expectedL1UserBalance)

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
	expectedL1UserBalance = expectedL1UserBalance.Sub(withdrawal.FinalizeGasCost()).Add(withdrawAmount)
	l1User.VerifyBalanceExact(expectedL1UserBalance)

	logger.Info("Withdrawal finalization test completed successfully")
}
