package presets

import (
	"context"
	"fmt"
	"math/big"

	"github.com/ethereum-optimism/optimism/op-chain-ops/devkeys"
	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum-optimism/optimism/op-devstack/presets"
	"github.com/ethereum-optimism/optimism/op-devstack/stack"
	"github.com/ethereum-optimism/optimism/op-devstack/sysgo"
	"github.com/ethereum-optimism/optimism/op-service/eth"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
	opsbind "github.com/succinctlabs/op-succinct/bindings"
	"github.com/succinctlabs/op-succinct/utils"
)

// FaultProofConfig holds configuration for fault proof proposer tests.
type FaultProofConfig struct {
	// Contract deployment configuration
	MaxChallengeDuration         uint64
	MaxProveDuration             uint64
	DisputeGameFinalityDelaySecs uint64

	// Proposer configuration
	ProposalIntervalInBlocks uint64
	FetchInterval            uint64
	RangeSplitCount          uint64
	MaxConcurrentRangeProofs uint64
	FastFinalityMode         bool
	FastFinalityProvingLimit uint64
	// Timeout is the proving timeout in seconds.
	// If nil, the default (4 hours / 14400s) is used.
	Timeout     *uint64
	EnvFilePath string

	// AggProofMode selects the SP1 verifier backend ("plonk" or "groth16").
	// Only applies for network proving (i.e. when utils.UseNetworkProver() is true).
	// If nil/empty, defaults to "plonk".
	AggProofMode *string
}

// DefaultFaultProofConfig returns the default configuration for fast tests.
func DefaultFaultProofConfig() FaultProofConfig {
	return FaultProofConfig{
		MaxChallengeDuration:         10, // Low for tests (vs 1 hour production)
		MaxProveDuration:             10, // Low for tests (vs 12 hours production)
		DisputeGameFinalityDelaySecs: 30, // Low for tests (vs 7 days production)
		ProposalIntervalInBlocks:     10,
		FetchInterval:                1,
		RangeSplitCount:              1,
		MaxConcurrentRangeProofs:     1,
		FastFinalityMode:             false,
		FastFinalityProvingLimit:     1,
	}
}

// FastFinalityFaultProofConfig returns configuration with fast finality mode enabled.
func FastFinalityFaultProofConfig() FaultProofConfig {
	cfg := DefaultFaultProofConfig()
	cfg.FastFinalityMode = true
	return cfg
}

// LongRunningFaultProofConfig returns configuration optimized for long-running progress tests.
// If NETWORK_PRIVATE_KEY is set, uses larger intervals tuned for network proving.
func LongRunningFaultProofConfig() FaultProofConfig {
	cfg := DefaultFaultProofConfig()
	cfg.MaxChallengeDuration = 1800 // =30m

	timeout := uint64(900) // =15m
	cfg.Timeout = &timeout

	if utils.UseNetworkProver() {
		cfg.ProposalIntervalInBlocks = 240
	} else {
		cfg.ProposalIntervalInBlocks = 120
	}
	return cfg
}

// LongRunningFastFinalityFaultProofConfig returns configuration for long-running fast finality tests.
// If NETWORK_PRIVATE_KEY is set, uses larger intervals tuned for network proving.
func LongRunningFastFinalityFaultProofConfig() FaultProofConfig {
	cfg := LongRunningFaultProofConfig()
	cfg.FastFinalityMode = true
	cfg.FastFinalityProvingLimit = 8
	return cfg
}

// ProposerOptions returns the proposer options for this configuration.
func (c FaultProofConfig) ProposerOptions() []sysgo.FaultProofProposerOption {
	opts := []sysgo.FaultProofProposerOption{
		sysgo.WithFPProposalIntervalInBlocks(c.ProposalIntervalInBlocks),
		sysgo.WithFPFetchInterval(c.FetchInterval),
		sysgo.WithFPRangeSplitCount(c.RangeSplitCount),
		sysgo.WithFPMaxConcurrentRangeProofs(c.MaxConcurrentRangeProofs),
		sysgo.WithFPFastFinalityMode(c.FastFinalityMode),
		sysgo.WithFPFastFinalityProvingLimit(c.FastFinalityProvingLimit),
	}
	if c.Timeout != nil {
		opts = append(opts, sysgo.WithFPTimeout(*c.Timeout))
	}
	if c.EnvFilePath != "" {
		opts = append(opts, sysgo.WithFPWriteEnvFile(c.EnvFilePath))
	}
	return opts
}

