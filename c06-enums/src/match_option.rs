// Demonstrates using Option<T> and pattern matching

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn run() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
}
