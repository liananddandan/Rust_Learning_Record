// Chapter 4: Understanding Ownership

mod ownership;
mod references;
mod slices;

fn main() {
    println!("--- Ownership Demo ---");
    ownership::run();

    println!("\n--- References and Borrowing Demo ---");
    references::run();

    println!("\n--- Slice Type Demo ---");
    slices::run();
}
