pub fn functions() {
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
