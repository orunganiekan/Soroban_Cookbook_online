# AGENTS.md

## Cursor Cloud specific instructions

### What this repo is
This is the **Soroban Cookbook**: a monorepo of ~100 Stellar/Soroban smart-contract
examples written in Rust (the primary product), plus an optional Next.js showcase
`webapp/` and an optional mdBook docs site under `book/`. Note the root `README.md`
is a mismatched template describing the unrelated `typos` tool — ignore it. Use
`CONTRIBUTING.md` and `scripts/` for the real workflow.

### Toolchain
The Rust toolchain auto-installs from `rust-toolchain.toml` (pinned `1.96.0`,
target `wasm32-unknown-unknown`, `rust-src`) on the first `cargo` invocation.
The startup update script additionally adds the `wasm32v1-none` target and runs
`bun install` in `webapp/` (Bun lives at `~/.bun/bin`). There is no committed
`Cargo.lock`, so `soroban-sdk` resolves to the latest compatible `26.x` release.

### Test (primary product)
Run contract tests on the host target (in-memory `soroban_sdk::Env`, no network):
- Whole workspace: `cargo test --workspace`
- Single example: `cd examples/<category>/<name> && cargo test`
- Integration/security suites: `cargo test -p integration-tests` / `cargo test --package security-tests`

### Lint (non-obvious gotcha)
- Format check: `cargo fmt --all --check`
- Clippy: run it via `./scripts/ci-workspace.sh clippy --tests --lib --target x86_64-unknown-linux-gnu -- -D warnings`.
  Do NOT run a bare `cargo clippy --workspace -- -D warnings`: several examples still
  use the deprecated `soroban_sdk::events::Events::publish`, so a bare run fails with
  `-D warnings`. The CI wrapper sets `RUSTFLAGS="-A deprecated"` (matching
  `.github/workflows/test.yml`) and excludes crates listed in
  `scripts/ci-exclude-packages.txt`.

### Build to WASM
Contracts build for the `wasm32v1-none` target (not the default `wasm32-unknown-unknown`):
- Single contract: `cargo build -p soroban-hello-world-contract --target wasm32v1-none --release`
- All contracts (CI wrapper): `./scripts/ci-workspace.sh build --target wasm32v1-none --release`
The `.cargo/config.toml` disables `reference-types` for this target so Soroban's WASM
validator accepts the output. Deploying via `scripts/deploy.sh` additionally needs the
Stellar CLI + testnet access (optional, not required for tests/build).

### Webapp (optional)
`webapp/` is a self-contained client-side Next.js 14 app (Bun package manager); it has
no backend/API/database. Run the dev server with `bun run dev` (from `webapp/`), served
on `http://localhost:3000`. Lint with `bun run lint`.
