use soroban_sdk::{Address, Env, Symbol};

pub fn created(env: &Env, id: u64, sender: &Address, recipient: &Address, deposit: i128) {
    env.events().publish(
        (Symbol::new(env, "StreamCreated"), id),
        (sender.clone(), recipient.clone(), deposit),
    );
}

pub fn withdrawn(env: &Env, id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "Withdrawn"), id),
        (recipient.clone(), amount),
    );
}

pub fn cancelled(env: &Env, id: u64, to_recipient: i128, to_sender: i128) {
    env.events().publish(
        (Symbol::new(env, "Cancelled"), id),
        (to_recipient, to_sender),
    );
}
