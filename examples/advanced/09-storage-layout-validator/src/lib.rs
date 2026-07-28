//! Declarative storage layout compatibility checking for Soroban upgrades.
//!
//! Layouts are **schemas you declare in code** — this example does not introspect
//! arbitrary deployed contract storage at runtime. You describe the `(key, type)`
//! pairs your contract keeps in storage (the same information you would encode in a
//! `DataKey` enum) and the validator reports whether upgrading from `current` to
//! `next` is safe, along with a step-by-step migration plan.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Env, Symbol, Vec};

/// Supported storage field types.
///
/// Kept intentionally small; extend this enum as your contract needs more types.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FieldType {
    U32 = 1,
    I128 = 2,
    Address = 3,
    Symbol = 4,
    Bytes = 5,
}

/// A single declared storage field: a key and its type.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDef {
    pub key: Symbol,
    pub field_type: FieldType,
}

/// A versioned storage layout schema.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLayout {
    pub version: u32,
    pub fields: Vec<FieldDef>,
}

/// The operation required to migrate a single key from `current` to `next`.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MigrationOp {
    /// Field is unchanged (same key, same type).
    Retain = 1,
    /// Field is new in `next`.
    Add = 2,
    /// Field exists in both but the type changed (breaking).
    TypeChange = 3,
    /// Field was removed in `next` (breaking).
    Remove = 4,
}

/// One entry in the migration plan.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationStep {
    pub key: Symbol,
    pub op: MigrationOp,
}

/// The result of validating `next` against `current`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    /// `true` when the upgrade introduces no breaking changes.
    pub compatible: bool,
    /// Duplicate keys detected within `next` (empty when none).
    pub collisions: Vec<Symbol>,
    /// The ordered migration plan (empty when a collision blocked validation).
    pub steps: Vec<MigrationStep>,
}

/// Errors returned by the validator.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LayoutError {
    /// A layout was malformed (for example, `version == 0`).
    MalformedLayout = 1,
}

#[contract]
pub struct StorageLayoutValidator;

#[contractimpl]
impl StorageLayoutValidator {
    /// Validate `next` against `current` and return a migration plan.
    ///
    /// A layout with duplicate keys is reported as incompatible with the offending
    /// keys listed in `collisions`. Otherwise the report lists a [`MigrationStep`]
    /// per key and is `compatible` only when every step is [`MigrationOp::Retain`]
    /// or [`MigrationOp::Add`].
    pub fn validate(
        env: Env,
        current: StorageLayout,
        next: StorageLayout,
    ) -> Result<ValidationReport, LayoutError> {
        ensure_well_formed(&current)?;
        ensure_well_formed(&next)?;

        let collisions = find_duplicate_keys(&env, &next);
        if !collisions.is_empty() {
            return Ok(ValidationReport {
                compatible: false,
                collisions,
                steps: Vec::new(&env),
            });
        }

        let steps = build_migration_plan(&env, &current, &next);
        let compatible = steps
            .iter()
            .all(|step| step.op != MigrationOp::TypeChange && step.op != MigrationOp::Remove);

        Ok(ValidationReport {
            compatible,
            collisions: Vec::new(&env),
            steps,
        })
    }

    /// Detect duplicate keys within a single declared layout.
    pub fn find_collisions(env: Env, layout: StorageLayout) -> Vec<Symbol> {
        find_duplicate_keys(&env, &layout)
    }
}

/// Reject layouts that are structurally invalid before comparing them.
fn ensure_well_formed(layout: &StorageLayout) -> Result<(), LayoutError> {
    if layout.version == 0 {
        return Err(LayoutError::MalformedLayout);
    }
    Ok(())
}

/// Return the list of keys that appear more than once in `layout`.
fn find_duplicate_keys(env: &Env, layout: &StorageLayout) -> Vec<Symbol> {
    let mut seen = Vec::new(env);
    let mut duplicates = Vec::new(env);

    for field in layout.fields.iter() {
        if contains_symbol(&seen, &field.key) {
            if !contains_symbol(&duplicates, &field.key) {
                duplicates.push_back(field.key.clone());
            }
        } else {
            seen.push_back(field.key.clone());
        }
    }
    duplicates
}

fn contains_symbol(haystack: &Vec<Symbol>, needle: &Symbol) -> bool {
    for item in haystack.iter() {
        if item == *needle {
            return true;
        }
    }
    false
}

/// Look up the type declared for `key` in `layout`, if present.
fn field_type_for_key(layout: &StorageLayout, key: &Symbol) -> Option<FieldType> {
    for field in layout.fields.iter() {
        if field.key == *key {
            return Some(field.field_type);
        }
    }
    None
}

/// Build the ordered migration plan comparing `current` to `next`.
fn build_migration_plan(
    env: &Env,
    current: &StorageLayout,
    next: &StorageLayout,
) -> Vec<MigrationStep> {
    let mut steps = Vec::new(env);

    for field in current.fields.iter() {
        match field_type_for_key(next, &field.key) {
            None => steps.push_back(MigrationStep {
                key: field.key.clone(),
                op: MigrationOp::Remove,
            }),
            Some(new_type) if new_type != field.field_type => steps.push_back(MigrationStep {
                key: field.key.clone(),
                op: MigrationOp::TypeChange,
            }),
            Some(_) => steps.push_back(MigrationStep {
                key: field.key.clone(),
                op: MigrationOp::Retain,
            }),
        }
    }

    for field in next.fields.iter() {
        if field_type_for_key(current, &field.key).is_none() {
            steps.push_back(MigrationStep {
                key: field.key.clone(),
                op: MigrationOp::Add,
            });
        }
    }

    steps
}

#[cfg(test)]
mod test;
