use std::ops::Add;

pub fn operators_and_overloading() {
    println!("Operators and overloading\n");

    // Rust allows for a limited form of operator overloading.
    // To support this there are specific traits which can be implemented in
    // order to overload the specified operator.

    // For example we can overload the addition operator in order to allow two
    // points to be added together, as follows.
    struct Point {
        x: i32,
        y: i32
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y
            }
        }
    }

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };

    let p3 = p1 + p2;

    println!(
        "Sum of Points is ( {}, {} )",
        p3.x, p3.y
    );

    // There are a number of operators in the std::ops module that can be
    // overloaded in this way.
    // See https://doc.rust-lang.org/std/ops/

    println!();
}
