#!/bin/bash

# Replace the values in the .env file
l1_rpc="$(kurtosis port print op el-1-geth-lighthouse rpc)"
l1_beacon_rpc="$(kurtosis port print op cl-1-lighthouse-geth http)"
l2_rpc="$(kurtosis port print op op-el-1-op-geth-op-node-op-kurtosis rpc)"
l2_node_rpc="$(kurtosis port print op op-cl-1-op-node-op-geth-op-kurtosis http)"

sed -i "s|^L1_RPC=\".*\"$|L1_RPC=\"http://$l1_rpc\"|" .env
sed -i "s|^L1_BEACON_RPC=\".*\"$|L1_BEACON_RPC=\"$l1_beacon_rpc\"|" .env
sed -i "s|^L2_RPC=\".*\"$|L2_RPC=\"$l2_rpc\"|" .env
sed -i "s|^L2_NODE_RPC=\".*\"$|L2_NODE_RPC=\"$l2_node_rpc\"|" .env

verifier_address=$(just deploy-mock-verifier)

# Extract the contract address from the output using grep and regex
address=$(echo "$verifier_address" | grep -oP "0: address \K0x[a-fA-F0-9]{40}")

# Check if the address was found
if [[ -n "$address" ]]; then
    # Check if VERIFIER_ADDRESS already exists in the .env file
    if grep -q "VERIFIER_ADDRESS=" .env; then
        # If it exists, update it with the new address
        sed -i "s|^VERIFIER_ADDRESS=\".*\"$|VERIFIER_ADDRESS=\"$address\"|" .env
    else
        # If it doesn't exist, append it to the .env file
        echo "VERIFIER_ADDRESS=$address" >> .env
    fi
    echo "VERIFIER_ADDRESS has been updated to $address."
else
    echo "Contract address not found in the output."
fi

l2oo_address=$(just deploy-oracle)

# Extract the contract address from the output using grep and regex
address=$(echo "$l2oo_address" | grep -oP "0: address \K0x[a-fA-F0-9]{40}")

# Check if the address was found
if [[ -n "$address" ]]; then
    # Check if L2OO_ADDRESS already exists in the .env file
    if grep -q "L2OO_ADDRESS=" .env; then
        # If it exists, update it with the new address
        sed -i "s|^L2OO_ADDRESS=\".*\"$|L2OO_ADDRESS=\"$address\"|" .env
    else
        # If it doesn't exist, append it to the .env file
        echo "L2OO_ADDRESS=$address" >> .env
    fi
    echo "L2OO_ADDRESS has been updated to $address."
else
    echo "Contract address not found in the output."
fi

# Replace the RPCs to work with docker compose
L1_RPC="el-1-geth-lighthouse:8545"
L1_BEACON_RPC="cl-1-lighthouse-geth:4000"
L2_RPC="op-el-1-op-geth-op-node-op-kurtosis:8545"
L2_NODE_RPC="op-cl-1-op-node-op-geth-op-kurtosis:8547"

sed -i "s|^L1_RPC=\".*\"$|L1_RPC=\"http://$L1_RPC\"|" .env
sed -i "s|^L1_BEACON_RPC=\".*\"$|L1_BEACON_RPC=\"http://$L1_BEACON_RPC\"|" .env
sed -i "s|^L2_RPC=\".*\"$|L2_RPC=\"http://$L2_RPC\"|" .env
sed -i "s|^L2_NODE_RPC=\".*\"$|L2_NODE_RPC=\"http://$L2_NODE_RPC\"|" .env