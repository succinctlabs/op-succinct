package utils

import (
	"context"
	"crypto/ecdsa"
	"fmt"
	"math/big"
	"sync/atomic"
	"time"

	"github.com/ethereum-optimism/optimism/op-service/apis"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/params"
)

// EthClient is the interface required by the load generator.
type EthClient interface {
	ChainID(ctx context.Context) (*big.Int, error)
	PendingNonceAt(ctx context.Context, account common.Address) (uint64, error)
	HeaderByNumber(ctx context.Context, number *big.Int) (*types.Header, error)
	SendTransaction(ctx context.Context, tx *types.Transaction) error
	TransactionReceipt(ctx context.Context, txHash common.Hash) (*types.Receipt, error)
}

// calcGasFees returns gasFeeCap and gasTipCap for EIP-1559 transactions.
func calcGasFees(baseFee *big.Int) (gasFeeCap, gasTipCap *big.Int) {
	gasFeeCap = new(big.Int).Mul(baseFee, big.NewInt(3))
	gasTipCap = big.NewInt(2 * params.GWei)
	if gasFeeCap.Cmp(gasTipCap) < 0 {
		gasFeeCap = new(big.Int).Add(gasTipCap, big.NewInt(params.GWei))
	}
	return gasFeeCap, gasTipCap
}

// ERC20Load generates realistic transaction load using ERC20 transfers.
// This creates diverse state access patterns similar to real mainnet blocks.
type ERC20Load struct {
	client  EthClient
	chainID *big.Int
	log     log.Logger

	// Faucet key for funding and minting
	faucetKey *ecdsa.PrivateKey

	// Generated accounts for transfers
	accounts []*ecdsa.PrivateKey

	// Contract
	tokenAddr common.Address
	deployed  atomic.Bool

	// Config
	accountCount     int // Number of accounts to generate
	blockFillPercent int // Target block fill percentage (1-100)

	// Stats
	txSent    atomic.Uint64
	blocksFed atomic.Uint64

	running atomic.Bool
	cancel  context.CancelFunc
	stopped chan struct{}
}

// ERC20LoadConfig configures the ERC20 load generator.
type ERC20LoadConfig struct {
	// AccountCount is the number of accounts to use for transfers.
	// More accounts = more state diversity. Default: 50
	AccountCount int

	// BlockFillPercent is the target block fill percentage (1-100).
	// TxPerBlock is calculated dynamically based on block gas limit.
	// Default: 100
	BlockFillPercent int
}

// DefaultERC20LoadConfig returns default configuration for OP Stack default gas limit (60M).
// With 100 accounts at 65k gas each, fills ~11% of a 60M block.
func DefaultERC20LoadConfig() ERC20LoadConfig {
	return ERC20LoadConfig{
		AccountCount:     100,
		BlockFillPercent: 100,
	}
}

// BaseMainnetERC20LoadConfig returns configuration sized for Base mainnet (375M gas limit).
// To fill 100% of a 375M block: 375M / 65k ≈ 5,769 accounts needed.
func BaseMainnetERC20LoadConfig() ERC20LoadConfig {
	return ERC20LoadConfig{
		AccountCount:     5500,
		BlockFillPercent: 100,
	}
}

// NewERC20Load creates a new ERC20 load generator.
func NewERC20Load(client EthClient, faucetKey *ecdsa.PrivateKey, cfg ERC20LoadConfig, logger log.Logger) (*ERC20Load, error) {
	if cfg.AccountCount < 2 {
		return nil, fmt.Errorf("accountCount must be >= 2, got %d", cfg.AccountCount)
	}
	if cfg.BlockFillPercent < 1 || cfg.BlockFillPercent > 100 {
		return nil, fmt.Errorf("blockFillPercent must be in [1, 100], got %d", cfg.BlockFillPercent)
	}

	chainID, err := client.ChainID(context.Background())
	if err != nil {
		return nil, fmt.Errorf("failed to get chain ID: %w", err)
	}

	// Generate deterministic accounts from faucet key
	accounts := make([]*ecdsa.PrivateKey, cfg.AccountCount)
	faucetBytes := crypto.FromECDSA(faucetKey)
	for i := 0; i < cfg.AccountCount; i++ {
		// Derive account key: keccak256(faucetKey || index)
		seed := crypto.Keccak256(append(faucetBytes, byte(i)))
		key, err := crypto.ToECDSA(seed)
		if err != nil {
			return nil, fmt.Errorf("failed to derive account %d: %w", i, err)
		}
		accounts[i] = key
	}

	return &ERC20Load{
		client:           client,
		chainID:          chainID,
		log:              logger,
		faucetKey:        faucetKey,
		accounts:         accounts,
		accountCount:     cfg.AccountCount,
		blockFillPercent: cfg.BlockFillPercent,
		stopped:          make(chan struct{}),
	}, nil
}

