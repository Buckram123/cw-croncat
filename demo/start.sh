#!/bin/bash

# Start local juno
just juno-local

# And download cw20 contract
just download-deps

# Recompile and optimize contracts and wait 2 seconds, so local juno have some tome to boot up
# NOTE: If any of the commands failed on your PC, try to increase sleep time 
just optimize && sleep 2

# Copy just compiled contracts inisde docker
docker cp 'artifacts/' cosmwasm:/artifacts

BINARY="docker exec -i cosmwasm junod" # junod of the docker
TXFLAG="--chain-id testing --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block" 

jq -r '.[0].mnemonic' ci/test_accounts.json | $BINARY keys add user1 --recover # Account of the user who will create tasks

# Deploy wasms
RULES_ID=$($BINARY tx wasm store "/artifacts/cw_rules.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')
CRONCAT_ID=$($BINARY tx wasm store "/artifacts/cw_croncat.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')

# Instantiate contracts and save Addresses
$BINARY tx wasm instantiate $RULES_ID '{}' --from validator --label "cw-rules" $TXFLAG -y --no-admin > /dev/null
RULES=$($BINARY query wasm list-contract-by-code $RULES_ID --output json | jq -r '.contracts[-1]')
INIT='{"denom":"ujunox","cw_rules_addr":"'$RULES'"}'
$BINARY tx wasm instantiate $CRONCAT_ID $INIT --from validator --label "croncat" $TXFLAG -y --no-admin > /dev/null
CONTRACT=$($BINARY query wasm list-contract-by-code $CRONCAT_ID --output json | jq -r '.contracts[-1]')
