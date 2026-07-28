#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, Ledger as _};

fn setup() -> (Env, TimeEvolutionNftClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(TimeEvolutionNft, ());
    let client = TimeEvolutionNftClient::new(&env, &id);
    client.try_initialize(&admin).unwrap();
    (env, client, admin)
}

#[test]
fn test_stage_advances_with_ledger_time() {
    let (mut env, client, admin) = setup();
    let owner = Address::generate(&env);
    client.try_mint(&admin, &owner, &1).unwrap();
    assert_eq!(client.life_stage(&1), symbol_short!("seed"));

    env.ledger().set_timestamp(STAGE_DURATION_SECS + 1);
    assert_eq!(client.life_stage(&1), symbol_short!("sprout"));

    env.ledger().set_timestamp(STAGE_DURATION_SECS * 3 + 1);
    assert_eq!(client.life_stage(&1), symbol_short!("ancient"));
}
