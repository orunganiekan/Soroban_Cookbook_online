# Dynamic NFT — Level Up

An NFT whose on-chain `HeroState` (level, XP, power) evolves when the owner calls `train`.

## Dynamic behavior

- Mint starts at level 1 with 0 XP and 10 power.
- Each `train` call adds XP; every 100 XP levels up and adds 5 power.
- `trait_label` maps level to a readable stage (`novice` → `legend`).

## Security / trust

- Only the collection admin can mint.
- Only the token owner can train (via `require_auth`).
- Stats live entirely on-chain; there is no external metadata fetch.

## Usage

```bash
cargo test --manifest-path examples/nfts/05-dynamic-nft-level-up/Cargo.toml
cargo build --target wasm32v1-none --release --manifest-path examples/nfts/05-dynamic-nft-level-up/Cargo.toml
```
