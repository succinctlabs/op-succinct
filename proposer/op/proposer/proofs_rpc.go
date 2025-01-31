package proposer

import (
	"context"

	gethrpc "github.com/ethereum/go-ethereum/rpc"
	"github.com/succinctlabs/op-succinct-go/proposer/db"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent"
	"github.com/succinctlabs/op-succinct-go/proposer/db/ent/proofrequest"
)

// Define a new Proofs API that will provider the Proofs API to the RPC server
type ProofsAPI struct {
	db *db.ProofDB
}

func NewProofsAPI(dbPath string, useCachedDb bool) (*ProofsAPI, error) {
	db, err := db.InitDB(dbPath, useCachedDb)
	if err != nil {
		return nil, err
	}

	return &ProofsAPI{
		db: db,
	}, nil
}

// GetProofsAPI returns the ProofsAPI struct
func GetProofsAPI(api *ProofsAPI) gethrpc.API {
	return gethrpc.API{
		Namespace: "proofs",
		Service:   api,
	}
}

func (pa *ProofsAPI) GetSpanProof(ctx context.Context, startBlock, endBlock uint64) ([]*ent.ProofRequest, error) {
	preqs, err := pa.db.GetProofRequestsWithBlockRangeAndStatus(proofrequest.TypeSPAN, startBlock, endBlock, proofrequest.StatusCOMPLETE)
	if err != nil {
		return nil, err
	}

	return preqs, nil
}
