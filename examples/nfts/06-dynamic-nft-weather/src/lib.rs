//! Dynamic NFT whose visual state follows weather pushed by an authorized oracle.
//!
//! Soroban contracts cannot call external HTTP APIs. Weather is simulated by an
//! on-chain oracle account that submits updates.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

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
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WeatherNftError {
    NotInitialized = 1,
    Unauthorized = 2,
    TokenNotFound = 3,
}

#[contract]
pub struct WeatherNft;

#[contractimpl]
impl WeatherNft {
    pub fn initialize(env: Env, admin: Address, oracle: Address) -> Result<(), WeatherNftError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(WeatherNftError::NotInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage()
            .instance()
            .set(&DataKey::Weather, &Weather::Sunny);
        Ok(())
    }

    pub fn mint(
        env: Env,
        admin: Address,
        to: Address,
        token_id: u32,
    ) -> Result<(), WeatherNftError> {
        let stored_admin = read_admin(&env)?;
        if stored_admin != admin {
            return Err(WeatherNftError::Unauthorized);
        }
        admin.require_auth();
        if env.storage().persistent().has(&DataKey::Owner(token_id)) {
            return Err(WeatherNftError::TokenNotFound);
        }
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
        Ok(())
    }

    /// Oracle pushes environmental state (mocked off-chain feed in production).
    pub fn set_weather(env: Env, oracle: Address, weather: Weather) -> Result<(), WeatherNftError> {
        oracle.require_auth();
        if read_oracle(&env)? != oracle {
            return Err(WeatherNftError::Unauthorized);
        }
        env.storage().instance().set(&DataKey::Weather, &weather);
        env.events()
            .publish((symbol_short!("weather"),), weather as u32);
        Ok(())
    }

    pub fn appearance(env: Env, token_id: u32) -> Result<Symbol, WeatherNftError> {
        let _ = read_owner(&env, token_id)?;
        let weather: Weather = env
            .storage()
            .instance()
            .get(&DataKey::Weather)
            .unwrap_or(Weather::Sunny);
        Ok(match weather {
            Weather::Sunny => symbol_short!("bright"),
            Weather::Rain => symbol_short!("misty"),
            Weather::Storm => symbol_short!("charged"),
        })
    }

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
