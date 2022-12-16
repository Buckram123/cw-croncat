use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const LIB_CONTRACT_ADDR: Item<Addr> = Item::new("lib_addr");
pub const LIB_CONTRACT_ADDR2: Item<Addr> = Item::new("lib2_addr");