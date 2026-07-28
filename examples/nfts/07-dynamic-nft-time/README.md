# Dynamic NFT — Time Evolution

An NFT whose life stage is derived from ledger time: it "grows" automatically as time
passes, with no owner action required.

## Dynamic behavior

- Each token stores its **mint timestamp** (`env.ledger().timestamp()` at mint).
- `life_stage(token_id)` computes `age = now - minted_at` and divides by
  `STAGE_DURATION_SECS` (1 day here) to pick a stage.
- Stages: `seed` (stage 0) → `sprout` (1) → `bloom` (2) → `ancient` (3+).

## API

| Function | Description |
| --- | --- |
| `initialize(admin)` | Set the collection admin (once). |
| `mint(admin, to, token_id)` | Admin-only mint; records the current ledger timestamp. |
| `life_stage(token_id)` | Current life-stage label derived from ledger time. |
| `minted_at(token_id)` | The token's recorded mint timestamp. |

## Security / trust

- Only the admin can mint (stored-admin check plus `require_auth`).
- Stages are deterministic from ledger time; no admin can fast-forward evolution
  without ledger time advancing. Validators control the timestamp only within protocol
  bounds.

## Events

- `("mint", "time")` → `(to, token_id)` on mint.

## Testing

Tests manipulate ledger time with the Soroban test utility
`env.ledger().set_timestamp(...)` and assert stage transitions, including the exact
boundary at each `STAGE_DURATION_SECS` multiple.

```bash
cargo test -p dynamic-nft-time
```

Build as WASM:

```bash
cargo build --target wasm32v1-none --release -p dynamic-nft-time
```

## 📚 Related Examples

- [01-basic-nft](../01-basic-nft/) — Ownership, authorization, and events this builds on
- [05-dynamic-nft-level-up](../05-dynamic-nft-level-up/) — Owner-driven dynamic state
- [06-dynamic-nft-weather](../06-dynamic-nft-weather/) — Oracle-driven dynamic state
