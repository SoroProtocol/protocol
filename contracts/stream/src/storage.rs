use soroban_sdk::{Env, Symbol};
use crate::types::Stream;

const KEY_NEXT_ID: &str = "next_id";
// ~30 days at 5s/ledger; bump TTL whenever a stream is written
const STREAM_TTL_THRESHOLD: u32 = 259_200;
const STREAM_TTL_EXTEND_TO: u32 = 518_400;

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
    env.storage().persistent().extend_ttl(&id, STREAM_TTL_THRESHOLD, STREAM_TTL_EXTEND_TO);
}
