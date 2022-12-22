use crate::{
    error::CoreError,
    msg::TaskRequest,
    types::{Action, Boundary, BoundaryValidated, Interval, Task, Transform},
};
use cosmwasm_std::{
    coins, testing::mock_dependencies, Addr, BankMsg, Binary, Coin, CosmosMsg, GovMsg, IbcMsg,
    IbcTimeout, Timestamp, Uint64, VoteOption, WasmMsg,
};
use cw_rules_core::types::{CroncatQuery, HasBalanceGte};
use hex::ToHex;
use sha2::{Digest, Sha256};

#[test]
fn is_valid_msg_once_block_based() {
    let task = TaskRequest {
        interval: Interval::Once,
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: Some(Uint64::from(8u64)),
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "alice".to_string(),
                msg: Binary::from(vec![]),
                funds: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert!(task
        .is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice2"),
            &Addr::unchecked("bob"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .is_ok());
}

#[test]
fn is_valid_msg_once_time_based() {
    let task = TaskRequest {
        interval: Interval::Once,
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(1_000_000_000_u64)),
            end: Some(Uint64::from(2_000_000_000_u64)),
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "alice".to_string(),
                msg: Binary::from(vec![]),
                funds: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert!(task
        .is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice2"),
            &Addr::unchecked("bob"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .is_ok());
}

#[test]
fn is_valid_msg_recurring() {
    let task = TaskRequest {
        interval: Interval::Block(10),
        boundary: None,
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "alice".to_string(),
                msg: Binary::from(vec![]),
                funds: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert!(task
        .is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice2"),
            &Addr::unchecked("bob"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .is_ok());
}

#[test]
fn is_valid_msg_wrong_account() {
    // Cannot create a task to execute on the cron manager when not the owner
    let task = TaskRequest {
        interval: Interval::Block(5),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "alice".to_string(),
                msg: Binary::from(vec![]),
                funds: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert_eq!(
        CoreError::InvalidAction {},
        task.is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .unwrap_err()
    );
}

#[test]
fn is_valid_msg_vote() {
    // A task with CosmosMsg::Gov Vote should return false
    let task = TaskRequest {
        interval: Interval::Block(5),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Gov(GovMsg::Vote {
                proposal_id: 0,
                vote: VoteOption::Yes,
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert_eq!(
        CoreError::InvalidAction {},
        task.is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .unwrap_err()
    );
}

#[test]
fn is_valid_msg_transfer() {
    // A task with CosmosMsg::Ibc Transfer should return false
    let task = TaskRequest {
        interval: Interval::Block(5),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Ibc(IbcMsg::Transfer {
                channel_id: "id".to_string(),
                to_address: "address".to_string(),
                amount: Coin::new(10, "coin"),
                timeout: IbcTimeout::with_timestamp(Timestamp::from_nanos(1_000_000_000)),
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert_eq!(
        CoreError::InvalidAction {},
        task.is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .unwrap_err()
    );
}

#[test]
fn is_valid_msg_burn() {
    // A task with CosmosMsg::Bank Burn should return false
    let task = TaskRequest {
        interval: Interval::Block(5),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert_eq!(
        CoreError::InvalidAction {},
        task.is_valid_msg_calculate_usage(
            mock_dependencies().as_ref().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .unwrap_err()
    );
}

#[test]
fn is_valid_msg_send_doesnt_fail() {
    // A task with CosmosMsg::Bank Send should return true
    let task = TaskRequest {
        interval: Interval::Block(5),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Bank(BankMsg::Send {
                to_address: "address".to_string(),
                amount: vec![Coin::new(10, "coin")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert!(task
        .is_valid_msg_calculate_usage(
            mock_dependencies().as_ref().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .is_ok());
}

#[test]
fn is_valid_msg_send_should_success() {
    // A task with CosmosMsg::Bank Send should return false
    let task = TaskRequest {
        interval: Interval::Block(1),
        boundary: Some(Boundary::Height {
            start: Some(Uint64::from(4u64)),
            end: None,
        }),
        stop_on_fail: false,
        actions: vec![Action {
            msg: CosmosMsg::Bank(BankMsg::Send {
                to_address: "address".to_string(),
                amount: vec![Coin::new(10, "atom")],
            }),
            gas_limit: Some(5),
        }],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert!(task
        .is_valid_msg_calculate_usage(
            mock_dependencies().as_ref().api,
            &Addr::unchecked("alice"),
            &Addr::unchecked("sender"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5
        )
        .is_ok());
}

#[test]
fn is_valid_empty_actions() {
    let task = TaskRequest {
        interval: Interval::Block(10),
        boundary: None,
        stop_on_fail: false,
        actions: vec![],
        queries: None,
        transforms: None,
        cw20_coin: Default::default(),
    };
    assert_eq!(
        task.is_valid_msg_calculate_usage(
            &mock_dependencies().api,
            &Addr::unchecked("alice2"),
            &Addr::unchecked("bob"),
            &Addr::unchecked("bob"),
            5,
            5,
            5,
            5,
        )
        .unwrap_err(),
        CoreError::InvalidAction {}
    );
}

#[test]
fn hashing() {
    let task = Task {
        owner_id: Addr::unchecked("bob"),
        interval: Interval::Block(5),
        boundary: BoundaryValidated {
            start: Some(4),
            end: None,
            is_block_boundary: Some(true),
        },
        stop_on_fail: false,
        total_deposit: Default::default(),
        amount_for_one_task: Default::default(),
        actions: vec![Action {
            msg: CosmosMsg::Wasm(WasmMsg::ClearAdmin {
                contract_addr: "alice".to_string(),
            }),
            gas_limit: Some(5),
        }],
        queries: Some(vec![CroncatQuery::HasBalanceGte(HasBalanceGte {
            address: "foo".to_string(),
            required_balance: coins(5, "atom").into(),
        })]),
        transforms: Some(vec![Transform {
            action_idx: 0,
            query_idx: 0,
            action_path: vec![].into(),
            query_response_path: vec![].into(),
        }]),
        version: String::from(""),
    };

    let message = format!(
        "{:?}{:?}{:?}{:?}{:?}{:?}",
        task.owner_id, task.interval, task.boundary, task.actions, task.queries, task.transforms
    );

    let hash = Sha256::digest(message.as_bytes());

    let encoded: String = hash.encode_hex();
    let bytes = encoded.as_bytes();

    // Tests
    assert_eq!(encoded, task.to_hash());
    assert_eq!(bytes, task.to_hash_vec());
}
