use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct VestingSchedule {
    pub id:           u64,
    pub funder:       Address,
    pub beneficiary:  Address,
    pub token:        Address,
    pub total_amount: i128,
    pub start_time:   u64,
    pub cliff_time:   u64,
    pub end_time:     u64,
    pub claimed:      i128,
    pub revoked:      bool,
}
