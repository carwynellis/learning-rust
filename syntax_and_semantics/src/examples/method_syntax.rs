use std;

pub fn method_syntax() {
    println!("Method Syntax\n");

    // Rust provides method style syntax via the impl keyword.
    // Consider the following example.

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // Methods take a first parameter which provides access to the instance
        // of the struct.
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };

    println!("Circle with radius: {} has area: {}", c.radius, c.area());

    // Note that multiple impl blocks may be provided as shown below.
    // Method calls can be chained provided that the method returns the struct
    // type. For example...

    impl Circle {
        // Grow method takes an increment that returns a new circle instance.
        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }
    }

    let increment = 2.0;

    println!(
        "Circle with radius: {} has area: {}",
        c.radius + increment,
        c.grow(increment).area(),
    );

    // Note that you can also define associated functions, that is functions
    // that do not take a self parameter. This is commonly used in rust to
    // implement constructors. For example...

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    // Note that associated functions are called using the :: syntax.
    let c2 = Circle::new(0.0, 0.0, 1.0);

    println!("Circle built with new function has area: {}", c2.area());

    // This pattern can also be extended to implement builders as shown below.

    // First we start with a struct that defines the properties to be used to
    // create a new circle.
    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    // We can then implement a builder using this struct that will ultimately
    // create a Circle instance.
    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder {
                // Define some default values.
                x: 0.0,
                y: 0.0,
                radius: 1.0,
            }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }

        fn finalize(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }

    // The builder can then be used to create a new Circle as follows.
    let c3 = CircleBuilder::new()
        .x(2.3)
        .y(4.5)
        .radius(50.0)
        .finalize();

    println!("Circle from builder has x: {}, y: {} and area: {}",
        c3.x,
        c3.y,
        c3.area(),
    );

    println!();
}
