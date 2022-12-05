#!/bin/bash

set -e

# Generic setup for local croncat contract
__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source ${__dir}/start.sh

# Download and deploy external contracts
wget https://github.com/CosmWasm/cw-plus/releases/latest/download/cw4_stake.wasm -O artifacts/cw4_stake.wasm
docker cp 'artifacts/cw4_stake.wasm' cosmwasm:/artifacts/cw4_stake.wasm
CW20_ID=$($BINARY tx wasm store "/artifacts/cw20_base.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')
CW4_ID=$($BINARY tx wasm store "/artifacts/cw4_stake.wasm" -y --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[-1].value')

# Recover admin1 and admin2 keys
jq -r '.[1].mnemonic' ci/test_accounts.json | $BINARY keys add admin1 --recover
jq -r '.[2].mnemonic' ci/test_accounts.json | $BINARY keys add admin2 --recover

# Check address of them
ADMIN1=$($BINARY keys show admin1 -a)
ADMIN2=$($BINARY keys show admin2 -a)

# Init CW20
$BINARY tx wasm instantiate $CW20_ID '{"name":"CronCatCoin","symbol":"cron","decimals":6,"initial_balances":[{"address":"'$($BINARY keys show user1 -a)'","amount":"1000"},{"address":"'$ADMIN1'","amount":"500"},{"address":"'$ADMIN2'","amount":"800"}]}' --from validator --label "cron_coin" $TXFLAG -y --no-admin > /dev/null
CW20_ADDR=$($BINARY query wasm list-contract-by-code $CW20_ID --output json | jq -r '.contracts[-1]')

# Init CW4
$BINARY tx wasm instantiate $CW4_ID '{"admin":"'$ADMIN1'","denom":{"cw20":"'$CW20_ADDR'"},"tokens_per_weight":"1","min_bond":"1","unbonding_period":{"height":1}}' --from validator --label "cw4_group" $TXFLAG -y --no-admin > /dev/null
CW4_ADDR=$($BINARY query wasm list-contract-by-code $CW4_ID --output json | jq -r '.contracts[-1]')

# Stake 500 for admin1 and 800 for admin2 cw20 tokens inside cw4 contract
BOND_MSG=$(echo -n '{"bond":{}}' | base64)
ADMIN1_CW4_STAKE='{"send": {"contract": "'$CW4_ADDR'", "amount": "500", "msg": "'$BOND_MSG'"}}'
$BINARY tx wasm execute $CW20_ADDR "$ADMIN1_CW4_STAKE" --from admin1 $TXFLAG -y
ADMIN2_CW4_STAKE='{"send": {"contract": "'$CW4_ADDR'", "amount": "800", "msg": "'$BOND_MSG'"}}'
$BINARY tx wasm execute $CW20_ADDR "$ADMIN2_CW4_STAKE" --from admin2 $TXFLAG -y

# Placeholder BASE64 messages
TRANSFER_CW20_B64=$(echo -n '{"transfer":{"recipient":"","amount":"1"}}' | base64)
QUERY_CW4_ADMIN_B64=$(echo -n '{"admin":{}}' | base64)
QUERY_CW4_STAKED_B64=$(echo -n '{"staked":{"address":""}}' | base64)

# Refill cw20 before task creation
CW20_SEND='{"send":{"contract":"'$CONTRACT'","amount":"1000","msg":""}}'
$BINARY tx wasm execute $CW20_ADDR "$CW20_SEND" --from user1 $TXFLAG -y

INSERTABLE_ROLL='{
  "create_task": {
    "task": {
      "interval": "Once",
      "boundary": null,
      "stop_on_fail": false,
      "actions": [
        {
          "msg": {
            "wasm": {
              "execute": {
                "contract_addr": "'$CW20_ADDR'",
                "msg": "'$TRANSFER_CW20_B64'",
                "funds": []
              }
            }
          },
          "gas_limit": 200000
        }
      ],
      "queries": [
        {
          "smart_query": {
            "contract_addr": "'$CW4_ADDR'",
            "msg": "'$QUERY_CW4_ADMIN_B64'",
            "path_to_query_value": [
              {
                "key": "admin"
              }
            ],
            "queries": [
              {
                "contract_addr": "'$CW4_ADDR'",
                "msg": "'$QUERY_CW4_STAKED_B64'",
                "path_to_msg_value": [
                  {
                    "key": "staked"
                  },
                  {
                    "key": "address"
                  }
                ],
                "path_to_query_value": [
                  {
                    "key": "stake"
                  }
                ]
              }
            ],
            "ordering": "unit_above",
            "value": "'$(echo -n '"500"' | base64)'"
          }
        },
        {
          "query": {
            "contract_addr": "'$CW4_ADDR'",
            "msg": "'$QUERY_CW4_ADMIN_B64'"
          }
        }
      ],
      "transforms": [
        {
          "action_idx": 0,
          "query_idx": 0,
          "action_path": [
            {
              "key": "transfer"
            },
            {
              "key": "amount"
            }
          ],
          "query_response_path": []
        },
        {
          "action_idx": 0,
          "query_idx": 1,
          "action_path": [
            {
              "key": "transfer"
            },
            {
              "key": "recipient"
            }
          ],
          "query_response_path": [
            {
              "key": "admin"
            }
          ]
        }
      ],
      "cw20_coins": [
        {
          "address": "'$CW20_ADDR'",
          "amount": "1000"
        }
      ]
    }
  }
}'

# echo "ROLL=$INSERTABLE_ROLL"

# Create a task
$BINARY tx wasm execute $CONTRACT "$INSERTABLE_ROLL" --amount 1000000ujunox --from user1 $TXFLAG -y > /dev/null

set +e

# echo "CRONCAT_CONTRACT_ADDRESS=$CONTRACT"
# echo "CW20_ADDR=$CW20_ADDR"
# echo "ADMIN2=$ADMIN2"
