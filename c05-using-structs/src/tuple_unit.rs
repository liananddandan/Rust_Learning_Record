// Demonstrates tuple structs and unit-like structs

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-like struct, useful for traits
#[derive(Debug)]
struct AlwaysEqual;

pub fn run() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color: {:?}", black);
    println!("Origin point: {:?}", origin);

    let subject = AlwaysEqual;
    println!("Unit-like struct: {:?}", subject);
}
