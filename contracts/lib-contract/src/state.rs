use cosmwasm_std::Addr;
use cw_croncat_core::types::{GasFraction, GenericBalance, SlotType};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    // Runtime
    pub paused: bool,
    pub owner_id: Addr,

    // Agent management
    // The minimum number of tasks per agent
    // Example: 10
    // Explanation: For every 1 agent, 10 tasks per slot are available.
    // NOTE: Caveat, when there are odd number of tasks or agents, the overflow will be available to first-come, first-serve. This doesn't negate the possibility of a failed txn from race case choosing winner inside a block.
    // NOTE: The overflow will be adjusted to be handled by sweeper in next implementation.
    pub min_tasks_per_agent: u64,
    pub agent_active_indices: Vec<(SlotType, u32, u32)>,
    // How many slots an agent can miss before being removed from the active queue
    pub agents_eject_threshold: u64,
    // The duration a prospective agent has to nominate themselves.
    // When a task is created such that a new agent can join,
    // The agent at the zeroth index of the pending agent queue has this time to nominate
    // The agent at the first index has twice this time to nominate (which would remove the former agent from the pending queue)
    // Value is in seconds
    pub agent_nomination_duration: u16,
    pub cw_rules_addr: Addr,

    // Economics
    pub agent_fee: u64,
    pub gas_fraction: GasFraction,
    pub gas_base_fee: u64,
    pub gas_action_fee: u64,
    pub proxy_callback_gas: u32,
    pub slot_granularity_time: u64,

    // Treasury
    // pub treasury_id: Option<Addr>,
    pub cw20_whitelist: Vec<Addr>, // TODO: Consider fee structure for whitelisted CW20s
    pub native_denom: String,
    pub available_balance: GenericBalance, // tasks + rewards balances
    pub staked_balance: GenericBalance, // surplus that is temporary staking (to be used in conjunction with external treasury)

    // The default amount of tasks to query
    pub limit: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const FILL: Item<Vec<u64>> = Item::new("FILL");
