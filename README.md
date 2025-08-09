# Rust_Learning_Record

# 01 - Get Started: Hello, World!

This is the first Rust program, demonstrating the use of the `println!` macro to output text to the terminal.

## Cargo Commands

```bash
# Compile the project
cargo build

# Run the compiled binary
cargo run

# Check code for errors without building
cargo check

# Clean up target directory
cargo clean
```


# 02 - Programming a Guessing Game

This project implements a simple number guessing game in Rust.

## How it works

- The program generates a random number between 1 and 100.
- The user types guesses into the terminal.
- The program tells the user whether the guess is too low, too high, or correct.
- The game continues until the user guesses correctly.

## How to Run

```bash
cargo run
```
# 03 - Common Programming Concepts

This project demonstrates core Rust programming concepts:

- Variables and Mutability
- Data Types (scalar and compound)
- Functions and return values
- Control Flow (if, loop, for)

## Run

```bash
cargo run
```

# 04 - Understanding Ownership

This project demonstrates key concepts in Rust's ownership system:

- Move semantics
- Borrowing and references
- Mutable and immutable references
- Slices and string manipulation

## Run

```bash
cargo run
```

# 05 - Using Structs to Structure Related Data

This project demonstrates:

- Creating and updating structs
- Using field init shorthand and struct update syntax
- Tuple structs and unit-like structs
- Implementing methods and associated functions (`impl`)

## Run

```bash
cargo run
```

# 06 - Enums and Pattern Matching

This project demonstrates:

- Defining enums with and without associated data
- Using `Option<T>` and matching on it
- Pattern matching with `match`
- Using enum variants to store structured data

## Run

```bash
cargo run
```

# 07 - Managing Projects

This project demonstrates how to:

- Organize code into modules and submodules
- Use `mod` and `pub mod` to control visibility
- Re-export with `pub use` to simplify public APIs
- Use `super::` and `crate::` paths

## Run

```bash
cargo run
```

# Chapter 8: Common Collections in Rust

This project demonstrates the usage of Rust's three main collection types introduced in Chapter 8 of *The Rust Programming Language*:

- `Vec<T>`: growable vectors
- `String`: UTF-8 encoded string type
- `HashMap<K, V>`: key-value store

Each collection type is covered in a dedicated module with clear examples and explanations.

---

- `main.rs`: Entry point, calls each module in turn
- `vec_examples.rs`: Demonstrates operations on vectors (creation, borrowing, iteration, mutation)
- `string_examples.rs`: Explores string manipulation and UTF-8 slicing
- `hashmap_examples.rs`: Covers insertion, access, ownership, and counting words with `HashMap`

---

## Run

```bash
cargo run
```

# Chapter 9 — Error Handling (Rust)

This crate demonstrates the two big categories of errors in Rust:

- **Unrecoverable errors** via `panic!` (program aborts)
- **Recoverable errors** via `Result<T, E>` (you decide how to handle them)

It also shows the `?` operator for **error propagation**, and the difference between
`unwrap`, `expect`, and explicit `match`.

## Project Layout
src/
├── main.rs # Calls each example module
├── panic_examples.rs # panic! and out-of-bounds access
├── result_examples.rs # Result + match, unwrap, expect, IO error kinds
└── propagation.rs # Propagating errors with ? and tests

## Run

```bash
cargo run
```


# Chapter 10 - Generics, Traits, and Lifetimes

This Rust learning project demonstrates the key concepts from **Chapter 10** of *The Rust Programming Language*:

- **Generics**: Generic functions, structs, enums, and trait bounds
- **Traits**: Defining traits, default methods, implementing traits for types, and trait objects
- **Lifetimes**: Preventing dangling references, lifetime annotations, and the `'static` lifetime

---

## 📂 Project Structure

ch10_generics_traits_lifetimes/
├── Cargo.toml
└── src/
├── main.rs # Entry point, runs examples from other modules
├── generics.rs # Generic functions, structs, and trait bounds
├── traits_demo.rs # Trait definition, default methods, impl Trait
├── trait_objects.rs # Using trait objects and dynamic dispatch
└── lifetimes.rs # Lifetime annotations and examples

```bash
cargo run
```
