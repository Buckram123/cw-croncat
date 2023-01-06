#!/bin/bash

SH_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
SH_DIR="$(cd -P "$(dirname "${SH_PATH}")";pwd)"
. $SH_DIR/base/init-vars.sh

if [ -z "$1" ]
then
    echo "Must provide contract address"
    exit 1
else 
    CONTRACT=$1
fi

GET_AGENT_IDS='{"get_agent_ids":{}}'
junod query wasm contract-state smart $CONTRACT "$GET_AGENT_IDS" $NODE