#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

fn field(key: Symbol, field_type: FieldType) -> FieldDef {
    FieldDef { key, field_type }
}

fn layout(env: &Env, version: u32, fields: &[FieldDef]) -> StorageLayout {
    let mut vec = Vec::new(env);
    for f in fields {
        vec.push_back(f.clone());
    }
    StorageLayout {
        version,
        fields: vec,
    }
}

fn setup() -> (Env, StorageLayoutValidatorClient<'static>) {
    let env = Env::default();
    let id = env.register(StorageLayoutValidator, ());
    let client = StorageLayoutValidatorClient::new(&env, &id);
    (env, client)
}

#[test]
fn test_compatible_addition() {
    let (env, client) = setup();
    let current = layout(
        &env,
        1,
        &[field(symbol_short!("admin"), FieldType::Address)],
    );
    let next = layout(
        &env,
        2,
        &[
            field(symbol_short!("admin"), FieldType::Address),
            field(symbol_short!("supply"), FieldType::U32),
        ],
    );

    let report = client.validate(&current, &next);
    assert!(report.compatible);
    assert!(report.collisions.is_empty());
}

#[test]
fn test_incompatible_type_change() {
    let (env, client) = setup();
    let current = layout(&env, 1, &[field(symbol_short!("balance"), FieldType::U32)]);
    let next = layout(&env, 2, &[field(symbol_short!("balance"), FieldType::I128)]);

    let report = client.validate(&current, &next);
    assert!(!report.compatible);
}

#[test]
fn test_key_collision_detection() {
    let (env, client) = setup();
    let broken = layout(
        &env,
        1,
        &[
            field(symbol_short!("fee"), FieldType::U32),
            field(symbol_short!("fee"), FieldType::U32),
        ],
    );
    let collisions = client.find_collisions(&broken);
    assert_eq!(collisions.len(), 1);
}

#[test]
fn test_remove_field_is_incompatible() {
    let (env, client) = setup();
    let current = layout(
        &env,
        1,
        &[
            field(symbol_short!("admin"), FieldType::Address),
            field(symbol_short!("paused"), FieldType::U32),
        ],
    );
    let next = layout(
        &env,
        2,
        &[field(symbol_short!("admin"), FieldType::Address)],
    );
    let report = client.validate(&current, &next);
    assert!(!report.compatible);
}

#[test]
fn test_malformed_layout_rejected() {
    let (env, client) = setup();
    let bad = StorageLayout {
        version: 0,
        fields: Vec::new(&env),
    };
    let good = layout(
        &env,
        1,
        &[field(symbol_short!("admin"), FieldType::Address)],
    );
    assert_eq!(
        client.try_validate(&bad, &good),
        Err(Ok(LayoutError::MalformedLayout))
    );
}
