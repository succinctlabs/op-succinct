#!/usr/bin/env bash
set -euo pipefail

# Usage: 
# ./create_new_game_branch.sh .env.example
#
# Description:
# This script creates a new game branch to recover from CHALLENGER_WINS game chain. It fetches the 
# parent game information, calculates the target L2 block number based on the proposal interval,
# retrieves the output root for that block, and creates a new game by calling the dispute factory's
# create() function.
#
# This script is part of the Succinct Dispute Game Recovery runbook:
# https://www.notion.so/clabsco/Succinct-Dispute-Game-Recovery-Handling-Proving-Failures-2c3ea6297b89801b87a7c47d97ba8a35?source=copy_link

# Optional: load env vars from file passed as first argument
if (( $# >= 1 )) && [[ -n "${1:-}" ]]; then
  ENV_FILE="$1"
  if [[ ! -f "$ENV_FILE" ]]; then
    echo "Error: env file not found: $ENV_FILE" >&2
    exit 1
  fi
  echo "Loading env vars from $ENV_FILE"
  set -a
  # shellcheck disable=SC1090
  source "$ENV_FILE"
  set +a
fi

# Required env vars
required_vars=(
  PROPOSER_PK
  L1_RPC_URL
  L2_NODE_URL
  DISPUTE_FACTORY_ADDRESS
  CHALLENGED_GAME_INDEX
  PROPOSAL_INTERVAL
)

for var in "${required_vars[@]}"; do
  if [[ -z "${!var:-}" ]]; then
    echo "Error: $var is not set" >&2
    exit 1
  fi
done

echo "Fetching init bond..."
init_bond_hex=$(cast call "$DISPUTE_FACTORY_ADDRESS" "initBonds(uint32)" 42 -r "$L1_RPC_URL")
init_bond=$(cast --to-dec "$init_bond_hex")
echo "Init bond: $init_bond"

parent_index=$(( CHALLENGED_GAME_INDEX - 1 ))
echo "Parent index: $parent_index"

echo "Fetching parent game tuple (type,timestamp,address)..."

result=$(cast call $DISPUTE_FACTORY_ADDRESS \
    "gameAtIndex(uint256)(uint32,uint64,address)" $parent_index \
    --rpc-url $L1_RPC_URL 2>/dev/null)
cast_exit_code=$?

if [ $cast_exit_code -ne 0 ] || [ -z "$result" ]; then
    echo "Error: No games found at index $parent_index." >&2
    exit 1
fi

# Parse the result - it comes as multiple lines
parent_address=$(echo "$result" | tail -n1)
echo "Parent game proxy address: $parent_address"

echo "Fetching parent game's L2 block number..."
parent_l2_block_number_raw=$(cast call "$parent_address" "l2BlockNumber()(uint256)" -r "$L1_RPC_URL")
# Some Foundry builds may append a humanized hint like "[8.528e6]". Keep only the first token.
parent_l2_block_number_tok=$(awk '{print $1}' <<<"$parent_l2_block_number_raw")
parent_l2_block_number=$(cast --to-dec "$parent_l2_block_number_tok")
echo "Parent L2 block: $parent_l2_block_number"

if ! [[ "$PROPOSAL_INTERVAL" =~ ^[0-9]+$ ]]; then
  echo "Error: PROPOSAL_INTERVAL must be a decimal integer (got '$PROPOSAL_INTERVAL')" >&2
  exit 1
fi

l2_block_number=$(( parent_l2_block_number + PROPOSAL_INTERVAL ))
echo "Target L2 block: $l2_block_number"

echo "Fetching output root for L2 block..."
l2_block_number_hex=$(printf '0x%x' "$l2_block_number")
root_json=$(cast rpc optimism_outputAtBlock "$l2_block_number_hex" -r "$L2_NODE_URL")

# Expecting JSON with .outputRoot
if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required but not installed. Please install jq." >&2
  exit 1
fi

root_claim=$(jq -r '.outputRoot // empty' <<<"$root_json")
if [[ -z "$root_claim" || ! "$root_claim" =~ ^0x[0-9a-fA-F]{64}$ ]]; then
  echo "Error: Could not parse outputRoot from RPC response:" >&2
  echo "$root_json" >&2
  exit 1
fi
echo "Output root: $root_claim"

echo "Preparing extra data (32 bytes L2 block number || 4 bytes parent index)..."
# 32-byte big-endian L2 block number
l2_block_number_b32=$(printf '0x%064x' "$l2_block_number")
# 4-byte big-endian parent index
parent_index_b4=$(printf '%08x' "$parent_index")
extra_data="0x${l2_block_number_b32:2}${parent_index_b4}"
echo "Extra data: $extra_data"

echo "Sending create() transaction to dispute factory..."
tx_hash=$(cast send "$DISPUTE_FACTORY_ADDRESS" "create(uint32,bytes32,bytes)" 42 "$root_claim" "$extra_data" -r "$L1_RPC_URL" --private-key "$PROPOSER_PK" --value "$init_bond")
echo "Transaction sent: $tx_hash"

echo "Done."
