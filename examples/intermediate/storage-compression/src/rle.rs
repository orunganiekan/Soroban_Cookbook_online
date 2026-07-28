//! Simple run-length encoding suitable for `no_std` Soroban contracts.

use soroban_sdk::Bytes;

/// Compress bytes using run-length encoding with a 4-byte original-length header.
pub fn compress(env: &soroban_sdk::Env, data: &Bytes) -> Bytes {
    let mut out = Bytes::new(env);
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());

    let mut i = 0u32;
    while i < data.len() {
        let current = data.get(i).unwrap();
        let mut run_length: u8 = 1;

        while run_length < 255 && i + (run_length as u32) < data.len() {
            let next = data.get(i + run_length as u32).unwrap();
            if next != current {
                break;
            }
            run_length += 1;
        }

        out.push_back(current);
        out.push_back(run_length);
        i += run_length as u32;
    }

    out
}

/// Decompress an RLE payload produced by [`compress`].
pub fn decompress(env: &soroban_sdk::Env, compressed: &Bytes) -> Bytes {
    let original_len = read_u32(compressed, 0);
    let mut out = Bytes::new(env);
    let mut i = 4u32;

    while i + 1 < compressed.len() {
        let value = compressed.get(i).unwrap();
        let run_length = compressed.get(i + 1).unwrap();
        for _ in 0..(run_length as u32) {
            out.push_back(value);
        }
        i += 2;
    }

    assert!(out.len() == original_len, "decompressed length mismatch");
    out
}

fn read_u32(bytes: &Bytes, index: u32) -> u32 {
    let mut result = 0u32;
    for offset in 0..4 {
        result = (result << 8) | (bytes.get(index + offset).unwrap() as u32);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Bytes, Env};

    #[test]
    fn round_trip() {
        let env = Env::default();
        let data = Bytes::from_slice(&env, b"aaaabbbb");
        let compressed = compress(&env, &data);
        assert_eq!(decompress(&env, &compressed), data);
    }
}
