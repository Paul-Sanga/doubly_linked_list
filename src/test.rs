#[cfg(test)]
use super::*;

#[test]
fn test_add_first() {
    let mut list = DoublyLinkedList::new();
    list.add_first(5);
    list.add_first(6);
    list.add_first(7);
    assert_eq!(list.size(), 3);
    assert_eq!(list.head(), Some(7));
    assert_eq!(list.tail(), Some(5));
}

#[test]
fn test_add_last() {
    let mut list = DoublyLinkedList::new();
    list.add_last(5);
    list.add_last(6);
    list.add_last(7);
    list.add_last(8);
    assert_eq!(list.head(), Some(5));
    assert_eq!(list.tail(), Some(8));
    assert_eq!(list.size(), 4);
}

#[test]
fn test_remove_first() {
    let mut list = DoublyLinkedList::new();
    list.add_last(5);
    list.add_last(6);
    list.add_last(7);
    list.add_last(8);
    assert_eq!(list.head(), Some(5));
    assert_eq!(list.tail(), Some(8));
    assert_eq!(list.size(), 4);
    list.remove_first().unwrap();
    list.remove_first().unwrap();
    assert_eq!(list.head(), Some(7));
    assert_eq!(list.tail(), Some(8));
    assert_eq!(list.size, 2);
    list.remove_first().unwrap();
    list.remove_first().unwrap();
    assert_eq!(list.size(), 0);
    assert_eq!(list.head(), None);
    assert_eq!(list.tail(), None);
    assert_eq!(
        list.remove_first(),
        Err("Can not remove from an empty list".to_string())
    );
}

#[test]
fn test_remove_last() {
    let mut list = DoublyLinkedList::new();
    list.add_first(5);
    list.add_first(6);
    list.add_first(7);
    assert_eq!(list.size(), 3);
    assert_eq!(list.head(), Some(7));
    assert_eq!(list.tail(), Some(5));
    list.remove_last().unwrap();
    assert_eq!(list.size(), 2);
    assert_eq!(list.head(), Some(7));
    assert_eq!(list.tail(), Some(6));
    list.remove_last().unwrap();
    assert_eq!(list.size(), 1);
    assert_eq!(list.head(), Some(7));
    assert_eq!(list.tail(), Some(7));
    list.remove_last().unwrap();
    assert_eq!(list.size(), 0);
    assert_eq!(list.head(), None);
    assert_eq!(list.tail(), None);
    assert_eq!(
        list.remove_last(),
        Err("Can not remove from an empty list".to_string())
    );
}

#[test]
fn test_find_by_index_from_tail() {
    let mut list = DoublyLinkedList::new();
    list.add_first(5);
    list.add_first(6);
    list.add_first(7);
    assert_eq!(list.find_by_index_from_tail(2), Some(7));
    assert_eq!(list.find_by_index_from_tail(1), Some(6));
    assert_eq!(list.find_by_index_from_tail(0), Some(5));
    assert_eq!(list.find_by_index_from_tail(300), None);
}

#[test]
fn test_add_before() {
    let mut list = DoublyLinkedList::new();
    list.add_last(5);
    list.add_last(6);
    list.add_last(7);
    list.add_last(8);
    list.add_before(5, 10).unwrap();
    assert_eq!(list.size(), 5);
    assert_eq!(list.head(), Some(10));
}
