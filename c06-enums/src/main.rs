// Chapter 6: Enums and Pattern Matching

mod basic_enum;
mod match_option;
mod coin_enum;

fn main() {
    println!("--- Basic Enum Usage ---");
    basic_enum::run();

    println!("\n--- Matching Option<T> ---");
    match_option::run();

    println!("\n--- Pattern Matching with Enums ---");
    coin_enum::run();
}
