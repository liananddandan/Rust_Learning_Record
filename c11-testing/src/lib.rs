/// Library code used by unit tests (inside `src`) and integration tests (inside `tests/`).
/// This crate demonstrates assertions, `should_panic`, `Result`-based tests, ignored tests,
/// testing private functions, and the difference between unit and integration tests.

/// Adds two integers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns a greeting string.
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// A simple type that panics on invalid construction to demonstrate `#[should_panic]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Guess {
    value: u32,
}

impl Guess {
    /// Creates a new `Guess`. Valid range: 1..=100. Panics otherwise.
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 { self.value }
}

// Private helper — unit tests in this module can still access it.
fn internal_adder(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
        assert_ne!(add(2, 2), 5, "2 + 2 should not be 5");
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Alice";
        let msg = greeting(name);
        assert!(msg.contains(name), "greeting '{msg}' should contain name '{name}'");
    }

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn guess_panics_on_out_of_range() {
        // This should panic; the expected message substring is checked.
        let _ = Guess::new(0);
    }

    #[test]
    fn result_based_test() -> Result<(), String> {
        if add(40, 2) == 42 { Ok(()) } else { Err("math is broken".into()) }
    }

    #[test]
    fn can_access_private_fn() {
        // Unit tests can access private items from the same module/crate.
        assert_eq!(internal_adder(1, 2), 3);
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn slow_or_external_test() {
        // Pretend this is slow (network, large IO, etc.)
        // We'll just assert true here.
        assert!(true);
    }
}