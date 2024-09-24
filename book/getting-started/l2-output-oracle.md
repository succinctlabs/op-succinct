# Deploy OP Succinct L2 Output Oracle

The first step in deploying OP Succinct is to deploy a Solidity smart contract that will verify ZKPs of OP derivation (OP's name for their state transition function) and contain the latest state root of your rollup.

## Deployment

### 1) Clone the `op-succinct` repo:

```bash
git clone https://github.com/succinctlabs/op-succinct.git
cd op-succinct
```

### 2) Set environment variables:

```bash
cp .env.example .env
```

Set the following environment variables:

```bash
L1_RPC=...
L2_RPC=...
L2_NODE_RPC=...
L2_BEACON_RPC=...
PRIVATE_KEY=...
ETHERSCAN_API_KEY=...
```

### 3) Navigate to the contracts directory:

```bash
cd contracts
```

### 4) Set Deployment Parameters

Inside the `contracts` folder there is a file called `opsuccinctl2ooconfig.json` that contains the parameters for the deployment. The parameters are automatically set based on your environment variables.

Advanced users can set parameters manually, but the defaults are recommended.

The following parameters need to be manually set:

| Parameter | Description |
|-----------|-------------|
| `proposer` | Ethereum address authorized to submit proofs. Set to `address(0)` to allow anyone to submit. |
| `challenger` | Ethereum address authorized to dispute proofs. Set to `address(0)` to disable disputes. |
| `finalizationPeriod` | The time period (in seconds) after which a proposed output becomes finalized. This is the time period after which you can withdraw your funds against the proposed output. |

All other parameters (`startingBlockNumber`, `submissionInterval`, `l2BlockTime`, `chainId`, `owner`, `vkey`, `rollupConfigHash`, `verifierGateway`, and `l2OutputOracleProxy`) are automatically set by the deployment scripts.

Note: For advanced users, there are additional flags available:
- Set `USE_CACHED_DB` to `true` for custom database configurations.

These advanced options should only be used if you fully understand their implications.

### 5) Deploy the `OPSuccinctL2OutputOracle` contract:

This foundry script will deploy the `OPSuccinctL2OutputOracle` contract to the specified L1 RPC and use the provided private key to sign the transaction:

```bash
forge script script/OPSuccinctDeployer.s.sol:OPSuccinctDeployer \
    --rpc-url $L1_RPC \
    --private-key $PRIVATE_KEY \
    --ffi \
    --verify \
    --verifier etherscan \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --broadcast
```

If successful, you should see the following output:

```
Script ran successfully.

== Return ==
0: address 0x9b520F7d8031d45Eb8A1D9fE911038576931ab95

## Setting up 1 EVM.

==========================

Chain 11155111

Estimated gas price: 11.826818849 gwei

Estimated total gas used for script: 3012823

Estimated amount required: 0.035632111845100727 ETH

==========================

##### sepolia
✅  [Success]Hash: 0xc57d97ac588563406183969e8ea15bc06496915547114b1df4e024c142df07b4
Contract Address: 0x2e4a7Dc6F19BdE1edF1040f855909afF7CcBeDeC
Block: 6633852
Paid: 0.00858210364707003 ETH (1503205 gas * 5.709203766 gwei)


##### sepolia
✅  [Success]Hash: 0x1343094b0be4e89594aedb57fb795d920e7cc1a76288485e8cf248fa206321ed
Block: 6633852
Paid: 0.001907479233443196 ETH (334106 gas * 5.709203766 gwei)


##### sepolia
✅  [Success]Hash: 0x708ce24c69c2637cadd6cffc654cbe2114e9ea4ec1e69838cd45c1fa27981713
Contract Address: 0x9b520F7d8031d45Eb8A1D9fE911038576931ab95
Block: 6633852
Paid: 0.00250654027540581 ETH (439035 gas * 5.709203766 gwei)

✅ Sequence #1 on sepolia | Total Paid: 0.012996123155919036 ETH (2276346 gas * avg 5.709203766 gwei)
                                                                                                          

==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
##
Start verification for (2) contracts
Start verifying contract `0x9b520F7d8031d45Eb8A1D9fE911038576931ab95` deployed on sepolia

Submitting verification for [lib/optimism/packages/contracts-bedrock/src/universal/Proxy.sol:Proxy] 0x9b520F7d8031d45Eb8A1D9fE911038576931ab95.

...
```

Keep note of the address of the `Proxy` contract that was deployed, which in this case is `0x9b520F7d8031d45Eb8A1D9fE911038576931ab95`. 

It is also returned by the script as `0: address 0x9b520F7d8031d45Eb8A1D9fE911038576931ab95`. 