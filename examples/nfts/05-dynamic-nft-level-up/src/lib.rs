//! Dynamic NFT whose on-chain stats evolve as its owner trains it.
//!
//! Each token carries a [`HeroState`] (level, XP, power) stored on-chain. The owner
//! calls [`LevelUpNft::train`] to earn XP; every [`XP_PER_LEVEL`] XP the hero levels
//! up and gains power. All state lives on-chain — there is no external metadata fetch.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// XP required to gain one level.
const XP_PER_LEVEL: u32 = 100;

/// Power granted per level gained.
const POWER_PER_LEVEL: u32 = 5;

/// Evolving on-chain state for a single token.
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum LevelUpError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAdmin = 3,
    TokenNotFound = 4,
    TokenAlreadyExists = 5,
    NotOwner = 6,
}

#[contract]
pub struct LevelUpNft;

#[contractimpl]
impl LevelUpNft {
    /// Set the collection admin. Callable once.
    pub fn initialize(env: Env, admin: Address) -> Result<(), LevelUpError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(LevelUpError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Mint a fresh hero to `to`. Admin-only.
    pub fn mint(env: Env, admin: Address, to: Address, token_id: u32) -> Result<(), LevelUpError> {
        admin.require_auth();
        if read_admin(&env)? != admin {
            return Err(LevelUpError::NotAdmin);
        }
        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(LevelUpError::TokenAlreadyExists);
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

    /// Owner earns XP; every [`XP_PER_LEVEL`] XP increases level and power.
    ///
    /// Returns the updated [`HeroState`]. Requires the token owner's authorization.
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

        // Saturating arithmetic keeps a very active hero from overflowing under the
        // workspace's overflow-checked test/release profiles.
        state.xp = state.xp.saturating_add(xp_gain);
        while state.xp >= XP_PER_LEVEL {
            state.xp -= XP_PER_LEVEL;
            state.level = state.level.saturating_add(1);
            state.power = state.power.saturating_add(POWER_PER_LEVEL);
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

    /// Read the current [`HeroState`] for a token.
    pub fn state(env: Env, token_id: u32) -> Result<HeroState, LevelUpError> {
        let _ = read_owner(&env, token_id)?;
        env.storage()
            .persistent()
            .get(&DataKey::State(token_id))
            .ok_or(LevelUpError::TokenNotFound)
    }

    /// Human-readable stage label derived from the hero's level.
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
