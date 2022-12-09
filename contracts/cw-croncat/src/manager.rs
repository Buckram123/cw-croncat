use crate::balancer::Balancer;
use crate::error::ContractError;
use crate::helpers::proxy_call_submsgs_price;
use crate::state::{Config, CwCroncat, QueueItem, TaskInfo};
use cosmwasm_std::{
    from_binary, Addr, Deps, DepsMut, Env, MessageInfo, Order, Reply, Response, StdResult, Storage,
};
use cw_croncat_core::traits::{FindAndMutate, Intervals};
use cw_croncat_core::types::{Agent, Interval, SlotType, Task};
use cw_rules_core::msg::{QueryConstruct, QueryConstructResponse};

impl<'a> CwCroncat<'a> {
    /// Executes a task based on the current task slot
    /// Computes whether a task should continue further or not
    /// Makes a cross-contract call with the task configuration
    /// Called directly by a registered agent
    pub fn proxy_call(
        &mut self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
    ) -> Result<Response, ContractError> {
        self.check_ready_for_proxy_call(deps.as_ref(), &info)?;
        self.check_agent(deps.storage, &info.sender, env.block.height)?;

        let cfg: Config = self.config.load(deps.storage)?;

        // get slot items, find the next task hash available
        // if empty slot found, let agent get paid for helping keep house clean
        let slot = self.get_current_slot_items(&env.block, deps.storage, Some(1));
        // Give preference for block-based slots
        let (slot_id, slot_type) = match slot {
            (Some(slot_id), _) => {
                let kind = SlotType::Block;
                (slot_id, kind)
            }
            (None, Some(slot_id)) => {
                let kind = SlotType::Cron;
                (slot_id, kind)
            }
            (None, None) => {
                return Ok(Response::new()
                    .add_attribute("method", "proxy_call")
                    .add_attribute("agent", &info.sender)
                    .add_attribute("has_task", "false"));
            }
        };

        let some_hash = self.pop_slot_item(deps.storage, slot_id, slot_type)?;

        // Get the task details
        // if no task, return error.
        let hash = if let Some(hash) = some_hash {
            hash
        } else {
            return Err(ContractError::NoTaskFound {});
        };

        //Get agent tasks with extra(if exists) from balancer
        let balancer_result = self
            .balancer
            .get_agent_tasks(
                &deps.as_ref(),
                &env,
                &self.config,
                &self.agent_active_queue,
                info.sender.clone(),
                slot,
            )?
            .ok_or(ContractError::NoTaskFound {})?;
        //Balanacer gives not task to this agent, return error
        let has_tasks = balancer_result.has_any_slot_tasks(slot_type);
        if !has_tasks {
            return Err(ContractError::NoTaskFound {});
        }

        // ----------------------------------------------------
        // TODO: FINISH!!!!!!
        // AGENT Task Allowance Logic: see line 339
        // ----------------------------------------------------

        // self.check_bank_msg(deps.as_ref(), &info, &env, &task)?;

        // TODO: Bring this back!
        // // Fee breakdown:
        // // - Used Gas: Task Txn Fee Cost
        // // - Agent Fee: Incentivize Execution SLA
        // //
        // // Task Fee Examples:
        // // Total Fee = Gas Fee + Agent Fee
        // // Total Balance = Task Deposit + Total Fee
        // //
        // // NOTE: Gas cost includes the cross-contract call & internal logic of this contract.
        // // Direct contract gas fee will be lower than task execution costs, however
        // // we require the task owner to appropriately estimate gas for overpayment.
        // // The gas overpayment will also accrue to the agent since there is no way to read
        // // how much gas was actually used on callback.
        // let call_fee_used = u128::from(task.gas).saturating_mul(self.gas_price);
        // let call_total_fee = call_fee_used.saturating_add(self.agent_fee);
        // let call_total_balance = task.deposit.0.saturating_add(call_total_fee);

        // // safety check and not burn too much gas.
        // if call_total_balance > task.total_deposit.0 {
        //     log!("Not enough task balance to execute task, exiting");
        //     // Process task exit, if no future task can execute
        //     return self.exit_task(hash);
        // }

        // TODO: Bring this back!
        // // Update agent storage
        // // Increment agent reward & task count
        // // Reward for agent MUST include the amount of gas used as a reimbursement
        // agent.balance = U128::from(agent.balance.0.saturating_add(call_total_fee));
        // agent.total_tasks_executed = U128::from(agent.total_tasks_executed.0.saturating_add(1));
        // self.available_balance = self.available_balance.saturating_sub(call_total_fee);

        // TODO: Bring this back!
        // // Reset missed slot, if any
        // if agent.last_missed_slot != 0 {
        //     agent.last_missed_slot = 0;
        // }
        // self.agents.insert(&env::signer_account_id(), &agent);

        // TODO: Bring this back!
        // // Decrease task balance, Update task storage
        // task.total_deposit = U128::from(task.total_deposit.0.saturating_sub(call_total_balance));
        // self.tasks.insert(&hash, &task);

        // TODO: Move to external rule query handler
        // Proceed to query loops if rules are found in the task
        // Each rule is chained into the next, then evaluated if response is true before proceeding
        // let mut rule_responses: Vec<Attribute> = vec![];
        // if task.rules.is_some() {
        //     let mut rule_success: bool = false;
        //     // let mut previous_msg: Option<Binary>;
        //     for (idx, rule) in task.clone().rules.unwrap().iter().enumerate() {
        //         let rule_res: RuleResponse = deps
        //             .querier
        //             .query_wasm_smart(&rule.contract_addr, &rule.msg)?;
        //         println!("{:?}", rule_res);
        //         rule_success = rule_res.0;

        //         // TODO: needs better approach
        //         d.push(Attribute::new(idx.to_string(), format!("{:?}", rule_res.1)));
        //     }
        //     if !rule_success {
        //         return Err(ContractError::CustomError {
        //             val: "Rule evaluated to false".to_string(),
        //         });
        //     }
        // }

        // Decrease cw20 balances for this call
        // TODO: maybe save task_cw20_balance_uses in the `Task` itself
        // let task_cw20_balance_uses = task.task_cw20_balance_uses(deps.api)?;
        // task.total_cw20_deposit
        //     .checked_sub_coins(&task_cw20_balance_uses)?;
        // Setup submessages for actions for this task
        // Each submessage in storage, computes & stores the "next" reply to allow for chained message processing.

        // Add submessages for all actions
        let next_idx = self.rq_next_id(deps.storage)?;
        let mut task = self.tasks.load(deps.storage, &hash)?;
        let (sub_msgs, fee_price) = proxy_call_submsgs_price(&task, cfg, next_idx)?;
        task.total_deposit.native.find_checked_sub(&fee_price)?;
        // Add balance to the agent
        self.add_agent_native(deps.storage, &info.sender, &fee_price)?;
        self.tasks.save(deps.storage, &hash, &task)?;
        // Keep track for later scheduling
        let self_addr = env.contract.address;
        self.rq_push(
            deps.storage,
            QueueItem {
                action_idx: 0,
                task_hash: Some(hash),
                contract_addr: Some(self_addr),
                task_is_extra: Some(balancer_result.has_any_slot_extra_tasks(slot_type)),
                agent_id: Some(info.sender.clone()),
                failure: None,
            },
        )?;

        // TODO: Add supported msgs if not a SubMessage?
        // Add the messages, reply handler responsible for task rescheduling
        let final_res = Response::new()
            .add_attribute("method", "proxy_call")
            .add_attribute("agent", info.sender)
            .add_attribute("slot_id", slot_id.to_string())
            .add_attribute("slot_kind", format!("{:?}", slot_type))
            .add_attribute("task_hash", task.to_hash())
            .add_submessages(sub_msgs);
        Ok(final_res)
    }

