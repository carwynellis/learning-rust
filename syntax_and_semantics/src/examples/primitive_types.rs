pub fn primitive_types() {
    println!("Primitive Types\n");

    // Rust has primitive boolean types
    let x = true;
    let y: bool = false;

    println!("x is {}, y is {}", x, y);

    // Char represents a single unicode character
    // Note - char is thus 4 bytes
    let some_char: char = 'c';

    println!("some_char is {}", some_char);

    // Rust has a range of signed and unsigned numeric types.
    // Where no explicit type is provided rust defaults as follows
    let some_int = 42;    // defaults to i32
    let some_float = 1.0; // defaults to f64

    println!("some_int is {}, some_float is {}", some_int, some_float);

    // Arrays
    let nums = [ 1, 2, 3 ];

    println!("nums contains {} elements", nums.len());

    // Rust also provides shorthand for array initialization
    let nums2 = [ 0 ; 20 ];

    println!("nums2 contains {} elements", nums2.len());

    // Array elements are accessed using zero-based subscripts
    let names = [ "Foo", "Bar", "Baz" ];

    println!("third name is {}", names[2]);

    // In rust a slice offers a 'view' into a data structure without
    // dupliating the contents of that structure.
    let a = [ 0, 1, 2, 3, 4 ];

    let slice_all = &a[..]; // refers to all elements of a
    let slice_mid = &a[1..4]; // refers to elements 1, 2, 3

    println!("slice_all has {} elements", slice_all.len());
    println!("slice_mid has {} elements", slice_mid.len());

    // Rust also offers tuples
    let some_tuple = (1, "hello");

    // Elements of a tuple can be accessed using a destructuring let.
    let (t1, t2) = some_tuple;
    println!("some_tuple contains ( {}, {} )", t1, t2);

    // Tuple fields can also be accessed with indexing syntax.
    // Note - like arrays, the indexes are zero based
    println!("second value of some_tuple is {}", some_tuple.1);

    println!();
}
