## Build

```
forge build
```

### Deploy on local forked network ethereum-sepolia-fork

```
# start a forked sepolia testnet
anvil -f https://rpc.ankr.com/eth_sepolia --chain-id 31337

# hack safe wallet, add deployer to signer with threshold 1
NETWORK=ethereum-sepolia-fork task InitForkedNetwork

# Dry run deploy
NETWORK=ethereum-sepolia-fork task Deploy

# Deploy
NETWORK=ethereum-sepolia-fork task Deploy -- --broadcast

```

### Deploy on local live network ethereum-sepolia
```
# Deploy
NETWORK=ethereum-sepolia task Deploy -- --broadcast
```