// Chapter 5: Using Structs to Structure Related Data

mod basics;
mod methods;
mod tuple_unit;

fn main() {
    println!("--- Basic Structs ---");
    basics::run();

    println!("\n--- Struct Methods ---");
    methods::run();

    println!("\n--- Tuple & Unit Structs ---");
    tuple_unit::run();
}
