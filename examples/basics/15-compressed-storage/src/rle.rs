//! Run-length encoding (RLE) for `no_std` Soroban contracts.
//!
//! The format is deterministic and safely reversible:
//!
//! ```text
//! [ original length : u32 big-endian ][ (value: u8, run_length: u8) ... ]
//! ```
//!
//! Each run stores a byte value followed by how many times it repeats (1..=255).
//! The 4-byte header records the original (uncompressed) length so decompression
//! can validate that it reconstructed exactly the right number of bytes.

use soroban_sdk::{Bytes, Env};

/// Length of the big-endian `u32` original-length header, in bytes.
const HEADER_LEN: u32 = 4;

/// Maximum run length encodable in a single `u8` counter.
const MAX_RUN: u8 = 255;

/// Compress `data` using run-length encoding with a 4-byte length header.
pub fn compress(env: &Env, data: &Bytes) -> Bytes {
    let mut out = Bytes::new(env);
    out.extend_from_slice(&data.len().to_be_bytes());

    let mut i = 0u32;
    while i < data.len() {
        let current = data.get(i).unwrap();
        let mut run_length: u8 = 1;

        while run_length < MAX_RUN && i + (run_length as u32) < data.len() {
            if data.get(i + run_length as u32).unwrap() != current {
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
///
/// # Panics
/// Panics if `compressed` is malformed or truncated — i.e. the reconstructed length
/// does not match the header's original-length field. Callers that might handle
/// untrusted input should guard against this.
pub fn decompress(env: &Env, compressed: &Bytes) -> Bytes {
    let original_len = read_u32(compressed, 0);
    let mut out = Bytes::new(env);

    let mut i = HEADER_LEN;
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

/// Read a big-endian `u32` from `bytes` starting at `index`.
fn read_u32(bytes: &Bytes, index: u32) -> u32 {
    let mut result = 0u32;
    for offset in 0..HEADER_LEN {
        result = (result << 8) | (bytes.get(index + offset).unwrap() as u32);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Bytes, Env};

    fn bytes(env: &Env, slice: &[u8]) -> Bytes {
        Bytes::from_slice(env, slice)
    }

    #[test]
    fn repeated_data_round_trips_and_shrinks() {
        let env = Env::default();
        let data = bytes(&env, b"aaaaabbbbcccccaaaaa");
        let compressed = compress(&env, &data);
        assert!(compressed.len() < data.len());
        assert_eq!(decompress(&env, &compressed), data);
    }

    #[test]
    fn non_repeating_data_round_trips_without_shrinking() {
        let env = Env::default();
        let data = bytes(&env, b"abcdef0123456789");
        let compressed = compress(&env, &data);
        // Two output bytes per unique input byte + header => never smaller here.
        assert!(compressed.len() >= data.len());
        assert_eq!(decompress(&env, &compressed), data);
    }

    #[test]
    fn empty_input_round_trips() {
        let env = Env::default();
        let data = Bytes::new(&env);
        let compressed = compress(&env, &data);
        // Only the 4-byte header, encoding original length 0.
        assert_eq!(compressed.len(), HEADER_LEN);
        assert_eq!(decompress(&env, &compressed), data);
    }

    #[test]
    fn single_byte_round_trips() {
        let env = Env::default();
        let data = bytes(&env, b"z");
        let compressed = compress(&env, &data);
        assert_eq!(decompress(&env, &compressed), data);
    }

    #[test]
    fn long_run_beyond_max_is_split() {
        let env = Env::default();
        // 300 identical bytes exceeds the 255 single-run cap, forcing two runs.
        let raw = [b'q'; 300];
        let data = bytes(&env, &raw);
        let compressed = compress(&env, &data);
        // header(4) + run(2) for 255 + run(2) for 45 = 8 bytes.
        assert_eq!(compressed.len(), 8);
        assert_eq!(decompress(&env, &compressed), data);
    }

    #[test]
    #[should_panic(expected = "decompressed length mismatch")]
    fn truncated_payload_panics() {
        let env = Env::default();
        let data = bytes(&env, b"aaaaaa");
        let compressed = compress(&env, &data);
        // Drop the final run byte to truncate the payload.
        let truncated = compressed.slice(0..compressed.len() - 1);
        let _ = decompress(&env, &truncated);
    }
}
