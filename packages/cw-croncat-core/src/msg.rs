use crate::types::{Action, Boundary, GenericBalance, Interval, Rule, Task};
use crate::types::{Agent, AgentResponse};
use cosmwasm_std::{Addr, Coin, Timestamp};
use cw20::Balance;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// NOTE: Which version is more practical?
// // Exporting a nice schema
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub enum Croncat {
//     Agent(Agent),
//     Task(Task),
//     ConfigResponse(GetConfigResponse),
//     BalancesResponse(GetBalancesResponse),
//     GetAgentIdsResponse(GetAgentIdsResponse),
//     GetAgentTasksResponse(GetAgentTasksResponse),
//     TaskRequest(TaskRequest),
//     TaskResponse(TaskResponse),
//     ValidateIntervalResponse(ValidateIntervalResponse),
//     GetAgentResponse(GetAgentResponse),
//     GetTasksResponse(GetTasksResponse),
//     GetTasksByOwnerResponse(GetTasksByOwnerResponse),
//     GetTaskResponse(GetTaskResponse),
//     GetTaskHashResponse(GetTaskHashResponse),
//     GetSlotHashesResponse(GetSlotHashesResponse),
//     GetSlotIdsResponse(GetSlotIdsResponse),
// }

// Exporting a nice schema
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Croncat {
    agent: Option<Agent>,
    task: Option<Task>,
    config_response: Option<GetConfigResponse>,
    balance_response: Option<GetBalancesResponse>,
    get_agent_ids_response: Option<GetAgentIdsResponse>,
    get_agent_tasks_response: Option<GetAgentTasksResponse>,
    task_request: Option<TaskRequest>,
    task_response: Option<TaskResponse>,
    validate_interval_response: Option<ValidateIntervalResponse>,
    get_agent_response: Option<GetAgentResponse>,
    get_tasks_response: Option<GetTasksResponse>,
    get_tasks_by_owner_response: Option<GetTasksByOwnerResponse>,
    get_task_response: Option<GetTaskResponse>,
    get_task_hash_response: Option<GetTaskHashResponse>,
    get_slot_hashes_response: Option<GetSlotHashesResponse>,
    get_slot_ids_response: Option<GetSlotIdsResponse>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // TODO: Submit issue for AppBuilder tests not working for -- deps.querier.query_bonded_denom()?;
    pub denom: String,
    pub owner_id: Option<Addr>,
    pub agent_nomination_duration: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateSettings {
        owner_id: Option<Addr>,
        slot_granularity: Option<u64>,
        paused: Option<bool>,
        agent_fee: Option<Coin>,
        gas_price: Option<u32>,
        proxy_callback_gas: Option<u32>,
        min_tasks_per_agent: Option<u64>,
        agents_eject_threshold: Option<u64>,
        // treasury_id: Option<Addr>,
    },
    MoveBalances {
        balances: Vec<Balance>,
        account_id: Addr,
    },

    RegisterAgent {
        payable_account_id: Option<Addr>,
    },
    UpdateAgent {
        payable_account_id: Addr,
    },
    CheckInAgent {},
    UnregisterAgent {},
    WithdrawReward {},

    CreateTask {
        task: TaskRequest,
    },
    RemoveTask {
        task_hash: String,
    },
    RefillTaskBalance {
        task_hash: String,
    },
    ProxyCall {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetBalances {},
    GetAgent {
        account_id: Addr,
    },
    GetAgentIds {},
    GetAgentTasks {
        account_id: Addr,
    },
    GetTasks {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    GetTasksByOwner {
        owner_id: Addr,
    },
    GetTask {
        task_hash: String,
    },
    GetTaskHash {
        task: Box<Task>,
    },
    ValidateInterval {
        interval: Interval,
    },
    GetSlotHashes {
        slot: Option<u64>,
    },
    GetSlotIds {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetConfigResponse {
    pub paused: bool,
    pub owner_id: Addr,
    // pub treasury_id: Option<Addr>,
    pub min_tasks_per_agent: u64,
    pub agent_active_index: u64,
    pub agents_eject_threshold: u64,
    pub agent_fee: Coin,
    pub gas_price: u32,
    pub proxy_callback_gas: u32,
    pub slot_granularity: u64,
    pub native_denom: String,
    pub agent_nomination_begin_time: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBalancesResponse {
    pub native_denom: String,
    pub available_balance: GenericBalance,
    pub staked_balance: GenericBalance,
    pub cw20_whitelist: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAgentIdsResponse {
    pub active: Vec<Addr>,
    pub pending: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAgentTasksResponse(pub (u64, u64));

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetSlotHashesResponse(pub (u64, Vec<String>, u64, Vec<String>));

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetSlotIdsResponse(pub (Vec<u64>, Vec<u64>));

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTasksResponse(pub Vec<TaskResponse>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAgentResponse(pub Option<AgentResponse>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTasksByOwnerResponse(pub Vec<TaskResponse>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTaskResponse(pub Option<TaskResponse>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidateIntervalResponse(pub bool);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTaskHashResponse(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaskRequest {
    pub interval: Interval,
    pub boundary: Boundary,
    pub stop_on_fail: bool,
    pub actions: Vec<Action>,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaskResponse {
    pub task_hash: String,
    pub owner_id: Addr,
    pub interval: Interval,
    pub boundary: Boundary,
    pub stop_on_fail: bool,
    pub total_deposit: Vec<Coin>,
    pub actions: Vec<Action>,
    pub rules: Option<Vec<Rule>>,
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, coins, BankMsg, CosmosMsg};
    use cw20::Cw20CoinVerified;

    use crate::types::{AgentStatus, BoundarySpec};

    use super::*;

    use super::Croncat;

    #[test]
    fn everything_can_be_de_serealized() {
        let generic_balance = GenericBalance {
            native: vec![coin(5, "test")],
            cw20: vec![Cw20CoinVerified {
                address: Addr::unchecked("juno1"),
                amount: 125u128.into(),
            }],
        };
        let agent = Agent {
            payable_account_id: Addr::unchecked("test"),
            balance: generic_balance.clone(),
            total_tasks_executed: 0,
            last_missed_slot: 3,
            register_start: Timestamp::from_nanos(5),
        }
        .into();

        let msg: CosmosMsg = BankMsg::Send {
            to_address: "you".to_string(),
            amount: coins(1015, "earth"),
        }
        .into();
        let task = Task {
            owner_id: Addr::unchecked("nobody".to_string()),
            interval: Interval::Immediate,
            boundary: Boundary {
                start: None,
                end: None,
            },
            stop_on_fail: false,
            total_deposit: vec![],
            actions: vec![Action {
                msg,
                gas_limit: Some(150_000),
            }],
            rules: None,
        }
        .into();

        let config_response = GetConfigResponse {
            paused: true,
            owner_id: Addr::unchecked("bob"),
            min_tasks_per_agent: 5,
            agent_active_index: 5,
            agents_eject_threshold: 5,
            agent_fee: coin(5, "earth"),
            gas_price: 2,
            proxy_callback_gas: 3,
            slot_granularity: 1,
            native_denom: "juno".to_string(),
            agent_nomination_begin_time: Some(Timestamp::from_nanos(5)),
        }
        .into();
        let balance_response = GetBalancesResponse {
            native_denom: "some".to_string(),
            available_balance: generic_balance.clone(),
            staked_balance: generic_balance.clone(),
            cw20_whitelist: vec![Addr::unchecked("bob")],
        }
        .into();
        let get_agent_ids_response = GetAgentIdsResponse {
            active: vec![Addr::unchecked("bob")],
            pending: vec![Addr::unchecked("bob")],
        }
        .into();
        let get_agent_tasks_response = GetAgentTasksResponse((0, 2)).into();
        let task_request = TaskRequest {
            interval: Interval::Block(5),
            boundary: Boundary {
                start: Some(BoundarySpec::Height(5)),
                end: Some(BoundarySpec::Time(Timestamp::from_nanos(64))),
            },
            stop_on_fail: true,
            actions: vec![],
            rules: None, // TODO
        }
        .into();
        let task_response_raw = TaskResponse {
            task_hash: "test".to_string(),
            owner_id: Addr::unchecked("bob"),
            interval: Interval::Cron("blah-blah".to_string()),
            boundary: Boundary {
                start: None,
                end: None,
            },
            stop_on_fail: true,
            total_deposit: vec![coin(5, "earth")],
            actions: vec![],
            rules: None,
        };
        let task_response = task_response_raw.clone().into();
        let validate_interval_response = ValidateIntervalResponse(false).into();
        let get_agent_response = GetAgentResponse(Some(AgentResponse {
            status: AgentStatus::Active,
            payable_account_id: Addr::unchecked("bob"),
            balance: generic_balance.clone(),
            total_tasks_executed: 2,
            last_missed_slot: 2,
            register_start: Timestamp::from_nanos(5),
        }))
        .into();
        let get_tasks_response = GetTasksResponse(vec![task_response_raw.clone()]).into();
        let get_tasks_by_owner_response =
            GetTasksByOwnerResponse(vec![task_response_raw.clone()]).into();
        let get_task_response = GetTaskResponse(Some(task_response_raw)).into();
        let get_task_hash_response = GetTaskHashResponse("asd".to_string()).into();
        let get_slot_hashes_response =
            GetSlotHashesResponse((5, vec!["bob".to_string()], 4, vec!["alice".to_string()]))
                .into();
        let get_slot_ids_response = GetSlotIdsResponse((vec![1], vec![3])).into();
        let croncat = Croncat {
            agent,
            task,
            config_response,
            balance_response,
            get_agent_ids_response,
            get_agent_tasks_response,
            task_request,
            task_response,
            validate_interval_response,
            get_agent_response,
            get_tasks_response,
            get_tasks_by_owner_response,
            get_task_response,
            get_task_hash_response,
            get_slot_hashes_response,
            get_slot_ids_response,
        };

        let ser = serde_json_wasm::to_string(&croncat);
        assert!(ser.is_ok());
        
        let deser: Result<Croncat, _> = serde_json_wasm::from_str(&ser.unwrap());
        assert!(deser.is_ok());
    }
}

//#[cfg_attr(not(target_arch = "wasm32"), derive(PartialEq))]