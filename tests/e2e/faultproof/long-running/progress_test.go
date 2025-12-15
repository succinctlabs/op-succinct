package longrunning

import (
	"context"
	"fmt"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// MaxProposerLag is the maximum allowed gap between L2 finalized head and the latest game's
// L2 block. Set to 150 with a 100-block proposal interval, providing 50 blocks of headroom
// to absorb transient spikes from proposer startup or game resolution/bond claiming.
const MaxProposerLag uint64 = 200

// TestFaultProofProposer_Progress verifies the proposer maintains acceptable lag for 20 minutes.
// The test succeeds if lag stays below MaxProposerLag throughout; fails immediately if exceeded.
func TestFaultProofProposer_Progress(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.LongRunningFaultProofConfig()
	sys, dgf := setupFaultProofSystem(t, cfg)

	err := utils.RunProgressTest(func() error {
		return checkProposerLag(t, sys, dgf, MaxProposerLag)
	})
	t.Require().NoError(err, "proposer progress check failed")
}

// TestFaultProofProposer_FastFinality_Progress verifies fast finality mode maintains acceptable lag for 20 minutes
// and ensures games are being proven (not just created).
func TestFaultProofProposer_FastFinality_Progress(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.LongRunningFastFinalityFaultProofConfig()
	sys, dgf := setupFaultProofSystem(t, cfg)

	err := utils.RunProgressTest(func() error {
		if err := checkProposerLag(t, sys, dgf, MaxFastFinalityLag); err != nil {
			return err
		}
		return checkAnchorStateLag(t, sys, dgf, cfg)
	})
	t.Require().NoError(err, "proposer progress check failed")

	// Verify fast finality is proving games
	ctx := t.Ctx()
	firstGame, err := dgf.GameAtIndex(ctx, 0)
	t.Require().NoError(err)
	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), firstGame.Proxy)
	t.Require().NoError(err)
	proven, err := fdg.IsProven(ctx)
	t.Require().NoError(err)
	t.Require().True(proven, "fast finality did not prove first game")
}

func setupFaultProofSystem(t devtest.T, cfg opspresets.FaultProofConfig) (*opspresets.FaultProofSystem, *utils.DgfClient) {
	sys := opspresets.NewFaultProofSystem(t, cfg)
	t.Log("=== Stack is running ===")
	return sys, sys.DgfClient(t)
}

func checkProposerLag(t devtest.T, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient, maxLag uint64) error {
	l2Finalized := sys.L2EL.BlockRefByLabel(eth.Finalized)

	fdg, game, err := getFdgWithLatestGame(t.Ctx(), sys, dgf)
	if err != nil {
		return err
	}
	if game == nil {
		t.Logf("Games: 0 | L2 finalized: %d | waiting...", l2Finalized.Number)
		return nil
	}

	gameL2Block, err := fdg.L2BlockNumber(t.Ctx())
	if err != nil {
		return err
	}

	var lag uint64
	if l2Finalized.Number > gameL2Block {
		lag = l2Finalized.Number - gameL2Block
	}
	t.Logf("L2 Finalized: %d | Latest Game L2 Block: %d | Lag: %d blocks",
		l2Finalized.Number, gameL2Block, lag)

	if lag > maxLag {
		return fmt.Errorf("proposer lag %d exceeds max %d", lag, maxLag)
	}
	return nil
}

func checkAnchorStateLag(t devtest.T, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient, cfg opspresets.FaultProofConfig) error {
	fdg, game, err := getFdgWithLatestGame(t.Ctx(), sys, dgf)
	if err != nil {
		return err
	}
	if game == nil {
		return nil
	}

	gameL2Block, err := fdg.L2BlockNumber(t.Ctx())
	if err != nil {
		return err
	}
	anchorL2Block, err := fdg.AnchorL2BlockNumber(t.Ctx(), sys.L1EL.EthClient(), game.GameType)
	if err != nil {
		return err
	}

	anchorLagBlocks := gameL2Block - anchorL2Block
	anchorLagSeconds := anchorLagBlocks * cfg.L2BlockTime
	t.Logf("Anchor Lag: game L2=%d, anchor L2=%d, lag=%d blocks (%ds), max=%ds",
		gameL2Block, anchorL2Block, anchorLagBlocks, anchorLagSeconds, cfg.MaxChallengeDuration)

	if anchorLagSeconds >= cfg.MaxChallengeDuration {
		return fmt.Errorf("anchor lag %d seconds exceeds MaxChallengeDuration %d seconds", anchorLagSeconds, cfg.MaxChallengeDuration)
	}
	return nil
}

func getFdgWithLatestGame(ctx context.Context, sys *opspresets.FaultProofSystem, dgf *utils.DgfClient) (*utils.FdgClient, *utils.GameAtIndexResult, error) {
	game, err := dgf.LatestGame(ctx)
	if err != nil || game == nil {
		return nil, nil, err
	}
	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
	if err != nil {
		return nil, nil, err
	}
	return fdg, game, nil
}
