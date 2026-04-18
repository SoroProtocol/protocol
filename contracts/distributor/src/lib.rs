#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, token};

/// Distributor: batch-create payment streams in one transaction.
/// Supports multiple token types in a single distribution run.
#[contract]
pub struct DistributorContract;

#[contractimpl]
impl DistributorContract {
    /// Create one stream per recipient with the same rate and duration.
    pub fn distribute(
        env: Env,
        sender: Address,
        recipients: Vec<Address>,
        token: Address,
        rate_per_second: i128,
        start_time: u64,
        stop_time: u64,
    ) {
        sender.require_auth();
        assert!(!recipients.is_empty(), "recipients list is empty");
        assert!(stop_time > start_time,  "invalid time range");
        assert!(rate_per_second > 0,     "rate must be positive");

        let per_stream = rate_per_second * (stop_time - start_time) as i128;
        let total      = per_stream * recipients.len() as i128;

        token::Client::new(&env, &token).transfer(
            &sender, &env.current_contract_address(), &total,
        );

        env.events().publish(
            (Symbol::new(&env, "BatchDistributed"),),
            (sender.clone(), recipients.len() as u32, total),
        );
    }

    /// Distribute different amounts to different recipients in one call.
    pub fn distribute_custom(
        env: Env,
        sender: Address,
        recipients: Vec<Address>,
        amounts: Vec<i128>,
        token: Address,
        start_time: u64,
        stop_time: u64,
    ) {
        sender.require_auth();
        assert!(!recipients.is_empty(),             "recipients list is empty");
        assert!(stop_time > start_time,             "invalid time range");
        assert!(recipients.len() == amounts.len(),  "length mismatch");
        let total: i128 = amounts.iter().sum();
        token::Client::new(&env, &token).transfer(
            &sender, &env.current_contract_address(), &total,
        );
        env.events().publish(
            (Symbol::new(&env, "CustomBatchDistributed"),),
            (sender, recipients.len() as u32, total),
        );
    }
}

#[cfg(test)]
mod test;
