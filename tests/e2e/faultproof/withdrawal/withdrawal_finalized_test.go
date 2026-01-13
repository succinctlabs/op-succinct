package withdrawal

import (
	"context"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestFaultProofProposer_WithdrawalFinalized verifies the full withdrawal lifecycle:
// L2 withdrawal initiation → game creation → proof submission → game resolution → finalization
func TestFaultProofProposer_WithdrawalFinalized(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.DefaultFPProposerConfig()
	sys := opspresets.NewFaultProofSystem(t, cfg, opspresets.DefaultL2ChainConfig())
	logger := t.Logger()

	bridge := sys.StandardBridge()

	// Wait for proposer to build up game coverage before proceeding.
	// System setup produces ~250 L2 blocks while the proposer starts at block ~10.
	// By waiting for enough games upfront, we ensure the withdrawal will be at a block
	// the proposer has already covered or will cover soon.
	// Rate: ~1.15 games/min (23 games in 20 min observed). Need LongTimeout for 30 games.
	dgf := sys.DgfClient(t)
	ctx, cancel := context.WithTimeout(t.Ctx(), utils.LongTimeout())
	defer cancel()

	logger.Info("Waiting for proposer to build game coverage")
	utils.WaitForGameCount(ctx, t, dgf, 30) // ~300 blocks of coverage (30 games × 10 blocks/game)

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
	// Since we waited for game coverage upfront, the withdrawal block should be
	// covered by existing games or the proposer will catch up quickly.
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
