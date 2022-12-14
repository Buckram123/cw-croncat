```bash
just juno-local
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
```

## RESULTS

### 315K lib_contract.wasm (20_000 u64):
FN: "121246"
LIB: "186235"
RAW_QUERY: "126354"
SMART_QUERY: "190818"
EXECUTE_MESSAGE: "186479"
EXECUTE_MESSAGE_WITH_REPLY: "189307"

### 163K lib_contract.wasm (0 u64):
FN: "121246"
LIB: "186235"
RAW_QUERY: "126354"
SMART_QUERY: "190818"
EXECUTE_MESSAGE: "186479"
EXECUTE_MESSAGE_WITH_REPLY: "189307"