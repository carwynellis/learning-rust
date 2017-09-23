#![allow(unused_variables)]

pub fn strings() {
    println!("Strings\n");

    // Strings are encoded as a stream of UTF-8 bytes and are not NUL
    // terminated so may include the NUL byte.

    // Rust has two main types of string, &str and String.

    // &str is also referred to as a string slice. A string slice is immutable.

    let greeting = "Hello there!"; // greeting: &'static str

    // Greeting is a binding to a string slice that is statically allocated.
    // Any function accepting a string slice, will also accept a string 
    // literal.

    // String literals can span multiple lines with or without preservation of
    // white space.
    let literal_with_whitespace = "This is a
multiline
  string
    with preservation of whitespace,
        including newlines.
";

    println!(
        "literal string with whitespace preserved: '{}'",
        literal_with_whitespace
    );

    // To ignore white space and newlines, delimit lines with a trailing \
    let literal_without_whitespace = "Foo\
        Bar\
        Baz";

    println!(
        "literal string without whitespace: '{}'",
        literal_without_whitespace
    );

    println!();

    // Note that you cannot normally access an str directly because it is an
    // unsized type which requires additional runtime information to be usable.
    // See unsized types.

    // Rust also has Strings which are heap allocated. Strings are growable.

    // Strings can be created by calling the to_string method on str.

    let mut some_string = "Hello".to_string();

    println!("some_string: {}", some_string);

    // some_string can be grown by appending another string to it.
    some_string.push_str(" World!");

    println!("some_string after push_str: {}", some_string);
    println!();

    // Strings can be passed to functions expecting a str by prefixing with &.

    // Note, coercing a String into an str is cheap, but creating a String from
    // an str requires memory to be allocated. So, only do this when absolutely
    // necessary.

    // Because strings are UTF-8 they do not support indexing.
    // E.g let s = "Hello"; let t = s[0]; // this will fail
    // Unicode doesn't strictly define a 'letter' so it's not easy to support
    // indexing of individual UTF-8 characters.

    // Strings can however be examined as individual bytes.
    let another_string = "Some characters";

    println!("String '{}' contains the bytes....", another_string);

    for b in another_string.as_bytes() {
        print!("{}, ", b);
    }

    println!("\n");

    // Strings can be sliced, for example...

    let first_five = &another_string[0..5];

    println!("First five characters of another_string are '{}'\n", first_five);

    // However this will fail at runtime if character boundaries are not
    // preserved.

    // An str can be concatenated to a String using the + operator.

    let hello = "Hello ".to_string();
    let world = "World!";

    // Note that these values are 'moved' after this call.
    let hello_world = hello + world;

    println!("Concatenation result is: {}", hello_world);

    // Two Strings can be concatenated, but the second must be prefixed with &.

    println!();
}
