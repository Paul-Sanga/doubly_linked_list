# Doubly Linked List in Rust

[![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

---

This project contains a simple, from-scratch implementation of a **Doubly Linked List** in Rust.  


## Features

- Insertion at the head and tail
- Deletion from the head and tail
- Traversal (forwards and backwards)
- Search for elements
- Safe API following Rust’s ownership and borrowing principles

## Structure

- Each node holds:
  - A value
  - A link to the next node (`Option<Rc<RefCell<Node>>>`)
  - A link to the previous node (using raw pointers or `Option<Weak<RefCell<Node>>>`, depending on implementation)
- The list keeps track of:
  - The head node
  - The tail node
  - The length


