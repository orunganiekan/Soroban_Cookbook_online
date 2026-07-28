#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Bytes, Env, Symbol};

fn setup() -> (Env, PaginationContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(PaginationContract, ());
    let client = PaginationContractClient::new(&env, &contract_id);
    (env, client)
}

fn item_symbol(env: &Env, i: u32) -> Symbol {
    let label = [
        (b'a' + (i / 26) as u8) as char,
        (b'a' + (i % 26) as u8) as char,
    ];
    let bytes = [label[0] as u8, label[1] as u8];
    Symbol::new(env, core::str::from_utf8(&bytes).unwrap())
}

fn add_numbered_items(env: &Env, client: &PaginationContractClient<'_>, count: u32) {
    for i in 0..count {
        client.add_item(&item_symbol(env, i));
    }
}

#[test]
fn test_add_and_retrieve_items() {
    let (_env, client) = setup();

    client.add_item(&symbol_short!("item1"));
    client.add_item(&symbol_short!("item2"));
    client.add_item(&symbol_short!("item3"));

    assert_eq!(client.count(), 3);
}

#[test]
fn test_pagination_first_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let page = client.list(&5, &None);
    assert_eq!(page.items.len(), 5);
    assert!(page.next_cursor.is_some());
    let cursor = page.next_cursor.unwrap();
    assert_eq!(decode_cursor(&cursor).unwrap(), 5);
}

#[test]
fn test_pagination_second_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let first = client.list(&5, &None);
    let page = client.list(&5, &first.next_cursor);
    assert_eq!(page.items.len(), 5);
    assert!(page.next_cursor.is_none());
}

#[test]
fn test_pagination_partial_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 10);

    let start = client.encode_cursor_for_index(&8);
    let page = client.list(&5, &Some(start));
    assert_eq!(page.items.len(), 2);
    assert!(page.next_cursor.is_none());
}

#[test]
fn test_pagination_empty_collection() {
    let (_env, client) = setup();

    let page = client.list(&10, &None);
    assert_eq!(page.items.len(), 0);
    assert!(page.next_cursor.is_none());
}

#[test]
fn test_pagination_cursor_beyond_bounds() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 5);

    let cursor = client.encode_cursor_for_index(&10);
    assert_eq!(
        client.try_list(&5, &Some(cursor)),
        Err(Ok(PaginationError::InvalidCursor))
    );
}

#[test]
fn test_pagination_at_end_returns_empty_page() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 5);

    let cursor = client.encode_cursor_for_index(&5);
    let page = client.list(&5, &Some(cursor));
    assert_eq!(page.items.len(), 0);
    assert!(page.next_cursor.is_none());
}

#[test]
fn test_get_item_by_index() {
    let (_env, client) = setup();

    client.add_item(&symbol_short!("A"));
    client.add_item(&symbol_short!("B"));
    client.add_item(&symbol_short!("C"));

    assert_eq!(client.get_item(&0), symbol_short!("A"));
    assert_eq!(client.get_item(&1), symbol_short!("B"));
    assert_eq!(client.get_item(&2), symbol_short!("C"));
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_get_item_out_of_bounds() {
    let (_env, client) = setup();

    client.add_item(&symbol_short!("A"));
    client.get_item(&10);
}

#[test]
fn test_invalid_page_size() {
    let (_env, client) = setup();

    client.add_item(&symbol_short!("A"));
    assert_eq!(
        client.try_list(&0, &None),
        Err(Ok(PaginationError::InvalidPageSize))
    );
}

#[test]
fn test_invalid_cursor_payload() {
    let (env, client) = setup();
    let bad = Bytes::from_array(&env, &[1, 2, 3]);
    assert_eq!(
        client.try_list(&5, &Some(bad)),
        Err(Ok(PaginationError::InvalidCursor))
    );
}

#[test]
fn test_cursor_encoding_is_stable() {
    let (_env, client) = setup();
    let a = client.encode_cursor_for_index(&42);
    let b = client.encode_cursor_for_index(&42);
    assert_eq!(a, b);
    assert_eq!(decode_cursor(&a).unwrap(), 42);
}

#[test]
fn test_page_size_capped_at_max() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 60);

    let page = client.list(&1_000, &None);
    assert_eq!(page.items.len(), MAX_PAGE_SIZE);
    assert!(page.next_cursor.is_some());
}

#[test]
fn test_pagination_full_flow() {
    let (env, client) = setup();
    add_numbered_items(&env, &client, 25);

    let mut cursor = None;
    let mut total_items = 0;

    loop {
        let page = client.list(&10, &cursor);
        total_items += page.items.len();

        match page.next_cursor {
            Some(c) => cursor = Some(c),
            None => break,
        }
    }

    assert_eq!(total_items, 25);
}
