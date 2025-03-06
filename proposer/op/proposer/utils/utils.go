package utils

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"math/big"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"time"

	"github.com/ethereum-optimism/optimism/op-node/cmd/batch_decoder/fetch"
	"github.com/ethereum-optimism/optimism/op-node/rollup"
	"github.com/ethereum-optimism/optimism/op-service/client"
	"github.com/ethereum-optimism/optimism/op-service/dial"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum-optimism/optimism/op-service/sources"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
)

var ErrNoSpanBatchFound = errors.New("no span batch found for the given block")
var ErrMaxDeviationExceeded = errors.New("max deviation exceeded")

// SpanBatchRange represents a range of L2 blocks covered by a span batch
type SpanBatchRange struct {
	Start uint64 `json:"start"`
	End   uint64 `json:"end"`
}

// BatchDecoderConfig is a struct that holds the configuration for the batch decoder.
type BatchDecoderConfig struct {
	L2GenesisTime     uint64
	L2GenesisBlock    uint64
	L2BlockTime       uint64
	BatchInboxAddress common.Address
	L2StartBlock      uint64
	L2EndBlock        uint64
	L2ChainID         *big.Int
	L2Node            dial.RollupClientInterface
	L1RPC             ethclient.Client
	L1Beacon          *sources.L1BeaconClient
	BatchSender       common.Address
	DataDir           string
}

// CustomBytes32 is a wrapper around eth.Bytes32 that can unmarshal from both
// full-length and minimal hex strings.
type CustomBytes32 eth.Bytes32

// Unmarshal some data into a CustomBytes32.
func (b *CustomBytes32) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	// Remove "0x" prefix if present.
	s = strings.TrimPrefix(s, "0x")

	// Pad the string to 64 characters (32 bytes) with leading zeros.
	s = fmt.Sprintf("%064s", s)

	// Add back the "0x" prefix.
	s = "0x" + s

	bytes, err := common.ParseHexOrString(s)
	if err != nil {
		return err
	}

	if len(bytes) != 32 {
		return fmt.Errorf("invalid length for Bytes32: got %d, want 32", len(bytes))
	}

	copy((*b)[:], bytes)
	return nil
}

// LoadOPStackRollupConfigFromChainID loads and parses the rollup config for the given L2 chain ID.
func LoadOPStackRollupConfigFromChainID(l2ChainId uint64) (*rollup.Config, error) {
	// Determine the path to the rollup config file.
	_, currentFile, _, _ := runtime.Caller(0)
	currentDir := filepath.Dir(currentFile)
	path := filepath.Join(currentDir, "..", "..", "..", "..", "configs", strconv.FormatUint(l2ChainId, 10), "rollup.json")

	// Read the rollup config file.
	rollupCfg, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read rollup config: %w", err)
	}

	// Parse the JSON config.
	var rawConfig map[string]interface{}
	if err := json.Unmarshal(rollupCfg, &rawConfig); err != nil {
		return nil, fmt.Errorf("failed to unmarshal rollup config: %w", err)
	}

	// Convert the Rust SuperchainConfig types to Go types, as they differ in a few places.
	convertedConfig, err := convertConfigTypes(rawConfig)
	if err != nil {
		return nil, fmt.Errorf("failed to convert config types: %w", err)
	}

	// Marshal the converted config back to JSON.
	modifiedConfig, err := json.Marshal(convertedConfig)
	if err != nil {
		return nil, fmt.Errorf("failed to re-marshal modified config: %w", err)
	}

	// Unmarshal into the actual rollup.Config struct.
	var config rollup.Config
	if err := json.Unmarshal(modifiedConfig, &config); err != nil {
		return nil, fmt.Errorf("failed to unmarshal modified rollup config: %w", err)
	}

	return &config, nil
}

