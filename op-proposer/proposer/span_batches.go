package proposer

import (
	"context"
	"fmt"
	"log"
	"math"
	"math/big"

	"github.com/ethereum-optimism/optimism/op-node/cmd/batch_decoder/fetch"
	"github.com/ethereum-optimism/optimism/op-node/cmd/batch_decoder/reassemble"
	"github.com/ethereum-optimism/optimism/op-node/rollup"
	"github.com/ethereum-optimism/optimism/op-service/client"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/moongate-forks/kona-sp1/op-proposer/proposer/db/ent"
)

func (l *L2OutputSubmitter) DeriveNewSpanBatches(ctx context.Context) error {
	// nextBlock is equal to the highest value in the `EndBlock` column of the db, plus 1
	latestEndBlock, err := l.db.GetLatestEndBlock()
	if err != nil {
		if ent.IsNotFound(err) {
			latestEndBlockU256, err := l.l2ooContract.LatestBlockNumber(&bind.CallOpts{Context: ctx})
			if err != nil {
				return fmt.Errorf("failed to get latest output index: %w", err)
			} else {
				latestEndBlock = latestEndBlockU256.Uint64()
			}
		} else {
			l.Log.Error("failed to get latest end requested", "err", err)
			return err
		}
	}
	nextBlock := latestEndBlock + 1
	l.Log.Info("deriving span batch for L2 block", "nextBlock", nextBlock)

	// use batch decoder to pull all batches from next block's L1 Origin through Finalized L1 from chain to disk
	err = l.FetchBatchesFromChain(ctx, nextBlock)
	if err != nil {
		l.Log.Error("failed to fetch batches from chain", "err", err)
		return err
	}

	for {
		// use batch decoder to reassemble the batches from disk to determine the start and end of relevant span batch
		start, end, err := l.GenerateSpanBatchRange(ctx, nextBlock, l.DriverSetup.Cfg.MaxSpanBatchDeviation)
		if err == reassemble.NoSpanBatchFoundError {
			l.Log.Info("no span batch found", "nextBlock", nextBlock)
			break
		} else if err == reassemble.MaxDeviationExceededError {
			l.Log.Info("max deviation exceeded, autofilling", "start", start, "end", end)
		} else if err != nil {
			l.Log.Error("failed to generate span batch range", "err", err)
			return err
		}

		// the nextBlock should always be the start of a new span batch, warn if not
		if start != nextBlock {
			l.Log.Warn("start block does not match next block", "start", start, "nextBlock", nextBlock)
		}

		l.Log.Info("found span batch range", "start", start, "end", end)

		tmpStart := nextBlock
		for {
			maxEnd := tmpStart + l.DriverSetup.Cfg.MaxBlockRangePerSpanProof - 1
			tmpEnd := uint64(math.Min(float64(maxEnd), float64(end)))

			// insert the new span into the db to be requested in the future
			l.Log.Info("inserting span proof request", "start", tmpStart, "end", tmpEnd)
			err = l.db.NewEntry("SPAN", tmpStart, tmpEnd)
			if err != nil {
				l.Log.Error("failed to insert proof request", "err", err)
				return err
			}

			if tmpEnd == end {
				break
			}

			tmpStart = tmpEnd + 1
		}

		nextBlock = end + 1
	}

	return nil
}

