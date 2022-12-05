# Simple demo of stacking rules with insertable message

## Task

### Queries: 
0. Check who is the admin of the cw4 contract
    - If current admin has more than "500" staked cw20 coins - this `rule` is ready (This query will return how much cw20 staked coins current admin has, for example: `"800"`)
1. Query current admin of cw4 contract (This will return admin to addr struct, for example: `{"admin": "addr"}`)

- Actions
0. Transfer "1" cw20 coin to the addr (`{"transfer":{"recipient":"","amount":"1"}}`)
- Transforms
0. Transform `action[0].transfer.amount` to the result of `query[0]`
1. Transform `action[0].transfer.recipent` to the result of `query[1].admin`


## Run the script
```bash
. ./demo/stacking_query_insertable_message.sh
```
NOTE: Using .(or equivalently "source") so variables of the script is saved

## What happens inside script
- Deploy and instantiate `cw4_stake` and `cw20_base` contracts from the latest `cw-plus` [release](https://github.com/CosmWasm/cw-plus/releases/latest).
- Initial cw20 balances:

| ADDR   | Balance |
|--------|---------|
| USER   | "1000"  |
| ADMIN1 | "500"   |
| ADMIN2 | "800"   |

- Admin of cw4 is "ADMIN1"
- "ADMIN1" and "ADMIN2" stake their cw20 coins inside cw4 contract
  
| ADDR   | Staked |
|--------|--------|
| ADMIN1 | "500"  |
| ADMIN2 | "800"  |

- [Task](#task) is created by "USER"

[TODO]: <> (REMOVE AFTER FIELDS UPDATED ON croncat-rs)
## Register AGENT
jq -r '.[3].mnemonic' ci/test_accounts.json | $BINARY keys add agent1 --recover
$BINARY tx wasm execute $CONTRACT '{"register_agent": {}}' --from agent1 -y $TXFLAG

## Check current cw20 balance of ADMIN2
```bash
$BINARY query wasm contract-state smart $CW20_ADDR '{"balance": {"address": "'$ADMIN2'"}}' -o json
```

## Update admin from "ADMIN1" to "ADMIN2"
```bash
$BINARY tx wasm execute $CW4_ADDR '{"update_admin": {"admin": "'$ADMIN2'"}}' --from admin1 -y $TXFLAG
```

## Get task hash
```bash
TASK_HASH=$($BINARY query wasm contract-state smart $CONTRACT '{"get_tasks_with_rules": {}}' -o json | jq -r '.data[0].task_hash')
```

[TODO]: <> (need to update croncat-rs to match new fields)
## Execute proxy_call
```bash
$BINARY tx wasm execute $CONTRACT '{"proxy_call": {"task_hash": "'$TASK_HASH'"}}' --from agent1 -y $TXFLAG
```

## Check updated cw20 balance of ADMIN2
```bash
$BINARY query wasm contract-state smart $CW20_ADDR '{"balance": {"address": "'$ADMIN2'"}}' -o json
```