// The JSON serialization of the Rust superchain-primitives types differ from the Go types (ex. U256 instead of Bytes32, U64 instead of uint64, etc.)
// This function converts the Rust types in the rollup config JSON to the Go types.
func convertConfigTypes(rawConfig map[string]interface{}) (map[string]interface{}, error) {
	// Convert genesis block numbers.
	if genesis, ok := rawConfig["genesis"].(map[string]interface{}); ok {
		convertBlockNumber(genesis, "l1")
		convertBlockNumber(genesis, "l2")
		convertSystemConfig(genesis)
	}

	// Convert base fee parameters.
	convertBaseFeeParams(rawConfig, "base_fee_params")
	convertBaseFeeParams(rawConfig, "canyon_base_fee_params")

	return rawConfig, nil
}

// convertBlockNumber converts the block number from hex string to integer.
func convertBlockNumber(data map[string]interface{}, key string) {
	if block, ok := data[key].(map[string]interface{}); ok {
		if number, ok := block["number"].(string); ok {
			if intNumber, err := strconv.ParseInt(strings.TrimPrefix(number, "0x"), 16, 64); err == nil {
				block["number"] = intNumber
			}
		}
	}
}

// convertSystemConfig converts the overhead and scalar fields in the system config.
func convertSystemConfig(genesis map[string]interface{}) {
	if systemConfig, ok := genesis["system_config"].(map[string]interface{}); ok {
		convertBytes32Field(systemConfig, "overhead")
		convertBytes32Field(systemConfig, "scalar")
	}
}

// convertBytes32Field converts a hex string to CustomBytes32 which can unmarshal from both
// full-length and minimal hex strings.
func convertBytes32Field(data map[string]interface{}, key string) {
	if value, ok := data[key].(string); ok {
		var customValue CustomBytes32
		if err := customValue.UnmarshalJSON([]byte(`"` + value + `"`)); err == nil {
			data[key] = eth.Bytes32(customValue)
		}
	}
}

// convertBaseFeeParams converts the max_change_denominator from hex string to integer.
func convertBaseFeeParams(rawConfig map[string]interface{}, key string) {
	if params, ok := rawConfig[key].(map[string]interface{}); ok {
		if maxChangeDenominator, ok := params["max_change_denominator"].(string); ok {
			if intValue, err := strconv.ParseInt(strings.TrimPrefix(maxChangeDenominator, "0x"), 16, 64); err == nil {
				params["max_change_denominator"] = intValue
			}
		}
	}
}

// / Get the L2 block number for the given L2 timestamp.
func TimestampToBlock(rollupCfg *rollup.Config, l2Timestamp uint64) uint64 {
	return ((l2Timestamp - rollupCfg.Genesis.L2Time) / rollupCfg.BlockTime) + rollupCfg.Genesis.L2.Number
}

// Set up the batch decoder config.
func setupBatchDecoderConfig(config *BatchDecoderConfig) (*rollup.Config, error) {
	rollupCfg, err := LoadOPStackRollupConfigFromChainID(config.L2ChainID.Uint64())
	if err != nil {
		return nil, err
	}

	if config.L2GenesisTime != rollupCfg.Genesis.L2Time {
		config.L2GenesisTime = rollupCfg.Genesis.L2Time
		fmt.Printf("L2GenesisTime overridden: %v\n", config.L2GenesisTime)
	}
	if config.L2GenesisBlock != rollupCfg.Genesis.L2.Number {
		config.L2GenesisBlock = rollupCfg.Genesis.L2.Number
		fmt.Printf("L2GenesisBlock overridden: %v\n", config.L2GenesisBlock)
	}
	if config.L2BlockTime != rollupCfg.BlockTime {
		config.L2BlockTime = rollupCfg.BlockTime
		fmt.Printf("L2BlockTime overridden: %v\n", config.L2BlockTime)
	}
	if config.BatchInboxAddress != rollupCfg.BatchInboxAddress {
		config.BatchInboxAddress = rollupCfg.BatchInboxAddress
		fmt.Printf("BatchInboxAddress overridden: %v\n", config.BatchInboxAddress)
	}

	return rollupCfg, nil
}