    /// Executes a task based on the current task slot
    /// Computes whether a task should continue further or not
    /// Makes a cross-contract call with the task configuration
    /// Called directly by a registered agent
    pub fn proxy_call_with_queries(
        &mut self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        task_hash: String,
    ) -> Result<Response, ContractError> {
        self.check_ready_for_proxy_call(deps.as_ref(), &info)?;
        self.check_agent(deps.storage, &info.sender, env.block.height)?;
        let hash = task_hash.as_bytes();

        let cfg: Config = self.config.load(deps.storage)?;
        let some_task = self
            .tasks_with_queries
            .may_load(deps.storage, task_hash.as_bytes())?;
        let mut task = some_task.ok_or(ContractError::NoTaskFound {})?;

        let task_ready =
            self.task_with_query_ready(task.interval.clone(), deps.as_ref(), hash, &env)?;
        if !task_ready {
            return Err(ContractError::CustomError {
                val: "Task is not ready".to_string(),
            });
        }
        // self.check_bank_msg(deps.as_ref(), &info, &env, &task)?;
        let queries = if let Some(queries) = task.queries.clone() {
            queries
        } else {
            // TODO: else should be unreachable
            return Err(ContractError::NoQueriesForThisTask { task_hash });
        };
        // Check rules
        let queries_res: QueryConstructResponse = deps.querier.query_wasm_smart(
            &cfg.cw_rules_addr,
            &cw_rules_core::msg::QueryMsg::QueryConstruct(QueryConstruct { queries }),
        )?;
        if !queries_res.result {
            return Err(ContractError::QueriesNotReady {
                index: from_binary(&queries_res.data[0])?,
            });
        };

        // Add submessages for all actions
        let next_idx = self.rq_next_id(deps.storage)?;
        // This may be different to the one we keep in the storage
        // due to the insertable messages
        let (sub_msgs, fee_price) = match task
            .replace_values(
                deps.api,
                &env.contract.address,
                &task_hash,
                queries_res.data,
            )
            .map_err(Into::into)
            .and(proxy_call_submsgs_price(&task, cfg, next_idx))
        {
            Ok((sub_msgs, fee_price)) => (sub_msgs, fee_price),
            Err(err) => {
                let resp = self.remove_task(deps.storage, &task_hash, None)?;
                return Ok(resp
                    .add_attribute("method", "proxy_call")
                    .add_attribute("agent", info.sender)
                    .add_attribute("task_hash", task_hash)
                    .add_attribute("task_with_queries", true.to_string())
                    .add_attribute("task_removed_without_execution", err.to_string()));
            }
        };

        // Add balance to the agent
        self.add_agent_native(deps.storage, &info.sender, &fee_price)?;

        self.tasks_with_queries
            .update(deps.storage, hash, |task| -> Result<_, ContractError> {
                let mut task = task.ok_or(ContractError::NoTaskFound {})?;
                task.total_deposit.native.find_checked_sub(&fee_price)?;
                Ok(task)
            })?;
        // Keep track for later scheduling
        self.rq_push(
            deps.storage,
            QueueItem {
                action_idx: 0,
                task_hash: Some(task_hash.clone().into_bytes()),
                contract_addr: Some(env.contract.address),
                task_is_extra: Some(false),
                agent_id: Some(info.sender.clone()),
                failure: None,
            },
        )?;
        // TODO: Add supported msgs if not a SubMessage?
        // Add the messages, reply handler responsible for task rescheduling
        let final_res = Response::new()
            .add_attribute("method", "proxy_call")
            .add_attribute("agent", info.sender)
            .add_attribute("task_hash", task_hash)
            .add_attribute("task_with_queries", true.to_string())
            .add_submessages(sub_msgs);
        Ok(final_res)
    }

