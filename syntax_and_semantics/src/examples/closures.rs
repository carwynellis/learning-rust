pub fn closures() {
    println!("Closures\n");

    // A simple closure looks like this
    let plus_one = |x: i32| x + 1;

    // Arguments to the closure are delimited by pipes.
    // We can also create multi-line closures, for example in the following
    // rather contrived example.
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    }; // Note - you really do need the semi-colon

    println!("{} plus_one is {}", 2, plus_one(2));
    println!("{} plus_two is {}", 2, plus_two(2));

    // Note that closures do not require type annotations for arguments or the
    // return type which differs from function definitions.

    // The environment for a closure can include bindings from its enclosing
    // scope, in addition to parameters and local bindings.
    // For example...
    let num = 5;
    let plus_num = |x| x + num;

    println!("{} plus_num {} is {}", 2, num, plus_num(2));

    // The borrow checker will ensure that mutable references to bindings used
    // by a closure cannot be taken in later declarations.

    // A move keyword is also available that allows a closure to take ownership
    // of a copy of a binding.

    // For example, the following will mutate the mutable binding as you would
    // expect.
    let mut y = 5;

    {
        // The closure here takes a mutable reference to y so is able to modify
        // the value declared outside the scope of this block.
        let mut add_y = |x| y += x;
        add_y(5);
    }

    println!("y after call to add_y(5) is {}", y);

    // However, using the move keyword we get the following.
    let mut z = 5;
    {
        let mut add_z = move |x| z += x;
        add_z(5);
    }

    println!("z after call to move add_z(5) is {}", z);

    // add_z modifies a copy of z so the declaration in the outer scope is left
    // unmodified.

    // Another way to think about move closures is that they have their own
    // stack frame. Without a move, the closure may be tied to the stack frame
    // that created it. This means that generally you cannot return a non-move
    // closure from a function.

    // Further information about the underlying implementation and how to
    // return a closure is available at
    // https://doc.rust-lang.org/book/first-edition/closures.html

    println!();
}
