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

			if req.Method == "optimism_safeHeadAtL1Block" {
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

				var safeHead uint64
				switch blockNumber {
				case 50 - 20:
					safeHead = 200
				case 60 - 20:
					safeHead = 100
				case 70 - 20:
					w.WriteHeader(http.StatusInternalServerError)
				default:
					w.WriteHeader(http.StatusBadRequest)
					return
				}

				response := map[string]interface{}{
					"jsonrpc": "2.0",
					"id":      req.ID, // Use the same ID from the request
					"result": &eth.SafeHeadResponse{
						SafeHead: eth.BlockID{
							Number: safeHead,
							Hash:   common.Hash{},
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

	t.Run("success", func(t *testing.T) {
		maxBlock := uint64(100)
		l1BlockNumber := uint64(50)
		result, err := proofsAPI.maxBlockL1Limit(ctx, maxBlock, l1BlockNumber)
		assert.NoError(t, err)
		assert.Equal(t, maxBlock, result)
	})

	t.Run("decrease maxBlock", func(t *testing.T) {
		maxBlock := uint64(200)
		l1BlockNumber := uint64(60)
		result, err := proofsAPI.maxBlockL1Limit(ctx, maxBlock, l1BlockNumber)
		assert.NoError(t, err)
		assert.Equal(t, uint64(100), result)
	})

	t.Run("error getting L1 head", func(t *testing.T) {
		result, err := proofsAPI.maxBlockL1Limit(ctx, 70, 70)
		assert.Error(t, err)
		assert.Equal(t, uint64(0), result)
	})
}
