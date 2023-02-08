use crate::{msg::{ExecuteMsg, InstantiateMsg}, feegrant::BasicAllowance};
use cosmwasm_std::{coins, Addr};
use cw_multi_test::{App, AppBuilder, Executor};

use super::{contracts, ADMIN, DENOM};

#[test]
fn basic_allowance() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(ADMIN), coins(150_000_000, DENOM))
            .unwrap()
    });
    let code_id = app.store_code(contracts::contract());
    let limit = coins(100, DENOM);
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN),
            &InstantiateMsg {},
            &limit,
            "aloha",
            None,
        )
        .unwrap();
    app.execute_contract(
        Addr::unchecked(ADMIN),
        contract_addr,
        &ExecuteMsg::BasicAllowance(BasicAllowance {
            spend_limit: vec![],
            expiration: None,
        }),
        &limit,
    ).unwrap();
}
