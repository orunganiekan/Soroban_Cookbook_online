# Dynamic NFT — Weather

NFT appearance tags change when an authorized **oracle** account pushes weather updates.

## Dynamic behavior

- Global `Weather` enum (`Sunny`, `Rain`, `Storm`) is stored in instance storage.
- `appearance(token_id)` maps weather to a visual tag (`bright`, `misty`, `charged`).

## External data (honest model)

Soroban contracts **cannot** call HTTP weather APIs. In production, an off-chain service reads a feed and invokes `set_weather` as the configured oracle. This example uses `mock_all_auths` in tests to simulate those transactions.

## Security / trust

- Users must trust the oracle address configured at `initialize`.
- Anyone can read appearance; only the oracle can change weather.

## Usage

```bash
cargo test --manifest-path examples/nfts/06-dynamic-nft-weather/Cargo.toml
cargo build --target wasm32v1-none --release --manifest-path examples/nfts/06-dynamic-nft-weather/Cargo.toml
```
