#![allow(dead_code)]

pub fn matching() {
    println!("Match\n");

    // Rust supports matching with exhaustiveness checking.

    // A value can be matched as follows.
    let x = 2;

    match x {
        1   => println!("one"),
        2   => println!("two"),
        3   => println!("three"),
        _   => println!("remaining numbers left as exercise for reader"),
    }

    // The last case is the default case which will be triggered if there were
    // no preceding matches. Without this the compiler will warn that the
    // match is not exhaustive, e.g. there are missing cases that should be
    // declared.

    // Match is also an expression so the result can be bound as follows.
    let number = match x {
        1   => "one",
        2   => "two",
        3   => "three",
        _   => "more typing required",
    };

    println!("{} matched {}", x, number);

    // We can also match on enums, as follows.

    // Redefine the enum from the previous example through the magic of copy
    // and paste.
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    // to_owned creates an owned String from a given string slice.
    // TODO - look into why this is necessary...
    let message = Message::Write("Hallo Welt!".to_owned());

    // Match on the message in a fairly useless way.
    match message {
        Message::Quit                   => println!("Byeee!"),
        Message::ChangeColor(r, g, b)   => println!("Changing color to ({}, {}, {})", r, g, b),
        Message::Move { x, y: y_alias } => println!("Got move x: {} y: {}", x, y_alias),
        Message::Write(message)         => println!("Got message '{}'", message),
    }

    println!();
}
