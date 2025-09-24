#!/usr/bin/env bash
set -e

OLD_PROPOSER=${OLD_PROPOSER:-}
OLD_CHALLENGER=${OLD_CHALLENGER:-}
NEW_PROPOSER=${NEW_PROPOSER:-}
NEW_CHALLENGER=${NEW_CHALLENGER:-}
ACCESS_MANAGER=${ACCESS_MANAGER:-}
RPC_URL=${RPC_URL:-}
PRIVATE_KEY=${PRIVATE_KEY:-}

[[ -z "$OLD_PROPOSER" ]] && echo "OLD_PROPOSER is not set" && exit 1
[[ -z "$OLD_CHALLENGER" ]] && echo "OLD_CHALLENGER is not set" && exit 1
[[ -z "$NEW_PROPOSER" ]] && echo "NEW_PROPOSER is not set" && exit 1
[[ -z "$NEW_CHALLENGER" ]] && echo "NEW_CHALLENGER is not set" && exit 1
[[ -z "$RPC_URL" ]] && echo "RPC_URL is not set" && exit 1
[[ -z "$PRIVATE_KEY" ]] && echo "PRIVATE_KEY is not set" && exit 1

echo "Setting new proposer $NEW_PROPOSER on AccessManager at $ACCESS_MANAGER"
cast send $ACCESS_MANAGER "setProposer(address,bool)" $NEW_PROPOSER true --rpc-url $RPC_URL --private-key $PRIVATE_KEY
echo "Unsetting old proposer $OLD_PROPOSER on AccessManager at $ACCESS_MANAGER"
cast send $ACCESS_MANAGER "setProposer(address,bool)" $OLD_PROPOSER false --rpc-url $RPC_URL --private-key $PRIVATE_KEY
echo "Setting new challenger $NEW_CHALLENGER on AccessManager at $ACCESS_MANAGER"
cast send $ACCESS_MANAGER "setChallenger(address,bool)" $NEW_CHALLENGER true --rpc-url $RPC_URL --private-key $PRIVATE_KEY
echo "Unsetting old challenger $OLD_CHALLENGER on AccessManager at $ACCESS_MANAGER"
cast send $ACCESS_MANAGER "setChallenger(address,bool)" $OLD_CHALLENGER false --rpc-url $RPC_URL --private-key $PRIVATE_KEY
echo "Done rotating proposer and challenger on AccessManager at $ACCESS_MANAGER"
