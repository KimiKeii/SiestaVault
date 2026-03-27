#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, Symbol};

#[test]
fn test_happy_path_minting() {
    let env = Env::default();
    let contract_id = env.register(SiestaVault, ());
    let client = SiestaVaultClient::new(&env, &contract_id);

    let artist = Address::generate(&env);
    let buyer = Address::generate(&env);
    let art_hash = Symbol::new(&env, "art_123_hash");

    client.mint_and_buy(&artist, &buyer, &art_hash, &100, &10);

    // Verify storage reflects the new owner
    assert_eq!(client.get_owner(&art_hash), buyer);
}

#[test]
#[should_panic(expected = "This artwork hash has already been minted.")]
fn test_duplicate_registration_fails() {
    let env = Env::default();
    let contract_id = env.register(SiestaVault, ());
    let client = SiestaVaultClient::new(&env, &contract_id);

    let artist = Address::generate(&env);
    let buyer = Address::generate(&env);
    let art_hash = Symbol::new(&env, "unique_art");

    client.mint_and_buy(&artist, &buyer, &art_hash, &100, &10);
    // Attempting to mint the same hash again
    client.mint_and_buy(&artist, &buyer, &art_hash, &100, &10);
}

#[test]
fn test_state_verification_royalty() {
    let env = Env::default();
    let contract_id = env.register(SiestaVault, ());
    let client = SiestaVaultClient::new(&env, &contract_id);

    let artist = Address::generate(&env);
    let buyer = Address::generate(&env);
    let art_hash = Symbol::new(&env, "art_456");
    let royalty_val = 15;

    client.mint_and_buy(&artist, &buyer, &art_hash, &100, &royalty_val);
    
    // Verify royalty state is stored correctly
    assert_eq!(client.get_royalty(&art_hash), royalty_val);
}