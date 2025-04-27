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
    )
}