// WithSuccinctFPProposer creates a fault proof proposer with custom configuration.
func WithSuccinctFPProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs, cfg FaultProofConfig, chain L2ChainConfig) stack.CommonOption {
	// Set batcher's MaxBlocksPerSpanBatch to match the proposal interval
	maxBlocksPerSpanBatch := int(cfg.ProposalIntervalInBlocks)
	return withSuccinctPreset(dest, chain, maxBlocksPerSpanBatch, cfg.AggProofMode, func(opt *stack.CombinedOption[*sysgo.Orchestrator], ids sysgo.DefaultSingleChainInteropSystemIDs, l2ChainID eth.ChainID) {
		if !utils.UseNetworkProver() {
			opt.Add(sysgo.WithSuperDeploySP1MockVerifier(ids.L1EL, l2ChainID))
		}

		// Build FDG deployment options
		fdgOpts := []sysgo.FdgOption{
			sysgo.WithFdgL2StartingBlockNumber(1),
			sysgo.WithFdgMaxChallengeDuration(cfg.MaxChallengeDuration),
			sysgo.WithFdgMaxProveDuration(cfg.MaxProveDuration),
			sysgo.WithFdgDisputeGameFinalityDelaySecs(cfg.DisputeGameFinalityDelaySecs),
		}

		opt.Add(sysgo.WithSuperDeployOPSuccinctFaultDisputeGame(ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL, fdgOpts...))
		opt.Add(sysgo.WithSuperSuccinctFaultProofProposer(ids.L2AProposer, ids.L1CL, ids.L1EL, ids.L2ACL, ids.L2AEL, cfg.ProposerOptions()...))
	})
}

// WithDefaultSuccinctFPProposer creates a fault proof proposer with default configuration.
func WithDefaultSuccinctFPProposer(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPProposer(dest, DefaultFaultProofConfig(), DefaultL2ChainConfig())
}

// WithSuccinctFPProposerFastFinality creates a fault proof proposer optimized for fast finality.
func WithSuccinctFPProposerFastFinality(dest *sysgo.DefaultSingleChainInteropSystemIDs) stack.CommonOption {
	return WithSuccinctFPProposer(dest, FastFinalityFaultProofConfig(), DefaultL2ChainConfig())
}

// FaultProofSystem wraps MinimalWithProposer and provides access to faultproof-specific features.
type FaultProofSystem struct {
	*presets.MinimalWithProposer
	proposer sysgo.FaultProofProposer
	orch     *sysgo.Orchestrator
	ids      sysgo.DefaultSingleChainInteropSystemIDs
	cfg      FaultProofConfig
}

// DgfClient creates a DisputeGameFactory client for the faultproof system.
func (s *FaultProofSystem) DgfClient(t devtest.T) *utils.DgfClient {
	dgfAddr := s.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()
	dgf, err := utils.NewDgfClient(s.L1EL.EthClient(), dgfAddr)
	t.Require().NoError(err, "failed to create DGF client")
	return dgf
}

// StopProposer stops the faultproof proposer (for restart testing).
func (s *FaultProofSystem) StopProposer() {
	s.proposer.Stop()
}

// StartProposer starts the faultproof proposer (for restart testing).
func (s *FaultProofSystem) StartProposer() {
	s.proposer.Start()
}

// NewFaultProofSystem creates a new fault proof test system with custom configuration.
func NewFaultProofSystem(t devtest.T, cfg FaultProofConfig, chain L2ChainConfig) *FaultProofSystem {
	var ids sysgo.DefaultSingleChainInteropSystemIDs
	sys, prop, orch := newSystemWithProposer(t, WithSuccinctFPProposer(&ids, cfg, chain), &ids)

	fp, ok := prop.(sysgo.FaultProofProposer)
	t.Require().True(ok, "proposer must implement FaultProofProposer")

	return &FaultProofSystem{
		MinimalWithProposer: sys,
		proposer:            fp,
		orch:                orch,
		ids:                 ids,
		cfg:                 cfg,
	}
}

