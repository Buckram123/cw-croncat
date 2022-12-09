use crate::balancer::BalancerMode;
use crate::error::ContractError;
use crate::state::{Config, CwCroncat};
use cosmwasm_std::{
    has_coins, to_binary, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, SubMsg, Uint64, WasmMsg,
};
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg};
use cw_croncat_core::msg::{
    BalancesResponse, CwCroncatResponse, ExecuteMsg, GetBalancesResponse, GetConfigResponse,
    GetWalletBalancesResponse, RoundRobinBalancerModeResponse, SlotResponse,
    SlotWithQueriesResponse,
};

impl<'a> CwCroncat<'a> {
    pub(crate) fn query_config(&self, deps: Deps) -> StdResult<GetConfigResponse> {
        let c: Config = self.config.load(deps.storage)?;
        Ok(GetConfigResponse {
            paused: c.paused,
            owner_id: c.owner_id,
            // treasury_id: c.treasury_id,
            min_tasks_per_agent: c.min_tasks_per_agent,
            agent_active_indices: c.agent_active_indices,
            agents_eject_threshold: c.agents_eject_threshold,
            native_denom: c.native_denom,
            agent_fee: c.agent_fee,
            gas_fraction: c.gas_fraction,
            proxy_callback_gas: c.proxy_callback_gas,
            slot_granularity_time: c.slot_granularity_time,
            cw_rules_addr: c.cw_rules_addr,
            agent_nomination_duration: c.agent_nomination_duration,
            gas_base_fee: c.gas_base_fee,
            gas_action_fee: c.gas_action_fee,
            cw20_whitelist: c.cw20_whitelist,
            limit: c.limit,
        })
    }

    pub(crate) fn query_balances(
        &self,
        deps: Deps,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> StdResult<GetBalancesResponse> {
        let c: Config = self.config.load(deps.storage)?;

        let from_index = from_index.unwrap_or_default();
        let limit = limit.unwrap_or(c.limit);

        let available_native_balance = self
            .availible_native_balance
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|res| match res {
                Ok((denom, amount)) => Ok(Coin { denom, amount }),
                Err(err) => Err(err),
            })
            .collect::<StdResult<Vec<Coin>>>()?;

        let available_cw20_balance = self
            .available_cw20_balance
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|res| match res {
                Ok((address, amount)) => Ok(Cw20CoinVerified { address, amount }),
                Err(err) => Err(err),
            })
            .collect::<StdResult<Vec<Cw20CoinVerified>>>()?;

