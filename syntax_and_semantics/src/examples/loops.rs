pub fn loops() {
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
