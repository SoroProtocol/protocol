#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, token};

mod types;
pub use types::VestingSchedule;

const VESTING_TTL_THRESHOLD: u32 = 259_200;
const VESTING_TTL_EXTEND_TO: u32 = 518_400;

#[contract]
pub struct VestingContract;

#[contractimpl]
impl VestingContract {
    /// Create a vesting schedule with optional cliff period.
    pub fn create(
        env: Env,
        funder: Address,
        beneficiary: Address,
        token: Address,
        total_amount: i128,
        start_time: u64,
        cliff_time: u64,
        end_time: u64,
    ) -> u64 {
        funder.require_auth();
        assert!(cliff_time >= start_time, "cliff must be >= start");
        assert!(end_time > cliff_time,    "end must be after cliff");
        assert!(total_amount > 0,         "amount must be positive");

        token::Client::new(&env, &token).transfer(
            &funder, &env.current_contract_address(), &total_amount,
        );

        let id: u64 = env.storage().instance()
            .get(&Symbol::new(&env, "cnt")).unwrap_or(0);
        env.storage().persistent().set(&id, &VestingSchedule {
            id, funder: funder.clone(), beneficiary, token, total_amount,
            start_time, cliff_time, end_time,
            claimed: 0, revoked: false,
        });
        env.storage().persistent().extend_ttl(&id, VESTING_TTL_THRESHOLD, VESTING_TTL_EXTEND_TO);
        env.storage().instance().set(&Symbol::new(&env, "cnt"), &(id + 1));
        env.events().publish(
            (Symbol::new(&env, "VestingCreated"), id),
            (funder, beneficiary, total_amount),
        );
        id
    }

    /// Claim vested tokens. Only callable by beneficiary.
    pub fn claim(env: Env, schedule_id: u64) -> i128 {
        let mut vs: VestingSchedule = env.storage().persistent()
            .get(&schedule_id).expect("schedule not found");
        vs.beneficiary.require_auth();
        assert!(!vs.revoked, "schedule has been revoked");
        let claimable = Self::_vested(&env, &vs) - vs.claimed;
        assert!(claimable > 0, "nothing to claim");
        vs.claimed += claimable;
        env.storage().persistent().set(&schedule_id, &vs);
        env.storage().persistent().extend_ttl(&schedule_id, VESTING_TTL_THRESHOLD, VESTING_TTL_EXTEND_TO);
        token::Client::new(&env, &vs.token).transfer(
            &env.current_contract_address(), &vs.beneficiary, &claimable,
        );
        env.events().publish(
            (Symbol::new(&env, "VestingClaimed"), schedule_id),
            (&vs.beneficiary, claimable),
        );
        claimable
    }

    /// Revoke an unvested schedule. Returns unvested tokens to funder.
    pub fn revoke(env: Env, schedule_id: u64, funder: Address) -> i128 {
        let mut vs: VestingSchedule = env.storage().persistent()
            .get(&schedule_id).expect("schedule not found");
        assert!(funder == vs.funder, "caller is not the original funder");
        funder.require_auth();
        assert!(!vs.revoked, "already revoked");
        let vested    = Self::_vested(&env, &vs);
        let claimable = vested - vs.claimed;
        let refund    = vs.total_amount - vested;
        if claimable > 0 {
            token::Client::new(&env, &vs.token).transfer(
                &env.current_contract_address(), &vs.beneficiary, &claimable,
            );
        }
        if refund > 0 {
            token::Client::new(&env, &vs.token).transfer(
                &env.current_contract_address(), &funder, &refund,
            );
        }
        vs.revoked = true;
        vs.claimed += claimable;
        env.storage().persistent().set(&schedule_id, &vs);
        env.storage().persistent().extend_ttl(&schedule_id, VESTING_TTL_THRESHOLD, VESTING_TTL_EXTEND_TO);
        env.events().publish(
            (Symbol::new(&env, "VestingRevoked"), schedule_id),
            (&funder, claimable, refund),
        );
        refund
    }

    pub fn vested_of(env: Env, schedule_id: u64) -> i128 {
        let vs: VestingSchedule = env.storage().persistent()
            .get(&schedule_id).expect("schedule not found");
        Self::_vested(&env, &vs)
    }

    pub fn get_schedule(env: Env, schedule_id: u64) -> VestingSchedule {
        env.storage().persistent().get(&schedule_id).expect("not found")
    }

    fn _vested(env: &Env, vs: &VestingSchedule) -> i128 {
        let now = env.ledger().timestamp();
        if now < vs.cliff_time { return 0; }
        if now >= vs.end_time  { return vs.total_amount; }
        let duration = (vs.end_time  - vs.cliff_time) as i128;
        let elapsed  = (now          - vs.cliff_time) as i128;
        vs.total_amount * elapsed / duration
    }
}

#[cfg(test)]
mod test;
