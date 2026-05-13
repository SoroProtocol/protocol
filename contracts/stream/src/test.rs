#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, token, Address, Env};

fn mk_env() -> (Env, Address, Address, Address) {
    let env       = Env::default();
    env.mock_all_auths();
    let admin     = Address::generate(&env);
    let sender    = Address::generate(&env);
    let recipient = Address::generate(&env);
    let tok       = env.register_stellar_asset_contract_v2(admin.clone()).address();
    token::StellarAssetClient::new(&env, &tok).mint(&sender, &1_000_000_000_i128);
    (env, sender, recipient, tok)
}

#[test]
fn test_create_and_get() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    let id = client.create(&sender, &recipient, &tok, &100, &0, &1000);
    assert_eq!(id, 0);
    assert_eq!(client.next_stream_id(), 1);
}

#[test]
fn test_withdraw_mid_stream() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &100, &0, &1000);
    env.ledger().set_timestamp(300);
    let amount = client.withdraw(&0);
    assert_eq!(amount, 30_000);
    assert_eq!(client.balance_of(&0), 0);
}

#[test]
fn test_balance_capped_at_stop_time() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &100, &0, &500);
    // now >> stop_time: balance should be capped at 500*100
    env.ledger().set_timestamp(99999);
    assert_eq!(client.balance_of(&0), 50_000);
}

#[test]
fn test_cancel_splits_correctly() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &1000, &0, &1000);
    env.ledger().set_timestamp(200);
    let (to_rec, to_send) = client.cancel(&0);
    assert_eq!(to_rec,  200_000);
    assert_eq!(to_send, 800_000);
}

#[test]
fn test_balance_before_start_is_zero() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(500);
    client.create(&sender, &recipient, &tok, &100, &1000, &2000);
    assert_eq!(client.balance_of(&0), 0);
}

#[test]
#[should_panic]
fn test_withdraw_by_non_recipient_panics() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &100, &0, &1000);
    env.ledger().set_timestamp(500);
    // sender is not the recipient — should panic on require_auth
    let stranger = Address::generate(&env);
    env.mock_auths(&[]);  // remove blanket mock so auth is enforced
    let _ = stranger;
    // withdraw without recipient auth must fail
    client.withdraw(&0);
}

#[test]
#[should_panic(expected = "stop_time must be after start_time")]
fn test_invalid_time_range() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    client.create(&sender, &recipient, &tok, &100, &500, &100);
}

#[test]
#[should_panic]
fn test_cancel_by_non_sender_panics() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &100, &0, &1000);
    env.mock_auths(&[]);  // enforce real auth
    let _ = (sender, recipient);
    client.cancel(&0);
}

#[test]
#[should_panic(expected = "already cancelled")]
fn test_double_cancel_panics() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    client.create(&sender, &recipient, &tok, &100, &0, &1000);
    client.cancel(&0);
    client.cancel(&0);
}
