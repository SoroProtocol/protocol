use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Stream {
    pub id:              u64,
    pub sender:          Address,
    pub recipient:       Address,
    pub token:           Address,
    pub rate_per_second: i128,
    pub start_time:      u64,
    pub stop_time:       u64,
    pub withdrawn:       i128,
    pub cancelled:       bool,
}
