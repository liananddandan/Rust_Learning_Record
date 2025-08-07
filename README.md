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