func (l *L2OutputSubmitter) FetchBatchesFromChain(ctx context.Context, nextBlock uint64) error {
	proposerConfig := l.DriverSetup.Cfg
	l1Client := l.DriverSetup.L1Client

	l1Origin, finalizedL1, err := l.getL1OriginAndFinalized(ctx, nextBlock)
	if err != nil {
		return err
	}

	cCtx, cancel := context.WithTimeout(context.Background(), l.Cfg.NetworkTimeout)
	defer cancel()
	chainID, err := l1Client.ChainID(cCtx)
	if err != nil {
		log.Fatal(err)
		return err
	}
	beaconAddr := proposerConfig.BeaconRpc
	var beacon *sources.L1BeaconClient
	if beaconAddr != "" {
		beaconClient := sources.NewBeaconHTTPClient(client.NewBasicHTTPClient(beaconAddr, nil))
		beaconCfg := sources.L1BeaconClientConfig{FetchAllSidecars: false}
		beacon = sources.NewL1BeaconClient(beaconClient, beaconCfg)
		_, err := beacon.GetVersion(cCtx)
		if err != nil {
			log.Fatal(fmt.Errorf("failed to check L1 Beacon API version: %w", err))
			return err
		}
	} else {
		fmt.Println("L1 Beacon endpoint not set. Unable to fetch post-ecotone channel frames")
		return err
	}

	batchInbox, batcherAddress := common.Address{}, common.Address{}
	rollupCfg, err := rollup.LoadOPStackRollupConfig(l.Cfg.L2ChainID)
	if err != nil {
		l.Log.Warn("failed to load rollup config, trying cli args: %w", err)
		if (l.Cfg.BatcherAddress == common.Address{} || l.Cfg.BatchInbox == common.Address{}) {
			return err
		}
		batchInbox = l.Cfg.BatchInbox
		batcherAddress = l.Cfg.BatcherAddress
	} else {
		batchInbox = rollupCfg.BatchInboxAddress
		batcherAddress = rollupCfg.Genesis.SystemConfig.BatcherAddr
	}

	l.Log.Info("Fetching batches from L1 Origin to Finalized L1", "l1 origin", l1Origin, "Finalized L1", finalizedL1)
	fetchConfig := fetch.Config{
		Start:   l1Origin,
		End:     finalizedL1,
		ChainID: chainID,
		BatchSenders: map[common.Address]struct{}{
			batcherAddress: {},
		},
		BatchInbox:         batchInbox,
		OutDirectory:       proposerConfig.TxCacheOutDir,
		ConcurrentRequests: proposerConfig.BatchDecoderConcurrentReqs,
	}

	// TODO: Optimization to avoid it refetching same batches.
	totalValid, _ := fetch.Batches(l1Client, beacon, fetchConfig)
	l.Log.Info("Successfully fetched batches", "totalValid", totalValid)
	return nil
}

func (l *L2OutputSubmitter) GenerateSpanBatchRange(ctx context.Context, nextBlock, maxSpanBatchDeviation uint64) (uint64, uint64, error) {
	batchInbox, l2BlockTime, genesisTimestamp := common.Address{}, uint64(0), uint64(0)
	rollupCfg, err := rollup.LoadOPStackRollupConfig(l.Cfg.L2ChainID)
	if err != nil {
		if (l.Cfg.BatchInbox == common.Address{}) {
			return 0, 0, err
		}
		cCtx, cancel := context.WithTimeout(ctx, l.Cfg.NetworkTimeout)
		defer cancel()
		callOpts := &bind.CallOpts{
			From:    l.Txmgr.From(),
			Context: cCtx,
		}
		l2BlockTimeU256, err := l.l2ooContract.L2BLOCKTIME(callOpts)
		if err != nil {
			return 0, 0, fmt.Errorf("error pulling l2 block time from L2OO contract: %w", err)
		}
		l2BlockTime = l2BlockTimeU256.Uint64()

		genesisTimestampU256, err := l.l2ooContract.StartingTimestamp(callOpts)
		if err != nil {
			return 0, 0, fmt.Errorf("error pulling genesis timestamp from L2OO contract: %w", err)
		}
		genesisTimestamp = genesisTimestampU256.Uint64()
	} else {
		batchInbox = rollupCfg.BatchInboxAddress
		l2BlockTime = rollupCfg.BlockTime
		genesisTimestamp = rollupCfg.Genesis.L2Time
	}

	reassembleConfig := reassemble.Config{
		BatchInbox:    batchInbox,
		InDirectory:   l.DriverSetup.Cfg.TxCacheOutDir,
		OutDirectory:  "",
		L2ChainID:     new(big.Int).SetUint64(l.Cfg.L2ChainID),
		L2GenesisTime: genesisTimestamp,
		L2BlockTime:   l2BlockTime,
	}

	return reassemble.GetSpanBatchRange(reassembleConfig, rollupCfg, nextBlock, maxSpanBatchDeviation)
}

func (l *L2OutputSubmitter) getL1OriginAndFinalized(ctx context.Context, nextBlock uint64) (uint64, uint64, error) {
	cCtx, cancel := context.WithTimeout(ctx, l.Cfg.NetworkTimeout)
	defer cancel()

	rollupClient, err := l.RollupProvider.RollupClient(ctx)
	if err != nil {
		l.Log.Error("proposer unable to get rollup client", "err", err)
		return 0, 0, err
	}

	output, err := rollupClient.OutputAtBlock(cCtx, nextBlock)
	if err != nil {
		l.Log.Error("proposer unable to get sync status", "err", err)
		return 0, 0, err
	}
	l1Origin := output.BlockRef.L1Origin.Number

	// get the latest finalized L1
	status, err := rollupClient.SyncStatus(cCtx)
	if err != nil {
		l.Log.Error("proposer unable to get sync status", "err", err)
		return 0, 0, err
	}
	finalizedL1 := status.FinalizedL1.Number

	return l1Origin, finalizedL1, nil
}
