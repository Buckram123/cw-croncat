use crate::types::{BoundaryValidated, SlotType};
use cosmwasm_std::{Addr, Env};

pub trait ResultFailed {
    fn failed(&self) -> bool;
}

pub trait Intervals {
    fn next(
        &self,
        env: &Env,
        boundary: BoundaryValidated,
        slot_granularity_time: u64,
    ) -> (u64, SlotType);
    fn is_valid(&self) -> bool;
}

pub trait TaskHash {
    fn to_hash(&self) -> String;
    fn to_hash_vec(&self) -> Vec<u8>;
    fn is_valid_msg(&self, self_addr: &Addr, sender: &Addr, owner_id: &Addr) -> bool;
    fn to_gas_total(&self) -> u64;
}
