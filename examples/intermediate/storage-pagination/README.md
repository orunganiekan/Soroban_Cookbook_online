# Storage Pagination

Cursor-based pagination over a large, append-only on-chain collection, enabling
scalable reads that never exceed Soroban's per-call instruction/size limits.

## Per-item storage (why this scales)

Each item is stored under its **own** persistent key (`DataKey::Item(index)`), and a
single `DataKey::NextIndex` counter tracks the collection size. A page read loads only
the `page_size` entries it returns — it never deserializes the whole collection. This
is the key difference from a naive "store everything in one `Vec`" design, where every
read grows with the collection and eventually blows the instruction budget.

## Cursor-based pagination

`list(page_size, cursor)` returns a `Page { items, next_cursor }`:

- Pass `cursor = None` to fetch the first page.
- Feed the returned `next_cursor` back into the next `list` call.
- Stop when `next_cursor` is `None` (last page reached).

### Opaque cursor

The cursor is an **opaque** `Bytes` token — callers should treat it as a black box and
only pass back values produced by the contract. It is **not** Base64; it is a fixed
8-byte binary payload:

```
[ magic/version : u32 big-endian ][ item index : u32 big-endian ]
```

The magic/version marker lets the contract reject foreign or corrupted cursors and
evolve the format later. Encoding is deterministic: the same index always encodes to
the same bytes (`encode_cursor_for_index` exposes this for clients that resume from a
known position).

## Page-size limits

- `page_size` must be greater than zero.
- Requests larger than `MAX_PAGE_SIZE` (50) are transparently capped to 50, so one call
  can never fan out into an unbounded read.

## Error handling (no panics on bad input)

`list` returns a `Result<Page, PaginationError>`:

- `PaginationError::InvalidPageSize` — `page_size` was zero.
- `PaginationError::InvalidCursor` — the cursor was malformed (wrong length or magic) or
  pointed strictly beyond the end of the collection.

A cursor pointing **exactly** at the end of the collection is valid and returns an empty
final page with `next_cursor = None`. `get_item(index)` is a direct accessor and panics
on an out-of-range index, matching the example's accessor convention.

## API

| Function | Description |
| --- | --- |
| `add_item(item)` | Append an item, assigning the next index. |
| `list(page_size, cursor)` | Return one `Page`; `Result<Page, PaginationError>`. |
| `count()` | Total number of items. |
| `get_item(index)` | Direct lookup by absolute index (panics if absent). |
| `encode_cursor_for_index(index)` | Build the opaque cursor for a known index. |

## Build

```bash
cargo build --target wasm32v1-none --release -p storage-pagination
```

## Test

```bash
cargo test -p storage-pagination
```
