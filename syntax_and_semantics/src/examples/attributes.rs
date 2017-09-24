pub fn attributes() {
    println!("Attributes");
    // Rust provides a number of attributes which are analogous to annotations
    // in java or scala.

    // The set of valid attributes is predefined.

    // See https://doc.rust-lang.org/reference/attributes.html for available
    // attributes.
    println!();
}

// The following examples can be run with cargo test since they both use the
// test attribute.

// Attributes can be applied to the next item, as follows...
#[test]
pub fn some_test() {
    assert_eq!(1, 1);
}

// Or they can be declared within, and refer to, the item enclosing them.
// For example....
pub fn another_test() {
    #![test]
    assert_eq!(1, 1);
}
