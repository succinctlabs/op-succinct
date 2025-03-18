package proposer

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strconv"
	"testing"

	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/log"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type DriverMock struct {
	mock.Mock
}

func (m *DriverMock) GetL1HeadForL2Block(ctx context.Context, rollupClient *sources.RollupClient, l2End uint64) (uint64, error) {
	args := m.Called(ctx, rollupClient, l2End)
	return args.Get(0).(uint64), args.Error(1)
}

func TestMaxBlockL1Limit(t *testing.T) {
	mockDriver := &DriverMock{}
	logger := log.New()

	// Create a mock HTTP server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" {
			var req struct {
				Method string        `json:"method"`
				Params []interface{} `json:"params"`
				ID     int           `json:"id"`
			}
			if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				fmt.Fprintln(w, "Error decoding JSON:", err)
				return
			}

			if req.Method == "optimism_outputAtBlock" {
				blockNumberHex, ok := req.Params[0].(string)
				if !ok {
					w.WriteHeader(http.StatusBadRequest)
					fmt.Fprintln(w, "Invalid block number parameter")
					return
				}

				blockNumber, err := strconv.ParseUint(blockNumberHex[2:], 16, 64)
				if err != nil {
					w.WriteHeader(http.StatusBadRequest)
					fmt.Fprintln(w, "Invalid block number format:", err)
					return
				}

				var l1Number uint64
				switch blockNumber {
				case 100:
					l1Number = 50
				case 60:
					l1Number = 100
				default:
					w.WriteHeader(http.StatusBadRequest)
					return
				}

				response := map[string]interface{}{
					"jsonrpc": "2.0",
					"id":      req.ID, // Use the same ID from the request
					"result": &eth.OutputResponse{ // Your actual result data
						Version:    eth.Bytes32{},
						OutputRoot: eth.Bytes32{},
						BlockRef: eth.L2BlockRef{
							Number: l1Number,
							Hash:   common.Hash{},
							Time:   0,
							L1Origin: eth.BlockID{
								Number: l1Number,
								Hash:   common.Hash{},
							},
						},
					},
				}

				w.WriteHeader(http.StatusOK)
				if err := json.NewEncoder(w).Encode(response); err != nil {
					w.WriteHeader(http.StatusInternalServerError)
					fmt.Fprintln(w, "Error encoding JSON:", err)
					return
				}
			} else {
				w.WriteHeader(http.StatusNotFound)
			}
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer server.Close()

	proofsAPI := &ProofsAPI{
		logger:    logger,
		driver:    mockDriver,
		rollupRPC: server.URL,
	}

	ctx := context.Background()
	l1BlockNumber := uint64(50)

	t.Run("success", func(t *testing.T) {
		maxBlock := uint64(100)
		result, err := proofsAPI.maxBlockL1Limit(ctx, maxBlock, l1BlockNumber)
		assert.NoError(t, err)
		assert.Equal(t, maxBlock, result)
	})

	t.Run("decrease maxBlock", func(t *testing.T) {
		maxBlock := uint64(60)
		result, err := proofsAPI.maxBlockL1Limit(ctx, maxBlock, l1BlockNumber)
		assert.NoError(t, err)
		assert.Equal(t, uint64(99), result)
	})

	t.Run("error getting L1 head", func(t *testing.T) {
		// Create a mock HTTP server that returns an error
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusInternalServerError)
		}))
		defer server.Close()

		proofsAPI := &ProofsAPI{
			logger:    logger,
			driver:    mockDriver,
			rollupRPC: server.URL,
		}

		result, err := proofsAPI.maxBlockL1Limit(ctx, maxBlock, l1BlockNumber)
		assert.Error(t, err)
		assert.Equal(t, uint64(0), result)
	})
}
