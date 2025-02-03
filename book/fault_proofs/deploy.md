# Deploying OP Succinct Fault Dispute Game

This guide explains how to deploy the OP Succinct Fault Dispute Game contracts using the `DeployOPSuccinctDG.s.sol` script.

## Overview

The deployment script performs the following actions:
1. Deploys the `DisputeGameFactory` implementation and proxy
2. Deploys a mock SP1 verifier for testing or use a SP1 Verifier Gateway set in the environment variable `SP1_VERIFIER_GATEWAY` for production
3. Deploys the `OPSuccinctFaultDisputeGame` implementation
4. Configures the factory with initial bond and game implementation

## Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation) installed
- Access to an Ethereum node (local or network)
- Environment variables properly configured

## Configuration

Create a `.env` file in the contracts directory with the following variables:

```env
# Required Environment Variables for Game Configuration
GAME_TYPE=42                     # Unique identifier for the game type (uint32)
MAX_CHALLENGE_DURATION=604800    # Maximum duration for challenges in seconds
MAX_PROVE_DURATION=86400         # Maximum duration for proving in seconds

# SP1 Verifier Configuration (choose one)
USE_SP1_MOCK_VERIFIER=true       # Set to true to deploy a mock verifier for testing
# OR
SP1_VERIFIER_GATEWAY=0x...       # Address of the SP1 verifier gateway for production
```

## Deployment

1. Install dependencies:
   ```bash
   forge install
   ```

2. Change directory to contracts:
   ```bash
   cd contracts
   ```

3. Build the contracts:
   ```bash
   forge build
   ```

4. Run the deployment script:
   ```bash
   forge script script/fp/DeployOPSuccinctDG.s.sol --broadcast --rpc-url <RPC_URL> --private-key <PRIVATE_KEY>
   ```

## Contract Parameters

The deployment script sets up the following parameters:

- **Initial Bond**: 0.01 ETH (configurable in the script)
- **Proof Reward**: 0.01 ETH (configurable in the script)
- **Genesis Parameters**:
  - `rollupConfigHash`
  - `aggregationVkey`
  - `rangeVkeyCommitment`
  - `genesisL2BlockNumber`
  - `genesisL2OutputRoot`

## Post-Deployment

After deployment, the script will output the addresses of:
- Factory Proxy
- Game Implementation
- SP1 Verifier

Save these addresses for future reference and configuration of other components (proposer, challenger, etc.).

## Security Considerations

- The deployer address will be set as the factory owner
- Initial parameters are set for testing - adjust for production
- The mock SP1 verifier (SP1_MOCK_VERIFIER=true) should ONLY be used for testing
- For production deployments, always provide a valid SP1_VERIFIER_GATEWAY address. Contract addresses for SP1 Verifier Gateways can be found [here](https://docs.succinct.xyz/docs/verification/onchain/contract-addresses).
- Review and adjust the bond and reward values based on network economics

## Troubleshooting

Common issues and solutions:

1. **Compilation Errors**:
   - Ensure Foundry is up to date
   - Run `forge clean && forge build`

2. **Deployment Failures**:
   - Check RPC connection
   - Verify sufficient ETH balance
   - Confirm gas settings

## Next Steps

After deployment:

1. Update the proposer configuration with the factory address
2. Configure the challenger with the game parameters
3. Test the deployment with a sample game
4. Monitor initial games for correct behavior
