set fallback := true
set dotenv-load

default:
  @just --list

# Runs the kona-host program for a single block.
run-single l2_block_num use-cache="false":
  #!/usr/bin/env bash
  CACHE_FLAG=""
  if [ "{{use-cache}}" = "true" ]; then
    CACHE_FLAG="--use-cache"
  fi
  cargo run --bin single --release -- --l2-block {{l2_block_num}} $CACHE_FLAG

# Runs the kona-host program for multiple blocks.
run-multi start end use-cache="false" prove="false":
  #!/usr/bin/env bash
  CACHE_FLAG=""
  if [ "{{use-cache}}" = "true" ]; then
    CACHE_FLAG="--use-cache"
  fi
  PROVE_FLAG=""
  if [ "{{prove}}" = "true" ]; then
    PROVE_FLAG="--prove"
  fi

  cargo run --bin multi --release -- --start {{start}} --end {{end}} $CACHE_FLAG $PROVE_FLAG

# Runs the client program in native execution mode. Modified version of Kona Native Client execution:
# https://github.com/ethereum-optimism/kona/blob/ae71b9df103c941c06b0dc5400223c4f13fe5717/bin/client/justfile#L65-L108
run-client-native l2_block_num l1_rpc='${L1_RPC}' l1_beacon_rpc='${L1_BEACON_RPC}' l2_rpc='${L2_RPC}' verbosity="-vvvv":
  #!/usr/bin/env bash
  L1_NODE_ADDRESS="{{l1_rpc}}"
  L1_BEACON_ADDRESS="{{l1_beacon_rpc}}"
  L2_NODE_ADDRESS="{{l2_rpc}}"
  echo "L1 Node Address: $L1_NODE_ADDRESS"
  echo "L1 Beacon Address: $L1_BEACON_ADDRESS"
  echo "L2 Node Address: $L2_NODE_ADDRESS"
  HOST_BIN_PATH="./kona-host"
  CLIENT_BIN_PATH="./target/release-client-lto/zkvm-client"
  L2_BLOCK_NUMBER="{{l2_block_num}}"
  L2_BLOCK_SAFE_HEAD=$((L2_BLOCK_NUMBER - 1))
  L2_OUTPUT_STATE_ROOT=$(cast block --rpc-url $L2_NODE_ADDRESS --field stateRoot $L2_BLOCK_SAFE_HEAD)
  L2_HEAD=$(cast block --rpc-url $L2_NODE_ADDRESS --field hash $L2_BLOCK_SAFE_HEAD)
  L2_OUTPUT_STORAGE_HASH=$(cast proof --rpc-url $L2_NODE_ADDRESS --block $L2_BLOCK_SAFE_HEAD 0x4200000000000000000000000000000000000016 | jq -r '.storageHash')
  L2_OUTPUT_ENCODED=$(cast abi-encode "x(uint256,bytes32,bytes32,bytes32)" 0 $L2_OUTPUT_STATE_ROOT $L2_OUTPUT_STORAGE_HASH $L2_HEAD)
  L2_OUTPUT_ROOT=$(cast keccak $L2_OUTPUT_ENCODED)
  echo "L2 Safe Head: $L2_BLOCK_SAFE_HEAD"
  echo "Safe Head Output Root: $L2_OUTPUT_ROOT"
  L2_CLAIM_STATE_ROOT=$(cast block --rpc-url $L2_NODE_ADDRESS --field stateRoot $L2_BLOCK_NUMBER)
  L2_CLAIM_HASH=$(cast block --rpc-url $L2_NODE_ADDRESS --field hash $L2_BLOCK_NUMBER)
  L2_CLAIM_STORAGE_HASH=$(cast proof --rpc-url $L2_NODE_ADDRESS --block $L2_BLOCK_NUMBER 0x4200000000000000000000000000000000000016 | jq -r '.storageHash')
  L2_CLAIM_ENCODED=$(cast abi-encode "x(uint256,bytes32,bytes32,bytes32)" 0 $L2_CLAIM_STATE_ROOT $L2_CLAIM_STORAGE_HASH $L2_CLAIM_HASH)
  L2_CLAIM=$(cast keccak $L2_CLAIM_ENCODED)
  echo "L2 Block Number: $L2_BLOCK_NUMBER"
  echo "L2 Claim Root: $L2_CLAIM"
  L2_BLOCK_TIMESTAMP=$(cast block --rpc-url $L2_NODE_ADDRESS $L2_BLOCK_NUMBER -j | jq -r .timestamp)
  L1_HEAD=$(cast block --rpc-url $L1_NODE_ADDRESS $(cast find-block --rpc-url $L1_NODE_ADDRESS $(($(cast 2d $L2_BLOCK_TIMESTAMP) + 300))) -j | jq -r .hash)
  echo "L1 Head: $L1_HEAD"
  L2_CHAIN_ID=10
  DATA_DIRECTORY="./data/$L2_BLOCK_NUMBER"
  echo "Saving Data to $DATA_DIRECTORY"
  echo "Building client program..."
  cargo build --bin zkvm-client --profile release-client-lto
  echo "Running host program with native client program..."
  cargo run --bin native-host --release -- \
    --l1-head $L1_HEAD \
    --l2-head $L2_HEAD \
    --l2-claim $L2_CLAIM \
    --l2-output-root $L2_OUTPUT_ROOT \
    --l2-block-number $L2_BLOCK_NUMBER \
    --l2-chain-id $L2_CHAIN_ID \
    --l1-node-address $L1_NODE_ADDRESS \
    --l1-beacon-address $L1_BEACON_ADDRESS \
    --l2-node-address $L2_NODE_ADDRESS \
    --exec $CLIENT_BIN_PATH \
    --data-dir $DATA_DIRECTORY \
    {{verbosity}}

  # Output the data required for the ZKVM execution.
  echo "$L1_HEAD $L2_OUTPUT_ROOT $L2_CLAIM $L2_BLOCK_NUMBER $L2_CHAIN_ID"

