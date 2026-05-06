#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

mod errors;
mod events;
mod storage;
mod types;

pub use types::Stream;
use storage::{load_stream, next_id, save_stream, set_next_id};

#[contract]
pub struct StreamContract;

#[contractimpl]
impl StreamContract {
    /// Create a payment stream. Deposits rate * duration tokens upfront.
    pub fn create(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        rate_per_second: i128,
        start_time: u64,
        stop_time: u64,
    ) -> u64 {
        sender.require_auth();
        assert!(stop_time > start_time, "stop_time must be after start_time");
        assert!(rate_per_second > 0,   "rate must be positive");
        assert!(sender != recipient,   "sender cannot equal recipient");

        let deposit = rate_per_second * (stop_time - start_time) as i128;
        token::Client::new(&env, &token).transfer(
            &sender, &env.current_contract_address(), &deposit,
        );

        let id = next_id(&env);
        save_stream(&env, id, &Stream {
            id, sender: sender.clone(), recipient: recipient.clone(),
            token, rate_per_second, start_time, stop_time,
            withdrawn: 0, cancelled: false,
        });
        set_next_id(&env, id + 1);
        events::created(&env, id, &sender, &recipient, deposit);
        id
    }

    /// Withdraw accrued balance. Callable only by recipient.
    pub fn withdraw(env: Env, stream_id: u64) -> i128 {
        let mut stream = load_stream(&env, stream_id);
        stream.recipient.require_auth();
        assert!(!stream.cancelled, "stream is cancelled");
        let amount = Self::_delta(&env, &stream);
        assert!(amount > 0, "nothing to withdraw");
        stream.withdrawn += amount;
        save_stream(&env, stream_id, &stream);
        token::Client::new(&env, &stream.token).transfer(
            &env.current_contract_address(), &stream.recipient, &amount,
        );
        events::withdrawn(&env, stream_id, &stream.recipient, amount);
        amount
    }

    /// Cancel stream. Sends accrued amount to recipient, refunds remainder to sender.
    pub fn cancel(env: Env, stream_id: u64) -> (i128, i128) {
        let mut stream = load_stream(&env, stream_id);
        stream.sender.require_auth();
        assert!(!stream.cancelled, "already cancelled");
        let to_recipient = Self::_delta(&env, &stream);
        let total     = stream.rate_per_second * (stream.stop_time - stream.start_time) as i128;
        let to_sender = total - stream.withdrawn - to_recipient;
        stream.cancelled = true;
        save_stream(&env, stream_id, &stream);
        let tc = token::Client::new(&env, &stream.token);
        let ca = env.current_contract_address();
        if to_recipient > 0 { tc.transfer(&ca, &stream.recipient, &to_recipient); }
        if to_sender    > 0 { tc.transfer(&ca, &stream.sender,    &to_sender);    }
        events::cancelled(&env, stream_id, to_recipient, to_sender);
        (to_recipient, to_sender)
    }

    pub fn balance_of(env: Env, stream_id: u64) -> i128 {
        let stream = load_stream(&env, stream_id);
        if stream.cancelled { return 0; }
        Self::_delta(&env, &stream)
    }

    pub fn get_stream(env: Env, stream_id: u64) -> Stream {
        load_stream(&env, stream_id)
    }

    /// Returns the next stream ID (equals the number of streams ever created).
    pub fn next_stream_id(env: Env) -> u64 {
        next_id(&env)
    }

    fn _delta(env: &Env, s: &Stream) -> i128 {
        let now = env.ledger().timestamp();
        if now <= s.start_time { return 0; }
        // Cap at stop_time so balance never exceeds total deposit
        let elapsed = now.min(s.stop_time) - s.start_time;
        (s.rate_per_second * elapsed as i128).saturating_sub(s.withdrawn)
    }
}

#[cfg(test)]
mod test;
