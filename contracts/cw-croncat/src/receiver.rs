use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw20::{Cw20CoinVerified, Cw20ReceiveMsg};

use crate::{ContractError, CwCroncat};

impl<'a> CwCroncat<'a> {
    /// Add cw20 coin to user balance, that sent this coins
    pub fn receive_cw20(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        let sender = deps.api.addr_validate(&msg.sender)?;
        let coin_address = info.sender;

        // Updating user balance
        let verified = Cw20CoinVerified {
            address: coin_address,
            amount: msg.amount,
        };
        let new_bal = self.add_user_cw20(deps.storage, &sender, &verified)?;

        // Updating contract balance
        self.add_available_cw20(deps.storage, &verified)?;
        Ok(Response::new()
            .add_attribute("method", "receive_cw20")
            .add_attribute("total_cw20_balance", format!("{new_bal:?}")))
    }
}