    /// Check that this task can be executed in current slot
    fn task_with_query_ready(
        &mut self,
        task_interval: Interval,
        deps: Deps,
        hash: &[u8],
        env: &Env,
    ) -> Result<bool, ContractError> {
        let task_ready = match task_interval {
            Interval::Cron(_) => {
                let block = self.time_map_queries.load(deps.storage, hash)?;
                env.block.height >= block
            }
            _ => {
                let time = self.block_map_queries.load(deps.storage, hash)?;
                env.block.time.nanos() >= time
            }
        };
        Ok(task_ready)
    }

    /// Logic executed on the completion of a proxy call
    /// Reschedule next task
    pub(crate) fn proxy_callback(
        &self,
        deps: DepsMut,
        env: Env,
        _msg: Reply,
        task: Task,
        queue_item: QueueItem,
    ) -> Result<Response, ContractError> {
        let task_hash = task.to_hash();
        // TODO: How can we compute gas & fees paid on this txn?
        // let out_of_funds = call_total_balance > task.total_deposit;

        let agent_id = queue_item.agent_id.ok_or(ContractError::Unauthorized {})?;

        // Parse interval into a future timestamp, then convert to a slot
        let cfg: Config = self.config.load(deps.storage)?;
        let (next_id, slot_kind) =
            task.interval
                .next(&env, task.boundary, cfg.slot_granularity_time);

        let response = if let Some(ref failure) = queue_item.failure {
            Response::new().add_attribute("with_failure", failure)
        } else {
            Response::new()
        };

        // if non-recurring, exit
        if task.interval == Interval::Once
            || (task.stop_on_fail && queue_item.failure.is_some())
            || task.verify_enough_balances(false).is_err()
            // If the next interval comes back 0, then this task should not schedule again
            || next_id == 0
        // proxy_call_with_rules makes it fail if rules aren't met
        {
            // Process task exit, if no future task can execute
            // Task has been removed, complete and rebalance internal balancer
            let task_info = TaskInfo {
                task,
                task_hash: task_hash.as_bytes().to_vec(),
                task_is_extra: queue_item.task_is_extra,
                slot_kind,
                agent_id,
            };
            self.complete_agent_task(deps.storage, env, &task_info)?;
            let resp = self.remove_task(deps.storage, &task_hash, None)?;
            Ok(response
                .add_attribute("method", "proxy_callback")
                .add_attribute("ended_task", task_hash)
                .add_attributes(resp.attributes)
                .add_submessages(resp.messages)
                .add_events(resp.events))
        } else {
            self.reschedule_task(
                task.with_queries(),
                slot_kind,
                deps.storage,
                task_hash,
                next_id,
            )?;

            Ok(response
                .add_attribute("method", "proxy_callback")
                .add_attribute("slot_id", next_id.to_string())
                .add_attribute("slot_kind", format!("{:?}", slot_kind)))
        }
    }

