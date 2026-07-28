#![no_std]
//! # Compressed Storage Example
//!
//! Demonstrates compressing byte payloads before writing them to Soroban persistent
//! storage, and contrasts raw vs compressed footprints so you can decide when
//! compression is worthwhile.
//!
//! Compression uses a small run-length encoding (RLE) codec kept in the [`rle`]
//! module. RLE shrinks data with long runs of identical bytes (logs, padding,
//! repetitive fields) but can *grow* already-varied or random-looking data, so this
//! example lets you compare the stored byte lengths directly rather than guessing.

pub mod rle;

use rle::{compress, decompress};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env};

/// Typed storage keys for the raw and compressed payloads of a given owner key.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    RawData(Address),
    CompressedData(Address),
}

/// Contract that stores raw and compressed payloads for side-by-side comparison.
#[contract]
pub struct CompressedStorageContract;

#[contractimpl]
impl CompressedStorageContract {
    /// Store raw bytes in persistent storage; returns the stored byte length.
    pub fn store_raw(env: Env, key: Address, data: Bytes) -> u32 {
        env.storage()
            .persistent()
            .set(&DataKey::RawData(key), &data);
        data.len()
    }

    /// Compress bytes with RLE, persist them, and return the compressed byte length.
    pub fn store_compressed(env: Env, key: Address, data: Bytes) -> u32 {
        let compressed = compress(&env, &data);
        env.storage()
            .persistent()
            .set(&DataKey::CompressedData(key), &compressed);
        compressed.len()
    }

    /// Read raw bytes back from storage.
    pub fn get_raw(env: Env, key: Address) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::RawData(key))
    }

    /// Read the compressed payload and decompress it before returning.
    pub fn get_decompressed(env: Env, key: Address) -> Option<Bytes> {
        env.storage()
            .persistent()
            .get(&DataKey::CompressedData(key))
            .map(|compressed: Bytes| decompress(&env, &compressed))
    }

    /// Compare stored payload sizes for `key`, returning `(raw_len, compressed_len)`.
    ///
    /// A missing payload is reported as length `0`.
    pub fn compare_stored_sizes(env: Env, key: Address) -> (u32, u32) {
        let raw_len = env
            .storage()
            .persistent()
            .get(&DataKey::RawData(key.clone()))
            .map(|raw: Bytes| raw.len())
            .unwrap_or(0);

        let compressed_len = env
            .storage()
            .persistent()
            .get(&DataKey::CompressedData(key))
            .map(|compressed: Bytes| compressed.len())
            .unwrap_or(0);

        (raw_len, compressed_len)
    }
}

#[cfg(test)]
mod test;
