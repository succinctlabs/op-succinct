package bootstrap

import (
	"context"
	"math"
	"math/big"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/ethereum-optimism/optimism/op-devstack/devtest"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rpc"
	opsbind "github.com/succinctlabs/op-succinct/bindings"
	opspresets "github.com/succinctlabs/op-succinct/presets"
	"github.com/succinctlabs/op-succinct/utils"
)

// TestFaultProofProposer_SystemUp verifies the proposer-only system boots correctly.
func TestFaultProofProposer_SystemUp(gt *testing.T) {
	t := devtest.ParallelT(gt)
	sys := opspresets.NewFaultProofSystem(t, opspresets.DefaultFPProposerConfig(), opspresets.DefaultL2ChainConfig())
	require := t.Require()
	logger := t.Logger()

	dgf := sys.DgfClient(t)
	logger.Info("Dispute Game Factory Address:", "address", sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr().Hex())

	gameCount, err := dgf.GameCount(t.Ctx())
	require.NoError(err, "failed to get game count from DGF")
	logger.Info("Dispute Game Count:", "count", gameCount)
	require.Equal(uint64(0), gameCount, "expected zero dispute games initially")
}

// TestFaultProofProposerAndChallenger_SystemUp verifies the proposer+challenger system boots correctly.
func TestFaultProofProposerAndChallenger_SystemUp(gt *testing.T) {
	t := devtest.ParallelT(gt)
	challengerCfg := opspresets.DefaultFPChallengerConfig()
	challengerCfg.EnvFilePath = filepath.Join(gt.TempDir(), "challenger.env")
	sys := opspresets.NewFaultProofSystem(t, opspresets.DefaultFPProposerConfig(), opspresets.DefaultL2ChainConfig(),
		opspresets.WithChallenger(challengerCfg))
	require := t.Require()
	logger := t.Logger()

	envFile, err := os.ReadFile(challengerCfg.EnvFilePath)
	require.NoError(err, "failed to read challenger environment")
	l2NodeRPC := strings.ReplaceAll(sys.L2CL.Escape().UserRPC(), "ws://", "http://")
	require.Contains(strings.Split(string(envFile), "\n"), "L2_NODE_RPC="+l2NodeRPC,
		"challenger must use this system's op-node RPC")

	dgf := sys.DgfClient(t)
	logger.Info("Dispute Game Factory Address:", "address", sys.L2Chain.Escape().Deployment().DisputeGameFactoryProxyAddr().Hex())

	gameCount, err := dgf.GameCount(t.Ctx())
	require.NoError(err, "failed to get game count from DGF")
	logger.Info("Dispute Game Count:", "count", gameCount)
	require.Equal(uint64(0), gameCount, "expected zero dispute games initially")
}

// TestFaultProofProposer_CreatesFirstGame verifies the proposer creates and proves a game.
func TestFaultProofProposer_CreatesFirstGame(gt *testing.T) {
	t := devtest.ParallelT(gt)
	cfg := opspresets.DefaultFPProposerConfig()
	sys := opspresets.NewFaultProofSystem(t, cfg, opspresets.DefaultL2ChainConfig())
	require := t.Require()
	logger := t.Logger()
	ctx, cancel := context.WithTimeout(t.Ctx(), utils.ShortTimeout())
	defer cancel()

	dgf := sys.DgfClient(t)
	utils.WaitForGameCount(ctx, t, dgf, 1)

	// Get first game
	game, err := dgf.GameAtIndex(ctx, 0)
	require.NoError(err, "failed to get first game from factory")
	logger.Info("First game created", "gameType", game.GameType, "timestamp", game.Timestamp, "proxy", game.Proxy.Hex())

	fdg, err := utils.NewFdgClient(sys.L1EL.EthClient(), game.Proxy)
	require.NoError(err, "failed to create Fault Dispute Game client")

	gameCaller, err := opsbind.NewOPSuccinctFaultDisputeGameCaller(game.Proxy, utils.NewEthCaller(sys.L1EL.EthClient()))
	require.NoError(err)
	startingAnchor, err := gameCaller.StartingOutputRoot(&bind.CallOpts{Context: ctx})
	require.NoError(err, "failed to read the first game's starting anchor")
	asrAddr, err := fdg.AnchorStateRegistryAddr(ctx)
	require.NoError(err)
	delayData, err := sys.L1EL.EthClient().Call(ctx, ethereum.CallMsg{
		To:   &asrAddr,
		Data: crypto.Keccak256([]byte("disputeGameFinalityDelaySeconds()"))[:4],
	}, rpc.LatestBlockNumber)
	require.NoError(err, "failed to read the deployed registry's finality delay")
	require.Len(delayData, 32)
	delay := new(big.Int).SetBytes(delayData)
	logger.Info("Deployed anchor configuration", "registry", asrAddr,
		"finalityDelay", delay, "startingBlock", startingAnchor.L2BlockNumber, "startingRoot", startingAnchor.Root)
	require.NoError(utils.VerifyOutputRoot(ctx, sys.L2EL.Escape().L2EthClient(),
		startingAnchor.L2BlockNumber.Uint64(), startingAnchor.Root), "initial anchor must match the local L2 state")
	require.Equal(new(big.Int).SetUint64(cfg.DisputeGameFinalityDelaySecs), delay,
		"deployed finality delay must match the test configuration")

	// Verify parent index
	parentIdx, err := fdg.ParentIndex(ctx)
	require.NoError(err, "failed to read parentIndex")
	logger.Info("Fault dispute game parent index", "parentIndex", parentIdx)
	require.Equal(uint32(math.MaxUint32), parentIdx, "unexpected parent index")

	// Verify root claim
	l2BlockNumber, err := fdg.L2BlockNumber(ctx)
	require.NoError(err, "failed to read L2 block number")
	logger.Info("Fault dispute game L2 block number", "l2BlockNumber", l2BlockNumber)

	rootClaim, err := fdg.RootClaim(ctx)
	require.NoError(err, "failed to read root claim")
	logger.Info("Fault dispute game root claim", "rootClaim", rootClaim)

	err = utils.VerifyOutputRoot(ctx, sys.L2EL.Escape().L2EthClient(), l2BlockNumber, rootClaim)
	require.NoError(err, "root claim verification failed")
}
