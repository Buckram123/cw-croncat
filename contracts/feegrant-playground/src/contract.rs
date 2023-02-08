#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    feegrant::{self, Any, BasicAllowance, MessageExt, MsgGrantAllowance},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:feegrant-playground";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BasicAllowance(allowance) => {
            execute_grant_basic_allowance(deps, env, info, allowance)
        }
        ExecuteMsg::PeriodicAllowance(allowance) => todo!(),
        ExecuteMsg::AllowedMsgAllowance {
            allowance,
            allowed_messages,
        } => todo!(),
    }
}

fn execute_grant_basic_allowance(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    basic_allowance: BasicAllowance,
) -> Result<Response, ContractError> {
    let allowance = Any {
        type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
        value: basic_allowance.to_bytes()?,
    };

    let msg_grant_allowance = MsgGrantAllowance {
        granter: env.contract.address.to_string(),
        grantee: info.sender.to_string(),
        allowance: Some(allowance),
    };

    Ok(Response::new().add_message(CosmosMsg::Stargate {
        type_url: "/cosmos.feegrant.v1beta1.MsgGrantAllowance".to_string(),
        value: Binary::from(msg_grant_allowance.to_bytes()?),
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
