pub fn run() {
    // Create string from literal
    let data = "initial contents";
    let s = data.to_string();

    // Equivalent way
    let s2 = String::from("initial contents");

    // Append using push_str and push
    let mut s3 = String::from("Hello");
    s3.push_str(", world!");
    s3.push('!');

    println!("s3 = {s3}");

    // Concatenate with + operator (moves left-hand string)
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = s4 + "-" + &s5 + "-" + &s6; // s4 is moved here

    println!("Concatenated string: {s7}");

    // Use format! for safer concatenation (no move)
    let s8 = format!("{s5}-{s6}");
    println!("Formatted string: {s8}");

    // String slicing (beware of UTF-8)
    let hello = "Здравствуйте"; // Each character is 2 bytes
    let s9 = &hello[0..4]; // This is safe only because it cuts at valid UTF-8 boundary
    println!("Sliced: {s9}");
}