// Deploy deploys the TestERC20 contract, funds accounts, and mints tokens.
func (e *ERC20Load) Deploy(ctx context.Context) error {
	if e.deployed.Load() {
		return nil
	}

	faucetAddr := crypto.PubkeyToAddress(e.faucetKey.PublicKey)
	nonce, err := e.client.PendingNonceAt(ctx, faucetAddr)
	if err != nil {
		return fmt.Errorf("failed to get nonce: %w", err)
	}

	header, err := e.client.HeaderByNumber(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to get header: %w", err)
	}

	gasFeeCap, gasTipCap := calcGasFees(header.BaseFee)

	// Deploy TestERC20 contract
	// Source: contracts/src/utils/TestERC20.sol
	bytecode := common.FromHex("6080604052348015600e575f5ffd5b50610a728061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061009c575f3560e01c806340c10f191161006457806340c10f191461015a57806370a082311461017657806395d89b41146101a6578063a9059cbb146101c4578063dd62ed3e146101f45761009c565b806306fdde03146100a0578063095ea7b3146100be57806318160ddd146100ee57806323b872dd1461010c578063313ce5671461013c575b5f5ffd5b6100a8610224565b6040516100b59190610772565b60405180910390f35b6100d860048036038101906100d39190610823565b61025d565b6040516100e5919061087b565b60405180910390f35b6100f661034a565b60405161010391906108a3565b60405180910390f35b610126600480360381019061012191906108bc565b61034f565b604051610133919061087b565b60405180910390f35b610144610491565b6040516101519190610927565b60405180910390f35b610174600480360381019061016f9190610823565b610496565b005b610190600480360381019061018b9190610940565b610569565b60405161019d91906108a3565b60405180910390f35b6101ae61057e565b6040516101bb9190610772565b60405180910390f35b6101de60048036038101906101d99190610823565b6105b7565b6040516101eb919061087b565b60405180910390f35b61020e6004803603810190610209919061096b565b6105cb565b60405161021b91906108a3565b60405180910390f35b")

	signer := types.LatestSignerForChainID(e.chainID)
	deployTx := types.MustSignNewTx(e.faucetKey, signer, &types.DynamicFeeTx{
		ChainID:   e.chainID,
		Nonce:     nonce,
		To:        nil,
		Value:     big.NewInt(0),
		GasTipCap: gasTipCap,
		GasFeeCap: gasFeeCap,
		Gas:       1_000_000,
		Data:      bytecode,
	})

	if err := e.client.SendTransaction(ctx, deployTx); err != nil {
		return fmt.Errorf("failed to deploy contract: %w", err)
	}

	receipt, err := waitMined(ctx, e.client, deployTx.Hash())
	if err != nil {
		return fmt.Errorf("failed to wait for deployment: %w", err)
	}

	if receipt.Status != types.ReceiptStatusSuccessful {
		return fmt.Errorf("deployment failed with status %d", receipt.Status)
	}

	e.tokenAddr = receipt.ContractAddress
	e.log.Info("Deployed TestERC20 contract", "address", e.tokenAddr, "gasUsed", receipt.GasUsed)
	nonce++

	// Fund accounts with ETH for gas
	fundAmount := new(big.Int).Mul(big.NewInt(1), big.NewInt(params.Ether)) // 1 ETH each
	for i, acc := range e.accounts {
		addr := crypto.PubkeyToAddress(acc.PublicKey)
		tx := types.MustSignNewTx(e.faucetKey, signer, &types.DynamicFeeTx{
			ChainID:   e.chainID,
			Nonce:     nonce,
			To:        &addr,
			Value:     fundAmount,
			GasTipCap: gasTipCap,
			GasFeeCap: gasFeeCap,
			Gas:       21000,
		})
		if err := e.client.SendTransaction(ctx, tx); err != nil {
			return fmt.Errorf("failed to fund account %d: %w", i, err)
		}
		nonce++
	}
	e.log.Info("Funded accounts with ETH", "count", len(e.accounts))

	// Mint tokens to all accounts
	mintAmount := new(big.Int).Mul(big.NewInt(1_000_000), big.NewInt(params.Ether)) // 1M tokens each
	for i, acc := range e.accounts {
		addr := crypto.PubkeyToAddress(acc.PublicKey)
		data := encodeCall("40c10f19", addr, mintAmount) // mint(address,uint256)
		tx := types.MustSignNewTx(e.faucetKey, signer, &types.DynamicFeeTx{
			ChainID:   e.chainID,
			Nonce:     nonce,
			To:        &e.tokenAddr,
			Value:     big.NewInt(0),
			GasTipCap: gasTipCap,
			GasFeeCap: gasFeeCap,
			Gas:       100_000,
			Data:      data,
		})
		if err := e.client.SendTransaction(ctx, tx); err != nil {
			return fmt.Errorf("failed to mint to account %d: %w", i, err)
		}
		nonce++
	}
	e.log.Info("Minted tokens to accounts", "count", len(e.accounts), "amountEach", "1000000")

	// Wait for last mint to be mined
	time.Sleep(2 * time.Second)

	e.deployed.Store(true)
	return nil
}

