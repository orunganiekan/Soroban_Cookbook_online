#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

fn setup() -> (Env, LevelUpNftClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(LevelUpNft, ());
    let client = LevelUpNftClient::new(&env, &id);
    client.initialize(&admin);
    (env, client, admin)
}

#[test]
fn test_mint_sets_initial_state_and_trait() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);

    let state = client.state(&1);
    assert_eq!(state.level, 1);
    assert_eq!(state.xp, 0);
    assert_eq!(state.power, 10);
    assert_eq!(client.trait_label(&1), symbol_short!("novice"));
}

#[test]
fn test_only_admin_can_mint() {
    let (env, client, _admin) = setup();
    let stranger = Address::generate(&env);
    let owner = Address::generate(&env);
    assert_eq!(
        client.try_mint(&stranger, &owner, &1),
        Err(Ok(LevelUpError::NotAdmin))
    );
}

#[test]
fn test_duplicate_mint_rejected() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);
    assert_eq!(
        client.try_mint(&admin, &owner, &1),
        Err(Ok(LevelUpError::TokenAlreadyExists))
    );
}

#[test]
fn test_double_initialize_rejected() {
    let (_env, client, admin) = setup();
    assert_eq!(
        client.try_initialize(&admin),
        Err(Ok(LevelUpError::AlreadyInitialized))
    );
}

#[test]
fn test_non_owner_cannot_train() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);
    client.mint(&admin, &owner, &1);
    assert_eq!(
        client.try_train(&stranger, &1, &10),
        Err(Ok(LevelUpError::NotOwner))
    );
}

#[test]
fn test_training_accumulates_xp_without_levelup() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);

    let state = client.train(&owner, &1, &40);
    assert_eq!(state.level, 1);
    assert_eq!(state.xp, 40);
    assert_eq!(state.power, 10);
    assert_eq!(client.trait_label(&1), symbol_short!("novice"));
}

#[test]
fn test_single_levelup_at_threshold() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);

    // Exactly one level threshold: level 1 -> 2, XP resets to 0, +5 power.
    let state = client.train(&owner, &1, &100);
    assert_eq!(state.level, 2);
    assert_eq!(state.xp, 0);
    assert_eq!(state.power, 15);
    assert_eq!(client.trait_label(&1), symbol_short!("adept"));
}

#[test]
fn test_multi_level_progression_and_labels() {
    let (env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);

    // 250 XP -> two full levels (+ carry 50): level 3, xp 50, power 20.
    let state = client.train(&owner, &1, &250);
    assert_eq!(state.level, 3);
    assert_eq!(state.xp, 50);
    assert_eq!(state.power, 20);
    assert_eq!(client.trait_label(&1), symbol_short!("veteran"));

    // Push beyond level 3 to reach the `legend` label.
    let state = client.train(&owner, &1, &150);
    assert_eq!(state.level, 5);
    assert_eq!(client.trait_label(&1), symbol_short!("legend"));
}
