#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Ledger as _},
    Address, Env,
};

fn setup() -> (Env, TimeEvolutionNftClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    // Start at a known ledger time so `minted_at` is deterministic (0 here).
    env.ledger().set_timestamp(0);
    let admin = Address::generate(&env);
    let id = env.register(TimeEvolutionNft, ());
    let client = TimeEvolutionNftClient::new(&env, &id);
    client.initialize(&admin);
    (env, client, admin)
}

fn mint_token(env: &Env, client: &TimeEvolutionNftClient<'static>, admin: &Address) -> u32 {
    let owner = Address::generate(env);
    client.mint(admin, &owner, &1);
    1
}

#[test]
fn test_initial_stage_is_seed() {
    let (env, client, admin) = setup();
    let id = mint_token(&env, &client, &admin);
    assert_eq!(client.minted_at(&id), 0);
    assert_eq!(client.life_stage(&id), symbol_short!("seed"));
}

#[test]
fn test_full_stage_progression() {
    let (env, client, admin) = setup();
    let id = mint_token(&env, &client, &admin);

    env.ledger().set_timestamp(STAGE_DURATION_SECS);
    assert_eq!(client.life_stage(&id), symbol_short!("sprout"));

    env.ledger().set_timestamp(STAGE_DURATION_SECS * 2);
    assert_eq!(client.life_stage(&id), symbol_short!("bloom"));

    env.ledger().set_timestamp(STAGE_DURATION_SECS * 3);
    assert_eq!(client.life_stage(&id), symbol_short!("ancient"));
}

#[test]
fn test_stage_boundaries() {
    let (env, client, admin) = setup();
    let id = mint_token(&env, &client, &admin);

    // Just before the first boundary: still seed.
    env.ledger().set_timestamp(STAGE_DURATION_SECS - 1);
    assert_eq!(client.life_stage(&id), symbol_short!("seed"));

    // Exactly on the boundary: advances to sprout.
    env.ledger().set_timestamp(STAGE_DURATION_SECS);
    assert_eq!(client.life_stage(&id), symbol_short!("sprout"));

    // Just before the bloom boundary: still sprout.
    env.ledger().set_timestamp(STAGE_DURATION_SECS * 2 - 1);
    assert_eq!(client.life_stage(&id), symbol_short!("sprout"));

    // Exactly on the ancient boundary.
    env.ledger().set_timestamp(STAGE_DURATION_SECS * 3);
    assert_eq!(client.life_stage(&id), symbol_short!("ancient"));

    // Far future stays ancient (saturating stage).
    env.ledger().set_timestamp(STAGE_DURATION_SECS * 100);
    assert_eq!(client.life_stage(&id), symbol_short!("ancient"));
}

#[test]
fn test_only_admin_can_mint() {
    let (env, client, _admin) = setup();
    let stranger = Address::generate(&env);
    let owner = Address::generate(&env);
    assert_eq!(
        client.try_mint(&stranger, &owner, &1),
        Err(Ok(TimeNftError::NotAdmin))
    );
}

#[test]
fn test_duplicate_mint_rejected() {
    let (env, client, admin) = setup();
    let id = mint_token(&env, &client, &admin);
    let owner = Address::generate(&env);
    assert_eq!(
        client.try_mint(&admin, &owner, &id),
        Err(Ok(TimeNftError::TokenAlreadyExists))
    );
}

#[test]
fn test_life_stage_unknown_token_errors() {
    let (_env, client, _admin) = setup();
    assert_eq!(
        client.try_life_stage(&999),
        Err(Ok(TimeNftError::TokenNotFound))
    );
}
