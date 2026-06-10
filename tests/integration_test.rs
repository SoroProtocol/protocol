//! Integration tests — verifies full stream and vesting lifecycles.

#[cfg(test)]
mod integration {
    use soroban_sdk::{testutils::{Address as _, Ledger}, token, Address, Env};

    #[test]
    fn test_stream_full_lifecycle() {
        use soroban_stream::StreamContract;

        let env       = Env::default();
        env.mock_all_auths();
        let admin     = Address::generate(&env);
        let sender    = Address::generate(&env);
        let recipient = Address::generate(&env);
        let tok = env.register_stellar_asset_contract_v2(admin).address();
        token::StellarAssetClient::new(&env, &tok).mint(&sender, &10_000_000_i128);

        let cid    = env.register_contract(None, StreamContract);
        let client = soroban_stream::StreamContractClient::new(&env, &cid);

        // create stream: 100 stroops/sec for 1000 seconds = 100_000 deposit
        env.ledger().set_timestamp(0);
        let id = client.create(&sender, &recipient, &tok, &100, &0, &1000);
        assert_eq!(id, 0);
        assert_eq!(client.balance_of(&id), 0);

        // partial withdraw at t=300 → 30_000
        env.ledger().set_timestamp(300);
        assert_eq!(client.balance_of(&id), 30_000);
        let withdrawn = client.withdraw(&id);
        assert_eq!(withdrawn, 30_000);
        assert_eq!(client.balance_of(&id), 0);

        // cancel at t=500 → 50_000 accrued since last withdraw, 20_000 remaining to sender
        env.ledger().set_timestamp(500);
        let (to_rec, to_send) = client.cancel(&id);
        assert_eq!(to_rec,  20_000); // 500-300=200 seconds * 100
        assert_eq!(to_send, 50_000); // 1000-500=500 seconds * 100
        assert_eq!(client.balance_of(&id), 0);
    }

    #[test]
    fn test_vesting_cliff_then_stream() {
        use soroban_vesting::VestingContract;

        let env  = Env::default();
        env.mock_all_auths();
        let fund = Address::generate(&env);
        let bene = Address::generate(&env);
        let tok  = env.register_stellar_asset_contract_v2(fund.clone()).address();
        token::StellarAssetClient::new(&env, &tok).mint(&fund, &10_000_i128);

        let vcid   = env.register_contract(None, VestingContract);
        let vclient = soroban_vesting::VestingContractClient::new(&env, &vcid);

        // create: 1000 tokens, cliff at 500, fully vested at 1000
        env.ledger().set_timestamp(0);
        let id = vclient.create(&fund, &bene, &tok, &1000, &0, &500, &1000);

        // before cliff: nothing vested
        env.ledger().set_timestamp(300);
        assert_eq!(vclient.vested_of(&id), 0);

        // after cliff, halfway through: 500 vested
        env.ledger().set_timestamp(750);
        assert_eq!(vclient.vested_of(&id), 500);
        let claimed = vclient.claim(&id);
        assert_eq!(claimed, 500);

        // at end: remaining 500 claimable
        env.ledger().set_timestamp(1000);
        assert_eq!(vclient.vested_of(&id), 1000);
        let claimed2 = vclient.claim(&id);
        assert_eq!(claimed2, 500);
    }
}
