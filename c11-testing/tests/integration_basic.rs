// Each file in `tests/` compiles as a separate test binary that uses the library crate.
// You import the crate by its package name from Cargo.toml (here: `ch11_testing`).

use ch11_testing::*;
mod common;

#[test]
fn integration_add_and_greet() {
    common::setup();
    assert_eq!(add(3, 4), 7);

    let msg = greeting("Bob");
    assert!(msg.starts_with("Hello,"));
    assert!(msg.ends_with("!"));
}

#[test]
fn construct_valid_guess() {
    let g = Guess::new(42);
    assert_eq!(g.value(), 42);
}