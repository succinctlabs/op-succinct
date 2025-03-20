# Quick Start Guide: OP Succinct Fault Dispute Game

This guide provides the fastest path to try out OP Succinct fault dispute games by deploying contracts and running a proposer to create games.

## Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- L1 and L2 archive node RPC endpoints. L2 node should be configured with SafeDB enabled. See [SafeDB Configuration](./best_practices.md#safe-db-configuration) for more details.
- ETH on L1 for:
  - Contract deployment
  - Game bonds (configurable in factory)
  - Challenge bonds (proof rewards)
  - Transaction fees

## Step 1: Deploy Contracts

1. Clone and setup the repository:
    ```bash
    git clone https://github.com/succinctlabs/op-succinct.git
    cd op-succinct/contracts
    forge install
    ```

2. Create a `.env` file in the contracts directory:
    ```env
    # example .env file

    # Required - set these values
    GAME_TYPE=42 # The game type to use
    DISPUTE_GAME_FINALITY_DELAY_SECONDS=604800 # The time in seconds for the dispute game to be finalized
    MAX_CHALLENGE_DURATION=604800 # The maximum time in seconds for a challenge to be made
    MAX_PROVE_DURATION=86400 # The maximum time in seconds for a proof to be made after a challenge is made
    STARTING_ROOT=0xbd4ab027af3c4db0c6b1329dad6d9f8e2505accabec75bfaa2b6b8033c1e60c5 # The starting root for the game
    STARTING_L2_BLOCK_NUMBER=791000 # The starting L2 block number for the game corresponding to the starting root

    # For testing, use mock verifier
    USE_SP1_MOCK_VERIFIER=true

    # For testing, use permissionless mode
    PERMISSIONLESS_MODE=true
    ```

See [Getting the Starting Root](./deploy.md#getting-the-starting-root) for more information on how to get the starting root.

3. Deploy contracts:
    ```bash
    forge script script/fp/DeployOPSuccinctFDG.s.sol --broadcast --rpc-url <L1_RPC_URL> --private-key <PRIVATE_KEY>
    ```

Save the output addresses, particularly the `FACTORY_ADDRESS` output as "Factory Proxy: 0x...".

## Step 2: Run the Proposer

1. Create a `.env.proposer` file in the project root directory:
    ```env
    # Required Configuration
    L1_RPC=<L1_RPC_URL>
    L1_BEACON_RPC=<L1_BEACON_RPC_URL>
    L2_RPC=<L2_RPC_URL>
    L2_NODE_RPC=<L2_NODE_RPC_URL>
    FACTORY_ADDRESS=<FACTORY_ADDRESS_FROM_DEPLOYMENT>
    GAME_TYPE=42
    PRIVATE_KEY=<PRIVATE_KEY>
    NETWORK_PRIVATE_KEY=0x0000000000000000000000000000000000000000000000000000000000000001 # Dummy key for non-fast finality mode
    ```

2. Run the proposer:
    ```bash
    cargo run --bin proposer
    ```


## Step 3: Run the Challenger

1. Create a `.env.challenger` file in the project root directory:
    ```env
    # Required Configuration
    L1_RPC=<L1_RPC_URL>
    L2_RPC=<L2_RPC_URL>
    FACTORY_ADDRESS=<FACTORY_ADDRESS_FROM_DEPLOYMENT>
    GAME_TYPE=42
    PRIVATE_KEY=<PRIVATE_KEY>
    ```

2. Run the challenger:
    ```bash
    cargo run --bin challenger
    ```

## Monitoring

- Games are created every 1800 blocks by default (via `PROPOSAL_INTERVAL_IN_BLOCKS` in `.env.proposer`. See [Optional Environment Variables for Proposer](./proposer.md#optional-environment-variables)).
- Track games via block explorer using factory/game addresses and tx hashes from logs.
- Both proposer and challenger attempt to resolve eligible games after challenge period.

## Troubleshooting

Common issues:
- **Deployment fails**: Check RPC connection and ETH balance.
- **Proposer won't start**: Verify environment variables and addresses.
- **Games not creating**: Check proposer logs for errors and L1,L2 RPC endpoints.

For detailed configuration and advanced features, see:
- [Best Practices](./best_practices.md)
- [Full Contract Deployment Guide](./deploy.md)
- [Proposer Documentation](./proposer.md)
- [Challenger Documentation](./challenger.md)
