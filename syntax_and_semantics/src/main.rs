extern crate syntax_and_semantics;

use syntax_and_semantics::examples::*;

// Examples from Rust Book - Syntax and Semantics.
// See https://doc.rust-lang.org/book/first-edition/syntax-and-semantics.html
fn main() {
    variable_bindings();
    functions();
    primitive_types();
    if_examples();
    loops();
    vectors();
    ownership();
    references_and_borrowing();
    lifetimes();
    mutability();
    structs();
    enums();
    matching();
    patterns();
    method_syntax();
    strings();
    generics();
    traits();
    drop();
    if_let_while_let();
    trait_objects();
    closures();
    universal_function_call_syntax();
    const_and_static();
}
