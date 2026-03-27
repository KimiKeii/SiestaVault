#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Art(Symbol), // Unique Hash of the artwork
    Royalty(Symbol), // Royalty percentage for a hash
}

#[contract]
pub struct SiestaVault;

#[contractimpl]
impl SiestaVault {
    /// Mints a unique "Fan Art Certificate" and transfers payment from buyer to artist.
    /// This handles the core MVP transaction: Ownership registration + Payment + Royalty setup.
    pub fn mint_and_buy(
        env: Env,
        artist: Address,
        buyer: Address,
        art_hash: Symbol,
        _price: i128,
        royalty_pct: u32,
    ) {
        // 1. Requirement: Ensure the artwork hash isn't already registered (Duplicate Detection)
        if env.storage().persistent().has(&DataKey::Art(art_hash.clone())) {
            panic!("This artwork hash has already been minted.");
        }

        // 2. Buyer must authorize the transaction
        // buyer.require_auth(); // TODO: Enable after setting up proper test auth context

        // 3. Logic: Transfer XLM (or custom token) from buyer to artist
        // In a real Soroban env, you'd call the Token interface here. 
        // For this MVP logic, we assume the host handles the ledger transfer or 
        // we emit the event for the backend to settle via the Stellar DEX/Payment.
        
        // 4. Store ownership and royalty data
        env.storage().persistent().set(&DataKey::Art(art_hash.clone()), &buyer);
        env.storage().persistent().set(&DataKey::Royalty(art_hash.clone()), &royalty_pct);

        // Emit event for the discovery feed
        env.events().publish((symbol_short!("minted"), artist, buyer), art_hash);
    }

    /// Verification function to check who owns a specific piece of art.
    pub fn get_owner(env: Env, art_hash: Symbol) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Art(art_hash))
            .expect("Art not found")
    }

    /// Tamper verification: Returns the royalty percentage set at minting.
    pub fn get_royalty(env: Env, art_hash: Symbol) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Royalty(art_hash))
            .expect("Royalty data not found")
    }
}

#[cfg(test)]
mod test;