    /// Update time or block of next time this task should be executed
    fn reschedule_task(
        &self,
        task_with_queries: bool,
        slot_kind: SlotType,
        storage: &mut dyn Storage,
        task_hash: String,
        next_id: u64,
    ) -> Result<(), ContractError> {
        if task_with_queries {
            // Based on slot kind, put into block or cron slots
            match slot_kind {
                SlotType::Block => {
                    self.block_map_queries
                        .save(storage, task_hash.as_bytes(), &next_id)?;
                }
                SlotType::Cron => {
                    self.time_map_queries
                        .save(storage, task_hash.as_bytes(), &next_id)?;
                }
            }
        } else {
            // Get previous task hashes in slot, add as needed
            let update_vec_data = |d: Option<Vec<Vec<u8>>>| -> StdResult<Vec<Vec<u8>>> {
                match d {
                    // has some data, simply push new hash
                    Some(data) => {
                        let mut s = data;
                        s.push(task_hash.into_bytes());
                        Ok(s)
                    }
                    // No data, push new vec & hash
                    None => Ok(vec![task_hash.into_bytes()]),
                }
            };

            // Based on slot kind, put into block or cron slots
            match slot_kind {
                SlotType::Block => {
                    self.block_slots.update(storage, next_id, update_vec_data)?;
                }
                SlotType::Cron => {
                    self.time_slots.update(storage, next_id, update_vec_data)?;
                }
            }
        };
        Ok(())
    }

    fn check_ready_for_proxy_call(
        &self,
        deps: Deps,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        if !info.funds.is_empty() {
            return Err(ContractError::CustomError {
                val: "Must not attach funds".to_string(),
            });
        }
        let c: Config = self.config.load(deps.storage)?;
        if c.paused {
            return Err(ContractError::CustomError {
                val: "Contract paused".to_string(),
            });
        }
        Ok(())
    }

    fn check_agent(
        &mut self,
        storage: &mut dyn Storage,
        agent_addr: &Addr,
        last_executed_slot: u64,
    ) -> Result<Agent, ContractError> {
        // only registered agent signed, because micropayments will benefit long term
        let agent = self.agents.update(storage, agent_addr, |ag| match ag {
            Some(mut agent) => {
                agent.update(last_executed_slot);
                Ok(agent)
            }
            None => Err(ContractError::AgentNotRegistered {}),
        })?;

        let active_agents: Vec<Addr> = self.agent_active_queue.load(storage)?;

        // make sure agent is active
        if !active_agents.contains(agent_addr) {
            return Err(ContractError::AgentNotRegistered {});
        }
        Ok(agent)
    }

    fn complete_agent_task(
        &self,
        storage: &mut dyn Storage,
        env: Env,
        task_info: &TaskInfo,
    ) -> Result<(), ContractError> {
        // let TaskInfo {
        //     task_hash, task, ..
        // } = task_info;

        //no fail
        self.balancer.on_task_completed(
            storage,
            &env,
            &self.config,
            &self.agent_active_queue,
            task_info,
        )?;
        Ok(())
    }

    // // Check if the task is recurring and if it is, delete it
    // pub(crate) fn delete_non_recurring(&self, storage: &mut dyn Storage, task: &Task, response: Response, reply_submsg_failed: bool) -> Result<Response, ContractError> {
    //     if task.interval == Interval::Once || (task.stop_on_fail && reply_submsg_failed) {
    //         // Process task exit, if no future task can execute
    //         let rt = self.remove_task(storage, task.to_hash());
    //         if let Ok(..) = rt {
    //             let resp = rt.unwrap();
    //             response = response
    //                 .add_attributes(resp.attributes)
    //                 .add_submessages(resp.messages)
    //                 .add_events(resp.events);
    //         }
    //     };
    //     return Ok(response)
    // } else {}

    /// Helps manage and cleanup agents
    /// Deletes agents which missed more than agents_eject_threshold slot
    pub fn tick(&mut self, deps: DepsMut, env: Env) -> Result<Response, ContractError> {
        let current_slot = env.block.height;
        let cfg = self.config.load(deps.storage)?;
        let mut attributes = vec![];
        let mut submessages = vec![];
        for agent_id in self
            .agents
            .keys(deps.storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<Addr>>>()?
        {
            let agent = self.agents.load(deps.storage, &agent_id)?;
            if current_slot > agent.last_executed_slot + cfg.agents_eject_threshold {
                let resp = self
                    .unregister_agent(deps.storage, &agent_id, None)
                    .unwrap_or_default();
                // Save attributes and messages
                attributes.extend_from_slice(&resp.attributes);
                submessages.extend_from_slice(&resp.messages);
            }
        }
        let response = Response::new()
            .add_attribute("method", "tick")
            .add_attributes(attributes)
            .add_submessages(submessages);
        Ok(response)
    }
}
