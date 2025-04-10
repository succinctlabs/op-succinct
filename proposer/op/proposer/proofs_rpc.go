package proposer

import (
	"context"
	"fmt"
	"sort"
	"strconv"
	"time"

	"github.com/ethereum-optimism/optimism/op-service/dial"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/log"
	gethrpc "github.com/ethereum/go-ethereum/rpc"
	"github.com/succinctlabs/op-succinct-go/proposer/db"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent/proofrequest"
)

type L2OutputSubmitterer interface {
	GetL1HeadForL2Block(ctx context.Context, rollupClient *sources.RollupClient, l2End uint64) (uint64, error)
}

// Define a new Proofs API that will provider the Proofs API to the RPC server
type ProofsAPI struct {
	logger    log.Logger
	db        db.ProofDB
	mock      bool
	rollupRPC string
}

func NewProofsAPI(db db.ProofDB, logger log.Logger, mock bool, driver *L2OutputSubmitter) (*ProofsAPI, error) {
	return &ProofsAPI{
		logger:    logger,
		db:        db,
		mock:      mock,
		rollupRPC: driver.Cfg.RollupRpc,
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
	LastProvenBlock uint64 `json:"last_proven_block"`
	EndBlock        uint64 `json:"end_block"`
	ProofRequestID  string `json:"proof_request_id"`
}

type ProofsRPCError struct {
	// The error message
	Message string `json:"message"`
	// The error code
	Code int `json:"code"`
}

func (pa *ProofsRPCError) Error() string {
	return pa.Message
}
func (pa *ProofsRPCError) ErrorCode() int {
	return pa.Code
}

func (pa *ProofsAPI) RequestAggProof(ctx context.Context, lastProvenBlock, requestedEndBlock, l1BlockNumber uint64, l1BlockHash common.Hash) (*RequestAggProofResponse, gethrpc.Error) {
	pa.logger.Info("requesting agg proof from server", "start", lastProvenBlock, "max end", requestedEndBlock)

	requestedEndBlock, err := pa.endBlockL1Limit(ctx, requestedEndBlock, l1BlockNumber)
	if err != nil {
		return nil, &ProofsRPCError{
			fmt.Errorf("failed to get max block L1 limit: %w", err).Error(),
			-1000,
		}
	}

	// Store an Agg proof creation entry in the DB using the start block and the max block
	_, endBlock, err := pa.db.TryCreateAggProofFromSpanProofsLimit(lastProvenBlock, requestedEndBlock, l1BlockNumber, l1BlockHash.Hex())
	if err != nil {
		return nil, &ProofsRPCError{
			Message: fmt.Errorf("failed to create agg proof from span proofs: %w", err).Error(),
			Code:    -2000,
		}
	}

	pa.logger.Info("created new AGG proof", "from", lastProvenBlock, "to", endBlock)

	// Poll with a ticker for the aggproof request ID creation or the proof itself if it's a mock proof
	preqs := []*ent.ProofRequest{}
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()
	// End the loop when we find a proof request with status PROVING or COMPLETE
	for {
		select {
		case <-ctx.Done():
			return nil, &ProofsRPCError{
				Message: fmt.Sprintf("context done: %s", ctx.Err()),
				Code:    -3000,
			}
		case <-ticker.C:
			preqs, err = pa.db.GetProofRequestsWithBlockRange(proofrequest.TypeAGG, lastProvenBlock, endBlock)
			if err != nil {
				return nil, &ProofsRPCError{
					Message: fmt.Sprintf("failed to get proof requests with block range: %s", err.Error()),
					Code:    -4000,
				}
			}

			// sort the proof requests by ID
			sort.Slice(preqs, func(i, j int) bool {
				return preqs[i].ID < preqs[j].ID
			})

			// Take only the last one
			var preq *ent.ProofRequest
			if len(preqs) > 0 {
				preq = preqs[len(preqs)-1]
			} else {
				continue
			}

			switch preq.Status {
			case proofrequest.StatusPROVING, proofrequest.StatusCOMPLETE:
				pa.logger.Info(fmt.Sprintf("agg proof request is %s", preq.Status), "proof_request_id", preq.ID)
			case proofrequest.StatusFAILED:
				pa.logger.Warn("agg proof request failed", "proof_request_id", preq.ID)
			default:
				pa.logger.Info("agg proof request is still pending", "proof_request_id", preq.ID)
			}

			// We want to check all but we're only interested in the last one
			if pa.mock {
				// If we're mocking, return the DB id as the proof request ID
				proverRequestID := strconv.Itoa(preq.ID)
				return &RequestAggProofResponse{
					LastProvenBlock: lastProvenBlock,
					EndBlock:        endBlock,
					ProofRequestID:  proverRequestID,
				}, nil

			} else {
				if preq.ProverRequestID != "" {
					// If we're not mocking, then we should return the proof request ID
					return &RequestAggProofResponse{
						LastProvenBlock: lastProvenBlock,
						EndBlock:        endBlock,
						ProofRequestID:  preq.ProverRequestID,
					}, nil
				}
			}
		}
	}
}

func (pa *ProofsAPI) endBlockL1Limit(ctx context.Context, requestedEndBlock, l1BlockNumber uint64) (uint64, error) {
	rollupClient, err := dial.DialRollupClientWithTimeout(ctx, dial.DefaultDialTimeout, pa.logger, pa.rollupRPC)
	if err != nil {
		return 0, err
	}

	safeHead, err := rollupClient.SafeHeadAtL1Block(ctx, l1BlockNumber-20)
	if err != nil {
		return 0, fmt.Errorf("failed to get l1 origin: %w", err)
	}

	if safeHead.SafeHead.Number < requestedEndBlock {
		return safeHead.SafeHead.Number, nil
	}

	return requestedEndBlock, nil
}

func (pa *ProofsAPI) GetAggProof(ctx context.Context, id int) ([]byte, error) {
	pa.logger.Info("getting agg proof from server", "id", id)

	// Get the proof request by the proof request ID
	preq, err := pa.db.GetProofRequestByID(id)
	if err != nil {
		return nil, fmt.Errorf("failed to get proof request by prover request ID: %w", err)
	}

	// If the proof request is not found, return an error
	if preq == nil {
		return nil, fmt.Errorf("proof request not found")
	}

	// Return the corresponding error depending on the status
	switch preq.Status {
	case proofrequest.StatusPROVING:
		pa.logger.Info("agg proof request is still proving", "proof_request_id", preq.ID)
		return nil, &ProofsRPCError{
			Message: fmt.Sprintf("agg proof request is still proving: %d", preq.ID),
			Code:    -5000,
		}
	case proofrequest.StatusFAILED:
		pa.logger.Warn("agg proof request failed", "proof_request_id", preq.ID)
		return nil, &ProofsRPCError{
			Message: fmt.Sprintf("agg proof request failed: %d", preq.ID),
			Code:    -6000,
		}
	case proofrequest.StatusCOMPLETE:
		pa.logger.Info("agg proof request is complete", "proof_request_id", preq.ID)
	default:
		pa.logger.Info("agg proof request is still pending", "proof_request_id", preq.ID)
		return nil, &ProofsRPCError{
			Message: fmt.Sprintf("agg proof request is still pending: %d", preq.ID),
			Code:    -7000,
		}
	}

	return preq.Proof, nil
}
