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
    let s = client.get_stream(&0);
    assert_eq!(s.rate_per_second, 100);
    assert!(!s.cancelled);
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
    assert_eq!(amount, 30_000); // 300s * 100
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
#[should_panic(expected = "stop_time must be after start_time")]
fn test_invalid_time_range() {
    let (env, sender, recipient, tok) = mk_env();
    let cid    = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &cid);
    client.create(&sender, &recipient, &tok, &100, &500, &100);
}
