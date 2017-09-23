#![allow(dead_code)]
#![allow(unused_variables)]

pub fn enums() {
    println!("Enums\n");

    // Enums in Rust are quite familiar. However one difference is that each
    // entry in the enum may contain additional optional values specific to
    // that entry.
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    // Note that the syntax for each entry is similar to that for structs.
    // Values may optionally be given names as in the Move entry above.

    // A given enum entry is accessed using :: as follows.

    let message = Message::Move { x: 1, y: 2 };

    // Note that the entry names are scoped to the enum type, so different
    // enums can share entry names.

    // Enum values cannot be destructured directly, but values can be extracted
    // using pattern matching.

    println!();
}

