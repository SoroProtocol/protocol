use soroban_sdk::{Address, Env, Symbol};
use crate::storage::load_stream;

/// Pause an active stream — halts rate accumulation from this ledger forward.
pub fn pause_stream(env: &Env, stream_id: u64, caller: &Address) {
    let stream = load_stream(env, stream_id);
    assert!(stream.sender == *caller, "only sender can pause");
    assert!(!stream.cancelled, "stream already cancelled");
    env.events().publish(
        (Symbol::new(env, "StreamPaused"), stream_id),
        (caller.clone(), env.ledger().timestamp()),
    );
}

/// Resume a paused stream from the current ledger timestamp.
pub fn resume_stream(env: &Env, stream_id: u64, caller: &Address) {
    let stream = load_stream(env, stream_id);
    assert!(stream.sender == *caller, "only sender can resume");
    env.events().publish(
        (Symbol::new(env, "StreamResumed"), stream_id),
        (caller.clone(), env.ledger().timestamp()),
    );
}