        let staked_native_balance = self
            .staked_native_balance
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|res| match res {
                Ok((denom, amount)) => Ok(Coin { denom, amount }),
                Err(err) => Err(err),
            })
            .collect::<StdResult<Vec<Coin>>>()?;

        let staked_cw20_balance = self
            .staked_cw20_balance
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|res| match res {
                Ok((address, amount)) => Ok(Cw20CoinVerified { address, amount }),
                Err(err) => Err(err),
            })
            .collect::<StdResult<Vec<Cw20CoinVerified>>>()?;

        Ok(GetBalancesResponse {
            native_denom: c.native_denom,
            available_native_balance,
            available_cw20_balance,

            staked_native_balance,
            staked_cw20_balance,
            cw20_whitelist: c.cw20_whitelist,
        })
    }

    /// Returns user cw20 balances locked inside this contract
    pub(crate) fn query_wallet_balances(
        &self,
        deps: Deps,
        wallet: String,
    ) -> StdResult<GetWalletBalancesResponse> {
        let addr = deps.api.addr_validate(&wallet)?;
        let balances = self.users_balances.may_load(deps.storage, &addr)?;
        Ok(GetWalletBalancesResponse {
            cw20_balances: balances.unwrap_or_default(),
        })
    }

    /// Changes core configurations
    /// Should only be updated by owner -- in best case DAO based :)
    pub fn update_settings(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        payload: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        for coin in info.funds.iter() {
            if coin.amount.u128() > 0 {
                return Err(ContractError::AttachedDeposit {});
            }
        }
        let api = deps.api;
        match payload {
            ExecuteMsg::UpdateSettings {
                owner_id,
                slot_granularity_time,
                paused,
                agent_fee,
                gas_base_fee,
                gas_action_fee,
                gas_fraction,
                proxy_callback_gas,
                min_tasks_per_agent,
                agents_eject_threshold,
                // treasury_id,
            } => {
                self.config
                    .update(deps.storage, |mut config| -> Result<_, ContractError> {
                        if info.sender != config.owner_id {
                            return Err(ContractError::Unauthorized {});
                        }

                        if let Some(owner_id) = owner_id {
                            let owner_id = api.addr_validate(&owner_id)?;
                            config.owner_id = owner_id;
                        }
                        // if let Some(treasury_id) = treasury_id {
                        //     config.treasury_id = Some(treasury_id);
                        // }
                        if let Some(slot_granularity_time) = slot_granularity_time {
                            config.slot_granularity_time = slot_granularity_time;
                        }
                        if let Some(paused) = paused {
                            config.paused = paused;
                        }
                        if let Some(gas_base_fee) = gas_base_fee {
                            config.gas_base_fee = gas_base_fee.u64();
                        }
                        if let Some(gas_action_fee) = gas_action_fee {
                            config.gas_action_fee = gas_action_fee.u64();
                        }
                        if let Some(gas_fraction) = gas_fraction {
                            config.gas_fraction = gas_fraction;
                        }
                        if let Some(proxy_callback_gas) = proxy_callback_gas {
                            config.proxy_callback_gas = proxy_callback_gas;
                        }
                        if let Some(agent_fee) = agent_fee {
                            config.agent_fee = agent_fee;
                        }
                        if let Some(min_tasks_per_agent) = min_tasks_per_agent {
                            config.min_tasks_per_agent = min_tasks_per_agent;
                        }
                        if let Some(agents_eject_threshold) = agents_eject_threshold {
                            config.agents_eject_threshold = agents_eject_threshold;
                        }
                        Ok(config)
                    })?;
            }
            _ => unreachable!(),
        }
        let c: Config = self.config.load(deps.storage)?;
        Ok(Response::new()
            .add_attribute("method", "update_settings")
            .add_attribute("paused", c.paused.to_string())
            .add_attribute("owner_id", c.owner_id.to_string())
            // .add_attribute(
            //     "treasury_id",
            //     c.treasury_id
            //         .unwrap_or_else(|| Addr::unchecked(""))
            //         .to_string(),
            // )
            .add_attribute("min_tasks_per_agent", c.min_tasks_per_agent.to_string())
            .add_attribute(
                "agent_active_indices",
                c.agent_active_indices
                    .iter()
                    .map(|a| format!("{:?}.{}", a.0, a.1))
                    .collect::<String>(),
            )
            .add_attribute(
                "agents_eject_threshold",
                c.agents_eject_threshold.to_string(),
            )
            .add_attribute("native_denom", c.native_denom)
            .add_attribute("agent_fee", c.agent_fee.to_string())
            //.add_attribute("gas_price", c.gas_fraction.to_string())
            .add_attribute("proxy_callback_gas", c.proxy_callback_gas.to_string())
            .add_attribute("slot_granularity_time", c.slot_granularity_time.to_string()))
    }

    /// Move Balance
    /// Allows owner to move balance to DAO or to let treasury transfer to itself only.
    /// This is a restricted method for moving funds utilized in growth management strategies.
    pub fn move_balances(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        balances: Vec<Balance>,
        account_id: String,
    ) -> Result<Response, ContractError> {
        let account_id = deps.api.addr_validate(&account_id)?;
        let config = self.config.load(deps.storage)?;

        // // Check if is owner OR the treasury account making the transfer request
        // if let Some(treasury_id) = config.treasury_id.clone() {
        //     if treasury_id != info.sender && config.owner_id != info.sender {
        //         return Err(ContractError::Unauthorized {});
        //     }
        // } else
        if info.sender != config.owner_id {
            return Err(ContractError::Unauthorized {});
        }

        // for now, only allow movement of funds between owner and treasury
        // let check_account = config
        //     .treasury_id
        //     .clone()
        //     .unwrap_or_else(|| config.owner_id.clone());
        let check_account = config.owner_id.clone();
        if check_account != account_id && config.owner_id != account_id {
            return Err(ContractError::CustomError {
                val: "Cannot move funds to this account".to_string(),
            });
        }

        // Querier guarantees to returns up-to-date data, including funds sent in this handle message
        // https://github.com/CosmWasm/wasmd/blob/master/x/wasm/internal/keeper/keeper.go#L185-L192
        let state_balances = deps.querier.query_all_balances(&env.contract.address)?;
        let mut has_fund_err = false;

        let messages: Result<Vec<SubMsg>, ContractError> = balances
            .iter()
            .map(|balance| -> Result<SubMsg<_>, ContractError> {
                match balance {
                    Balance::Native(balance) => {
                        // check has enough
                        let bal = balance.clone().into_vec();
                        let has_c = has_coins(&state_balances, &bal[0]);
                        if !has_c {
                            has_fund_err = true;
                            // TODO: refactor to not need
                            return Ok(SubMsg::new(BankMsg::Send {
                                to_address: account_id.clone().into(),
                                amount: vec![Coin::new(0, "")],
                            }));
                        }

                        // Update internal registry balance
                        for coin in bal.iter() {
                            self.subtract_availible_native(deps.storage, coin)?;
                        }
                        Ok(SubMsg::new(BankMsg::Send {
                            to_address: account_id.clone().into(),
                            amount: bal,
                        }))
                    }
                    Balance::Cw20(token) => {
                        // check has enough
                        let avail_bal = self
                            .available_cw20_balance
                            .may_load(deps.storage, &token.address)?
                            .unwrap_or_default();
                        if avail_bal < token.amount {
                            has_fund_err = true;
                            // TODO: refactor to not need
                            return Ok(SubMsg::new(BankMsg::Send {
                                to_address: account_id.clone().into(),
                                amount: vec![Coin::new(0, "")],
                            }));
                        }

                        // Update internal registry balance
                        self.subtract_availible_cw20(deps.storage, token)?;

                        let msg = Cw20ExecuteMsg::Transfer {
                            recipient: account_id.clone().into(),
                            amount: token.amount,
                        };
                        Ok(SubMsg::new(WasmMsg::Execute {
                            contract_addr: token.address.to_string(),
                            msg: to_binary(&msg)?,
                            funds: vec![],
                        }))
                    }
                }
            })
            .collect();

        // failed
        if has_fund_err {
            return Err(ContractError::CustomError {
                val: "Not enough funds".to_string(),
            });
        }

        Ok(Response::new()
            .add_attribute("method", "move_balance")
            .add_attribute("account_id", account_id.to_string())
            .add_submessages(messages?))
    }

    pub(crate) fn get_state(
        &self,
        deps: Deps,
        env: Env,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> StdResult<CwCroncatResponse> {
        let default_limit = self.config.load(deps.storage)?.limit;
        let size: u64 = self.task_total.load(deps.storage)?.min(default_limit);
        let from_index_unwrap = from_index.unwrap_or_default();
        let limit_unwrap = limit.unwrap_or(default_limit).min(size) as usize;

        let mut agents = Vec::with_capacity(limit_unwrap);
        for agent in self
            .agents
            .keys(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap)
        {
            let agent_info = self.query_get_agent(deps, env.clone(), agent?.to_string())?;
            agents.push(agent_info.unwrap());
        }

        let time_slots: Vec<SlotResponse> = self
            .time_slots
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap as usize)
            .map(|res| {
                let res = res.unwrap();
                SlotResponse {
                    slot: res.0.into(),
                    tasks: res.1,
                }
            })
            .collect();

        let block_slots: Vec<SlotResponse> = self
            .block_slots
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap as usize)
            .map(|res| {
                let res = res.unwrap();
                SlotResponse {
                    slot: res.0.into(),
                    tasks: res.1,
                }
            })
            .collect();

        let balances: Vec<BalancesResponse> = self
            .users_balances
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap as usize)
            .map(|res| {
                let res = res.unwrap();
                BalancesResponse {
                    address: res.0,
                    balances: res.1,
                }
            })
            .collect();

        let balancer_mode = match self.balancer.mode {
            BalancerMode::ActivationOrder => RoundRobinBalancerModeResponse::ActivationOrder,
            BalancerMode::Equalizer => RoundRobinBalancerModeResponse::Equalizer,
        };

        let time_slots_queries: Vec<SlotWithQueriesResponse> = self
            .time_map_queries
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap as usize)
            .map(|res| {
                let res = res.unwrap();
                SlotWithQueriesResponse {
                    task_hash: res.0,
                    slot: res.1.into(),
                }
            })
            .collect();

        let block_slots_queries: Vec<SlotWithQueriesResponse> = self
            .block_map_queries
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index_unwrap as usize)
            .take(limit_unwrap as usize)
            .map(|res| {
                let res = res.unwrap();
                SlotWithQueriesResponse {
                    task_hash: res.0,
                    slot: res.1.into(),
                }
            })
            .collect();

        Ok(CwCroncatResponse {
            config: self.query_config(deps)?,

            agent_active_queue: self.agent_active_queue.load(deps.storage)?,
            agent_pending_queue: self
                .agent_pending_queue
                .iter(deps.storage)?
                .take(50)
                .collect::<StdResult<Vec<cosmwasm_std::Addr>>>()?,
            agents,

            tasks: self.query_get_tasks(deps, None, None)?,
            task_total: Uint64::from(self.task_total.load(deps.storage)?),

            time_slots,
            block_slots,

            tasks_with_queries: self.query_get_tasks_with_queries(deps, from_index, limit)?,
            tasks_with_queries_total: Uint64::from(
                self.tasks_with_queries_total.load(deps.storage)?,
            ),
            time_slots_queries,
            block_slots_queries,

            reply_index: Uint64::from(self.reply_index.load(deps.storage)?),
            agent_nomination_begin_time: self.agent_nomination_begin_time.load(deps.storage)?,

            balances,
            balancer_mode,
        })
    }
}
