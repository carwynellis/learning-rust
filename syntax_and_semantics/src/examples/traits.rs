use std;

pub fn traits() {
    println!("Traits\n");

    // A trait tells the Rust compiler about functionality a type must provide.
    // Traits are similar to method syntax, except that now a trait is used to
    // define the method signature.
    struct Circle {
        radius: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // The area method is now available for Circle instances.

    let c = Circle { radius: 12.0 };

    println!("Circle with radius: {} has area: {}", c.radius, c.area());

    // Note - Self may be used in a type annotation to refer to an instance of
    // the type implementing this trait passed as a parameter. Self, &Self or
    // &mut Self may be used depending on the level of ownership required.

    // For example implementing an IsLarger trait using Self references.
    trait IsLarger {
        fn is_larger(&self, &Self) -> bool;
    }

    impl IsLarger for Circle {
        fn is_larger(&self, other: &Self) -> bool {
            self.area() > other.area()
        }
    }

    let d = Circle { radius: 24.0 };

    println!(
        "Circle with radius: {} is larger than circle with radius: {} - {}",
        d.radius,
        c.radius,
        d.is_larger(&c)
    );

    // Generic functions can declare trait bounds on the types they accept.
    // This allows a generic function to accept only values that implement a
    // given trait.

    // We can use this with the has area trait to ensure that a generic function
    // that prints the area of a given shape, will only accept arguemtns that
    // provide a HasArea impl.

    // So using the HasArea trait above we can also define a square as follows.
    struct Square {
       side_length: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side_length * self.side_length
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("shape has area: {}", shape.area());
    }

    // The print_area will accept any instance that provides a HasArea impl.
    let circle = Circle { radius: 100.0 };
    let square = Square { side_length: 100.0 };

    println!("\nCircle print_area");
    print_area(circle);
    println!("\nSquare print_area");
    print_area(square);

    // Trait bounds can also be declared on impl declarations for generic structs.

    // Multiple trait bounds may be declared using the + operator e.g.
    // fn foo<T: bar + baz>(something: T)

    // where can be used instead to make things a little easier to read.

    // The example above becomes
    //  fn foo<T>(something: T) where T: bar + baz

    // This is particularly useful when there are several types with bounds in
    // play.

    // Traits can be implemented for any type, such as f32.

    // Traits will only apply if they are in scope.

    // Default methods can also be defined in the trait declaration.

    // If an implemented trait depends on another trait then the implementor
    // must provide an implementation for both. For example...
    trait Foo {
        fn foo(&self);
    }

    trait FooBar : Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }

    let baz = Baz {};

    println!("\nTrait inheritence example");
    baz.foo();
    baz.foobar();

    // Rust can also derive implementations for a limited set of traits
    // provided in the standard library.
    // TODO - look into this and provide a decent example.

    println!();
}
