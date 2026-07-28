# Storage Pagination

Cursor-based pagination for large on-chain collections, enabling scalable reads without exceeding instruction limits.

## What It Demonstrates

- Cursor-based pagination (not offset scans over a monolithic `Vec` in storage)
- Bounded page sizes via `MAX_PAGE_SIZE`
- Stable, deterministic cursor encoding (8-byte opaque payload)
- Per-item persistent keys so each page read loads only `page_size` entries
- First page, subsequent pages, partial last page, and empty collection behavior
- Invalid cursor handling

## API

```rust
// Append-only collection
add_item(env, item)

// page_size must be > 0 and is capped at MAX_PAGE_SIZE (50)
list(env, page_size, cursor: Option<Bytes>) -> Result<Page, PaginationError>

count(env) -> u32
get_item(env, index) -> Symbol
```

`Page` contains `items` and `next_cursor`. Pass `None` for the first page, then pass `next_cursor` from the previous response.

## Cursor Encoding

Cursors are opaque `Bytes` values produced by the contract:

| Offset | Size | Content |
|--------|------|---------|
| 0 | 4 | Magic `0x50470001` (big-endian) |
| 4 | 4 | Absolute item index for the next page (big-endian) |

The encoding is stable across calls: the same index always yields the same cursor bytes. Clients may base64-encode these bytes for transport in HTTP/JSON APIs; the on-chain contract uses raw bytes.

Use `encode_cursor_for_index` in tests or tooling when you need a cursor for a known index.

## Consistency Guarantees and Limitations

**Guarantees (append-only collection):**

- Item order matches append order (absolute indices `0..count-1`).
- A cursor remains valid while the collection only grows at the end and indices are not removed.
- Pages are deterministic: the same `(page_size, cursor)` pair returns the same items for a fixed ledger state.

**Limitations:**

- This example does **not** support deletion or reordering. Removing items would leave holes and break cursor semantics.
- Cursors are absolute indices, not content hashes. If you need pagination over a mutating set, maintain a separate stable ordering index (see `event-history` for ring-buffer style history).
- Concurrent writes between paginated reads can insert new items after the cursor; clients may see new tail items on later pages. For strict snapshots, read `count` first and treat pagination as a best-effort view, or pause writes during export.
- Very large `page_size` values are clamped to `MAX_PAGE_SIZE` to bound work per invocation.

## Building and Testing

From the repository root (package is a workspace member):

```bash
cargo test -p storage-pagination
cargo build --target wasm32-unknown-unknown --release -p storage-pagination
```

## Related Examples

- [`event-history`](../event-history/) — audit log with cursor pagination and capacity trimming
- [`iterable-mappings`](../iterable-mappings/) — enumerable key/value maps
