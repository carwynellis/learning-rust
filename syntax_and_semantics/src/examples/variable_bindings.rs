pub fn variable_bindings() {
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
