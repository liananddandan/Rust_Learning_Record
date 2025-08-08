mod vec_examples;
mod string_examples;
mod hashmap_examples;

fn main() {
    println!("--- Vector Examples ---");
    vec_examples::run();

    println!("\n--- String Examples ---");
    string_examples::run();

    println!("\n--- HashMap Examples ---");
    hashmap_examples::run();
}
