#!/usr/bin/env bash
set -euo pipefail

# Require env vars
[ -z "${NETWORK:-}" ] && echo "Need to set the VERSION via env" && exit 1;

# Check network
case $NETWORK in
  "mainnet")
    echo "Detected network: $NETWORK"
    BLOCKSCOUT_URL=https://eth.blockscout.com/api/
    CHAIN_ID=1
    ;;
  "sepolia")
    echo "Detected network: $NETWORK"
    BLOCKSCOUT_URL=https://eth-sepolia.blockscout.com/api/
    CHAIN_ID=11155111
    ;;
  "holesky")
    echo "Detected network: $NETWORK"
    BLOCKSCOUT_URL=https://eth-holesky.blockscout.com/api/
    CHAIN_ID=17000
    ;;
  *)
    echo "Unsupported network: $NETWORK" && exit 1
    ;;
esac

verify() {
    CONSTRUCTOR_SIG=${3:-}
    if [ "${BLOCKSCOUT_API_KEY:-}" ]; then
        echo ">>> [Blockscout] $2"
        if [ -z ${CONSTRUCTOR_SIG:-} ]; then
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --etherscan-api-key=$BLOCKSCOUT_API_KEY \
                --verifier-url=$BLOCKSCOUT_URL \
                --watch
        else
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --etherscan-api-key=$BLOCKSCOUT_API_KEY \
                --verifier-url=$BLOCKSCOUT_URL \
                --constructor-args $(cast abi-encode $CONSTRUCTOR_SIG ${@:4}) \
                --watch
        fi
    fi
    if [ "${ETHERSCAN_API_KEY:-}" ]; then
        echo ">>> [Etherscan] $2"
        if [ -z ${CONSTRUCTOR_SIG:-} ]; then
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --etherscan-api-key=$ETHERSCAN_API_KEY \
                --watch
        else
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --etherscan-api-key=$ETHERSCAN_API_KEY \
                --constructor-args $(cast abi-encode $CONSTRUCTOR_SIG ${@:4}) \
                --watch
        fi
    fi
    if [ "${TENDERLY_URL:-}" ] && [ "${TENDERLY_API_KEY:-}" ]; then
        echo ">>> [Tenderly] $2"
        if [ -z ${CONSTRUCTOR_SIG:-} ]; then
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --verifier-url=$TENDERLY_URL \
                --etherscan-api-key=$TENDERLY_API_KEY \
                --watch
        else
            forge verify-contract $1 $2 \
                --chain-id $CHAIN_ID \
                --verifier-url=$TENDERLY_URL \
                --etherscan-api-key=$TENDERLY_API_KEY \
                --constructor-args $(cast abi-encode $CONSTRUCTOR_SIG ${@:4}) \
                --watch
        fi
    fi
}

verify $AM AccessManager "constructor(uint256,address)" $FALLBACK_TIMEOUT_FP_SECS $DISPUTE_GAME_FACTORY_ADDRESS
verify $OSG OPSuccinctFaultDisputeGame "constructor(uint64,uint64,address,address,bytes32,bytes32,bytes32,uint256,address,address)" \
    $MAX_CHALLENGE_DURATION \
    $MAX_PROVE_DURATION \
    $DISPUTE_GAME_FACTORY_ADDRESS \
    $VERIFIER_ADDRESS \
    $RCH \
    $AVK \
    $RVC \
    $CHALLENGER_BOND_WEI \
    $ANCHOR_STATE_REGISTRY_ADDRESS \
    $AM
