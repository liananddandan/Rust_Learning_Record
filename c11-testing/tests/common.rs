// Shared helpers for integration tests. Any file in `tests/` can `mod common;` and use `common::setup()`.

pub fn setup() {
    // Put shared initialization here (env vars, logging, test data dirs, etc.).
    // Keeping it simple for the demo.
    println!("[common::setup] integration test setup complete");
}