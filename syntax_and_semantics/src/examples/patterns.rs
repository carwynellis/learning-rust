pub fn patterns() {
    println!("Patterns\n");

    // Patterns are quite common in Rust and are used in variable bindings,
    // match expressions and in other places.

    // Be aware of scope when pattern matching.
    // In the following example the value x within the match block shadows that
    // of the outer scope.

    let x = 1;
    let c = 'c';

    match c {
        x => println!("In match scope. x: {}, c: {}", x, c),
    }

    println!("Out of match scope. x: {}, c: {}", x, c);

    // Multiple patterns may be matched with the | operator.
    match x {
        1 | 2   => println!("x is one or two"),
        _       => println!("x is neither one nor two"),
    }

    // Destructuring

    // Compound data types such as structs, tuples and enums, can all be
    // destructured with patterns.

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("origin: ({}, {})", x, y),
    }

    // : can be used to provide field aliases...
    match origin {
        Point { x: alias_x, y: alias_y } =>
            println!("origin: ({}, {})", alias_x, alias_y),
    }

    // The pattern need only specify the members of interest...
    match origin {
        Point { x, .. } => println!("matched only x - got {}", x),
    }
    // ...and this applies to any member, not just the first.
    match origin {
        Point { y, .. } => println!("matched only y - got {}", y),
    }

    // _ can be used in any pattern to disregard the type and value. For
    // example...

    let result: Result<i32, &'static str> = Ok(12);

    match result {
        Ok(value)   => println!("Got successful result: {}", value),
        // In this pattern we discard the error value.
        Err(_)      => println!("An error occurred"),
    }

    // _ can be used to ignore values in more complex structures.
    let coord = (0, 1, 2);

    match coord {
        (_, _, z) => println!("z component of coord is: {}", z),
    }

    // Note that _ does not create a binding to the value being ignored.

    // You can also obtain a reference to the value being matched.
    let foo = 1;

    match foo {
        ref x => println!("Got a reference to {}", x),
    }

    // These references can also be mutabble.
    let mut bar = 2;

    match bar {
        ref mut x => *x += 1,
    }

    println!("bar is now {}", bar);

    // A range of values can be matched with ...
    let baz = 10;

    match baz {
        1 ... 10    => println!("{} is in the range 1 - 10 inclusive", baz),
        _           => println!("{} is outside of the range 1 - 10", baz),
    }

    // Ranges can also be used with chars.
    let some_char = '€';

    match some_char {
        'a' ... 'z' => println!("{} is a lowercase letter", some_char),
        'A' ... 'Z' => println!("{} is an uppercase letter", some_char),
        _           => println!("{} is not a letter in any case", some_char),
    }

    // You can also bind names to values with the @ operator.

    let another_char = 'c';

    match another_char {
        lower @ 'a' ... 'z' => println!("{} is a lowercase letter", lower),
        other @ _           => println!("{} is not a letter in any case", other),
    }

    // Which is useful when extracting a value from a nested structure.

    struct Person {
        name: Option<String>,
    }

    let person_result: Result<Person, &'static str> =
        Ok(Person { name: Some("Foo Bar".to_string()) });

    match person_result {
        Ok(Person { name: Some(a @ _) }) => println!("Got a person with name '{}'", a),
        _ => println!("Unexpected result"),
    }

    // Note that when using @ with | the name needs to be bound to each pattern.
    let z = 1;

    match z {
        n @ 1 ... 5 | n @ 101 ... 105 => println!("{} in ranges 1-5 or 101-105", n),
        n @ _                         => println!("{} not in any range", n),
    }

    // Matches can also be qualified with guards as follows.
    let int_result: Result<i32, &'static str> = Ok(1);

    match int_result {
        Ok(n) if n < 10 => println!("Got Ok with value {}, less than 10", n),
        Ok(n)           => println!("Got Ok with value {}, greater than 10", n),
        _               => println!("Got something else"),
    }

    println!();
}
