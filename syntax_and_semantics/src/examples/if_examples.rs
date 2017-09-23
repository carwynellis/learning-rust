pub fn if_examples() {
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
