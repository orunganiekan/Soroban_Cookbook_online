# Storage Layout Validator

Validate **declared** storage layout schemas before upgrading a Soroban contract.

## Model

Layouts are explicit structs (`StorageLayout`) listing `(key, FieldType)` pairs — the same information you would encode in a Rust `DataKey` enum. The validator does **not** read live ledger entries from arbitrary deployments.

## Features

- Compatibility check between `current` and `next` layouts
- Duplicate key (collision) detection within a layout
- Migration plan with `Retain`, `Add`, `TypeChange`, and `Remove` steps

## Compatibility rules

| Change | Result |
|--------|--------|
| Add new field | Compatible (`Add`) |
| Keep field + type | Compatible (`Retain`) |
| Change field type | Incompatible (`TypeChange`) |
| Remove field | Incompatible (`Remove`) |
| Duplicate keys in one layout | Incompatible (collision list) |

## Usage

```bash
cargo test -p storage-layout-validator
cargo build --target wasm32v1-none --release -p storage-layout-validator
cargo clippy -p storage-layout-validator --all-targets -- -D warnings
```

## Migration guidance

1. Declare the current layout version in storage (`layout_version` instance key).
2. Run `validate` off-chain or via a governance contract before deploying WASM v2.
3. Apply `Add` steps lazily on first touch; never delete keys without an explicit migration that handles `Remove`/`TypeChange` manually.

## Security

- Declared layouts must match actual `DataKey` usage — drift between schema and code is not detected automatically.
- Treat `Remove` and `TypeChange` as breaking; require audited migration scripts.
