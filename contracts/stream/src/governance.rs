use soroban_sdk::{Address, Env, Symbol};

const KEY_FEE_BPS:       &str = "fee_bps";
const KEY_FEE_RECIPIENT: &str = "fee_recip";
const MAX_FEE_BPS:       u32  = 500; // 5%

/// Protocol fee in basis points (default: 30 = 0.3%).
pub fn fee_bps(env: &Env) -> u32 {
    env.storage().instance()
        .get(&Symbol::new(env, KEY_FEE_BPS))
        .unwrap_or(30u32)
}

/// Update protocol fee. Requires multisig admin auth.
pub fn set_fee_bps(env: &Env, admin: &Address, new_fee: u32) {
    admin.require_auth();
    assert!(new_fee <= MAX_FEE_BPS, "fee exceeds 5% maximum");
    env.storage().instance().set(&Symbol::new(env, KEY_FEE_BPS), &new_fee);
    env.events().publish(
        (Symbol::new(env, "FeeUpdated"),),
        (new_fee,),
    );
}

/// Set the address that receives protocol fees.
pub fn set_fee_recipient(env: &Env, admin: &Address, recipient: &Address) {
    admin.require_auth();
    env.storage().instance().set(&Symbol::new(env, KEY_FEE_RECIPIENT), recipient);
}
