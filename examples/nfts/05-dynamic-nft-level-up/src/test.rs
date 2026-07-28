#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

fn setup() -> (Env, LevelUpNftClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(LevelUpNft, ());
    let client = LevelUpNftClient::new(&env, &id);
    client.try_initialize(&admin).unwrap();
    (env, client, admin)
}

#[test]
fn test_level_increases_after_enough_xp() {
    let (_env, client, admin) = setup();
    let owner = Address::generate(&_env);
    client.try_mint(&admin, &owner, &1).unwrap();

    let state = client.train(&owner, &1, &250);
    assert_eq!(state.level, 3);
    assert_eq!(state.xp, 50);
    assert_eq!(state.power, 20);
    assert_eq!(client.trait_label(&1), symbol_short!("veteran"));
}

#[test]
fn test_non_owner_cannot_train() {
    let (_env, client, admin) = setup();
    let owner = Address::generate(&_env);
    let stranger = Address::generate(&_env);
    client.try_mint(&admin, &owner, &1).unwrap();
    assert_eq!(
        client.try_train(&stranger, &1, &10),
        Err(Ok(LevelUpError::NotOwner))
    );
}
