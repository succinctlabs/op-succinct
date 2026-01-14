package withdrawal

import (
	"testing"
	"time"

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
	respectedGameType := bridge.RespectedGameType()
	logger.Info("Using respected game type", "gameType", respectedGameType)

	// Wait for games to cover current L2 block before initiating withdrawal.
	// This ensures the proposer has caught up to L2, so the withdrawal proof
	// can be constructed from consistent, verified state.
	dgf := sys.DgfClient(t)
	currentL2Block := sys.L2EL.BlockRefByLabel(eth.Unsafe).Number
	logger.Info("Waiting for game coverage before withdrawal", "currentL2Block", currentL2Block)
	waitForGameCoveringBlock(t, dgf, sys, currentL2Block, respectedGameType)

	// Phase 1: Initiate withdrawal on L2
	logger.Info("Phase 1: Initiating withdrawal on L2")
	withdrawal := bridge.InitiateWithdrawal(withdrawAmount, l2User)
	logger.Info("Withdrawal initiated", "blockHash", withdrawal.InitiateBlockHash())

	// Verify L2 balance after initiation
	expectedL2UserBalance := depositAmount.Sub(withdrawAmount).Sub(withdrawal.InitiateGasCost())
	l2User.VerifyBalanceExact(expectedL2UserBalance)

	// Phase 2: Prove withdrawal on L1
	// forGamePublished will wait for a game covering the withdrawal block.
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

// waitForGameCoveringBlock waits until a game exists that covers the given L2 block number.
// This ensures the proposer has caught up to L2 state before we initiate a withdrawal.
func waitForGameCoveringBlock(t devtest.T, dgf *utils.DgfClient, sys *opspresets.FaultProofSystem, targetBlock uint64, gameType uint32) {
	ctx := t.Ctx()
	logger := t.Logger()

	t.Require().Eventuallyf(func() bool {
		game, err := dgf.LatestGame(ctx)
		if err != nil || game == nil {
			logger.Info("No games found yet, waiting...")
			return false
		}

		// Only consider games of the required type
		if game.GameType != gameType {
			logger.Info("Latest game is wrong type", "gameType", game.GameType, "expected", gameType)
			return false
		}

		// Get the L2 block number this game covers
		fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
		if err != nil {
			logger.Warn("Failed to create FDG client", "err", err)
			return false
		}

		gameL2Block, err := fdg.L2BlockNumber(ctx)
		if err != nil {
			logger.Warn("Failed to get game L2 block", "err", err)
			return false
		}

		logger.Info("Checking game coverage", "gameL2Block", gameL2Block, "targetBlock", targetBlock)
		return gameL2Block >= targetBlock
	}, 40*time.Minute, 1*time.Second, "waiting for game covering L2 block %d", targetBlock)

	logger.Info("Game coverage achieved", "targetBlock", targetBlock)
}
