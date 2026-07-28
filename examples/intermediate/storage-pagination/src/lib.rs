//! # Storage Pagination Contract
//!
//! Cursor-based pagination over a large, append-only on-chain collection.
//!
//! Each item is stored under its **own** persistent key (`DataKey::Item(index)`)
//! rather than inside one big `Vec`. A page read therefore loads only the
//! `page_size` entries it returns instead of deserializing the entire collection,
//! which keeps each call within Soroban's instruction/size limits no matter how
//! large the collection grows.
//!
//! Callers page through the collection with an **opaque** cursor (`Bytes`): they
//! pass `None` for the first page and then feed each returned `Page::next_cursor`
//! back into the next `list` call until it is `None`.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Bytes, Env, Symbol, Vec};

/// Maximum number of items returned by a single `list` call.
///
/// Clients may request fewer; larger requests are transparently capped to this
/// value so a single call can never fan out into an unbounded read.
pub const MAX_PAGE_SIZE: u32 = 50;

/// Magic/version marker embedded in every cursor (`"PG"` + version 1).
///
/// Encoded as a big-endian `u32`; lets us reject foreign or corrupted cursors and
/// evolve the cursor format later without silently misreading old tokens.
const CURSOR_MAGIC: u32 = 0x5047_0001;

/// Exact byte length of a valid cursor: `magic (4) + index (4)`.
const CURSOR_LEN: u32 = 8;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Next index to assign (also the total item count). Kept in instance storage.
    NextIndex,
    /// An individual item, keyed by its absolute index. Kept in persistent storage.
    Item(u32),
}

/// One page of results returned by [`PaginationContract::list`].
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    /// The items in this page (up to `min(page_size, MAX_PAGE_SIZE)` of them).
    pub items: Vec<Symbol>,
    /// Opaque cursor for the next page, or `None` when this is the last page.
    pub next_cursor: Option<Bytes>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PaginationError {
    /// `page_size` was zero.
    InvalidPageSize = 1,
    /// The cursor was malformed, had the wrong magic/length, or pointed past the end.
    InvalidCursor = 2,
}

#[contract]
pub struct PaginationContract;

#[contractimpl]
impl PaginationContract {
    /// Append an item to the collection, assigning it the next absolute index.
    ///
    /// Each item is written under its own key so appends and page reads stay O(1)
    /// in the size of the collection.
    pub fn add_item(env: Env, item: Symbol) {
        let index = read_next_index(&env);
        env.storage().persistent().set(&DataKey::Item(index), &item);
        env.storage()
            .instance()
            .set(&DataKey::NextIndex, &(index + 1));
    }

    /// Return one page of items.
    ///
    /// * `page_size` — must be greater than zero; capped at [`MAX_PAGE_SIZE`].
    /// * `cursor` — an opaque token from a previous [`Page::next_cursor`], or `None`
    ///   to start at the beginning.
    ///
    /// # Errors
    /// * [`PaginationError::InvalidPageSize`] if `page_size == 0`.
    /// * [`PaginationError::InvalidCursor`] if the cursor is malformed or points
    ///   strictly beyond the end of the collection.
    pub fn list(env: Env, page_size: u32, cursor: Option<Bytes>) -> Result<Page, PaginationError> {
        if page_size == 0 {
            return Err(PaginationError::InvalidPageSize);
        }

        let limit = page_size.min(MAX_PAGE_SIZE);
        let total = read_next_index(&env);

        let start = match cursor {
            None => 0,
            Some(token) => decode_cursor(&token)?,
        };

        // A cursor pointing strictly past the end is invalid; a cursor pointing
        // exactly at the end is valid and yields an empty final page.
        if start > total {
            return Err(PaginationError::InvalidCursor);
        }
        if start == total {
            return Ok(Page {
                items: Vec::new(&env),
                next_cursor: None,
            });
        }

        let end = start.saturating_add(limit).min(total);
        let mut items = Vec::new(&env);
        let mut index = start;
        while index < end {
            if let Some(item) = env.storage().persistent().get(&DataKey::Item(index)) {
                items.push_back(item);
            }
            index += 1;
        }

        let next_cursor = if end >= total {
            None
        } else {
            Some(encode_cursor(&env, end))
        };

        Ok(Page { items, next_cursor })
    }

    /// Total number of items stored.
    pub fn count(env: Env) -> u32 {
        read_next_index(&env)
    }

    /// Fetch a single item by its absolute index.
    ///
    /// # Panics
    /// Panics if no item exists at `index` (out-of-range lookup), matching the
    /// direct-accessor convention of the existing example.
    pub fn get_item(env: Env, index: u32) -> Symbol {
        env.storage()
            .persistent()
            .get(&DataKey::Item(index))
            .unwrap_or_else(|| panic!("Index out of bounds"))
    }

    /// Encode an absolute index into the opaque cursor format used by `list`.
    ///
    /// Useful for clients that want to resume from a known position.
    pub fn encode_cursor_for_index(env: Env, index: u32) -> Bytes {
        encode_cursor(&env, index)
    }
}

fn read_next_index(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::NextIndex)
        .unwrap_or(0)
}

/// Build a stable cursor payload: `[magic: u32 BE][index: u32 BE]` (8 bytes).
fn encode_cursor(env: &Env, index: u32) -> Bytes {
    let mut bytes = Bytes::new(env);
    bytes.extend_from_slice(&CURSOR_MAGIC.to_be_bytes());
    bytes.extend_from_slice(&index.to_be_bytes());
    bytes
}

/// Decode and validate a cursor, returning the absolute index it points to.
fn decode_cursor(cursor: &Bytes) -> Result<u32, PaginationError> {
    if cursor.len() != CURSOR_LEN {
        return Err(PaginationError::InvalidCursor);
    }

    let magic = u32::from_be_bytes([
        cursor.get(0).unwrap(),
        cursor.get(1).unwrap(),
        cursor.get(2).unwrap(),
        cursor.get(3).unwrap(),
    ]);
    if magic != CURSOR_MAGIC {
        return Err(PaginationError::InvalidCursor);
    }

    Ok(u32::from_be_bytes([
        cursor.get(4).unwrap(),
        cursor.get(5).unwrap(),
        cursor.get(6).unwrap(),
        cursor.get(7).unwrap(),
    ]))
}

#[cfg(test)]
mod test;
