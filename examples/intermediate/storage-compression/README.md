# Storage Compression

Intermediate example showing run-length encoding (RLE) before persistent storage and how to compare footprint trade-offs.

## What it demonstrates

- Raw vs compressed persistent storage for the same logical key
- `compress` / `decompress` helpers (`rle` module) with round-trip correctness
- `compare_sizes` for stored byte lengths (not fabricated gas numbers)

## When compression helps

- Long runs of identical bytes (logs, padding, repetitive JSON fields)
- Large blobs where storage rent dominates

## When compression hurts

- Small payloads (header overhead exceeds savings)
- High-entropy data (already random)
- Hot paths that read/write every ledger — CPU for encode/decode adds instruction cost

## Gas / resource measurement

This cookbook does **not** embed exact Soroban resource meters in unit tests. To measure locally:

1. Build the contract WASM (`cargo build --target wasm32v1-none --release -p storage-compression`).
2. Use `soroban contract invoke` with `--cost` (or your SDK's simulation API) for `store_raw` vs `store_compressed` on representative payloads.
3. Compare **storage bytes** via `compare_sizes` and **simulation metrics** from the CLI — treat CLI output as environment-specific, not universal constants.

## Commands

```bash
cargo test -p storage-compression
cargo build --target wasm32v1-none --release -p storage-compression
```

## Limitations

- RLE is illustrative, not a general-purpose codec (no dictionary, no entropy coding).
- Migration requires storing a format version byte if you change algorithms.
- Decompression runs in the same transaction as reads — budget accordingly.
