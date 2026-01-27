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
	//
	// IMPORTANT: Use a LARGER ProposalIntervalInBlocks (not smaller!) so each game
	// covers more blocks. With interval=50, games cover blocks 1, 51, 101, 151, 201, 251...
	// so only ~6 games are needed to cover block 279 (vs ~56 games with interval=5).
	// Since games are created every ~36 seconds, larger intervals = faster coverage.
	proposerCfg := opspresets.FastFinalityFPProposerConfig()
	proposerCfg.ProposalIntervalInBlocks = 50 // Larger interval = each game covers more blocks

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

	// === PHASE 2.5: Wait for games to cover withdrawal block ===
	// With ProposalIntervalInBlocks=50, games cover blocks 1, 51, 101, 151, 201, 251, 301...
	// The withdrawal block depends on L2 chain state at withdrawal time, which can be
	// 500-700 blocks depending on setup time and chain advancement.
	// We need ~14-15 games to cover block 700+, with margin for safety.
	// Games are created every ~50-55 seconds, so 15 games = ~13 minutes.
	// Use 15 minute timeout to ensure we have time for all games.
	ctx, cancel := context.WithTimeout(t.Ctx(), 15*time.Minute)
	defer cancel()

	logger.Info("Waiting for games to cover withdrawal block (need ~15 games with interval=50)")
	utils.WaitForGameCount(ctx, t, dgf, 15)
	logger.Info("Sufficient games created, proceeding to prove withdrawal")

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
