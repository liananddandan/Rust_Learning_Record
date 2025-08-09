mod panic_examples;
mod result_examples;
mod propagation;

fn main() {
    println!("=== Chapter 9: Error Handling ===\n");

    println!("-- panic! examples --");
    panic_examples::run();

    println!("\n-- Result basics (recoverable errors) --");
    result_examples::run();

    println!("\n-- Propagating errors with ? --");
    match propagation::username_from_file("examples/username.txt") {
        Ok(name) => println!("username_from_file: {}", name),
        Err(e) => println!("username_from_file error: {e}"),
    }

    // A shorter version using `?` can only be used in functions that return Result,
    // so we demo it inside that module.
}