// Start begins generating ERC20 transfer load.
func (e *ERC20Load) Start(ctx context.Context) error {
	if !e.deployed.Load() {
		if err := e.Deploy(ctx); err != nil {
			return err
		}
	}

	e.stopped = make(chan struct{})
	e.running.Store(true)
	ctx, e.cancel = context.WithCancel(ctx)
	go e.run(ctx)
	return nil
}

// Stop stops the load generator.
func (e *ERC20Load) Stop() {
	if !e.running.Load() {
		return
	}
	if e.cancel != nil {
		e.cancel()
	}
	<-e.stopped
}

func (e *ERC20Load) run(ctx context.Context) {
	defer close(e.stopped)
	defer e.running.Store(false)

	var lastBlock uint64
	transferIdx := 0

	// Track nonces for each account
	nonces := make([]uint64, len(e.accounts))
	for i, acc := range e.accounts {
		addr := crypto.PubkeyToAddress(acc.PublicKey)
		nonce, err := e.client.PendingNonceAt(ctx, addr)
		if err != nil {
			e.log.Error("Failed to get initial nonce", "account", i, "error", err)
			return
		}
		nonces[i] = nonce
	}

	for {
		select {
		case <-ctx.Done():
			e.log.Info("ERC20 load stopped",
				"txSent", e.txSent.Load(),
				"blocksFed", e.blocksFed.Load())
			return
		default:
		}

		header, err := e.client.HeaderByNumber(ctx, nil)
		if err != nil {
			e.log.Debug("Failed to get header", "error", err)
			time.Sleep(100 * time.Millisecond)
			continue
		}

		if header.Number.Uint64() == lastBlock {
			time.Sleep(100 * time.Millisecond)
			continue
		}
		lastBlock = header.Number.Uint64()

		gasFeeCap, gasTipCap := calcGasFees(header.BaseFee)

		// Calculate txPerBlock based on block gas limit and fill percentage
		// ERC20 transfer uses ~65,000 gas
		const gasPerTransfer = uint64(65_000)
		targetGas := header.GasLimit * uint64(e.blockFillPercent) / 100
		txPerBlock := int(targetGas / gasPerTransfer)
		if txPerBlock < 1 {
			txPerBlock = 1
		}

		// Send transfers from different accounts to create state diversity
		for i := 0; i < txPerBlock; i++ {
			// Round-robin through accounts as senders
			senderIdx := transferIdx % len(e.accounts)
			// Pick a different receiver
			receiverIdx := (transferIdx + 1) % len(e.accounts)

			sender := e.accounts[senderIdx]
			receiver := crypto.PubkeyToAddress(e.accounts[receiverIdx].PublicKey)

			// Transfer 1 token
			amount := big.NewInt(1)
			data := encodeCall("a9059cbb", receiver, amount) // transfer(address,uint256)

			signer := types.LatestSignerForChainID(e.chainID)
			tx := types.MustSignNewTx(sender, signer, &types.DynamicFeeTx{
				ChainID:   e.chainID,
				Nonce:     nonces[senderIdx],
				To:        &e.tokenAddr,
				Value:     big.NewInt(0),
				GasTipCap: gasTipCap,
				GasFeeCap: gasFeeCap,
				Gas:       65_000, // ERC20 transfer gas
				Data:      data,
			})

			if err := e.client.SendTransaction(ctx, tx); err != nil {
				e.log.Debug("Failed to send transfer", "error", err)
			} else {
				nonces[senderIdx]++
				e.txSent.Add(1)
			}

			transferIdx++
		}

		e.blocksFed.Add(1)
		if e.blocksFed.Load()%10 == 0 {
			e.log.Info("ERC20 load progress",
				"block", lastBlock,
				"txSent", e.txSent.Load(),
				"txPerBlock", txPerBlock,
				"accounts", len(e.accounts))
		}
	}
}

