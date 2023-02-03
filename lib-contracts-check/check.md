```bash
just optimize juno-local
# Wait some time so chain started
. ./lib-contracts-check/deploy_everything.sh
```

## Checking gas usage for fn versus function call
```bash
#FN
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_fn":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

#Lib
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_lib":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

#Raw Query
$BINARY tx wasm execute $CONTRACT '{"query_config_raw":{}}' --from validator $TXFLAG -y | grep gas_used

#Smart Query
$BINARY tx wasm execute $CONTRACT '{"query_config_smart":{}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary Ex
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_lib_ex":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary Ex with reply
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_lib_ex_reply":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary query and get config(same contract)
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_config_lib":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary query and get config(different contracts)
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_lib_config_lib2":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary execute and get config(same contracts)
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_config_lib_ex":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Validate boundary execute and get config(different contracts)
$BINARY tx wasm execute $CONTRACT '{"validate_boundary_lib_config_lib2_ex":{"interval": "Once"}}' --from validator $TXFLAG -y | grep gas_used

# Simple transfer
$BINARY tx wasm execute $CONTRACT '{"transfer_single_coin":{"funds": []}}' --from validator $TXFLAG -y | grep gas_used

```

## RESULTS

### 321K lib_contract.wasm (20_000 u64):
FN: "121246"
LIB: "186235"
RAW_QUERY: "126354"
SMART_QUERY: "190818"
EXECUTE_MESSAGE: "186479"
EXECUTE_MESSAGE_WITH_REPLY: "189307"
TWO_QUERY_SAME_CONTRACT: "254902"
TWO_QUERY_DIFFERENT_CONTRACTS: "256277"
TWO_EXECUTES_SAME_CONTRACT: "253999"
TWO_EXECUTES_DIFFERENT_CONTRACT: "255375"

### 165K lib_contract.wasm (0 u64):
FN: "121246"
LIB: "186235"
RAW_QUERY: "126354"
SMART_QUERY: "190818"
EXECUTE_MESSAGE: "186479"
EXECUTE_MESSAGE_WITH_REPLY: "189307"
TWO_QUERY_SAME_CONTRACT: "254902"
TWO_QUERY_DIFFERENT_CONTRACTS: "256277"
TWO_EXECUTES_SAME_CONTRACT: "253999"
TWO_EXECUTES_DIFFERENT_CONTRACT: "255375"