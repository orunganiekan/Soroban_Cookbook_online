# Storage Layout Validator

Validate **declared** storage layout schemas before upgrading a Soroban contract, so
you can catch breaking storage changes before deploying a new WASM.

## Model

Layouts are explicit structs (`StorageLayout`) listing `(key, FieldType)` pairs — the
same information you would otherwise encode in a Rust `DataKey` enum. The validator
does **not** read live ledger entries from arbitrary deployments; it compares the two
schemas you declare in code.

## Features

- **Compatibility check** between a `current` and a `next` layout
- **Duplicate key (collision) detection** within a single layout
- **Migration plan** describing each key as `Retain`, `Add`, `TypeChange`, or `Remove`

## Compatibility rules

| Change                      | Result                     |
| --------------------------- | -------------------------- |
| Add new field               | Compatible (`Add`)         |
| Keep field + type           | Compatible (`Retain`)      |
| Change field type           | Incompatible (`TypeChange`)|
| Remove field                | Incompatible (`Remove`)    |
| Duplicate keys in one layout| Incompatible (collision)   |

## API

- `validate(current, next) -> Result<ValidationReport, LayoutError>` — compares two
  layouts and returns `{ compatible, collisions, steps }`. Returns
  `LayoutError::MalformedLayout` if either layout is invalid (for example `version == 0`).
- `find_collisions(layout) -> Vec<Symbol>` — returns the duplicate keys in a layout.

## Usage

```rust
let report = client.validate(&current_layout, &next_layout);
if report.compatible {
    // safe to deploy the upgrade
}
```

## Migration guidance

1. Declare the current layout version in storage (for example a `layout_version`
   instance key).
2. Run `validate` off-chain or via a governance contract before deploying WASM v2.
3. Apply `Add` steps lazily on first touch; never delete keys without an explicit
   migration that handles `Remove` / `TypeChange` manually.

## Security

- Declared layouts must match actual `DataKey` usage — drift between the schema and
  the code is not detected automatically.
- Treat `Remove` and `TypeChange` as breaking; require audited migration scripts.

## Testing

```bash
cargo test -p storage-layout-validator
```

Build as WASM:

```bash
cargo build --target wasm32v1-none --release -p storage-layout-validator
```

## 📚 Related Examples

- [02-timelock](../02-timelock/) — Delayed execution useful for gating upgrades
- [01-multi-party-auth](../01-multi-party-auth/) — Authorization patterns for admin actions
- [Advanced Examples](../) — Other complex patterns
