package proposer

import (
	"context"
	"fmt"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/log"
	gethrpc "github.com/ethereum/go-ethereum/rpc"
	"github.com/succinctlabs/op-succinct-go/proposer/db"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent/proofrequest"
)

// Define a new Proofs API that will provider the Proofs API to the RPC server
type ProofsAPI struct {
	l    log.Logger
	db   db.ProofDB
	mock bool
}

func NewProofsAPI(db db.ProofDB, l log.Logger, mock bool) (*ProofsAPI, error) {
	return &ProofsAPI{
		l:    l,
		db:   db,
		mock: mock,
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
	pa.l.Info("requesting agg proof from server", "start", startBlock, "max end", maxBlock)

	// Return the proof request ID or the mock proof request ID
	proofRequestID := "mock_proof_request_id"

	// Store an Agg proof creation entry in the DB using the start block and the max block
	created, endBlock, err := pa.db.TryCreateAggProofFromSpanProofsLimit(startBlock, maxBlock, l1BlockNumber, l1BlockHash.Hex())
	if err != nil {
		return nil, fmt.Errorf("failed to create agg proof from span proofs: %w", err)
	}

	if created {
		pa.l.Info("created new AGG proof", "from", startBlock, "to", endBlock)
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
					pa.l.Info("agg proof request is still proving", "proof_request_id", proofRequestID)
				case proofrequest.StatusCOMPLETE:
					pa.l.Info("agg proof request is complete", "proof_request_id", proofRequestID)
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
