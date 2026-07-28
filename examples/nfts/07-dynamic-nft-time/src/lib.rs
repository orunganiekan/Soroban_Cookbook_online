//! Dynamic NFT whose life stage advances with ledger time.
//!
//! Each token records its mint timestamp. `life_stage` derives the current stage from
//! the elapsed ledger time (`env.ledger().timestamp()` minus the mint time), so the
//! token "grows" purely as a function of on-chain time — no owner action required.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// Seconds per evolution stage (1 day in this teaching example).
pub const STAGE_DURATION_SECS: u64 = 86_400;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Owner(u32),
    MintedAt(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TimeNftError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAdmin = 3,
    TokenNotFound = 4,
    TokenAlreadyExists = 5,
}

#[contract]
pub struct TimeEvolutionNft;

#[contractimpl]
impl TimeEvolutionNft {
    /// Set the collection admin. Callable once.
    pub fn initialize(env: Env, admin: Address) -> Result<(), TimeNftError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TimeNftError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Mint a token to `to`, recording the current ledger timestamp. Admin-only.
    pub fn mint(env: Env, admin: Address, to: Address, token_id: u32) -> Result<(), TimeNftError> {
        admin.require_auth();
        if read_admin(&env)? != admin {
            return Err(TimeNftError::NotAdmin);
        }
        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(TimeNftError::TokenAlreadyExists);
        }
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
        env.storage()
            .persistent()
            .set(&DataKey::MintedAt(token_id), &env.ledger().timestamp());
        env.events().publish(
            (symbol_short!("mint"), symbol_short!("time")),
            (to, token_id),
        );
        Ok(())
    }

    /// Derive the token's life stage from elapsed ledger time since mint.
    ///
    /// Stages advance every [`STAGE_DURATION_SECS`]:
    /// `seed` (stage 0) → `sprout` (1) → `bloom` (2) → `ancient` (3+).
    pub fn life_stage(env: Env, token_id: u32) -> Result<Symbol, TimeNftError> {
        let minted_at = Self::minted_at(env.clone(), token_id)?;
        let age = env.ledger().timestamp().saturating_sub(minted_at);
        let stage = age / STAGE_DURATION_SECS;
        Ok(match stage {
            0 => symbol_short!("seed"),
            1 => symbol_short!("sprout"),
            2 => symbol_short!("bloom"),
            _ => symbol_short!("ancient"),
        })
    }

    /// Read the mint timestamp for a token.
    pub fn minted_at(env: Env, token_id: u32) -> Result<u64, TimeNftError> {
        let _ = read_owner(&env, token_id)?;
        env.storage()
            .persistent()
            .get(&DataKey::MintedAt(token_id))
            .ok_or(TimeNftError::TokenNotFound)
    }
}

fn read_admin(env: &Env) -> Result<Address, TimeNftError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(TimeNftError::NotInitialized)
}

fn read_owner(env: &Env, token_id: u32) -> Result<Address, TimeNftError> {
    env.storage()
        .persistent()
        .get(&DataKey::Owner(token_id))
        .ok_or(TimeNftError::TokenNotFound)
}

#[cfg(test)]
mod test;
