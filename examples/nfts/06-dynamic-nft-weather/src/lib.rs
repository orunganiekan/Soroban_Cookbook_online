//! Dynamic NFT whose appearance follows weather pushed by an authorized oracle.
//!
//! ## Off-chain data model
//!
//! Soroban contracts **cannot** call HTTP endpoints or weather APIs directly. Instead,
//! an off-chain service reads a real weather feed and submits the result on-chain by
//! invoking [`WeatherNft::set_weather`] as the configured *oracle* account. This
//! contract only trusts that single oracle address; the tests use `mock_all_auths` to
//! simulate those oracle transactions.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

/// The externally supplied weather state.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Weather {
    Sunny = 0,
    Rain = 1,
    Storm = 2,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Oracle,
    Weather,
    Owner(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum WeatherNftError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAdmin = 3,
    NotOracle = 4,
    TokenNotFound = 5,
    TokenAlreadyExists = 6,
}

#[contract]
pub struct WeatherNft;

#[contractimpl]
impl WeatherNft {
    /// Configure the admin and the trusted oracle. Callable once. Weather starts `Sunny`.
    pub fn initialize(env: Env, admin: Address, oracle: Address) -> Result<(), WeatherNftError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(WeatherNftError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage()
            .instance()
            .set(&DataKey::Weather, &Weather::Sunny);
        Ok(())
    }

    /// Mint a token to `to`. Admin-only.
    pub fn mint(
        env: Env,
        admin: Address,
        to: Address,
        token_id: u32,
    ) -> Result<(), WeatherNftError> {
        admin.require_auth();
        if read_admin(&env)? != admin {
            return Err(WeatherNftError::NotAdmin);
        }
        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(WeatherNftError::TokenAlreadyExists);
        }
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
        env.events().publish(
            (symbol_short!("mint"), symbol_short!("weather")),
            (to, token_id),
        );
        Ok(())
    }

    /// Oracle pushes the current environmental state (an off-chain feed in production).
    ///
    /// Only the configured oracle account may call this.
    pub fn set_weather(env: Env, oracle: Address, weather: Weather) -> Result<(), WeatherNftError> {
        oracle.require_auth();
        if read_oracle(&env)? != oracle {
            return Err(WeatherNftError::NotOracle);
        }
        env.storage().instance().set(&DataKey::Weather, &weather);
        env.events()
            .publish((symbol_short!("weather"),), weather as u32);
        Ok(())
    }

    /// Derive the token's appearance tag from the current global weather.
    pub fn appearance(env: Env, token_id: u32) -> Result<Symbol, WeatherNftError> {
        let _ = read_owner(&env, token_id)?;
        Ok(match Self::current_weather(env.clone()) {
            Weather::Sunny => symbol_short!("bright"),
            Weather::Rain => symbol_short!("misty"),
            Weather::Storm => symbol_short!("charged"),
        })
    }

    /// Read the current global weather.
    pub fn current_weather(env: Env) -> Weather {
        env.storage()
            .instance()
            .get(&DataKey::Weather)
            .unwrap_or(Weather::Sunny)
    }
}

fn read_admin(env: &Env) -> Result<Address, WeatherNftError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(WeatherNftError::NotInitialized)
}

fn read_oracle(env: &Env) -> Result<Address, WeatherNftError> {
    env.storage()
        .instance()
        .get(&DataKey::Oracle)
        .ok_or(WeatherNftError::NotInitialized)
}

fn read_owner(env: &Env, token_id: u32) -> Result<Address, WeatherNftError> {
    env.storage()
        .persistent()
        .get(&DataKey::Owner(token_id))
        .ok_or(WeatherNftError::TokenNotFound)
}

#[cfg(test)]
mod test;