// encodeCall encodes a (address,uint256) function call.
func encodeCall(selector string, to common.Address, amount *big.Int) []byte {
	data := common.FromHex(selector)
	data = append(data, common.LeftPadBytes(to.Bytes(), 32)...)
	data = append(data, common.LeftPadBytes(amount.Bytes(), 32)...)
	return data
}

// waitMined polls for a transaction receipt until it's mined or context is cancelled.
func waitMined(ctx context.Context, client EthClient, txHash common.Hash) (*types.Receipt, error) {
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()
	for {
		receipt, err := client.TransactionReceipt(ctx, txHash)
		if err == nil {
			return receipt, nil
		}
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		case <-ticker.C:
		}
	}
}

// L2ClientAdapter adapts apis.L2EthClient to the EthClient interface.
// This allows the load generator to work with sysgo's L2EL nodes.
type L2ClientAdapter struct {
	inner apis.L2EthClient
}

// NewL2ClientAdapter creates a new adapter for apis.L2EthClient.
func NewL2ClientAdapter(client apis.L2EthClient) *L2ClientAdapter {
	return &L2ClientAdapter{inner: client}
}

func (a *L2ClientAdapter) ChainID(ctx context.Context) (*big.Int, error) {
	return a.inner.ChainID(ctx)
}

func (a *L2ClientAdapter) PendingNonceAt(ctx context.Context, account common.Address) (uint64, error) {
	return a.inner.PendingNonceAt(ctx, account)
}

func (a *L2ClientAdapter) HeaderByNumber(ctx context.Context, number *big.Int) (*types.Header, error) {
	// Use InfoByLabel for latest block
	var info eth.BlockInfo
	var err error
	if number == nil {
		info, err = a.inner.InfoByLabel(ctx, eth.Unsafe)
	} else {
		info, err = a.inner.InfoByNumber(ctx, number.Uint64())
	}
	if err != nil {
		return nil, err
	}
	// Convert BlockInfo to Header (fields used by loadgen)
	return &types.Header{
		Number:   new(big.Int).SetUint64(info.NumberU64()),
		BaseFee:  info.BaseFee(),
		GasLimit: info.GasLimit(),
	}, nil
}

func (a *L2ClientAdapter) SendTransaction(ctx context.Context, tx *types.Transaction) error {
	return a.inner.SendTransaction(ctx, tx)
}

func (a *L2ClientAdapter) TransactionReceipt(ctx context.Context, txHash common.Hash) (*types.Receipt, error) {
	return a.inner.TransactionReceipt(ctx, txHash)
}
