//! # Tests for the Compressed Storage Example
//!
//! Client-based tests covering raw/compressed round-trips, size comparison, and the
//! trade-off between repeated and non-repeating data. Focused RLE codec tests live in
//! `rle.rs`.

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env};

fn setup() -> (Env, CompressedStorageContractClient<'static>, Address) {
    let env = Env::default();
    let contract_id = env.register(CompressedStorageContract, ());
    let client = CompressedStorageContractClient::new(&env, &contract_id);
    let key = Address::generate(&env);
    (env, client, key)
}

#[test]
fn test_raw_storage_round_trip() {
    let (env, client, key) = setup();
    let data = Bytes::from_slice(&env, b"hello world");

    let raw_len = client.store_raw(&key, &data);
    assert_eq!(raw_len, data.len());
    assert_eq!(client.get_raw(&key), Some(data));
}

#[test]
fn test_compressed_round_trip_and_shrinks_for_repeated_data() {
    let (env, client, key) = setup();
    let data = Bytes::from_slice(&env, b"aaaaabbbbcccccaaaaa");

    let raw_len = client.store_raw(&key, &data);
    let compressed_len = client.store_compressed(&key, &data);

    assert_eq!(raw_len, data.len());
    assert!(
        compressed_len < raw_len,
        "compression should reduce size for repeated bytes"
    );

    // Decompressed output must exactly match the original, including its length.
    let decompressed = client.get_decompressed(&key).unwrap();
    assert_eq!(decompressed, data);
    assert_eq!(decompressed.len(), data.len());

    assert_eq!(client.compare_stored_sizes(&key), (raw_len, compressed_len));
}

#[test]
fn test_non_repeating_data_does_not_shrink() {
    let (env, client, key) = setup();
    let data = Bytes::from_slice(&env, b"abcdef0123456789");

    let raw_len = client.store_raw(&key, &data);
    let compressed_len = client.store_compressed(&key, &data);

    assert_eq!(raw_len, data.len());
    assert!(
        compressed_len >= raw_len,
        "small unrelated bytes should not falsely report a reduction"
    );
    // Round-trip must still be correct even when compression does not help.
    assert_eq!(client.get_decompressed(&key), Some(data));
    assert_eq!(client.compare_stored_sizes(&key), (raw_len, compressed_len));
}

#[test]
fn test_decompressed_matches_original_exactly() {
    let (env, client, key) = setup();
    let data = Bytes::from_slice(&env, b"zzzzzzzzzzzzzzzz");

    client.store_compressed(&key, &data);
    let decompressed = client.get_decompressed(&key).unwrap();

    assert_eq!(decompressed, data);
    assert_eq!(decompressed.len(), data.len());
}

#[test]
fn test_empty_payload_round_trips() {
    let (env, client, key) = setup();
    let data = Bytes::new(&env);

    let raw_len = client.store_raw(&key, &data);
    let compressed_len = client.store_compressed(&key, &data);

    assert_eq!(raw_len, 0);
    assert_eq!(client.get_raw(&key), Some(data.clone()));
    assert_eq!(client.get_decompressed(&key), Some(data));
    assert_eq!(client.compare_stored_sizes(&key), (0, compressed_len));
}

#[test]
fn test_compare_sizes_reports_zero_for_missing_payloads() {
    let (_env, client, key) = setup();
    // Nothing stored yet: both lengths are zero.
    assert_eq!(client.compare_stored_sizes(&key), (0, 0));
    assert_eq!(client.get_raw(&key), None);
    assert_eq!(client.get_decompressed(&key), None);
}
