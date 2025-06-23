use crate::OPSuccinctL2OutputOracle::opSuccinctConfigsReturn;
use alloy_primitives::B256;
use alloy_sol_types::sol;

// Sourced from op-succinct/contracts/src/validity/OPSuccinctL2OutputOracle.sol
sol! {
    #[sol(rpc)]
    contract OPSuccinctL2OutputOracle {
        struct OpSuccinctConfig {
            bytes32 aggregationVkey;
            bytes32 rangeVkeyCommitment;
            bytes32 rollupConfigHash;
        }

        mapping(bytes32 => OpSuccinctConfig) public opSuccinctConfigs;

        uint256 public submissionInterval;

        function latestBlockNumber() public view returns (uint256);

        function updateAggregationVKey(bytes32 _aggregationVKey) external onlyOwner;

        function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) external onlyOwner;

        // Checkpointing L1 block hashes.
        function checkpointBlockHash(uint256 _blockNumber) external;

        // Proposing outputs when the output oracle is set to ZK mode.
        function proposeL2Output(bytes32 _configName, bytes32 _outputRoot, uint256 _l2BlockNumber, uint256 _l1BlockNumber, bytes memory _proof, address _proverAddress)
        external
        payable
        whenNotOptimistic;
    }
}

impl opSuccinctConfigsReturn {
    pub fn aggregation_vkey(&self) -> B256 {
        self._0
    }

    pub fn range_vkey_commitment(&self) -> B256 {
        self._1
    }

    pub fn rollup_config_hash(&self) -> B256 {
        self._2
    }
}

// Sourced from https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/dispute/DisputeGameFactory.sol.
sol! {
    /// @notice The current status of the dispute game.
    enum GameStatus {
        // The game is currently in progress, and has not been resolved.
        IN_PROGRESS,
        // The game has concluded, and the `rootClaim` was challenged successfully.
        CHALLENGER_WINS,
        // The game has concluded, and the `rootClaim` could not be contested.
        DEFENDER_WINS
    }

    /// @notice A `GameType` represents the type of game being played.
    type GameType is uint32;

    /// @notice A claim represents an MPT root representing the state of the fault proof program.
    type Claim is bytes32;

    /// @notice A dedicated timestamp type.
    type Timestamp is uint64;

    /// @notice A custom type for a generic hash.
    type Hash is bytes32;

    interface IInitializable {
        function initialize() external payable;
    }

    interface IDisputeGame is IInitializable {
        event Resolved(GameStatus indexed status);

        function createdAt() external view returns (Timestamp);
        function resolvedAt() external view returns (Timestamp);
        function status() external view returns (GameStatus);
        function gameType() external view returns (GameType gameType_);
        function gameCreator() external pure returns (address creator_);
        function rootClaim() external pure returns (Claim rootClaim_);
        function l1Head() external pure returns (Hash l1Head_);
        function l2BlockNumber() external pure returns (uint256 l2BlockNumber_);
        function extraData() external pure returns (bytes memory extraData_);
        function resolve() external returns (GameStatus status_);
        function gameData() external view returns (GameType gameType_, Claim rootClaim_, bytes memory extraData_);
        function wasRespectedGameTypeWhenCreated() external view returns (bool);
    }

    #[sol(rpc)]
    contract DisputeGameFactory {

         /// @notice Returns the required bonds for initializing a dispute game of the given type.
        mapping(GameType => uint256) public initBonds;

        /// @notice Creates a new DisputeGame proxy contract.
        /// @param _gameType The type of the DisputeGame - used to decide the proxy implementation.
        /// @param _rootClaim The root claim of the DisputeGame.
        /// @param _extraData Any extra data that should be provided to the created dispute game.
        /// @return proxy_ The address of the created DisputeGame proxy.
        function create(
            GameType _gameType,
            Claim _rootClaim,
            bytes calldata _extraData
        ) external payable returns (IDisputeGame proxy_);
    }
}

sol! {
    struct L2Output {
        uint64 zero;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}

sol! {
    #[sol(rpc)]
    contract SP1Blobstream {
        uint64 public latestBlock;
    }
}
