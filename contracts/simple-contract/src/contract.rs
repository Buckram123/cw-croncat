#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_slice, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg, WasmMsg,
};
use cw_croncat_core::types::BoundaryValidated;
use lib_contract::state::Config;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{LIB_CONTRACT_ADDR, LIB_CONTRACT_ADDR2};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let lib_contract = deps.api.addr_validate(&msg.lib_contract_addr)?;
    LIB_CONTRACT_ADDR.save(deps.storage, &lib_contract)?;
    let lib2_contract = deps.api.addr_validate(&msg.lib_contract_addr2)?;
    LIB_CONTRACT_ADDR2.save(deps.storage, &lib2_contract)?;
    Ok(Response::new().add_attribute("lib_contract_addr", lib_contract))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ValidateBoundaryFn { boundary, interval } => {
            let res = BoundaryValidated::validate_boundary(boundary, &interval)
                .map_err(|e| StdError::generic_err(e.to_string()))?;
            Ok(Response::new().add_attribute("result", format!("{res:?}")))
        }
        ExecuteMsg::ValidateBoundaryLib { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let res: BoundaryValidated = deps.querier.query_wasm_smart(
                &lib_contract_addr,
                &lib_contract::msg::QueryMsg::ValidateBoundary { boundary, interval },
            )?;
            Ok(Response::new().add_attribute("result", format!("{res:?}")))
        }
        ExecuteMsg::QueryConfigRaw {} => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let config = deps
                .querier
                .query_wasm_raw(lib_contract_addr, b"config")?
                .unwrap();
            let config: Config = from_slice(&config)?;
            Ok(Response::new().add_attribute("result", format!("{config:?}")))
        }
        ExecuteMsg::QueryConfigSmart {} => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let config: Config = deps.querier.query_wasm_smart(
                lib_contract_addr,
                &lib_contract::msg::QueryMsg::GetConfig {},
            )?;
            Ok(Response::new().add_attribute("result", format!("{config:?}")))
        }
        ExecuteMsg::ValidateBoundaryLibEx { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let validate_exec = WasmMsg::Execute {
                contract_addr: lib_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::ValidateBoundary {
                    boundary,
                    interval,
                })?,
                funds: vec![],
            };
            Ok(Response::new().add_message(validate_exec))
        }
        ExecuteMsg::ValidateBoundaryLibExReply { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let validate_exec = WasmMsg::Execute {
                contract_addr: lib_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::ValidateBoundary {
                    boundary,
                    interval,
                })?,
                funds: vec![],
            };
            let submsg = SubMsg::reply_always(validate_exec, 0);
            Ok(Response::new().add_submessage(submsg))
        }
        ExecuteMsg::ValidateBoundaryConfigLib { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let boundary: BoundaryValidated = deps.querier.query_wasm_smart(
                &lib_contract_addr,
                &lib_contract::msg::QueryMsg::ValidateBoundary { boundary, interval },
            )?;
            let config: Config = deps.querier.query_wasm_smart(
                &lib_contract_addr,
                &lib_contract::msg::QueryMsg::GetConfig {},
            )?;
            Ok(Response::new()
                .add_attribute("boundary", format!("{boundary:?}"))
                .add_attribute("config", format!("{config:?}")))
        }
        ExecuteMsg::ValidateBoundaryLibConfigLib2 { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let boundary: BoundaryValidated = deps.querier.query_wasm_smart(
                &lib_contract_addr,
                &lib_contract::msg::QueryMsg::ValidateBoundary { boundary, interval },
            )?;
            let lib2_contract_addr = LIB_CONTRACT_ADDR2.load(deps.storage)?;
            let config: Config = deps.querier.query_wasm_smart(
                &lib2_contract_addr,
                &lib_contract::msg::QueryMsg::GetConfig {},
            )?;
            Ok(Response::new()
                .add_attribute("boundary", format!("{boundary:?}"))
                .add_attribute("config", format!("{config:?}")))
        }
        ExecuteMsg::ValidateBoundaryConfigLibEx { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let validate_exec = WasmMsg::Execute {
                contract_addr: lib_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::ValidateBoundary {
                    boundary,
                    interval,
                })?,
                funds: vec![],
            };
            let boundary_exec = WasmMsg::Execute {
                contract_addr: lib_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::GetConfig {})?,
                funds: vec![],
            };
            Ok(Response::new()
                .add_message(validate_exec)
                .add_message(boundary_exec))
        }
        ExecuteMsg::ValidateBoundaryLibConfigLib2Ex { boundary, interval } => {
            let lib_contract_addr = LIB_CONTRACT_ADDR.load(deps.storage)?;
            let validate_exec = WasmMsg::Execute {
                contract_addr: lib_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::ValidateBoundary {
                    boundary,
                    interval,
                })?,
                funds: vec![],
            };
            let lib2_contract_addr = LIB_CONTRACT_ADDR2.load(deps.storage)?;
            let boundary_exec = WasmMsg::Execute {
                contract_addr: lib2_contract_addr.to_string(),
                msg: to_binary(&lib_contract::msg::ExecuteMsg::GetConfig {})?,
                funds: vec![],
            };
            Ok(Response::new()
                .add_message(validate_exec)
                .add_message(boundary_exec))
        }
        ExecuteMsg::TransferSingleCoin { funds } => {
            let msg = BankMsg::Send {
                to_address: info.sender.into_string(),
                amount: funds,
            };
            Ok(Response::new().add_message(msg))
        }
        ExecuteMsg::DoNothing {} => {
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(Default::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let data = msg.result.unwrap().data.unwrap();
    let res: BoundaryValidated = from_slice(&data.to_vec()[2..])?; // Can someone explain to me what first two bytes mean?
    Ok(Response::new().add_attribute("result", format!("{res:?}")))
}

#[cfg(test)]
mod tests {}
