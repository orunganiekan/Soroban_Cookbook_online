//! Declarative storage layout compatibility checking for Soroban upgrades.
//!
//! Layouts are **schemas you declare in code** — this example does not introspect
//! arbitrary deployed contract storage at runtime.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Env, Symbol, Vec};

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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldDef {
    pub key: Symbol,
    pub field_type: FieldType,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLayout {
    pub version: u32,
    pub fields: Vec<FieldDef>,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MigrationOp {
    Retain = 1,
    Add = 2,
    TypeChange = 3,
    Remove = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationStep {
    pub key: Symbol,
    pub op: MigrationOp,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReport {
    pub compatible: bool,
    pub collisions: Vec<Symbol>,
    pub steps: Vec<MigrationStep>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LayoutError {
    MalformedLayout = 1,
}

#[contract]
pub struct StorageLayoutValidator;

#[contractimpl]
impl StorageLayoutValidator {
    /// Validate `next` against `current` and return a migration plan.
    pub fn validate(
        env: Env,
        current: StorageLayout,
        next: StorageLayout,
    ) -> Result<ValidationReport, LayoutError> {
        ensure_well_formed(&env, &current)?;
        ensure_well_formed(&env, &next)?;

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

fn ensure_well_formed(_env: &Env, layout: &StorageLayout) -> Result<(), LayoutError> {
    if layout.version == 0 {
        return Err(LayoutError::MalformedLayout);
    }
    Ok(())
}

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

fn field_type_for_key(layout: &StorageLayout, key: &Symbol) -> Option<FieldType> {
    for field in layout.fields.iter() {
        if field.key == *key {
            return Some(field.field_type);
        }
    }
    None
}

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
