use alloy::sol;

sol! {
    type GameType is uint32;
    type Claim is bytes32;
    type Timestamp is uint64;

    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    contract DisputeGameFactory {
        mapping(GameType => IDisputeGame) public gameImpls;
        mapping(GameType => uint256) public initBonds;

        function gameCount() external view returns (uint256 gameCount_);

        function gameAtIndex(uint256 _index) external view returns (GameType gameType, Timestamp timestamp, IDisputeGame proxy);

        // extraData is a bytes array that contains the l2BlockNumber and parentIndex, and has length of 32 bytes and 4 bytes respectively
        function create(GameType gameType, Claim rootClaim, bytes extraData) external;
    }

    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IDisputeGame {}

    #[allow(missing_docs)]
    #[sol(rpc)]
    contract OPSuccinctFaultDisputeGame {
        function l2BlockNumber() public pure returns (uint256 l2BlockNumber_);
        function rootClaim() public pure returns (Claim rootClaim_);
        function status() public view returns (GameStatus status_);
        function claimData() public view returns (ClaimData memory claimData_);
        function resolve() external returns (GameStatus status_);
        function genesisL2BlockNumber() external view returns (uint256 genesisL2BlockNumber_);
    }

    #[derive(Debug, PartialEq)]
    enum GameStatus {
        IN_PROGRESS,
        DEFENDER_WINS,
        CHALLENGER_WINS
    }

    #[derive(Debug, PartialEq)]
    enum ProposalStatus {
        Unchallenged,
        Challenged,
        UnchallengedAndValidProofProvided,
        ChallengedAndValidProofProvided,
        Resolved
    }

    #[derive(Debug)]
    struct ClaimData {
        uint32 parentIndex;
        address counteredBy;
        address prover;
        Claim claim;
        ProposalStatus status;
        uint64 deadline;
    }

    struct L2Output {
        uint64 zero;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}
