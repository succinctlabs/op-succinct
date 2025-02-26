#!/bin/bash
set -e

# Check if command is provided
if [ "$1" = "proposer" ]; then
    exec proposer
elif [ "$1" = "challenger" ]; then
    exec challenger
else
    echo "Invalid command. Use either 'proposer' or 'challenger'"
    exit 1
fi
