#![allow(dead_code)]
use std::{
    cell::RefCell,
    mem::take,
    rc::{Rc, Weak},
};

mod test;

#[derive(Debug)]
pub struct Node {
    value: u32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: u32) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList {
    size: usize,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn head(&mut self) -> Option<u32> {
        self.head.as_ref().map(|head| head.borrow().value)
    }

    pub fn tail(&mut self) -> Option<u32> {
        self.tail
            .as_ref()
            .and_then(|tail| tail.upgrade())
            .map(|tail| tail.borrow().value)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_first(&mut self, value: u32) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        if self.size == 0 {
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
        } else {
            if let Some(head) = self.head.take() {
                node.borrow_mut().next = Some(Rc::clone(&head));
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
        }

        self.size += 1;
    }

    pub fn add_last(&mut self, value: u32) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.size == 0 {
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
        } else {
            if let Some(tail) = self.tail.take() {
                if let Some(tail) = tail.upgrade() {
                    node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                    tail.borrow_mut().next = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                }
            }
        }
        self.size += 1;
    }

    pub fn remove_first(&mut self) -> Result<(), String> {
        if self.size > 1 {
            if let Some(old_head) = self.head.take() {
                if let Some(new_head) = old_head.borrow_mut().next.take() {
                    self.head = Some(Rc::clone(&new_head))
                }
            }
        } else if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else {
            return Err("Can not remove from an empty list".to_string());
        }
        self.size -= 1;
        Ok(())
    }
}
