use crate::state::{Config, QueueItem};

use crate::ContractError::AgentNotRegistered;
use crate::{ContractError, CwCroncat};
use cosmwasm_std::{
    coin, to_binary, Addr, Api, BankMsg, Coin, CosmosMsg, Env, StdError, StdResult, Storage,
    SubMsg, Uint128, WasmMsg,
};
use cw20::{Cw20CoinVerified, Cw20ExecuteMsg};
use cw_croncat_core::msg::ExecuteMsg;
pub use cw_croncat_core::types::Task;
use cw_croncat_core::types::{gas_amount_with_agent_fee, AgentStatus};
//use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::ops::Div;
//use std::str::FromStr;
pub(crate) fn vect_difference<T: std::clone::Clone + std::cmp::PartialEq>(
    v1: &[T],
    v2: &[T],
) -> Vec<T> {
    v1.iter().filter(|&x| !v2.contains(x)).cloned().collect()
}

// pub(crate) fn from_raw_str(value: &str) -> Option<Coin> {
//     let re = Regex::new(r"^([0-9.]+)([a-z][a-z0-9]*)$").unwrap();
//     assert!(re.is_match(value));
//     let caps = re.captures(value)?;
//     let amount = caps.get(1).map_or("", |m| m.as_str());
//     let denom = caps.get(2).map_or("", |m| m.as_str());
//     if denom.len() < 3 || denom.len() > 128{
//         return Option::None;
//     }
//     Some(Coin::new(u128::from_str(amount).unwrap(), denom))
// }

impl<'a> CwCroncat<'a> {
    pub fn get_agent_status(
        &self,
        storage: &dyn Storage,
        env: Env,
        account_id: Addr,
    ) -> Result<AgentStatus, ContractError> {
        let c: Config = self.config.load(storage)?;
        let active = self.agent_active_queue.load(storage)?;

        // Pending
        let mut pending_iter = self.agent_pending_queue.iter(storage)?;
        // If agent is pending, Check if they should get nominated to checkin to become active
        let agent_position = if let Some(pos) = pending_iter.position(|address| {
            if let Ok(addr) = address {
                addr == account_id
            } else {
                false
            }
        }) {
            pos
        } else {
            // Check for active
            if active.contains(&account_id) {
                return Ok(AgentStatus::Active);
            } else {
                return Err(AgentNotRegistered {});
            }
        };

        // Edge case if last agent unregistered
        if active.is_empty() && agent_position == 0 {
            return Ok(AgentStatus::Nominated);
        };

        // Load config's task ratio, total tasks, active agents, and agent_nomination_begin_time.
        // Then determine if this agent is considered "Nominated" and should call CheckInAgent
        let max_agent_index =
            self.max_agent_nomination_index(storage, &c, env, &(active.len() as u64))?;
        let agent_status = match max_agent_index {
            Some(max_idx) if agent_position as u64 <= max_idx => AgentStatus::Nominated,
            _ => AgentStatus::Pending,
        };
        Ok(agent_status)
    }

