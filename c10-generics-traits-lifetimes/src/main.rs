mod generics;
mod traits_demo;
mod trait_objects;
mod lifetimes;

fn main() {
    println!("=== Chapter 10: Generics, Traits, and Lifetimes ===\n");

    println!("-- Generics --");
    generics::run();

    println!("\n-- Traits --");
    traits_demo::run();

    println!("\n-- Trait Objects (dyn Trait) --");
    trait_objects::run();

    println!("\n-- Lifetimes --");
    lifetimes::run();
}