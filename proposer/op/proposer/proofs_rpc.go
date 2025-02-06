package proposer

import (
	"context"
	"fmt"
	"time"

	"github.com/ethereum-optimism/optimism/op-service/dial"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/log"
	gethrpc "github.com/ethereum/go-ethereum/rpc"
	"github.com/succinctlabs/op-succinct-go/proposer/db"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent/proofrequest"
)

// Define a new Proofs API that will provider the Proofs API to the RPC server
type ProofsAPI struct {
	logger log.Logger
	db     db.ProofDB
	mock   bool
	driver *L2OutputSubmitter
}

func NewProofsAPI(db db.ProofDB, logger log.Logger, mock bool, driver *L2OutputSubmitter) (*ProofsAPI, error) {
	return &ProofsAPI{
		logger: logger,
		db:     db,
		mock:   mock,
		driver: driver,
	}, nil
}

// GetProofsAPI returns the ProofsAPI struct
func GetProofsAPI(api *ProofsAPI) gethrpc.API {
	return gethrpc.API{
		Namespace: "proofs",
		Service:   api,
	}
}

type RequestAggProofResponse struct {
	StartBlock     uint64 `json:"start_block"`
	EndBlock       uint64 `json:"end_block"`
	ProofRequestID string `json:"proof_request_id"`
}

func (pa *ProofsAPI) RequestAggProof(ctx context.Context, startBlock, maxBlock, l1BlockNumber uint64, l1BlockHash common.Hash) (*RequestAggProofResponse, error) {
	pa.logger.Info("requesting agg proof from server", "start", startBlock, "max end", maxBlock)

	// Return the proof request ID or the mock proof request ID
	proofRequestID := "mock_proof_request_id"

	// Get the L1 head for the L2 block
	rollupClient, err := dial.DialRollupClientWithTimeout(ctx, dial.DefaultDialTimeout, pa.logger, pa.driver.Cfg.RollupRpc)
	if err != nil {
		return nil, err
	}

	// If the L1 head corresponding to the maxBlock is higher than the L1 block number,
	// we need to decrease the maxBlock and query the L1 head again until we find an L1 head,
	// that is lower or equal to the L1 block number.
	maxL1Block, err := pa.driver.GetL1HeadForL2Block(ctx, rollupClient, maxBlock)
	if err != nil {
		return nil, fmt.Errorf("failed to get l1 head for l2 block: %w", err)
	}

	for ; maxL1Block > l1BlockNumber; maxBlock-- {
		maxL1Block, err = pa.driver.GetL1HeadForL2Block(ctx, rollupClient, maxBlock)
		if err != nil {
			return nil, fmt.Errorf("failed to get l1 head for l2 block: %w", err)
		}
	}

	// Store an Agg proof creation entry in the DB using the start block and the max block
	created, endBlock, err := pa.db.TryCreateAggProofFromSpanProofsLimit(startBlock, maxBlock, l1BlockNumber, l1BlockHash.Hex())
	if err != nil {
		return nil, fmt.Errorf("failed to create agg proof from span proofs: %w", err)
	}

	if created {
		pa.logger.Info("created new AGG proof", "from", startBlock, "to", endBlock)
	} else {
		// Return an error or mock
		if pa.mock {
			return &RequestAggProofResponse{
				StartBlock:     startBlock,
				EndBlock:       endBlock,
				ProofRequestID: "mock_proof_request_id",
			}, nil
		}

		// TODO: Should we return here the proof request ID or the proof?
		return nil, fmt.Errorf("failed to create agg proof from span proofs: already exists")
	}

	// Poll with a ticker for the aggproof request ID creation or the proof itself if it's a mock proof
	preqs := []*ent.ProofRequest{}
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()
	// End the loop when we find a proof request with status PROVING or COMPLETE
	for len(preqs) <= 0 {
		select {
		case <-ctx.Done():
			return nil, fmt.Errorf("context cancelled")
		case <-ticker.C:
			preqs, err = pa.db.GetProofRequestsWithBlockRange(proofrequest.TypeAGG, startBlock, endBlock)
			if err != nil {
				return nil, fmt.Errorf("failed to get proof request with block range: %w", err)
			}

			for _, preq := range preqs {
				if !pa.mock {
					// If we're not mocking, then we should return the proof request ID
					proofRequestID = preq.ProverRequestID
				}

				switch preq.Status {
				case proofrequest.StatusPROVING:
					pa.logger.Info("agg proof request is still proving", "proof_request_id", proofRequestID)
				case proofrequest.StatusCOMPLETE:
					pa.logger.Info("agg proof request is complete", "proof_request_id", proofRequestID)
				case proofrequest.StatusFAILED:
					return nil, fmt.Errorf("agg proof request failed")
				}
			}
		}
	}

	if !pa.mock {
		proofRequestID = preqs[0].ProverRequestID
	}

	return &RequestAggProofResponse{
		StartBlock:     startBlock,
		EndBlock:       endBlock,
		ProofRequestID: proofRequestID,
	}, nil
}
