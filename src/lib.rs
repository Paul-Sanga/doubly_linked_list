#![allow(dead_code)]
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};

mod test;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug + Copy + Display + PartialEq + PartialOrd> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    size: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug + Copy + Display + PartialEq + PartialOrd> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|head| head.borrow().value)
    }

    pub fn tail(&self) -> Option<T> {
        self.tail
            .as_ref()
            .and_then(|tail| tail.upgrade())
            .map(|tail| tail.borrow().value)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_first(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        if self.size == 0 {
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
        } else {
            self.head.as_ref().map(|head| {
                node.borrow_mut().next = Some(Rc::clone(&head));
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
            });
            self.head = Some(node);
        }

        self.size += 1;
    }

    pub fn add_last(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.size == 0 {
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
        } else {
            self.tail
                .as_ref()
                .and_then(|tail| tail.upgrade())
                .map(|tail| {
                    node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                    tail.borrow_mut().next = Some(Rc::clone(&node))
                });
            self.tail = Some(Rc::downgrade(&node));
        }
        self.size += 1;
    }

    pub fn remove_first(&mut self) -> Result<(), String> {
        if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else if self.size > 1 {
            if let Some(old_head) = self.head.take() {
                if let Some(new_head) = old_head.borrow_mut().next.take() {
                    self.head = Some(Rc::clone(&new_head))
                }
            }
        } else {
            return Err("Can not remove from an empty list".to_string());
        }
        self.size -= 1;
        Ok(())
    }

    pub fn remove_last(&mut self) -> Result<(), String> {
        if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else if self.size > 1 {
            if let Some(old_tail) = self.tail.take() {
                if let Some(old_tail) = old_tail.upgrade() {
                    if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                        if let Some(new_tail) = new_tail.upgrade() {
                            self.tail = Some(Rc::downgrade(&new_tail));
                        }
                    }
                }
            }
        } else {
            return Err("Can not remove from an empty list".to_string());
        }
        self.size -= 1;
        Ok(())
    }

    pub fn add_before(&mut self, seek: T, value: T) -> Result<(), String> {
        let mut current_opt = self.head.clone();
        while let Some(current_node) = current_opt {
            if current_node.borrow().value == seek {
                let new_node = Rc::new(RefCell::new(Node::new(value)));
                new_node.borrow_mut().next = Some(Rc::clone(&current_node));
                if let Some(prev_node) = current_node.borrow().prev.as_ref() {
                    if let Some(prev_node) = prev_node.upgrade() {
                        prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
                        new_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
                    }
                } else {
                    self.head = Some(Rc::clone(&new_node));
                }
                current_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.size += 1;
                return Ok(());
            }
            current_opt = current_node
                .borrow()
                .next
                .as_ref()
                .map(|node| Rc::clone(node));
        }
        Err(format!("Value {seek} not found in list"))
    }

    pub fn find_by_index_from_tail(&self, index: usize) -> Option<T> {
        let mut current_opt = self.tail.as_ref()?.upgrade();
        for _ in 0..index {
            current_opt = current_opt?.borrow().prev.as_ref()?.upgrade();
        }
        current_opt.map(|node| node.borrow().value)
    }

    pub fn iter(&self) -> DoublyLinkedListIter<T> {
        DoublyLinkedListIter {
            current: self.head.clone(),
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            let next = node.borrow_mut().next.take();
            current = next;
        }
        self.tail = None;
        self.size = 0;
    }
}

pub struct DoublyLinkedListIter<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Iterator for DoublyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current.take() {
            self.current = node.borrow().next.clone();
            Some(node.borrow().value)
        } else {
            None
        }
    }
}
