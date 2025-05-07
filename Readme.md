# 📚 Doubly Linked List in Rust

[![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

---

This project implements a generic, safe, and efficient **doubly linked list** in [Rust](https://www.rust-lang.org/). The list supports bidirectional traversal and standard list operations like insertions, deletions, and searching.

Built with the goals of mastering ownership, borrowing, and interior mutability in Rust.

---

## 🚀 Features

- 📦 Generic over any type `T`
- ➕ Insertion at both **head** and **tail**
- ➖ Deletion from both **head** and **tail**
- 🔍 Search for elements
- 🔁 Forward iteration via the `Iterator` trait
- 📏 Constant time updates at both ends
- 🧠 Uses `Rc<RefCell<>>` and `Weak` to manage memory safely

---

## 📂 Project Structure

├── src/
│ ├── lib.rs # DoublyLinkedList definition
│ └── test.rs # Unit tests
├── Cargo.toml
└── README.md

## 📌 Example

use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut list = DoublyLinkedList::new();

    list.add_last(10);
    list.add_last(20);
    list.add_first(5);

    println!("List contents:");
    for val in list.iter() {
        println!("{}", val);
    }

    list.remove_last().unwrap();
    list.remove_first().unwrap();

    println!("After deletion:");
    for val in list.iter() {
        println!("{}", val);
    }
}

## ✅ Supported Methods

// Constructor
let mut list = DoublyLinkedList::new();

// Insertions
list.add_first(value);
list.add_last(value);

// Deletions
list.remove_first();
list.remove_last();

// Iteration
for val in list.iter() {
    println!("{}", val);
}

// Search
list.contains(&value);

## 🧪 Running Tests

cargo test

## 🤔 Why Use Rc<RefCell<T>>?

Rust does not allow multiple mutable references by default. To build a safe doubly linked list:

Rc<T> is used for shared ownership of nodes.

RefCell<T> gives interior mutability.

Weak<T> avoids reference cycles in the prev pointer.

## 📚 Learning Goals

This project was created to solidify the following Rust concepts:

Ownership and borrowing

Interior mutability with RefCell

Smart pointer safety (Rc vs Weak)

Trait implementation (Iterator)

Data structure design in systems programming