fetch-rollup-config rollup_rpc="" l2_rpc="":
    #!/usr/bin/env sh
    set -e

    # Determine RPC URLs
    if [ -z "{{rollup_rpc}}" ]; then
        if [ -z "$ROLLUP_RPC" ]; then
            echo "Error: Must provide rollup rpc as argument or env variable (ROLLUP_RPC)" >&2
            exit 1
        else
            ROLLUP_RPC="$ROLLUP_RPC"
        fi
    else
        ROLLUP_RPC="{{rollup_rpc}}"
    fi

    if [ -z "{{l2_rpc}}" ]; then
        if [ -z "$L2_RPC" ]; then
            echo "Error: Must provide L2 rpc as argument or env variable (L2_RPC)" >&2
            exit 1
        else
            L2_RPC="$L2_RPC"
        fi
    else
        L2_RPC="{{l2_rpc}}"
    fi

    # Fetch rollup config
    cast rpc --rpc-url "$ROLLUP_RPC" optimism_rollupConfig > ./rollup-config.json

    # Fetch chain config
    cast rpc --rpc-url "$L2_RPC" debug_chainConfig > ./chain-config.json

    # Process and merge configs
    jq -s -e '
    .[0] as $rollup |
    .[1] as $chain |
    if ($chain.optimism.eip1559Elasticity == null or $chain.optimism.eip1559Denominator == null) then
      error("Error: Missing required values for base_fee_params in chain config")
    else
      $rollup *
      (
        {
          "regolithTime": $chain.regolithTime,
          "canyonTime": $chain.canyonTime,
          "deltaTime": $chain.deltaTime,
          "ecotoneTime": $chain.ecotoneTime,
          "fjordTime": $chain.fjordTime,
          "graniteTime": $chain.graniteTime,
          "holoceneTime": $chain.holoceneTime
        } | with_entries(select(.value != null))
      ) *
      {
        "base_fee_params": {
          "elasticity_multiplier": $chain.optimism.eip1559Elasticity,
          "max_change_denominator": $chain.optimism.eip1559Denominator
        }
      } *
      (
        if ($chain.optimism.eip1559DenominatorCanyon != null) then
          {
            "canyon_base_fee_params": {
              "elasticity_multiplier": $chain.optimism.eip1559Elasticity,
              "max_change_denominator": $chain.optimism.eip1559DenominatorCanyon
            }
          }
        else
          {}
        end
      ) |
      .genesis.system_config.batcherAddress = .genesis.system_config.batcherAddr |
      del(.genesis.system_config.batcherAddr)
    end
    ' ./rollup-config.json ./chain-config.json > ./temp.json

    # Check if jq command was successful
    if [ $? -ne 0 ]; then
        echo "Error: Failed to process JSON files. Make sure all required fields are present in the chain config." >&2
        rm ./temp.json ./chain-config.json ./rollup-config.json
        exit 1
    fi

    # If successful, move the temp file to the final location
    mv ./temp.json ./rollup-config.json

    # Clean up
    rm ./chain-config.json
