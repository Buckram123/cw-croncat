use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;
use croncat_sdk_core::types::{BalancesResponse, Config, GasPrice, UpdateConfig};
use cw20::{Cw20Coin, Cw20CoinVerified};

#[cw_serde]
pub struct InstantiateMsg {
    /// The native denominator of current chain
    pub denom: String,
    /// Address of the `cw_rules_addr` contract
    // TODO: this will be actually multiple mod-contracts
    pub cw_rules_addr: String,
    /// Address of the Tasks contract
    pub croncat_tasks_addr: String,
    /// Address of the Agents contract
    pub croncat_agents_addr: String,
    /// Address of the contract owner, defaults to the sender
    pub owner_id: Option<String>,
    /// Gas needed to cover [ExecuteMsg::ProxyCall] without any action
    pub gas_base_fee: Option<Uint64>,
    /// Gas needed to cover single non-wasm task's Action
    pub gas_action_fee: Option<Uint64>,
    /// Gas needed to cover single non-wasm query
    pub gas_query_fee: Option<Uint64>,
    /// Gas needed to cover single wasm query
    pub gas_wasm_query_fee: Option<Uint64>,
    /// Gas prices that expected to be used by the agent
    pub gas_price: Option<GasPrice>,
    /// The duration a prospective agent has to nominate themselves.
    /// When a task is created such that a new agent can join,
    /// The agent at the zeroth index of the pending agent queue has this time to nominate
    /// The agent at the first index has twice this time to nominate (which would remove the former agent from the pending queue)
    /// Value is in seconds
    pub agent_nomination_duration: Option<u16>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Updates the croncat Config.
    /// Note: it's shared across contracts
    UpdateConfig(UpdateConfig),
    // TODO:
    // MoveBalances {
    //     balances: Vec<Balance>,
    //     account_id: String,
    // },
    /// Execute current task in the queue or task with queries if task_hash given
    ProxyCall { task_hash: Option<String> },
    /// Receive cw20 coin
    Receive(cw20::Cw20ReceiveMsg),
    /// Withdraw Cw20 coins
    WithdrawCw20WalletBalances { cw20_amounts: Vec<Cw20Coin> },
    /// Kick inactive agents
    Tick {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Gets current croncat config
    #[returns(Config)]
    Config {},
    /// Gets manager available balances
    #[returns(BalancesResponse)]
    AvailableBalances {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    /// Gets Cw20 balances of the given wallet address
    #[returns(Vec<Cw20CoinVerified>)]
    Cw20WalletBalances {
        wallet: String,
        from_index: Option<u64>,
        limit: Option<u64>,
    },
}

#[cw_serde]
pub enum ReceiveMsg {
    RefillCw20Balance {},
}