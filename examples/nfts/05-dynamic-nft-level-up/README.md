# Dynamic NFT — Level Up

An NFT whose on-chain `HeroState` (level, XP, power) evolves when the owner trains it.
This shows how a token's traits can change over its lifetime while every piece of
state stays verifiable on-chain.

## Dynamic behavior

- Minting starts a hero at **level 1**, **0 XP**, **10 power**.
- Each `train` call adds XP; every **100 XP** (`XP_PER_LEVEL`) triggers a level-up that
  resets the XP remainder and adds **5 power** (`POWER_PER_LEVEL`).
- `trait_label` maps the level to a readable stage: `novice` → `adept` → `veteran` → `legend`.

## API

| Function | Description |
| --- | --- |
| `initialize(admin)` | Set the collection admin (once). |
| `mint(admin, to, token_id)` | Admin-only mint of a fresh hero. |
| `train(owner, token_id, xp_gain)` | Owner-only; adds XP and applies level-ups; returns the new `HeroState`. |
| `state(token_id)` | Read the current `HeroState`. |
| `trait_label(token_id)` | Read the current stage label. |

## Security / trust

- Only the collection admin can mint (checked against stored admin + `require_auth`).
- Only the token owner can train (`require_auth` on the owner).
- Stats live entirely on-chain; there is no external metadata fetch.

## Events

- `("mint", "hero")` → `(to, token_id)` on mint.
- `("levelup", "hero")` → `(token_id, level, power)` on every train call.

## Testing

```bash
cargo test -p dynamic-nft-level-up
```

Build as WASM:

```bash
cargo build --target wasm32v1-none --release -p dynamic-nft-level-up
```

## 📚 Related Examples

- [01-basic-nft](../01-basic-nft/) — Ownership, authorization, and events this builds on
- [06-dynamic-nft-weather](../06-dynamic-nft-weather/) — Oracle-driven dynamic state
- [07-dynamic-nft-time](../07-dynamic-nft-time/) — Time-driven dynamic state
