use std::collections::HashMap;

pub fn run() {
    // Create and insert into HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get value by reference (returns Option<&V>)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {team_name}: {score}");

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Updated Blue: {:?}", scores.get(&"Blue".to_string()));

    // Only insert if key doesn't exist
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100);

    println!("Final scores: {:?}", scores);

    // Update value based on existing value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", word_count);
}