    /// Calculate the biggest index of nomination for pending agents
    pub(crate) fn max_agent_nomination_index(
        &self,
        storage: &dyn Storage,
        cfg: &Config,
        env: Env,
        num_active_agents: &u64,
    ) -> Result<Option<u64>, ContractError> {
        let block_time = env.block.time.seconds();

        let agent_nomination_begin_time = self.agent_nomination_begin_time.load(storage)?;

        match agent_nomination_begin_time {
            Some(begin_time) => {
                let min_tasks_per_agent = cfg.min_tasks_per_agent;
                let total_tasks = self.task_total(storage)?;
                let num_agents_to_accept =
                    self.agents_to_let_in(&min_tasks_per_agent, num_active_agents, &total_tasks);

                if num_agents_to_accept > 0 {
                    let time_difference = block_time - begin_time.seconds();

                    let max_index = cmp::max(
                        time_difference.div(cfg.agent_nomination_duration as u64),
                        num_agents_to_accept - 1,
                    );
                    Ok(Some(max_index))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub fn agents_to_let_in(
        &self,
        max_tasks: &u64,
        num_active_agents: &u64,
        total_tasks: &u64,
    ) -> u64 {
        let num_tasks_covered = num_active_agents * max_tasks;
        if total_tasks > &num_tasks_covered {
            // It's possible there are more "covered tasks" than total tasks,
            // so use saturating subtraction to hit zero and not go below
            let total_tasks_needing_agents = total_tasks.saturating_sub(num_tasks_covered);
            let remainder = if total_tasks_needing_agents % max_tasks == 0 {
                0
            } else {
                1
            };
            total_tasks_needing_agents / max_tasks + remainder
        } else {
            0
        }
    }

    // Change balances of task and contract if action did transaction that went through
    pub fn task_after_action(
        &self,
        storage: &mut dyn Storage,
        api: &dyn Api,
        queue_item: QueueItem,
        ok: bool,
    ) -> Result<Task, ContractError> {
        let task_hash = queue_item.task_hash.unwrap();
        let mut task = self.get_task_by_hash(storage, &task_hash)?;
        if ok {
            let action_idx = queue_item.action_idx;
            let action = &task.actions[action_idx as usize];

            // update task balances and contract balances
            if let Some(sent) = action.bank_sent() {
                for coin in sent {
                    self.sub_availible_native(storage, coin)?;
                    task.total_deposit.sub_native_coin(coin)?;
                }
            } else if let Some(sent) = action.cw20_sent(api) {
                self.sub_availible_cw20(storage, &sent)?;
                task.total_deposit.sub_cw20_coin(&sent)?;
            };
            if task.with_queries() {
                self.tasks_with_queries.save(storage, &task_hash, &task)?;
            } else {
                self.tasks.save(storage, &task_hash, &task)?;
            }
        }
        Ok(task)
    }

    pub(crate) fn sub_availible_native(
        &self,
        storage: &mut dyn Storage,
        coin: &Coin,
    ) -> Result<Uint128, ContractError> {
        let new_bal = self
            .availible_native_balance
            .update(storage, &coin.denom, |bal| {
                bal.unwrap_or_default()
                    .checked_sub(coin.amount)
                    .map_err(StdError::overflow)
            })?;
        Ok(new_bal)
    }

    pub(crate) fn sub_availible_cw20(
        &self,
        storage: &mut dyn Storage,
        cw20: &Cw20CoinVerified,
    ) -> Result<Uint128, ContractError> {
        let new_bal = self
            .available_cw20_balance
            .update(storage, &cw20.address, |bal| {
                bal.unwrap_or_default()
                    .checked_sub(cw20.amount)
                    .map_err(StdError::overflow)
            })?;
        Ok(new_bal)
    }

    pub(crate) fn add_availible_native(
        &self,
        storage: &mut dyn Storage,
        coin: &Coin,
    ) -> Result<Uint128, ContractError> {
        let new_bal = self
            .availible_native_balance
            .update(storage, &coin.denom, |bal| {
                bal.unwrap_or_default()
                    .checked_add(coin.amount)
                    .map_err(StdError::overflow)
            })?;
        Ok(new_bal)
    }

    pub(crate) fn add_availible_cw20(
        &self,
        storage: &mut dyn Storage,
        cw20: &Cw20CoinVerified,
    ) -> Result<Uint128, ContractError> {
        let new_bal = self
            .available_cw20_balance
            .update(storage, &cw20.address, |bal| {
                bal.unwrap_or_default()
                    .checked_add(cw20.amount)
                    .map_err(StdError::overflow)
            })?;
        Ok(new_bal)
    }

    pub(crate) fn add_agent_native(
        &self,
        storage: &mut dyn Storage,
        agent_addr: &Addr,
        coin: &Coin,
    ) -> StdResult<Uint128> {
        let new_bal = self.agent_balances_native.update(
            storage,
            (agent_addr, &coin.denom),
            |bal| -> StdResult<Uint128> {
                let bal = bal.unwrap_or_default();
                Ok(bal.checked_add(coin.amount)?)
            },
        )?;
        Ok(new_bal)
    }

    pub(crate) fn add_user_cw20(
        &self,
        storage: &mut dyn Storage,
        user_addr: &Addr,
        cw20: &Cw20CoinVerified,
    ) -> StdResult<Uint128> {
        let new_bal = self.users_balances_cw20.update(
            storage,
            (user_addr, &cw20.address),
            |bal| -> StdResult<Uint128> {
                let bal = bal.unwrap_or_default();
                Ok(bal.checked_add(cw20.amount)?)
            },
        )?;
        Ok(new_bal)
    }

    pub(crate) fn sub_user_cw20(
        &self,
        storage: &mut dyn Storage,
        user_addr: &Addr,
        cw20: &Cw20CoinVerified,
    ) -> Result<Uint128, ContractError> {
        let current_balance = self
            .users_balances_cw20
            .may_load(storage, (user_addr, &cw20.address))?;
        let mut new_bal = if let Some(bal) = current_balance {
            bal
        } else {
            return Err(ContractError::EmptyBalance {});
        };
        new_bal = new_bal
            .checked_sub(cw20.amount)
            .map_err(StdError::overflow)?;
        if new_bal.is_zero() {
            self.users_balances_cw20
                .remove(storage, (user_addr, &cw20.address));
        } else {
            self.users_balances_cw20
                .save(storage, (user_addr, &cw20.address), &new_bal)?;
        }
        Ok(new_bal)
    }

    // Helper to distribute funds/tokens
    pub(crate) fn agent_withdraw_messages(
        &self,
        storage: &mut dyn Storage,
        agent_addr: &Addr,
        to: &Addr,
        native_keys: Vec<String>,
        cw20_keys: Vec<Addr>,
    ) -> StdResult<(Vec<SubMsg>, Vec<Coin>, Vec<Cw20CoinVerified>)> {
        let mut native_coins = Vec::with_capacity(native_keys.len());
        let mut cw20_coins = Vec::with_capacity(cw20_keys.len());
        let mut messages = Vec::with_capacity(cw20_keys.len() + 1);

        for native_key in native_keys {
            let old_amount = self
                .agent_balances_native
                .load(storage, (agent_addr, &native_key))?;

            self.agent_balances_native
                .remove(storage, (agent_addr, &native_key));
            if !old_amount.is_zero() {
                native_coins.push(coin(old_amount.u128(), native_key));
            }
        }
        if !native_coins.is_empty() {
            messages.push(SubMsg::new(BankMsg::Send {
                to_address: to.to_string(),
                amount: native_coins.clone(),
            }));
        }

        for cw20_key in cw20_keys {
            let old_amount = self
                .agent_balances_cw20
                .load(storage, (agent_addr, &cw20_key))?;
            self.agent_balances_cw20
                .remove(storage, (agent_addr, &cw20_key));
            cw20_coins.push(Cw20CoinVerified {
                address: cw20_key,
                amount: old_amount,
            });
        }

        let cw20_msgs: StdResult<Vec<_>> = cw20_coins
            .iter()
            .map(|c| {
                let msg = Cw20ExecuteMsg::Transfer {
                    recipient: to.to_string(),
                    amount: c.amount,
                };
                let exec = SubMsg::new(WasmMsg::Execute {
                    contract_addr: c.address.to_string(),
                    msg: to_binary(&msg)?,
                    funds: vec![],
                });
                Ok(exec)
            })
            .collect();
        messages.extend(cw20_msgs?);

        Ok((messages, native_coins, cw20_coins))
    }
}

/// Generate submsgs for this proxy call and the price for it
pub(crate) fn proxy_call_submsgs_price(
    task: &Task,
    cfg: Config,
    next_idx: u64,
) -> Result<(Vec<SubMsg>, Coin), ContractError> {
    let (sub_msgs, gas_total) = task.get_submsgs_with_total_gas(
        cfg.gas_base_fee,
        cfg.gas_action_fee,
        cfg.gas_query_fee,
        cfg.gas_wasm_query_fee,
        next_idx,
    )?;
    let gas_amount_with_agent_fee = gas_amount_with_agent_fee(gas_total, cfg.agent_fee)?;
    let price_amount = cfg.gas_price.calculate(gas_amount_with_agent_fee)?;
    let price = coin(price_amount, cfg.native_denom);
    Ok((sub_msgs, price))
}
/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    // /// Get Count
    // pub fn count<Q, T, CQ>(&self, querier: &Q) -> StdResult<CountResponse>
    // where
    //     Q: Querier,
    //     T: Into<String>,
    //     CQ: CustomQuery,
    // {
    //     let msg = QueryMsg::GetCount {};
    //     let query = WasmQuery::Smart {
    //         contract_addr: self.addr().into(),
    //         msg: to_binary(&msg)?,
    //     }
    //     .into();
    //     let res: CountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
    //     Ok(res)
    // }
}
