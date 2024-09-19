package proposer

import (
	"context"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent"
)

func (l *L2OutputSubmitter) DeriveNewSpanBatches(ctx context.Context) error {
	// nextBlock is equal to the highest value in the `EndBlock` column of the db, plus 1.
	latestL2EndBlock, err := l.db.GetLatestEndBlock()
	if err != nil {
		if ent.IsNotFound(err) {
			latestEndBlockU256, err := l.l2ooContract.LatestBlockNumber(&bind.CallOpts{Context: ctx})
			if err != nil {
				return fmt.Errorf("failed to get latest output index: %w", err)
			} else {
				latestL2EndBlock = latestEndBlockU256.Uint64()
			}
		} else {
			l.Log.Error("failed to get latest end requested", "err", err)
			return err
		}
	}
	newL2StartBlock := latestL2EndBlock + 1
	l.Log.Info("deriving span batch for L2 block", "nextBlock", newL2StartBlock)

	rollupClient, err := l.RollupProvider.RollupClient(ctx)
	if err != nil {
		return fmt.Errorf("failed to get rollup client: %w", err)
	}

	// Get the latest finalized L1 block.
	status, err := rollupClient.SyncStatus(ctx)
	if err != nil {
		l.Log.Error("proposer unable to get sync status", "err", err)
		return err
	}
	// Note: Originally, this used the L1 finalized block. However, to satisfy the new API, we now use the L2 finalized block.
	newL2EndBlock := status.FinalizedL2.Number

	// Once enough blocks have been produced, we can start adding SPAN proofs.
	if newL2EndBlock-l.Cfg.MaxBlockRangePerSpanProof > newL2StartBlock {
		l.Log.Info("Enough blocks have been produced, starting to add SPAN proofs", "start", newL2StartBlock, "end", newL2EndBlock)
		// Add a SPAN proof for every modulo MaxBlockRangePerSpanProof block.
		for start := newL2StartBlock; start <= newL2EndBlock; start += l.Cfg.MaxBlockRangePerSpanProof {
			end := min(start+l.Cfg.MaxBlockRangePerSpanProof, newL2EndBlock)
			err := l.db.NewEntry("SPAN", start, end)
			l.Log.Info("new span proof request", "start", start, "end", end)
			if err != nil {
				l.Log.Error("failed to insert proof request", "err", err, "start", start, "end", end)
				return err
			}
		}
	}

	return nil
}
