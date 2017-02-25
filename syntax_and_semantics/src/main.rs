// Examples from Rust Book - Syntax and Semantics.
// See https://doc.rust-lang.org/book/syntax-and-semantics.html
fn main() {
    variable_bindings();
    functions();
    primitive_types();
    if_examples();
    loops();
    vectors();
}

fn variable_bindings() {
    println!("Variable Bindings\n");

    // Simple variable binding using let.
    let x = 5;

    println!("The value of x is: {}", x);

    // The left hand side of a let statement is a pattern. This allows
    // statements such as the following.
    let (y, z) = (1, 2);

    println!("The value of (y,z) is: ({},{})", y, z);

    // Rust has type inference although types may be explicitly declared.
    let foo: i32 = 42;

    println!("The value of foo is: {}", foo);

    // i32 is a 32 bit signed integer.
    // uXX can be used to declare an unsigned integer.
    // Supported sizes are 8, 16, 32 and 64 bits.

    // Rust defaults to immutable bindings.
    // The mut keyword can be used to explictly declare that a binding is
    // mutable.
    // Note - rust prefers snake case when naming bindings.
    let mut mutable_x = 5;
    println!("mutable_x has been initialized with the value: {}", mutable_x);

    mutable_x = 10;
    println!("The value of mutable_x is now {}", mutable_x);

    // Scoping.
    // Rust supports shadowing, either within nested blocks or repeated binding
    // declarations.
    let bar: u8 = 20;
    println!("The value of bar is: {}", bar);

    // We can shadow the bar value within a nested block.
    {
        println!("The value of bar within the block is: {}", bar);
        let bar = 56;
        println!("The value of bar within the block is now: {}", bar);
    }

    // The outer bar is still bound to the original value.
    println!("Outside of the block, bar is still: {}", bar);

    // We can bind a new value to bar in the same scope.
    let bar = 0;

    println!("Bar is now: {}, ", bar);

    println!();
}

fn functions() {
    println!("Functions\n");

    print_number(123);

    print_sum(12, 24);

    println!("1 + 1 = {}", add_one(1));

    println!("1 + 2 = {}", add_two(1));

    // Function pointers.
    // We can create variable bindings to functions as follows.
    let add_one_p: fn(i32) -> (i32) = add_one;

    println!("1 + 1 = {}", add_one_p(1));

    // Note that type inference is also supported.
    let add_two_p = add_two;

    println!("1 + 2 = {}", add_two_p(1));

    println!();
}

// A simple function that accepts a parameter.
// Parameter types must be explicitly declared.
fn print_number(n: i32) {
    println!("got number: {}", n);
}

// Multiple parameters are simply declared as a comma separated list.
fn print_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// Rust functions return exactly one value which is declared by its type as
// follows.
fn add_one(x: i32) -> i32 {
    // Note - no semicolon is used here since we want x + 1 to be an expression
    // that returns a value, rather than a statement, which returns an empty
    // tuple ().
    // rustc will warn if a semicolon terminates this line.
    x + 1
}

// A return keyword is available should early or explicit returns be required.
fn add_two(x: i32) -> i32 {
    return x + 2;
}

fn primitive_types() {
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

fn if_examples() {
    println!("If\n");

    // In Rust, if operates as you might expect.
    let x = 5;

    if x == 5 {
        println!("x is 5!");
    }
    else if x ==6 {
        println!("x is 6!");
    }
    else {
        println!("x is something else!");
    }

    // However, like in some other languages, if is also and expression which
    // makes the following possible.
    let y = if x == 5 { 10 } else { 0 };

    println!("y is {}", y);

    println!();
}

fn loops() {
    println!("Loops\n");

    // The infinite loop is the simplest form of loop in rust.
    // loop {
    //     println!("Looping!");
    // }

    // Rust also has while loops.
    let mut x = 5;

    while x > 0 {
        println!("while loop: x is {}", x);
        x -= 1;
    }

    println!();

    // For loops are also present, which use a neater syntax than the standard
    // c style loop
    // Note - 0..10 evaluates to 0-9 inclusive, i.e. the upper bound is
    // exclusive.
    for x in 0..10 {
        println!("for loop: x is {}", x);
    }

    println!();

    // Ranges can also be enumerated, if the position within a range is needed.
    for (index, value) in (5..10).enumerate() {
        println!("for with enumeration: value {} - index {}", value, index);

    }

    println!();

    // As can iterators...
    let lines = "First\nSecond\nThird\n".lines();

    for (line_number, line) in lines.enumerate() {
        println!("line {}: {}", line_number, line);
    }

    println!();

    // Ending iteration early.
    // As an alternative to a while loop a loop { } construct can be terminated
    // early by either using break or return.
    let mut x = 0;
    let limit = 5;

    loop {
        println!("for loop: x {}", x);
        x += 1;
        if x > limit {
            println!("x has reached limit {}, breaking loop", limit);
            break;
        }
    }

    println!();

    // Rust also provides a continue keyword that skips to the next iteration.
    // For example the following will only print out even numbers.
    for x in 1..11 {
        if x % 2 == 1 { continue; }
        println!("for loop with continue skipping odd values: {}", x);
    }

    println!();

    // Labelling of nested loops is also supported. The labels can be passed to
    // break and continue. The following will only print the x and y values
    // when both are equal.
    'outer: for x in 1..11 {
        'inner: for y in 1..11 {
            if x % 2 == 1 { continue; }
            if y % 2 == 1 { continue; }
            println!("nested for loops with labels: x {}, y {}", x, y);
        }
    }

    println!();
}

fn vectors() {
    println!("Vectors\n");

    // A vector is a dynamically sized array.
    let mut v = vec![1, 2, 3, 4, 5];

    // Note that vec! also supports initialization of a number of elements to
    // a given value, e.g vec![0, 2] is the same as vec![0, 0]

    // Vectors are stored as contiguous arrays on the heap.

    // Elements of a vector can be accessed in the same way as arrays.
    println!("The third element of v is {}", v[2]);

    println!();

    // Note that vector indexes must be stored using a usize.

    // Out of bounds access.
    // To avoid a thread panic, vectors provide get and getMut accessors that
    // return None if the value does not exist.
    match v.get(10) {
        Some(x) => println!("Value of the requestd element is {}" , x),
        None => println!("No value exists in v at the requested position")
    }

    println!();

    // There are three ways to iterate through a vector.

    // Immutable Reference.
    for i in &v {
        println!("Iterating over a reference to v: {}", i);
    }

    println!();

    // Mutable reference.
    for i in &mut v {
        println!("Iterating over a mutable reference to v: {}", i);
    }

    println!();

    // Taking ownership of the vector and its element.
    for i in v {
        println!("Iterating over v by taking ownership: {}", i);
    }

    println!();

    // Note that once you have taken owenership of the vector it cannot be
    // iterated over again. The reference based approaches allow the vector to
    // be iterated over multiple times.
}
