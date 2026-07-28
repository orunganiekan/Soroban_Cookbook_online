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
    client.try_initialize(&admin, &oracle).unwrap();
    (env, client, admin, oracle)
}

#[test]
fn test_appearance_changes_with_weather() {
    let (_env, client, admin, oracle) = setup();
    let owner = Address::generate(&_env);
    client.try_mint(&admin, &owner, &7).unwrap();
    assert_eq!(client.appearance(&7), symbol_short!("bright"));

    client.try_set_weather(&oracle, &Weather::Storm).unwrap();
    assert_eq!(client.appearance(&7), symbol_short!("charged"));
}

#[test]
fn test_only_oracle_updates_weather() {
    let (_env, client, _admin, oracle) = setup();
    let impostor = Address::generate(&_env);
    assert_eq!(
        client.try_set_weather(&impostor, &Weather::Rain),
        Err(Ok(WeatherNftError::Unauthorized))
    );
    client.try_set_weather(&oracle, &Weather::Rain).unwrap();
    assert_eq!(client.current_weather(), Weather::Rain);
}
