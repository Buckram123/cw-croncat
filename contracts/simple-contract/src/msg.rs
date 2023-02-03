use cosmwasm_std::Coin;
use cw_croncat_core::types::Boundary;
use cw_croncat_core::types::Interval;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub lib_contract_addr: String,
    pub lib_contract_addr2: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ValidateBoundaryFn {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryLib {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    QueryConfigRaw {},
    QueryConfigSmart {},

    ValidateBoundaryLibEx {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryLibExReply {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryConfigLib {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryLibConfigLib2 {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryConfigLibEx {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    ValidateBoundaryLibConfigLib2Ex {
        boundary: Option<Boundary>,
        interval: Interval,
    },

    TransferSingleCoin {
        funds: Vec<Coin>,
    },
    DoNothing {},
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
