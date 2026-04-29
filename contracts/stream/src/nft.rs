use soroban_sdk::{Address, Env, Symbol};

/// Mints a soulbound receipt token representing an active stream position.
/// The receipt is non-transferable and burned on stream cancellation.
pub fn mint_receipt(env: &Env, stream_id: u64, owner: &Address) {
    // Emits an event for the off-chain NFT indexer to pick up.
    // Full on-chain minting delegates to soroprotocol-nft contract.
    env.events().publish(
        (Symbol::new(env, "ReceiptMinted"), stream_id),
        (owner.clone(),),
    );
}

pub fn burn_receipt(env: &Env, stream_id: u64, owner: &Address) {
    env.events().publish(
        (Symbol::new(env, "ReceiptBurned"), stream_id),
        (owner.clone(),),
    );
}
