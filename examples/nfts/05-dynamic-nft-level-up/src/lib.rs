//! Dynamic NFT whose on-chain stats level up as the owner earns experience.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeroState {
    pub level: u32,
    pub xp: u32,
    pub power: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Owner(u32),
    State(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LevelUpError {
    NotInitialized = 1,
    Unauthorized = 2,
    TokenNotFound = 3,
    NotOwner = 4,
}

const XP_PER_LEVEL: u32 = 100;

#[contract]
pub struct LevelUpNft;

#[contractimpl]
impl LevelUpNft {
    pub fn initialize(env: Env, admin: Address) -> Result<(), LevelUpError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(LevelUpError::NotInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn mint(env: Env, admin: Address, to: Address, token_id: u32) -> Result<(), LevelUpError> {
        let stored_admin = read_admin(&env)?;
        if stored_admin != admin {
            return Err(LevelUpError::Unauthorized);
        }
        admin.require_auth();

        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(LevelUpError::TokenNotFound);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
        env.storage().persistent().set(
            &DataKey::State(token_id),
            &HeroState {
                level: 1,
                xp: 0,
                power: 10,
            },
        );

        env.events().publish(
            (symbol_short!("mint"), symbol_short!("hero")),
            (to, token_id),
        );
        Ok(())
    }

    /// Owner earns XP; every `XP_PER_LEVEL` XP increases level and power.
    pub fn train(
        env: Env,
        owner: Address,
        token_id: u32,
        xp_gain: u32,
    ) -> Result<HeroState, LevelUpError> {
        owner.require_auth();
        ensure_owner(&env, token_id, &owner)?;

        let mut state: HeroState = env
            .storage()
            .persistent()
            .get(&DataKey::State(token_id))
            .ok_or(LevelUpError::TokenNotFound)?;

        state.xp = state.xp.saturating_add(xp_gain);
        while state.xp >= XP_PER_LEVEL {
            state.xp -= XP_PER_LEVEL;
            state.level = state.level.saturating_add(1);
            state.power = state.power.saturating_add(5);
        }

        env.storage()
            .persistent()
            .set(&DataKey::State(token_id), &state);
        env.events().publish(
            (symbol_short!("levelup"), symbol_short!("hero")),
            (token_id, state.level, state.power),
        );
        Ok(state)
    }

    pub fn state(env: Env, token_id: u32) -> Result<HeroState, LevelUpError> {
        let _ = read_owner(&env, token_id)?;
        env.storage()
            .persistent()
            .get(&DataKey::State(token_id))
            .ok_or(LevelUpError::TokenNotFound)
    }

    pub fn trait_label(env: Env, token_id: u32) -> Result<Symbol, LevelUpError> {
        let state = Self::state(env.clone(), token_id)?;
        Ok(match state.level {
            1 => symbol_short!("novice"),
            2 => symbol_short!("adept"),
            3 => symbol_short!("veteran"),
            _ => symbol_short!("legend"),
        })
    }
}

fn read_admin(env: &Env) -> Result<Address, LevelUpError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(LevelUpError::NotInitialized)
}

fn read_owner(env: &Env, token_id: u32) -> Result<Address, LevelUpError> {
    env.storage()
        .persistent()
        .get(&DataKey::Owner(token_id))
        .ok_or(LevelUpError::TokenNotFound)
}

fn ensure_owner(env: &Env, token_id: u32, owner: &Address) -> Result<(), LevelUpError> {
    if read_owner(env, token_id)? != *owner {
        return Err(LevelUpError::NotOwner);
    }
    Ok(())
}

#[cfg(test)]
mod test;
