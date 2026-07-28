#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Bytes, Env};

fn setup() -> (Env, StorageCompressionClient<'static>) {
    let env = Env::default();
    let id = env.register(StorageCompression, ());
    let client = StorageCompressionClient::new(&env, &id);
    (env, client)
}

#[test]
fn test_repeated_bytes_compress_smaller() {
    let (env, client) = setup();
    let key = symbol_short!("doc");
    let data = Bytes::from_slice(&env, b"xxxxxxxxxxxxxxxx");

    let raw_len = client.store_raw(&key, &data);
    let compressed_len = client.store_compressed(&key, &data);
    assert!(compressed_len < raw_len);
    assert_eq!(client.load_decompressed(&key), Some(data));
    let sizes = client.compare_sizes(&key);
    assert_eq!(sizes.raw_len, raw_len);
    assert_eq!(sizes.compressed_len, compressed_len);
}

#[test]
fn test_random_bytes_may_not_shrink() {
    let (env, client) = setup();
    let key = symbol_short!("rnd");
    let data = Bytes::from_slice(&env, b"abcdef0123456789");

    let raw_len = client.store_raw(&key, &data);
    let compressed_len = client.store_compressed(&key, &data);
    assert!(compressed_len >= raw_len);
}

#[test]
fn test_helper_round_trip() {
    let (env, client) = setup();
    let data = Bytes::from_slice(&env, b"aaabbbccc");
    let compressed = client.compress_bytes(&data);
    assert_eq!(client.decompress_bytes(&compressed), data);
}