// GameImplConfig holds the configuration needed to deploy a new game implementation.
// This is extracted from the current deployed game implementation.
type GameImplConfig struct {
	FactoryProxy        common.Address
	VerifierAddress     common.Address
	AnchorStateRegistry common.Address
	AccessManager       common.Address
	RollupConfigHash    [32]byte
	AggregationVkey     [32]byte
	RangeVkeyCommitment [32]byte
	MaxChallengeDuration uint64
	MaxProveDuration     uint64
	ChallengerBondWei    *big.Int
}

// OPSuccinctGameType is the game type used for OP Succinct fault dispute games.
const OPSuccinctGameType = uint32(42)

// GetGameImplConfig queries the current game implementation to get its configuration.
// This is used to deploy a new game implementation with the same configuration but different vkeys.
func (s *FaultProofSystem) GetGameImplConfig(ctx context.Context, t devtest.T) (*GameImplConfig, error) {
	client := s.L1EL.EthClient()
	dgfAddr := s.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()

	// Create DGF binding to get the game implementation address
	dgf, err := opsbind.NewDisputeGameFactoryCaller(dgfAddr, utils.NewEthCaller(client))
	if err != nil {
		return nil, fmt.Errorf("bind DGF: %w", err)
	}

	// Get the game implementation address for our game type
	gameImplAddr, err := dgf.GameImpls(&bind.CallOpts{Context: ctx}, OPSuccinctGameType)
	if err != nil {
		return nil, fmt.Errorf("get game impl address: %w", err)
	}

	// Create game binding to query its configuration
	game, err := opsbind.NewOPSuccinctFaultDisputeGameCaller(gameImplAddr, utils.NewEthCaller(client))
	if err != nil {
		return nil, fmt.Errorf("bind game impl: %w", err)
	}

	opts := &bind.CallOpts{Context: ctx}

	// Query all configuration values
	verifier, err := game.Sp1Verifier(opts)
	if err != nil {
		return nil, fmt.Errorf("get verifier: %w", err)
	}

	asr, err := game.AnchorStateRegistry(opts)
	if err != nil {
		return nil, fmt.Errorf("get anchor state registry: %w", err)
	}

	accessManager, err := game.AccessManager(opts)
	if err != nil {
		return nil, fmt.Errorf("get access manager: %w", err)
	}

	rollupConfigHash, err := game.RollupConfigHash(opts)
	if err != nil {
		return nil, fmt.Errorf("get rollup config hash: %w", err)
	}

	aggVkey, err := game.AggregationVkey(opts)
	if err != nil {
		return nil, fmt.Errorf("get aggregation vkey: %w", err)
	}

	rangeVkey, err := game.RangeVkeyCommitment(opts)
	if err != nil {
		return nil, fmt.Errorf("get range vkey commitment: %w", err)
	}

	maxChallengeDuration, err := game.MaxChallengeDuration(opts)
	if err != nil {
		return nil, fmt.Errorf("get max challenge duration: %w", err)
	}

	maxProveDuration, err := game.MaxProveDuration(opts)
	if err != nil {
		return nil, fmt.Errorf("get max prove duration: %w", err)
	}

	challengerBond, err := game.ChallengerBond(opts)
	if err != nil {
		return nil, fmt.Errorf("get challenger bond: %w", err)
	}

	return &GameImplConfig{
		FactoryProxy:         dgfAddr,
		VerifierAddress:      verifier,
		AnchorStateRegistry:  asr,
		AccessManager:        accessManager,
		RollupConfigHash:     rollupConfigHash,
		AggregationVkey:      aggVkey,
		RangeVkeyCommitment:  rangeVkey,
		MaxChallengeDuration: maxChallengeDuration,
		MaxProveDuration:     maxProveDuration,
		ChallengerBondWei:    challengerBond,
	}, nil
}

