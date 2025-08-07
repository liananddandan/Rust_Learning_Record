// Demonstrates mutable and immutable references
pub fn run() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("After change: {s}");

    // Only one mutable reference allowed at a time
    let r1 = &s; // immutable
    let r2 = &s; // immutable
    println!("r1 = {}, r2 = {}", r1, r2);

    // Can't combine mutable and immutable references in the same scope
    // let r3 = &mut s; // Error if r1 or r2 still used
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
