// Demonstrates methods and associated functions on structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method with self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method with self and argument
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated function (like a static method)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {:?}", rect1);
    println!("Area: {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(20);
    println!("Square: {:?}", square);
}
