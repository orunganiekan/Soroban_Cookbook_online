//! # Storage Pagination Contract
//!
//! Cursor-based pagination over a large on-chain collection. Each item is stored
//! under its own persistent key so a page read only loads `page_size` entries
//! instead of deserializing the full collection.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Bytes, Env, Symbol, Vec,
};

/// Maximum items returned per `list` call (clients may request less).
pub const MAX_PAGE_SIZE: u32 = 50;

const CURSOR_MAGIC: u32 = 0x5047_0001; // "PG" + version 1
const CURSOR_LEN: u32 = 8;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    NextIndex,
    Item(u32),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    pub items: Vec<Symbol>,
    /// Opaque cursor for the next page (`None` when there is no next page).
    pub next_cursor: Option<Bytes>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PaginationError {
    InvalidPageSize = 1,
    InvalidCursor = 2,
}

#[contract]
pub struct PaginationContract;

#[contractimpl]
impl PaginationContract {
    /// Append an item to the collection.
    pub fn add_item(env: Env, item: Symbol) {
        let index = read_next_index(&env);
        env.storage()
            .persistent()
            .set(&DataKey::Item(index), &item);
        env.storage()
            .instance()
            .set(&DataKey::NextIndex, &(index + 1));
    }

    /// Return one page of items.
    ///
    /// * `page_size` — bounded by [`MAX_PAGE_SIZE`]; must be greater than zero.
    /// * `cursor` — opaque token from a previous `Page::next_cursor`, or `None` for the first page.
    pub fn list(
        env: Env,
        page_size: u32,
        cursor: Option<Bytes>,
    ) -> Result<Page, PaginationError> {
        if page_size == 0 {
            return Err(PaginationError::InvalidPageSize);
        }

        let limit = page_size.min(MAX_PAGE_SIZE);
        let total = read_next_index(&env);

        let start = match cursor {
            None => 0,
            Some(token) => decode_cursor(&token)?,
        };

        if start > total {
            return Err(PaginationError::InvalidCursor);
        }

        if start >= total {
            return Ok(Page {
                items: Vec::new(&env),
                next_cursor: None,
            });
        }

        let end = start.saturating_add(limit).min(total);
        let mut page_items = Vec::new(&env);
        let mut index = start;
        while index < end {
            if let Some(item) = env.storage().persistent().get(&DataKey::Item(index)) {
                page_items.push_back(item);
            }
            index += 1;
        }

        let next_cursor = if end >= total {
            None
        } else {
            Some(encode_cursor(&env, end))
        };

        Ok(Page {
            items: page_items,
            next_cursor,
        })
    }

    /// Total number of items stored.
    pub fn count(env: Env) -> u32 {
        read_next_index(&env)
    }

    /// Fetch a single item by absolute index.
    pub fn get_item(env: Env, index: u32) -> Symbol {
        env.storage()
            .persistent()
            .get(&DataKey::Item(index))
            .unwrap_or_else(|| panic!("Index out of bounds"))
    }

    /// Encode an absolute index as the opaque cursor returned by `list`.
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

/// Stable cursor payload: `[magic: u32 BE][index: u32 BE]` (8 bytes).
fn encode_cursor(env: &Env, index: u32) -> Bytes {
    let mut bytes = Bytes::new(env);
    bytes.extend_from_slice(&CURSOR_MAGIC.to_be_bytes());
    bytes.extend_from_slice(&index.to_be_bytes());
    bytes
}

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

    let index = u32::from_be_bytes([
        cursor.get(4).unwrap(),
        cursor.get(5).unwrap(),
        cursor.get(6).unwrap(),
        cursor.get(7).unwrap(),
    ]);
    Ok(index)
}

#[cfg(test)]
mod test;
