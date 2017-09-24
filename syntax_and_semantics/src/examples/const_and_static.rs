pub fn const_and_static() {
    println!("Const and Static\n");
    // Rust has a way of defining constants with the const keyword.
    // Unlike let bindings you must provide a type annotation.
    const N: i32 = 42;

    println!("const N has value: {}", N);

    // Constants live for the entire lifetime and have no fixed address in
    // memory. They will be inlined wherever they're used so a reference to a
    // constant may not always point to the same location in memory.

    // Rust also provides static members, analogous to global variables. static
    // members are not inlined and always refer to a single fixed location in
    // memory.

    // Like const a type annotation must be provided for static members.
    static P: i32 = 24;

    println!("static P has value: {}", P);

    // Statics can also be mutable by declaring with the mut keyword.
    static mut R: i32 = 6;

    // Since both reading and mutating a static are not thread safe, both must
    // occur within an unsafe block.

    unsafe {
        println!("static R, before mutation, has value: {}", R);
        R += 1;
        println!("static R, after mutation, has value: {}", R);
    }

    // A static member must be Sync (a type for which it is safe to share
    // references between threads), and must not have a Drop implementation.

    // Both static and const members must be initialised using a constant
    // expression, so for example, the result of a function call at runtime
    // cannot be used.

    println!();
}
