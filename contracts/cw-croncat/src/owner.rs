use crate::error::ContractError;
use crate::state::{Config, CwCroncat};
use cosmwasm_std::{
    has_coins, to_binary, Addr, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, SubMsg, Uint128, WasmMsg,
};
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg};
use cw_croncat_core::msg::{ExecuteMsg, GetBalancesResponse, GetWalletBalancesResponse};

impl<'a> CwCroncat<'a> {
    pub(crate) fn query_config(&self, deps: Deps) -> StdResult<Config> {
        let c: Config = self.config.load(deps.storage)?;
        Ok(c)
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

        Ok(GetBalancesResponse {
            native_denom: c.native_denom,
            available_native_balance,
            available_cw20_balance,

            cw20_whitelist: c.cw20_whitelist,
        })
    }

    /// Returns user cw20 balances locked inside this contract
    pub(crate) fn query_wallet_balances(
        &self,
        deps: Deps,
        wallet: String,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> StdResult<GetWalletBalancesResponse> {
        let addr = deps.api.addr_validate(&wallet)?;
        let c = self.config.load(deps.storage)?;

        let from_index = from_index.unwrap_or_default();
        let limit = limit.unwrap_or(c.limit);

        let balances = self
            .users_balances_cw20
            .prefix(&addr)
            .range(deps.storage, None, None, Order::Ascending)
            .skip(from_index as usize)
            .take(limit as usize)
            .collect::<StdResult<Vec<(Addr, Uint128)>>>()?;
        let cw20_balances = balances
            .into_iter()
            .map(|(cw20_addr, amount)| Cw20CoinVerified {
                address: cw20_addr,
                amount,
            })
            .collect();
        Ok(GetWalletBalancesResponse { cw20_balances })
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
                gas_query_fee,
                gas_wasm_query_fee,
                gas_price,
                proxy_callback_gas,
                min_tasks_per_agent,
                agents_eject_threshold,
                // treasury_id,
            } => {
                let owner_id = if let Some(addr) = owner_id {
                    Some(api.addr_validate(&addr)?)
                } else {
                    None
                };
                self.config
                    .update(deps.storage, |old_config| -> Result<_, ContractError> {
                        if info.sender != old_config.owner_id {
                            return Err(ContractError::Unauthorized {});
                        }

                        let new_config = Config {
                            paused: paused.unwrap_or(old_config.paused),
                            owner_id: owner_id.unwrap_or(old_config.owner_id),
                            min_tasks_per_agent: min_tasks_per_agent
                                .unwrap_or(old_config.min_tasks_per_agent),
                            agents_eject_threshold: agents_eject_threshold
                                .unwrap_or(old_config.agents_eject_threshold),
                            agent_nomination_duration: old_config.agent_nomination_duration,
                            cw_rules_addr: old_config.cw_rules_addr,
                            agent_fee: agent_fee.unwrap_or(old_config.agent_fee),
                            gas_price: gas_price.unwrap_or(old_config.gas_price),
                            gas_base_fee: gas_base_fee
                                .map(Into::into)
                                .unwrap_or(old_config.gas_base_fee),
                            gas_action_fee: gas_action_fee
                                .map(Into::into)
                                .unwrap_or(old_config.gas_action_fee),
                            gas_query_fee: gas_query_fee
                                .map(Into::into)
                                .unwrap_or(old_config.gas_query_fee),
                            gas_wasm_query_fee: gas_wasm_query_fee
                                .map(Into::into)
                                .unwrap_or(old_config.gas_wasm_query_fee),
                            proxy_callback_gas: proxy_callback_gas
                                .unwrap_or(old_config.proxy_callback_gas),
                            slot_granularity_time: slot_granularity_time
                                .unwrap_or(old_config.slot_granularity_time),
                            cw20_whitelist: old_config.cw20_whitelist,
                            native_denom: old_config.native_denom,
                            limit: old_config.limit,
                        };
                        Ok(new_config)
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
                "agents_eject_threshold",
                c.agents_eject_threshold.to_string(),
            )
            .add_attribute("native_denom", c.native_denom)
            .add_attribute("agent_fee", c.agent_fee.to_string())
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
                            self.sub_availible_native(deps.storage, coin)?;
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
                        self.sub_availible_cw20(deps.storage, token)?;

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
}
