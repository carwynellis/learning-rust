#![allow(unused_variables)]

pub fn raw_pointers() {
    println!("Raw pointers\n");

    // Basics

    // Rust has a number of different smart pointer types, however it also
    // provides two types of raw pointer types. These provide no compile time
    // safety and are unsafe to use.

    // The raw pointer types are *const T and *mut T and are provided for cases
    // where Rust's safety guarantees need to be circumvented for some reason.

    // Creating a raw pointer is safe, for example...
    let x = 5;
    let raw = &x as *const i32;

    let mut y = 10;
    let mut_raw = &mut y as *mut i32;

    // However, dereferencing a raw pointer is not, and unsafe must be used.
    let points_at = unsafe { *raw };

    println!("raw points at {}", points_at);

    // Raw pointers are useful for FFI (foreign function interfaces), for
    // example for integrating with c code.
    // More information at https://doc.rust-lang.org/book/first-edition/ffi.html

    // References and Raw Pointers

    // At runtime, a raw pointer * and a reference pointing to the same piece
    // of data have an identical representation.  In fact, an &T reference will
    // implicitly coerce  to an *const T raw pointer in safe code and similarly
    // for the mut variants

    // Going in the opposite direction is not safe and the programmer must
    // ensure that the resulting pointer must satisfy the aliasing and
    // mutability laws of references.

    // The recommended method for conversion is as follows.

    // Explicit cast
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // Implicit coercion
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }

    // The &*x dereferencing style is preferred to using a transmute. The latter
    // is far more powerful than necessary, and the more restricted operation is
    // harder to use incorrectly; for example, it requires that x is a pointer
    // (unlike transmute).

    println!();
}
