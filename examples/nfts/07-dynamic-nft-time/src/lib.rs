//! Dynamic NFT whose life stage advances with ledger time.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// Seconds per evolution stage in this teaching example.
pub const STAGE_DURATION_SECS: u64 = 86_400;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Owner(u32),
    MintedAt(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TimeNftError {
    NotInitialized = 1,
    TokenNotFound = 2,
    Unauthorized = 3,
}

#[contract]
pub struct TimeEvolutionNft;

#[contractimpl]
impl TimeEvolutionNft {
    pub fn initialize(env: Env, admin: Address) -> Result<(), TimeNftError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TimeNftError::NotInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn mint(env: Env, admin: Address, to: Address, token_id: u32) -> Result<(), TimeNftError> {
        let stored_admin = read_admin(&env)?;
        if stored_admin != admin {
            return Err(TimeNftError::Unauthorized);
        }
        admin.require_auth();
        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(TimeNftError::TokenNotFound);
        }
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
        env.storage()
            .persistent()
            .set(&DataKey::MintedAt(token_id), &env.ledger().timestamp());
        Ok(())
    }

    pub fn life_stage(env: Env, token_id: u32) -> Result<Symbol, TimeNftError> {
        let _ = read_owner(&env, token_id)?;
        let minted_at: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::MintedAt(token_id))
            .unwrap_or(0);
        let age = env.ledger().timestamp().saturating_sub(minted_at);
        let stage = age / STAGE_DURATION_SECS;
        Ok(match stage {
            0 => symbol_short!("seed"),
            1 => symbol_short!("sprout"),
            2 => symbol_short!("bloom"),
            _ => symbol_short!("ancient"),
        })
    }

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
