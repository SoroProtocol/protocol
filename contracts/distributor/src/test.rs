#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env, Vec};

fn mk_env() -> (Env, Address, Address, Address) {
    let env   = Env::default();
    env.mock_all_auths();
    let admin  = Address::generate(&env);
    let sender = Address::generate(&env);
    let tok    = env.register_stellar_asset_contract_v2(admin.clone()).address();
    token::StellarAssetClient::new(&env, &tok).mint(&sender, &10_000_000_i128);
    (env, admin, sender, tok)
}

#[test]
#[should_panic(expected = "recipients list is empty")]
fn test_distribute_empty_recipients_panics() {
    let (env, _admin, sender, tok) = mk_env();
    let cid    = env.register_contract(None, DistributorContract);
    let client = DistributorContractClient::new(&env, &cid);
    let empty: Vec<Address> = Vec::new(&env);
    client.distribute(&sender, &empty, &tok, &100, &0, &1000);
}

#[test]
#[should_panic(expected = "rate must be positive")]
fn test_distribute_zero_rate_panics() {
    let (env, _admin, sender, tok) = mk_env();
    let cid    = env.register_contract(None, DistributorContract);
    let client = DistributorContractClient::new(&env, &cid);
    let mut recipients = Vec::new(&env);
    recipients.push_back(Address::generate(&env));
    client.distribute(&sender, &recipients, &tok, &0, &0, &1000);
}

#[test]
#[should_panic(expected = "invalid time range")]
fn test_distribute_invalid_time_range_panics() {
    let (env, _admin, sender, tok) = mk_env();
    let cid    = env.register_contract(None, DistributorContract);
    let client = DistributorContractClient::new(&env, &cid);
    let mut recipients = Vec::new(&env);
    recipients.push_back(Address::generate(&env));
    client.distribute(&sender, &recipients, &tok, &100, &1000, &500);
}
