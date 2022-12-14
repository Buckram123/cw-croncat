BINARY="docker exec -i cosmwasm junod"
set -e
just optimize
TXFLAG="--chain-id testing --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block"
docker cp 'artifacts/' cosmwasm:/artifacts

LIB_ID=$($BINARY tx wasm store "/artifacts/lib_contract.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')
CONTRACT_ID=$($BINARY tx wasm store "/artifacts/simple_contract.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')

$BINARY tx wasm instantiate $LIB_ID '{}' --from validator --label "cw_rules" $TXFLAG -y --no-admin
LIB=$($BINARY query wasm list-contract-by-code $LIB_ID --output json | jq -r '.contracts[-1]')

$BINARY tx wasm instantiate $CONTRACT_ID '{"lib_contract_addr":"'$LIB'"}' --from validator --label "cw_rules" $TXFLAG -y --no-admin
CONTRACT=$($BINARY query wasm list-contract-by-code $CONTRACT_ID --output json | jq -r '.contracts[-1]')
set +e
