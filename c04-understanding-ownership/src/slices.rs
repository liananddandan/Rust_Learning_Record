// Demonstrates string slicing
pub fn run() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("First word: {hello}, Second word: {world}");

    let first = first_word(&s);
    println!("First word (slice): {first}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
