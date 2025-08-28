#!/usr/bin/env bash
set -e

OPTIMISM_PORTAL=${OPTIMISM_PORTAL:-}
GAME_TYPE=${GAME_TYPE:-"42"}
RPC_URL=${RPC_URL:-}
PRIVATE_KEY=${PRIVATE_KEY:-}

[[ -z "$OPTIMISM_PORTAL" ]] && echo "OPTIMISM_PORTAL is not set" && exit 1
[[ -z "$RPC_URL" ]] && echo "RPC_URL is not set" && exit 1
[[ -z "$PRIVATE_KEY" ]] && echo "PRIVATE_KEY is not set" && exit 1

echo "Setting respected game type to $GAME_TYPE on OptimismPortal at $OPTIMISM_PORTAL"
cast send $OPTIMISM_PORTAL "setRespectedGameType(uint32)" $GAME_TYPE --rpc-url $RPC_URL --private-key $PRIVATE_KEY
echo "Set respected game type to $GAME_TYPE on OptimismPortal at $OPTIMISM_PORTAL"
