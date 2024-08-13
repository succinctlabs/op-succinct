#!/bin/bash

# Start the server
RUST_LOG=info /usr/local/bin/server &

# Wait for the server to start (adjust sleep time if needed)
sleep 5

# Run op-proposer
/usr/local/bin/op-proposer \
    --poll-interval=${POLL_INTERVAL:-12s} \
    --rollup-rpc=${ROLLUP_RPC} \
    --l2oo-address=${L2OO_ADDRESS} \
    --private-key=${PRIVATE_KEY} \
    --l1-eth-rpc=${L1_ETH_RPC} \
    --beacon-rpc=${BEACON_RPC} \
    --l2-chain-id=${L2_CHAIN_ID} \
    --max-concurrent-proof-requests=${MAX_CONCURRENT_PROOF_REQUESTS}
