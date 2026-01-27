// Package fpwithdrawal tests the full withdrawal lifecycle through OptimismPortal2.
package fpwithdrawal

import (
	"context"
	"testing"
	"time"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestFaultProof_WithdrawalFinalized verifies the complete withdrawal lifecycle:
// 1. User initiates withdrawal on L2 (L2ToL1MessagePasser)
// 2. Proposer creates game covering the withdrawal block
// 3. Withdrawal is proven against the dispute game (OptimismPortal2.proveWithdrawal)
// 4. Game resolves as DEFENDER_WINS
// 5. Finality delay elapses
// 6. Withdrawal is finalized (OptimismPortal2.finalizeWithdrawalTransaction)
// 7. Funds are received on L1
func TestFaultProof_WithdrawalFinalized(gt *testing.T) {
	t := devtest.SerialT(gt)
	logger := t.Logger()

	// === SETUP ===
	// Configure proposer with fast finality mode - this allows the proposer to create
	// games for recent blocks (unsafe head) rather than only finalized blocks.
	// This is critical for the withdrawal test because the chain advances rapidly
	// during deposit/withdrawal operations.
	proposerCfg := opspresets.FastFinalityFPProposerConfig()
	proposerCfg.ProposalIntervalInBlocks = 5 // Create games very frequently for withdrawal test

	sys := opspresets.NewFaultProofSystem(t, proposerCfg, opspresets.DefaultL2ChainConfig())

	// Log DGF address for debugging (this is the OPSuccinct DGF that should have game type 42)
	dgfAddr := sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	logger.Info("DGF address from L2Deployment", "address", dgfAddr.Hex())

	// Create DgfClient using same DGF address - verify it works
	dgf := sys.DgfClient(t)

	// Get the standard bridge DSL
	bridge := sys.StandardBridge()

	// Also log the DGF address from L2Chain to compare
	l2ChainDgfAddr := sys.L2Chain.DisputeGameFactoryProxyAddr()
	logger.Info("DGF address from L2Chain (used by StandardBridge)", "address", l2ChainDgfAddr.Hex())
	if dgfAddr != l2ChainDgfAddr {
		logger.Error("DGF ADDRESS MISMATCH!",
			"L2Deployment", dgfAddr.Hex(),
			"L2Chain", l2ChainDgfAddr.Hex())
	}

	// Log the respected game type - this should be 42
	gameType := bridge.RespectedGameType()
	logger.Info("FaultProof withdrawal test starting",
		"respectedGameType", gameType,
		"dgfAddress", dgfAddr.Hex())

	// Now try the StandardBridge methods that query the DGF
	// These should work now that we've verified game type 42 is registered
	logger.Info("Testing StandardBridge DGF queries",
		"withdrawalDelay", bridge.WithdrawalDelay(),
		"gameResolutionDelay", bridge.GameResolutionDelay(),
		"disputeGameFinalityDelay", bridge.DisputeGameFinalityDelay())

	// === Fund test accounts ===
	initialL1Balance := eth.OneThirdEther

	// l1User and l2User share same private key
	l1User := sys.FunderL1.NewFundedEOA(initialL1Balance)
	l2User := l1User.AsEL(sys.L2EL) // Only receives funds via the deposit

	depositAmount := eth.OneTenthEther
	withdrawalAmount := eth.OneHundredthEther

	// === PHASE 1: Deposit from L1 to L2 ===
	// The max amount of withdrawal is limited to the total amount of deposit
	// We trigger deposit first to fund the L1 ETHLockbox to satisfy the invariant
	// IMPORTANT: Do deposit/withdrawal EARLY before waiting for games, so the withdrawal
	// block number is low enough for games to catch up within the 90s timeout.
	logger.Info("Phase 1: Depositing ETH from L1 to L2", "amount", depositAmount)

	deposit := bridge.Deposit(depositAmount, l1User)
	expectedL1UserBalance := initialL1Balance.Sub(depositAmount).Sub(deposit.GasCost())
	l1User.VerifyBalanceExact(expectedL1UserBalance)

	expectedL2UserBalance := depositAmount
	l2User.VerifyBalanceExact(expectedL2UserBalance)

	logger.Info("Deposit complete, L2 balance confirmed")

	// === PHASE 2: Initiate withdrawal on L2 ===
	logger.Info("Phase 2: Initiating withdrawal on L2", "amount", withdrawalAmount)

	withdrawal := bridge.InitiateWithdrawal(withdrawalAmount, l2User)
	expectedL2UserBalance = expectedL2UserBalance.Sub(withdrawalAmount).Sub(withdrawal.InitiateGasCost())
	l2User.VerifyBalanceExact(expectedL2UserBalance)

	logger.Info("Withdrawal initiated on L2",
		"initiateBlockHash", withdrawal.InitiateBlockHash().Hex())

	// === PHASE 2.5: Ensure proposer is active ===
	// Wait for at least one game to be created, confirming the proposer is running.
	// The Prove() call below has its own 90-second timeout to find a game covering
	// the withdrawal block - no need to wait for many games upfront.
	ctx, cancel := context.WithTimeout(t.Ctx(), 2*time.Minute)
	defer cancel()

	logger.Info("Waiting for at least 1 game to confirm proposer is active")
	utils.WaitForGameCount(ctx, t, dgf, 1)
	logger.Info("Proposer is active, proceeding to prove withdrawal")

	// === PHASE 3: Prove withdrawal on L1 ===
	// This waits for a game covering the withdrawal block to be published (90s timeout in bridge.go)
	logger.Info("Phase 3: Proving withdrawal on L1 (waiting for game)")

	withdrawal.Prove(l1User)
	expectedL1UserBalance = expectedL1UserBalance.Sub(withdrawal.ProveGasCost())
	l1User.VerifyBalanceExact(expectedL1UserBalance)

	logger.Info("Withdrawal proven on L1")

	// === PHASE 4: Wait for game resolution ===
	logger.Info("Phase 4: Waiting for game resolution (DEFENDER_WINS)")

	// Advance time until game is resolvable
	sys.AdvanceTime(bridge.GameResolutionDelay())
	withdrawal.WaitForDisputeGameResolved()

	logger.Info("Game resolved as DEFENDER_WINS")

	// === PHASE 5: Wait for finality delay ===
	logger.Info("Phase 5: Waiting for finality delay")

	// Advance time to when game finalization and proof finalization delay has expired
	remainingDelay := max(bridge.WithdrawalDelay()-bridge.GameResolutionDelay(), bridge.DisputeGameFinalityDelay())
	sys.AdvanceTime(remainingDelay)

	logger.Info("Finality delay elapsed")

	// === PHASE 6: Finalize withdrawal on L1 ===
	logger.Info("Phase 6: Finalizing withdrawal on L1",
		"proofMaturity", bridge.WithdrawalDelay(),
		"gameResolutionDelay", bridge.GameResolutionDelay(),
		"gameFinalityDelay", bridge.DisputeGameFinalityDelay())

	withdrawal.Finalize(l1User)
	expectedL1UserBalance = expectedL1UserBalance.Sub(withdrawal.FinalizeGasCost()).Add(withdrawalAmount)
	l1User.VerifyBalanceExact(expectedL1UserBalance)

	// === PHASE 7: Verify funds received ===
	logger.Info("Phase 7: Withdrawal finalized successfully!",
		"withdrawalAmount", withdrawalAmount,
		"finalL1Balance", expectedL1UserBalance)

	logger.Info("✅ TestFaultProof_WithdrawalFinalized PASSED: Full withdrawal lifecycle verified")
}
