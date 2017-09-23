pub fn if_let_while_let() {
    println!("if let and while let\n");

    // if let permits pattern matching within the condition of an if statement.
    // This can make certain statements more concise.

    // For example, consider working with an optional value.
    let something = Some(1);

    // Typically we only need to do something when there is some value.

    // We can match on the value...
    match something {
        Some(x) => println!("Got value {} when matching on the option", x),
        None    => {}
    }

    // ...or perhaps test if the value is present before doing something.
    if something.is_some() {
        let x = something.unwrap();
        println!("Got value {} when testing if the option has a value", x);
    }

    // However both approaches involve a degree of boiler plate.
    // Using if let we can express the same thing as follows...
    if let Some(x) = something {
        println!("Got value {} from option when using if let", x);
    }

    // If the pattern matches any appropriate values are bound to the specified
    // identifiers which can then be used within the subsequent block.
    // Note that you can also use else to take an alternative action should the
    // pattern not match.
    if let None = something {
        println!("the option does not contain a value");
    }
    else {
        println!("the option contains a value");
    }

    println!();

    // Along the same lines, while let can be used to loop until a pattern does
    // not match anymore.
    let mut v = vec![1, 3, 5, 7, 11];

    while let Some(x) = v.pop() {
        println!("while let - v.pop returned Some({})", x);
    }

    // In both cases, you need only express the happy path if that's all you
    // care about.

    println!();
}
