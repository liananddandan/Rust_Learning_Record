// Demonstrates common programming concepts in Rust

fn main() {
    println!("=== Variables and Mutability ===");
    variables_and_mutability();

    println!("\n=== Data Types ===");
    data_types();

    println!("\n=== Functions ===");
    functions();

    println!("\n=== Control Flow ===");
    control_flow();
}

// ----------------------------
// 1. Variables and Mutability
// ----------------------------
fn variables_and_mutability() {
    let x = 5;
    println!("x is: {x}");

    let mut y = 10;
    println!("y is: {y}");

    y = 15;
    println!("mutated y is: {y}");

    let x = x + 1; // shadowing
    println!("shadowed x is: {x}");
}

// ----------------------------
// 2. Data Types
// ----------------------------
fn data_types() {
    // Scalar types
    let int: i32 = -42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    println!("int: {int}, float: {float}, bool: {boolean}, char: {character}");

    // Compound types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tuple;
    println!("tuple destructure: a={a}, b={b}, c={c}");

    let array = [1, 2, 3, 4, 5];
    println!("array[0] = {}", array[0]);
}

// ----------------------------
// 3. Functions
// ----------------------------
fn functions() {
    println!("Calling another_function...");
    another_function(42);

    let res = five();
    println!("five() returns: {res}");

    let plus = plus_one(7);
    println!("plus_one(7): {plus}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// ----------------------------
// 4. Control Flow
// ----------------------------
fn control_flow() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("number is: {result}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("loop result: {result}");

    for n in (1..4).rev() {
        println!("{n}!");
    }
    println!("LIFTOFF!!!");
}
