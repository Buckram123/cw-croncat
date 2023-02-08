use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub(crate) fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_reply(crate::contract::reply);
    Box::new(contract)
}
