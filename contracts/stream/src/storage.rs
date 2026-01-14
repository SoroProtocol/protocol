use soroban_sdk::{Env, Symbol};
use crate::types::Stream;

const KEY_NEXT_ID: &str = "next_id";

pub fn next_id(env: &Env) -> u64 {
    env.storage().instance().get(&Symbol::new(env, KEY_NEXT_ID)).unwrap_or(0u64)
}

pub fn set_next_id(env: &Env, id: u64) {
    env.storage().instance().set(&Symbol::new(env, KEY_NEXT_ID), &id);
}

pub fn load_stream(env: &Env, id: u64) -> Stream {
    env.storage()
        .persistent()
        .get::<u64, Stream>(&id)
        .expect("stream not found")
}

pub fn save_stream(env: &Env, id: u64, stream: &Stream) {
    env.storage().persistent().set(&id, stream);
}
