use std::fmt::Debug;

// A generic function that returns a reference to the largest item in a slice.
// Note the trait bounds: we need `PartialOrd` to compare, and `Debug` just for printing.
// We return `&T` so we don't move/clone values.
pub fn largest<T: PartialOrd + Debug>(list: &[T]) -> &T {
    // Borrow the first element as the initial candidate.
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct\#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

// You can add concrete impl blocks for specific T (specialized behavior)
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];
    let max_num = largest(&numbers);
    println!("largest(numbers) = {:?}", max_num);

    let chars = vec!['y', 'm', 'a', 'q'];
    let max_ch = largest(&chars);
    println!("largest(chars) = {:?}", max_ch);

    let p = Point { x: 3, y: 5 };
    println!("p.x() = {}", p.x());

    let p2 = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("distance_from_origin = {:.2}", p2.distance_from_origin());
}