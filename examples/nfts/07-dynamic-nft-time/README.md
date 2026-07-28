# Dynamic NFT — Time Evolution

NFT life stage is derived from `env.ledger().timestamp()` minus the mint timestamp.

## Dynamic behavior

- `STAGE_DURATION_SECS` (1 day in this example) defines stage boundaries.
- Stages: `seed` → `sprout` → `bloom` → `ancient`.

## Security / trust

- Stages are deterministic from ledger time; miners/validators control timestamp within protocol bounds.
- No admin can fast-forward evolution without ledger time advancing.

## Usage

```bash
cargo test --manifest-path examples/nfts/07-dynamic-nft-time/Cargo.toml
cargo build --target wasm32v1-none --release --manifest-path examples/nfts/07-dynamic-nft-time/Cargo.toml
```
