pub fn structs() {
    println!("Structs\n");

    // As shown above, Structs provide a way to define more complex types.
    // For example we can combine a Point type defining a point in two
    // dimensional space as follows.
    // Note that by convention, struct names are camel cased, with the first
    // letter capitalised.

    struct Point {
        x: i32,
        y: i32
    }

    let some_point = Point { x: 12, y: 54 };

    println!("some_point defined as ({}, {})", some_point.x, some_point.y);

    // Use a mutable binding to a struct to allow mutability of the values
    // contained withing.

    let mut mut_point = Point { x: 1, y: 1 };

    mut_point.x = 100;

    println!(
        "mut_point after modification is ({}, {})",
        mut_point.x,
        mut_point.y
    );

    // Structs support an update syntax using .. which allows a struct to be
    // created using values from another.
    // In this case another_point takes the x value of some_point and uses the
    // y value we specify.

    let another_point = Point { y: 12, .. some_point };

    println!("another_point is ({}, {})", another_point.x, another_point.y);

    // Rust also offers Tuple Structs, a hybrid between tuples and structs.
    // These types have a name, but their fields do not.
    // For example...
    struct Color(i32, i32, i32);

    let red = Color(255, 0, 0);

    println!("red defined as ({}, {}, {})", red.0, red.1, red.2);

    // Tuple structs can be useful in the single element case, since we can
    // give a meaningful type to a single value. Incidentally this is referred
    // to as the 'newtype' pattern.

    struct Inches(i32);

    let length = Inches(10);

    // The value of the type can be extracted using a destructuring let as
    // follows.
    let Inches(length_value) = length;

    // It's always possible to use a struct instead of a tuple struct, and this
    // is typically clearer.

    println!("length is {} inches", length_value);

    // Note, that it's also possible to define a 'unit like' struct, that has
    // no members. This can be handy in specific situations, for example when
    // extending a trait that requires a Struct to be declared that is not
    // needed in the implementation.

    println!();
}
