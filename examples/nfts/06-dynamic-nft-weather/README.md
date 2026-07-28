# Dynamic NFT — Weather

An NFT whose appearance tag changes when an authorized **oracle** account pushes a new
weather state on-chain.

## Dynamic behavior

- A global `Weather` value (`Sunny`, `Rain`, `Storm`) is kept in instance storage.
- `appearance(token_id)` maps the current weather to a visual tag:
  `Sunny → bright`, `Rain → misty`, `Storm → charged`.

## External data (honest model)

Soroban contracts **cannot** call HTTP endpoints or weather APIs directly. In
production an off-chain service reads a real weather feed and submits the value on-chain
by calling `set_weather` as the configured oracle account. This contract trusts only
that single oracle address. The tests use `mock_all_auths` to simulate those oracle
transactions.

## API

| Function | Description |
| --- | --- |
| `initialize(admin, oracle)` | Set admin + trusted oracle (once); weather starts `Sunny`. |
| `mint(admin, to, token_id)` | Admin-only mint. |
| `set_weather(oracle, weather)` | Oracle-only global weather update. |
| `appearance(token_id)` | Current appearance tag for a token. |
| `current_weather()` | Current global weather. |

## Security / trust

- Users must trust the oracle address configured at `initialize`.
- Only the admin can mint; only the oracle can change weather (both enforced with a
  stored-address check plus `require_auth`).
- Anyone can read `appearance` / `current_weather`.

## Events

- `("mint", "weather")` → `(to, token_id)` on mint.
- `("weather",)` → `weather as u32` on every oracle update.

## Testing

```bash
cargo test -p dynamic-nft-weather
```

Build as WASM:

```bash
cargo build --target wasm32v1-none --release -p dynamic-nft-weather
```

## 📚 Related Examples

- [01-basic-nft](../01-basic-nft/) — Ownership, authorization, and events this builds on
- [05-dynamic-nft-level-up](../05-dynamic-nft-level-up/) — Owner-driven dynamic state
- [07-dynamic-nft-time](../07-dynamic-nft-time/) — Time-driven dynamic state
