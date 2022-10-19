use cosmwasm_std::{coins, BankMsg};
use cw_croncat_core::{
    msg::TaskRequest,
    types::{Action, Interval},
};

use crate::{ADDR_1, ADDR_2, ADDR_3};

pub(crate) fn three_send_actions(denom: &str) -> TaskRequest {
    TaskRequest {
        interval: Interval::Block(3),
        boundary: None,
        stop_on_fail: false,
        actions: vec![
            Action {
                msg: BankMsg::Send {
                    to_address: ADDR_1.to_string(),
                    amount: coins(1001, denom),
                }
                .into(),
                gas_limit: None,
            },
            Action {
                msg: BankMsg::Send {
                    to_address: ADDR_2.to_string(),
                    amount: coins(1002, denom),
                }
                .into(),
                gas_limit: None,
            },
            Action {
                msg: BankMsg::Send {
                    to_address: ADDR_3.to_string(),
                    amount: coins(1003, denom),
                }
                .into(),
                gas_limit: None,
            },
        ],
        rules: None,
        cw20_coins: vec![],
    }
}
