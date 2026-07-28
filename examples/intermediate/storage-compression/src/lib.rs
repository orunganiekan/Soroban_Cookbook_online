//! Storage compression helpers and a contract that compares raw vs compressed footprints.

#![no_std]

pub mod rle;

use rle::{decompress, compress};
use soroban_sdk::{contract, contractimpl, contracttype, Bytes, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Raw(Symbol),
    Compressed(Symbol),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SizeComparison {
    pub raw_len: u32,
    pub compressed_len: u32,
}

#[contract]
pub struct StorageCompression;

#[contractimpl]
impl StorageCompression {
    /// Store uncompressed bytes under `key` and return the byte length written.
    pub fn store_raw(env: Env, key: Symbol, data: Bytes) -> u32 {
        env.storage().persistent().set(&DataKey::Raw(key), &data);
        data.len()
    }

    /// Compress with run-length encoding, persist, and return compressed length.
    pub fn store_compressed(env: Env, key: Symbol, data: Bytes) -> u32 {
        let compressed = compress(&env, &data);
        env.storage()
            .persistent()
            .set(&DataKey::Compressed(key), &compressed);
        compressed.len()
    }

    pub fn load_raw(env: Env, key: Symbol) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Raw(key))
    }

    pub fn load_decompressed(env: Env, key: Symbol) -> Option<Bytes> {
        env.storage()
            .persistent()
            .get(&DataKey::Compressed(key))
            .map(|payload| decompress(&env, &payload))
    }

    /// Compare stored payload sizes for the same logical `key`.
    pub fn compare_sizes(env: Env, key: Symbol) -> SizeComparison {
        let raw_len = env
            .storage()
            .persistent()
            .get(&DataKey::Raw(key.clone()))
            .map(|b: Bytes| b.len())
            .unwrap_or(0);
        let compressed_len = env
            .storage()
            .persistent()
            .get(&DataKey::Compressed(key))
            .map(|b: Bytes| b.len())
            .unwrap_or(0);
        SizeComparison {
            raw_len,
            compressed_len,
        }
    }

    /// Expose compression for off-chain tooling / unit tests without storage writes.
    pub fn compress_bytes(env: Env, data: Bytes) -> Bytes {
        compress(&env, &data)
    }

    pub fn decompress_bytes(env: Env, data: Bytes) -> Bytes {
        decompress(&env, &data)
    }
}

#[cfg(test)]
mod test;