// Get the L1 boundaries corresponding to the given L2 block range. Specifically, get the L1 origin
// for the first block and an L1 block 10 minutes after the last block to ensure that the batches
// were posted to L1 for these blocks in that period. Pick blocks where it's nearly guaranteeed that
// the relevant batches were posted to L1.
func GetL1SearchBoundaries(rollupClient dial.RollupClientInterface, l1Client ethclient.Client, startBlock, endBlock uint64) (uint64, uint64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	output, err := rollupClient.OutputAtBlock(ctx, startBlock)
	if err != nil {
		return 0, 0, fmt.Errorf("failed to get output at start block: %w", err)
	}
	startL1Origin := output.BlockRef.L1Origin.Number

	// Get the diff in seconds between startL1Origin and startL1Origin -1 to get the L1 block time.
	block, err := l1Client.BlockByNumber(ctx, big.NewInt(int64(startL1Origin)))
	if err != nil {
		return 0, 0, fmt.Errorf("failed to get block at start L1 origin: %w", err)
	}
	startBlockTime := block.Time()

	// Get the L1 block time by retrieving the timestamp diff between two consecutive L1 blocks.
	block, err = l1Client.BlockByNumber(ctx, big.NewInt(int64(startL1Origin-1)))
	if err != nil {
		return 0, 0, fmt.Errorf("failed to get block at start L1 origin - 1: %w", err)
	}
	l1BlockTime := startBlockTime - block.Time()

	// Get the L1 origin for the last block.
	output, err = rollupClient.OutputAtBlock(ctx, endBlock)
	if err != nil {
		return 0, 0, fmt.Errorf("failed to get output at end block: %w", err)
	}

	// Fetch an L1 block that is at least 10 minutes after the end block to guarantee that the batches have been posted.
	endL1Origin := output.BlockRef.L1Origin.Number + (uint64(60/l1BlockTime) * 10)

	return startL1Origin, endL1Origin, nil
}

// Read all of the batches posted to the BatchInbox contract in the given L1 block range. Once the
// batches are fetched, they are written to the given data directory.
func fetchBatchesBetweenL1Blocks(config BatchDecoderConfig, rollupCfg *rollup.Config, l1Start, l1End uint64) error {
	// Clear the out directory so that loading the transaction frames is fast. Otherwise, when loading thousands of transactions,
	// this process can become quite slow.
	err := os.RemoveAll(config.DataDir)
	if err != nil {
		return fmt.Errorf("failed to clear out directory: %w", err)
	}

	fetchConfig := fetch.Config{
		Start:   l1Start,
		End:     l1End,
		ChainID: rollupCfg.L1ChainID,
		BatchSenders: map[common.Address]struct{}{
			config.BatchSender: {},
		},
		BatchInbox:         config.BatchInboxAddress,
		OutDirectory:       config.DataDir,
		ConcurrentRequests: 10,
	}

	totalValid, totalInvalid := fetch.Batches(&config.L1RPC, config.L1Beacon, fetchConfig)

	fmt.Printf("Fetched batches in range [%v,%v). Found %v valid & %v invalid batches\n", fetchConfig.Start, fetchConfig.End, totalValid, totalInvalid)

	return nil
}

// Setup the L1 Beacon client.
func SetupBeacon(l1BeaconUrl string) (*sources.L1BeaconClient, error) {
	if l1BeaconUrl == "" {
		fmt.Println("L1 Beacon endpoint not set. Unable to fetch post-ecotone channel frames")
		return nil, nil
	}

	beaconClient := sources.NewBeaconHTTPClient(client.NewBasicHTTPClient(l1BeaconUrl, nil))
	beaconCfg := sources.L1BeaconClientConfig{FetchAllSidecars: false}
	beacon := sources.NewL1BeaconClient(beaconClient, beaconCfg)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	_, err := beacon.GetVersion(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to check L1 Beacon API version: %w", err)
	}

	return beacon, nil
}
