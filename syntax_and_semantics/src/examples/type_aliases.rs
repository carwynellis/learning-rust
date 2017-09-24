#![allow(dead_code)]
#![allow(unused_variables)]

use std::result;

pub fn type_aliases() {
    println!("Type aliases");

    // The type keyword lets you declare an alias for a type.
    type Name = String;

    // You can then use this type as if it were a real type.
    let name: Name = "Foo".to_string();

    // Note that since this is a type alias, a comparison of a String and a
    // Name will succeed because they both refer to the same type.

    // Type aliases can also be used with generics.
    enum ConcreteError {
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T, ConcreteError>;

    println!();
}
