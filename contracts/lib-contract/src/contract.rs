use crate::state::{Config, FILL};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw_croncat_core::types::{BoundaryValidated, GasFraction};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CONFIG;

// Trying to increase wasm size to check how it affects gas cost
const FILLER: [u64; 0] = [450; 0];

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        paused: false,
        owner_id: info.sender.clone(),
        min_tasks_per_agent: 5,
        agent_active_indices: Default::default(),
        agents_eject_threshold: 5,
        agent_nomination_duration: 10_000,
        cw_rules_addr: info.sender,
        agent_fee: 5,
        gas_fraction: GasFraction {
            numerator: 1,
            denominator: 10,
        },
        gas_base_fee: 100,
        gas_action_fee: 1000,
        proxy_callback_gas: 15,
        slot_granularity_time: 150,
        cw20_whitelist: vec![],
        native_denom: "denom".to_owned(),
        available_balance: Default::default(),
        staked_balance: Default::default(),
        limit: 20,
    };

    CONFIG.save(deps.storage, &config)?;
    FILL.save(deps.storage, &Vec::from(FILLER))?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ValidateBoundary { boundary, interval } => {
            let validated = BoundaryValidated::validate_boundary(boundary, &interval)
                .map_err(|e| StdError::generic_err(e.to_string()))?;
            let validated_bin = to_binary(&validated)?;
            Ok(Response::new().set_data(validated_bin))
        }
        ExecuteMsg::GetConfig {} => {
            let config = CONFIG.load(deps.storage)?;
            let config_bin = to_binary(&config)?;
            Ok(Response::new().set_data(config_bin))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::ValidateBoundary { boundary, interval } => to_binary(
            &BoundaryValidated::validate_boundary(boundary, &interval)
                .map_err(|e| StdError::generic_err(e.to_string()))?,
        ),
    }
}

#[cfg(test)]
mod tests {}
