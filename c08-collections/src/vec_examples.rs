pub fn run() {
    // Create a new vector and add values
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Access values using index
    let third = &v[2];
    println!("The third element is {third}");

    // Access values safely using .get()
    match v.get(100) {
        Some(value) => println!("Value at index 100: {value}"),
        None => println!("No value at index 100"),
    }

    // Borrowing rules: can't push while borrowed
    let first = &v[0];
    // v.push(4); // Uncommenting this will cause a compile error
    println!("First element is: {first}");

    // Mutable iteration and dereferencing
    for i in &mut v {
        *i += 10;
    }

    println!("Updated vector: {:?}", v);
}
