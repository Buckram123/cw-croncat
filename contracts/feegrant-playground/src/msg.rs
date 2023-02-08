use std::time::Duration;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Timestamp};

use crate::feegrant::{BasicAllowance, PeriodicAllowance, Allowance};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    BasicAllowance(BasicAllowance),
    PeriodicAllowance(PeriodicAllowance),
    AllowedMsgAllowance {
        allowance: Option<Allowance>,
        allowed_messages: Vec<String>,
    },
}

#[cw_serde]
pub enum QueryMsg {

}