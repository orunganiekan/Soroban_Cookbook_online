# Compressed Storage

This example shows how to compress byte payloads before storing them in Soroban
persistent storage, and how to reason about the trade-off between raw and compressed
storage by comparing their byte lengths.

## What this example shows

- Storing a raw payload and reading it back
- Compressing a payload with run-length encoding (RLE) before storing it
- Decompressing the stored payload back to the exact original bytes
- Comparing the raw vs compressed byte lengths for the same key
- Understanding when compression helps and when it does not

## Why compression can reduce storage size

In Soroban, storage rent and resource cost scale with the size of a ledger entry, so
storing fewer bytes for the same information can lower on-ledger footprint. This example
lets you **measure** that by comparing stored byte lengths with `compare_stored_sizes`.

It deliberately does **not** print exact gas numbers or storage-cost percentages —
those are environment-specific. It only compares byte lengths, which are objective.

## When RLE helps vs. hurts

RLE replaces a run of identical bytes with a `(value, count)` pair, so:

- **Helps** for data with long runs of repeated bytes (logs, padding, repetitive JSON
  fields, zero-filled regions).
- **Hurts** for already-varied or random-looking data: because each run costs two bytes,
  non-repeating input can compress to *larger* than the original. The tests assert this
  case explicitly so the example never claims a false reduction.

## Compression format (high level)

The RLE codec lives in [`src/rle.rs`](./src/rle.rs) and uses a deterministic,
reversible layout:

```text
[ original length : u32 big-endian ][ (value: u8, run_length: u8) ... ]
```

The 4-byte header records the original length so decompression can verify it
reconstructed exactly the right number of bytes (a truncated/malformed payload fails
this check rather than returning wrong data). Runs longer than 255 are split across
multiple `(value, count)` pairs.

## Key functions

- `store_raw(key, data)` — store raw bytes; returns the raw length
- `store_compressed(key, data)` — compress then store; returns the compressed length
- `get_raw(key)` — read the raw bytes back
- `get_decompressed(key)` — read and decompress the compressed payload
- `compare_stored_sizes(key)` — returns `(raw_len, compressed_len)`

## Practical guidance

Always compare the stored length before choosing to compress. Compression also adds CPU
work to encode/decode within the transaction, so it makes the most sense when it clearly
reduces the on-ledger byte footprint for large, repetitive payloads.

## Test

```bash
cargo test -p compressed-storage
```

Build as WASM:

```bash
cargo build --target wasm32v1-none --release -p compressed-storage
```
