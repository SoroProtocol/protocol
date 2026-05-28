#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, token, Address, Env};

fn mk(total: i128) -> (Env, Address, Address, Address) {
    let env  = Env::default();
    env.mock_all_auths();
    let fund = Address::generate(&env);
    let bene = Address::generate(&env);
    let tok  = env.register_stellar_asset_contract_v2(fund.clone()).address();
    token::StellarAssetClient::new(&env, &tok).mint(&fund, &(total * 10));
    (env, fund, bene, tok)
}

#[test]
fn test_cliff_blocks_early_claim() {
    let (env, fund, bene, tok) = mk(1000);
    let cid    = env.register_contract(None, VestingContract);
    let client = VestingContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    let id = client.create(&fund, &bene, &tok, &1000, &0, &500, &1000);
    env.ledger().set_timestamp(300);
    assert_eq!(client.vested_of(&id), 0);
    env.ledger().set_timestamp(750);
    assert_eq!(client.vested_of(&id), 500); // 50% through cliff→end window
    env.ledger().set_timestamp(1000);
    assert_eq!(client.vested_of(&id), 1000);
}

#[test]
fn test_partial_then_full_claim() {
    let (env, fund, bene, tok) = mk(1000);
    let cid    = env.register_contract(None, VestingContract);
    let client = VestingContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    let id = client.create(&fund, &bene, &tok, &1000, &0, &0, &1000);
    env.ledger().set_timestamp(400);
    assert_eq!(client.claim(&id), 400);
    env.ledger().set_timestamp(700);
    assert_eq!(client.claim(&id), 300);
    env.ledger().set_timestamp(1000);
    assert_eq!(client.claim(&id), 300);
}

#[test]
#[should_panic(expected = "schedule has been revoked")]
fn test_claim_after_revoke_panics() {
    let (env, fund, bene, tok) = mk(1000);
    let cid    = env.register_contract(None, VestingContract);
    let client = VestingContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    let id = client.create(&fund, &bene, &tok, &1000, &0, &0, &1000);
    env.ledger().set_timestamp(500);
    client.revoke(&id, &fund);
    // claim on a revoked schedule must panic
    client.claim(&id);
}

#[test]
fn test_revoke_correct_split() {
    let (env, fund, bene, tok) = mk(1000);
    let cid    = env.register_contract(None, VestingContract);
    let client = VestingContractClient::new(&env, &cid);
    env.ledger().set_timestamp(0);
    // 1000 tokens, no cliff, vests 0→1000 over 0→1000
    let id = client.create(&fund, &bene, &tok, &1000, &0, &0, &1000);
    // revoke at t=300: 300 vested (goes to beneficiary), 700 unvested (goes back to funder)
    env.ledger().set_timestamp(300);
    let refund = client.revoke(&id, &fund);
    assert_eq!(refund, 700);
    let vs = client.get_schedule(&id);
    assert!(vs.revoked);
    assert_eq!(vs.claimed, 300);
}

#[test]
#[should_panic(expected = "cliff must be >= start")]
fn test_cliff_before_start_panics() {
    let (env, fund, bene, tok) = mk(1000);
    let cid    = env.register_contract(None, VestingContract);
    let client = VestingContractClient::new(&env, &cid);
    client.create(&fund, &bene, &tok, &1000, &500, &100, &1000);
}
