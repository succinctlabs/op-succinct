# Configuration

The last step is to update your OP Stack configuration to use the new `OPSuccinctL2OutputOracle` contract managed by the `op-succinct-proposer` service.

> ⚠️ **Caution**: When upgrading to the `OPSuccinctL2OutputOracle` contract, maintain the existing `finalizationPeriod` for a duration equal to at least one `finalizationPeriod`. Failure to do so will result in immediate finalization of all pending output roots upon upgrade, which is unsafe. Only after this waiting period has elapsed should you set the `finalizationPeriod` to 0.

## Self-Managed OP Stack Chains

If you are using a self-managed OP Stack chain, you will need to use your `ADMIN` key to update the existing `L2OutputOracle` implementation. Recall that the `L2OutputOracle` is a proxy contract that is upgradeable using the `ADMIN` key.

### `ADMIN` key is an EOA

To update the `L2OutputOracle` implementation, run the following command in `/contracts`. If the owner of the `L2OutputOracle` is NOT an EOA corresponding to `PRIVATE_KEY`, set `SKIP_UPGRADE_CALL` to `true`. 

```bash
forge script script/OPSuccinctUpgrader.s.sol:OPSuccinctUpgrader false \
    --rpc-url $L1_RPC \
    --private-key $PRIVATE_KEY \
    --verify \
    --verifier etherscan \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --broadcast \
    --ffi
```

### `ADMIN` key is not an EOA

```bash
```bash
forge script script/OPSuccinctUpgrader.s.sol:OPSuccinctUpgrader true \
    --rpc-url $L1_RPC \
    --private-key $PRIVATE_KEY \
    --verify \
    --verifier etherscan \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --broadcast \
    --ffi
```
```

## RaaS Providers

More information for how to configure an OP Stack RaaS provider deployment will be available soon.