use std::fs;
use std::io;

// Demonstrates recoverable errors using `Result`, `match`,
// `unwrap`, and `expect`.
pub fn run() {
    // A. Using `match` to handle Result explicitly
    match fs::read_to_string("README.md") {
        Ok(content) => println!("Read README.md OK ({} bytes)", content.len()),
        Err(err) => println!("Couldn't read README.md: {err}"),
    }

    // B. Using `unwrap` (will panic on Err)
    // Commented out to keep demo safe:
    // let content = fs::read_to_string("definitely_missing.txt").unwrap();

    // C. Using `expect` with a better error message (still panics on Err)
    // Commented out to keep demo safe:
    // let content = fs::read_to_string("missing.txt")
    //     .expect("Failed to read missing.txt");

    // D. Mapping IO errors for nicer branching
    match fs::read_to_string("maybe.txt") {
        Ok(_) => println!("maybe.txt read OK"),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("maybe.txt not found; that's fine, continuing...")
        }
        Err(e) => println!("maybe.txt failed with unexpected error: {e}"),
    }
}
