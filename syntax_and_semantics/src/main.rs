use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

// Examples from Rust Book - Syntax and Semantics.
// See https://doc.rust-lang.org/book/syntax-and-semantics.html
fn main() {
    variable_bindings();
    functions();
    primitive_types();
    if_examples();
    loops();
    vectors();
    ownership();
    references_and_borrowing();
    lifetimes();
    mutability();
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

fn ownership() {
    println!("Ownership\n");

    // Ownership can be passed to a function, which is the default behaviour in
    // rust. For example...
    let v = vec![1, 2, 3];

    take(v);

    // At this point we no-longer have ownership of v, and any attempt to
    // access v will result in an error.

    // The following code, if uncommented, will fail at compile time.
    // for i in v {
    //     println!("take iterating over v, at element {}", i);
    // }

    println!();

    // Primitive types implement the Copy trait which assigns a copy of the
    // value, and so ownership is not also moved with assignement.
    let i: i32 = 12;

    println!("doubled value is {}", double_i32(i));
    println!("i is still accessible after being passed, i = {}", i);

    println!();
}

fn take(v: Vec<i32>) {
    println!("take function has ownership of v");

    for i in v {
        println!("take iterating over v, at element {}", i);
    }
}

fn double_i32(i: i32) -> i32 {
    i * 2
}

fn references_and_borrowing() {
    println!("References and Borrowing\n");

    // Immutable references can be borrowed as follows.
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = sum_two_vecs(&v1, &v2);

    println!("Sum of v1 and v2 is {}", answer);

    // By default references are immutable.

    // Mutable references are also supported. For example...
    let mut x = 5;

    // Note that the following only works because we introduce an extra scope.
    {
        let y = &mut x;
        *y += 1;
    }

    println!("x is now {}", x);

    println!();
}

// Folds over a vec to compute the sum.
fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn sum_two_vecs(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    sum_vec(v1) + sum_vec(v2)
}

fn lifetimes() {
    println!("Lifetimes\n");

    // When passing references we can run into ambiguity about whether the
    // referenced resource will still exist at the point that we attempt to use
    // it.

    // Rust provides lifetime annotations which allow the lifetime of
    // references to be explicitly annotated. The compiler can then make use of
    // these annotations to determine whether the code is safe to run or not.

    // Lifetime annotations are usually required for more complex scenarios
    // since the compiler is able to check simpler cases without assistance.

    // Example using function that must declare lifetimes.
    let mut data = vec![1, 2, 3, 4, 5, 6, 7];

    let filtered;

    // We introduce a new scope to create ambiguity about the lifetime of one
    // of the parameters we will pass to a function.
    // This will not compile if the less_than function does not provide
    // lifetime annotations.
    // In this case filtered is in an outer scope relative to limit, which is
    // in an inner scope.
    // The function will need to declare two scopes and associate the result
    // with the scope where it will be used, in this case, the outer scope.
    {
        let limit = 5;
        filtered = less_than(&mut data, &limit);
    }

    println!(
        "Filtered numbers: {}",
        filtered
            .iter()
            .fold(
                String::new(),
                |acc, &elem| if acc.is_empty() { acc + &elem.to_string() } else { acc + ", " + &elem.to_string() }
            )
    );

    println!();
}

// This function must explicitly declare lifetimes for the incoming parameter
// references, and the result.
// In this case we declare that the lifetime of the result will share the outer
// scope of the incoming data parameter.
fn less_than<'outer, 'inner>(data: &'outer mut Vec<i32>, limit: &'inner i32) -> &'outer Vec<i32> {
    data.retain(|&d| d < *limit);
    return data;
}

fn mutability() {
    println!("Mutability\n");

    // Variables are immutable by default.
    // Mutability can be declared with the mut keyword.
    let mut x = 1;
    println!("x is {}", x);

    x = 2;
    println!("x is {} after assignment", x);

    // A reference to a value must also be declared mutable if that reference
    // is to be used to modify the value.
    {
        // Note that ref_x is immutable in that it cannot be changed to point to
        // another resource.
        let ref_x = &mut x;
        *ref_x = 12;
    }

    println!("x is now {} after modification via mutable reference", x);

    // Interior vs Exterior Mutability

    // Immutability refers to the exterior mutability of a resource. The 
    // resource itself can still have mutable interior state. For example.
    let arc = Arc::new(5);
    let arc_clone = arc.clone();
    println!("arc_clone is {}", arc_clone);
    // The Arc (reference counting ponter) instance referred to by x is still
    // able to update its internal state to allocate a reference to y, even
    // though the variable binding, x, is immutable.

    // In Rust, you can think of immutability as 'is this safe to have more
    // than one pointer to it?'.

    // The following example shows interior mutability, that is, obtaining a
    // mutable reference to a value held within another instance.
    // An immutable reference to a RefCell...
    let ref_cell = RefCell::new(42);
    {
        // We can borrow a mutable reference as follows...
        let mut ref_to_cell = ref_cell.borrow_mut();
        // which can be used to modify the value.
        *ref_to_cell = 24;
    }
    println!("ref_cell is now {}", ref_cell.into_inner());

    // Field Level Mutability

    // Mutability is either a property of a borrow or a binding.

    // This means that you cannot have a struct for example with a mixture of
    // immutable and mutable fields.

    // However a cell can be used to emulate field level mutability as follows.
    struct Point {
        a: i32,
        b: Cell<i32>,
    }

    let point = Point { a: 5, b: Cell::new(6) };

    point.b.set(7);

    println!("point after setting mutable field b: Point({}, {:?})", 
        point.a, point.b
    );

    println!();
}