// DeployGameImplWithFakeVkeys deploys a new game implementation with fake vkeys.
// This simulates a hardfork scenario where the on-chain vkeys don't match the proposer's computed vkeys.
// The new implementation uses the same configuration as the current one, except for the vkeys.
func (s *FaultProofSystem) DeployGameImplWithFakeVkeys(
	ctx context.Context,
	t devtest.T,
	fakeAggVkey, fakeRangeVkey string,
) (common.Address, error) {
	// Get current game configuration
	implCfg, err := s.GetGameImplConfig(ctx, t)
	if err != nil {
		return common.Address{}, fmt.Errorf("get game impl config: %w", err)
	}

	// Build deployment config with fake vkeys
	deployCfg := sysgo.GameImplDeployConfig{
		FactoryProxy:         implCfg.FactoryProxy,
		VerifierAddress:      implCfg.VerifierAddress,
		AnchorStateRegistry:  implCfg.AnchorStateRegistry,
		AccessManager:        implCfg.AccessManager,
		AggregationVkey:      fakeAggVkey,
		RangeVkeyCommitment:  fakeRangeVkey,
		RollupConfigHash:     fmt.Sprintf("0x%x", implCfg.RollupConfigHash),
		MaxChallengeDuration: implCfg.MaxChallengeDuration,
		MaxProveDuration:     implCfg.MaxProveDuration,
		ChallengerBondWei:    implCfg.ChallengerBondWei.String(),
	}

	// Deploy the new implementation
	newImplAddr, err := s.orch.DeployGameImplWithVkeys(s.ids.L1EL, deployCfg)
	if err != nil {
		return common.Address{}, fmt.Errorf("deploy game impl with vkeys: %w", err)
	}

	t.Logger().Info("Deployed new game implementation with fake vkeys",
		"address", newImplAddr,
		"fakeAggVkey", fakeAggVkey[:20]+"...",
		"fakeRangeVkey", fakeRangeVkey[:20]+"...")

	return newImplAddr, nil
}

// UpgradeGameImplementation upgrades the DisputeGameFactory to use a new game implementation.
// This simulates a hardfork where the game implementation is upgraded mid-operation.
func (s *FaultProofSystem) UpgradeGameImplementation(
	ctx context.Context,
	t devtest.T,
	newImplAddr common.Address,
) error {
	dgfAddr := s.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr()

	// Create proper ethclient from RPC URL (required for bind.ContractTransactor interface)
	l1EL, ok := s.orch.GetL1EL(s.ids.L1EL)
	if !ok {
		return fmt.Errorf("L1 EL node not found")
	}
	rpcClient, err := rpc.DialContext(ctx, l1EL.UserRPC())
	if err != nil {
		return fmt.Errorf("dial L1 RPC: %w", err)
	}
	defer rpcClient.Close()
	client := ethclient.NewClient(rpcClient)

	// Get L1 chain ID and private key for transaction signing
	l1ChainID := s.ids.L1EL.ChainID()
	l1PAOKey, err := s.orch.GetKeys().Secret(devkeys.L1ProxyAdminOwnerRole.Key(l1ChainID.ToBig()))
	if err != nil {
		return fmt.Errorf("get L1ProxyAdminOwnerRole key: %w", err)
	}

	// Create transaction options
	transactOpts, err := bind.NewKeyedTransactorWithChainID(l1PAOKey, l1ChainID.ToBig())
	if err != nil {
		return fmt.Errorf("create transact opts: %w", err)
	}
	transactOpts.Context = ctx

	// Create DGF transactor binding
	dgf, err := opsbind.NewDisputeGameFactoryTransactor(dgfAddr, client)
	if err != nil {
		return fmt.Errorf("bind DGF transactor: %w", err)
	}

	// Call setImplementation to upgrade
	tx, err := dgf.SetImplementation(transactOpts, OPSuccinctGameType, newImplAddr)
	if err != nil {
		return fmt.Errorf("set implementation: %w", err)
	}

	t.Logger().Info("Upgrading game implementation",
		"newImpl", newImplAddr,
		"txHash", tx.Hash())

	// Wait for transaction to be mined
	receipt, err := bind.WaitMined(ctx, client, tx)
	if err != nil {
		return fmt.Errorf("wait for tx: %w", err)
	}

	if receipt.Status != 1 {
		return fmt.Errorf("tx failed: status=%d", receipt.Status)
	}

	t.Logger().Info("Game implementation upgraded successfully",
		"newImpl", newImplAddr,
		"blockNumber", receipt.BlockNumber)

	return nil
}
