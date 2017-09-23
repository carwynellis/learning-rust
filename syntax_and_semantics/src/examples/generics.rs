#![allow(unused_variables)]
#![allow(dead_code)]

use std;

pub fn generics() {
    println!("Generics\n");

    // The Rust standard library makes use of generics, which provide
    // parametric polymorphism. For example the Option makes use of generics.
    let x: Option<i32> = Some(5);

    // Functions can be declared generic with as many types as required.
    let y = simple_generic_function(12);
    let z = multiple_typed_generic_function(1, "Foo");

    println!("simple_generic_function returned {}", y);
    println!("multiple_typed_generic_function returned {}", z);

    // Generic types can also be declared for structs.
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 12, y: 42 };
    let float_point = Point { x: 1.2, y: 4.2 };

    // It then follows that struct impls also support generic types.
    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    // Note that explicit type annotations will be required where the compiler
    // is unable to resolve ambiguous types.

    println!();
}

fn simple_generic_function<T>(x: T) -> T {
    return x;
}

fn multiple_typed_generic_function<A, B>(a: A, b: B) -> A {
    return a;
}
