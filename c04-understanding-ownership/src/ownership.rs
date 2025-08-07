// Demonstrates basic ownership and move semantics
pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    println!("s2: {s2}");
    // println!("s1: {s1}"); // Error: s1 is moved

    let s3 = String::from("world");
    let len = calculate_length(&s3); // Borrowing
    println!("The length of '{}' is {}.", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
