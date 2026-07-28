#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

fn setup() -> (Env, WeatherNftClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let id = env.register(WeatherNft, ());
    let client = WeatherNftClient::new(&env, &id);
    client.initialize(&admin, &oracle);
    (env, client, admin, oracle)
}

#[test]
fn test_default_weather_is_sunny_bright() {
    let (env, client, admin, _oracle) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &7);
    assert_eq!(client.current_weather(), Weather::Sunny);
    assert_eq!(client.appearance(&7), symbol_short!("bright"));
}

#[test]
fn test_appearance_changes_with_each_weather() {
    let (env, client, admin, oracle) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &7);

    client.set_weather(&oracle, &Weather::Rain);
    assert_eq!(client.appearance(&7), symbol_short!("misty"));

    client.set_weather(&oracle, &Weather::Storm);
    assert_eq!(client.appearance(&7), symbol_short!("charged"));

    client.set_weather(&oracle, &Weather::Sunny);
    assert_eq!(client.appearance(&7), symbol_short!("bright"));
}

#[test]
fn test_only_oracle_can_update_weather() {
    let (env, client, _admin, oracle) = setup();
    let impostor = Address::generate(&env);
    assert_eq!(
        client.try_set_weather(&impostor, &Weather::Rain),
        Err(Ok(WeatherNftError::NotOracle))
    );
    // The real oracle succeeds and the change is persisted.
    client.set_weather(&oracle, &Weather::Rain);
    assert_eq!(client.current_weather(), Weather::Rain);
}

#[test]
fn test_only_admin_can_mint() {
    let (env, client, _admin, _oracle) = setup();
    let stranger = Address::generate(&env);
    let owner = Address::generate(&env);
    assert_eq!(
        client.try_mint(&stranger, &owner, &1),
        Err(Ok(WeatherNftError::NotAdmin))
    );
}

#[test]
fn test_duplicate_mint_rejected() {
    let (env, client, admin, _oracle) = setup();
    let owner = Address::generate(&env);
    client.mint(&admin, &owner, &1);
    assert_eq!(
        client.try_mint(&admin, &owner, &1),
        Err(Ok(WeatherNftError::TokenAlreadyExists))
    );
}

#[test]
fn test_appearance_unknown_token_errors() {
    let (_env, client, _admin, _oracle) = setup();
    assert_eq!(
        client.try_appearance(&999),
        Err(Ok(WeatherNftError::TokenNotFound))
    );
}
