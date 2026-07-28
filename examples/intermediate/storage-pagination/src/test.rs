#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Bytes, Env, Symbol};

fn setup() -> (Env, PaginationContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(PaginationContract, ());
    let client = PaginationContractClient::new(&env, &contract_id);
    (env, client)
}

/// Deterministic two-letter symbol for item `i` (e.g. 0 -> "aa", 27 -> "bb").
fn item_symbol(env: &Env, i: u32) -> Symbol {
    let bytes = [b'a' + (i / 26) as u8, b'a' + (i % 26) as u8];
    Symbol::new(env, core::str::from_utf8(&bytes).unwrap())
}

fn add_numbered_items(env: &Env, client: &PaginationContractClient<'_>, count: u32) {
    for i in 0..count {
        client.add_item(&item_symbol(env, i));
    }
}

// 1. Adding items and counting them.
#[test]
fn test_add_and_count_items() {
    let (_env, client) = setup();
    assert_eq!(client.count(), 0);
    client.add_item(&symbol_short!("item1"));
    client.add_item(&symbol_short!("item2"));
    client.add_item(&symbol_short!("item3"));
    assert_eq!(client.count(), 3);
}

// 2. First page.
#[test]
fn test_first_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let page = client.list(&5, &None);
    assert_eq!(page.items.len(), 5);
    assert_eq!(page.items.get(0).unwrap(), item_symbol(&env, 0));
    assert_eq!(page.items.get(4).unwrap(), item_symbol(&env, 4));
    assert!(page.next_cursor.is_some());
    assert_eq!(decode_cursor(&page.next_cursor.unwrap()).unwrap(), 5);
}

// 3. Second page using the returned cursor.
#[test]
fn test_second_page_via_cursor() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let first = client.list(&5, &None);
    let page = client.list(&5, &first.next_cursor);
    assert_eq!(page.items.len(), 5);
    assert_eq!(page.items.get(0).unwrap(), item_symbol(&env, 5));
    assert_eq!(page.items.get(4).unwrap(), item_symbol(&env, 9));
    assert!(page.next_cursor.is_none());
}

// 4. Partial final page.
#[test]
fn test_partial_final_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let start = client.encode_cursor_for_index(&8);
    let page = client.list(&5, &Some(start));
    assert_eq!(page.items.len(), 2);
    assert_eq!(page.items.get(0).unwrap(), item_symbol(&env, 8));
    assert_eq!(page.items.get(1).unwrap(), item_symbol(&env, 9));
    assert!(page.next_cursor.is_none());
}

// 5. Empty collection.
#[test]
fn test_empty_collection() {
    let (_env, client) = setup();
    let page = client.list(&10, &None);
    assert_eq!(page.items.len(), 0);
    assert!(page.next_cursor.is_none());
}

// 6. Cursor strictly beyond collection bounds -> InvalidCursor.
#[test]
fn test_cursor_beyond_bounds_is_invalid() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 5);

    let cursor = client.encode_cursor_for_index(&10);
    assert_eq!(
        client.try_list(&5, &Some(cursor)),
        Err(Ok(PaginationError::InvalidCursor))
    );
}

// 7. Cursor exactly at the end -> empty final page (not an error).
#[test]
fn test_cursor_at_end_returns_empty_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 5);

    let cursor = client.encode_cursor_for_index(&5);
    let page = client.list(&5, &Some(cursor));
    assert_eq!(page.items.len(), 0);
    assert!(page.next_cursor.is_none());
}

// 8. get_item.
#[test]
fn test_get_item() {
    let (_env, client) = setup();
    client.add_item(&symbol_short!("A"));
    client.add_item(&symbol_short!("B"));
    client.add_item(&symbol_short!("C"));

    assert_eq!(client.get_item(&0), symbol_short!("A"));
    assert_eq!(client.get_item(&1), symbol_short!("B"));
    assert_eq!(client.get_item(&2), symbol_short!("C"));
}

// 9. Out-of-range item lookup panics ("Index out of bounds").
#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_get_item_out_of_range_panics() {
    let (_env, client) = setup();
    client.add_item(&symbol_short!("A"));
    client.get_item(&10);
}

// 10. Invalid page size -> InvalidPageSize.
#[test]
fn test_invalid_page_size() {
    let (_env, client) = setup();
    client.add_item(&symbol_short!("A"));
    assert_eq!(
        client.try_list(&0, &None),
        Err(Ok(PaginationError::InvalidPageSize))
    );
}

// 11. Invalid cursor payload (wrong length/magic) -> InvalidCursor.
#[test]
fn test_invalid_cursor_payload() {
    let (env, client) = setup();

    // Too short.
    let short = Bytes::from_array(&env, &[1, 2, 3]);
    assert_eq!(
        client.try_list(&5, &Some(short)),
        Err(Ok(PaginationError::InvalidCursor))
    );

    // Correct length, wrong magic.
    let bad_magic = Bytes::from_array(&env, &[0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        client.try_list(&5, &Some(bad_magic)),
        Err(Ok(PaginationError::InvalidCursor))
    );
}

// 12. Cursor encoding is deterministic/stable and round-trips.
#[test]
fn test_cursor_encoding_is_stable() {
    let (_env, client) = setup();
    let a = client.encode_cursor_for_index(&42);
    let b = client.encode_cursor_for_index(&42);
    assert_eq!(a, b);
    assert_eq!(a.len(), 8);
    assert_eq!(decode_cursor(&a).unwrap(), 42);
}

// 13. Page size is capped at MAX_PAGE_SIZE.
#[test]
fn test_page_size_capped_at_max() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 60);

    let page = client.list(&1_000, &None);
    assert_eq!(page.items.len(), MAX_PAGE_SIZE);
    assert!(page.next_cursor.is_some());
    assert_eq!(
        decode_cursor(&page.next_cursor.unwrap()).unwrap(),
        MAX_PAGE_SIZE
    );
}

// 14. Full end-to-end pagination flow visits every item exactly once.
// 15. Multiple pages with no duplicate or skipped items.
#[test]
fn test_full_flow_no_duplicates_or_gaps() {
    let (env, client) = setup();
    let total = 25u32;
    add_numbered_items(&env, &client, total);

    let mut cursor: Option<Bytes> = None;
    let mut seen = 0u32;
    loop {
        let page = client.list(&10, &cursor);
        for offset in 0..page.items.len() {
            // Items must arrive in strict ascending order with no gaps/repeats.
            assert_eq!(page.items.get(offset).unwrap(), item_symbol(&env, seen));
            seen += 1;
        }
        match page.next_cursor {
            Some(c) => cursor = Some(c),
            None => break,
        }
    }
    assert_eq!(seen, total);
}
