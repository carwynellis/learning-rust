use std::mem;

pub fn coercion() {
    println!("Coercion\n");

    // The most common case of coercion is removing mutability from a reference.
    // E.g. &mut T to &T
    // An analagous conversion is to remove mutability from a raw pointer.
    // E.g. *mut T to *const T
    // References can also be coerced to raw pointers.
    // E.g. &T to *const T
    //      &mut T to *mut T
    // Custom conversion may be defined using Deref.
    // Coercion is transitive.

    // The as keyword does safe casting.
    let x: i32 = 5;
    let y = x as i64;

    println!("i32 value {} cast as i64 is {}", x, y);

    // There are three major categories of safe cast:
    //   - explicit coercions
    //   - casts between numeric types
    //   - pointer casts

    // Note that casting is not transitive.
    // Even if e as T1 as T2 is a valid expression, e as T2 is not necesarily
    // so.

    // Explicit coercions

    // A cast e as U is valid if e has type T and T coerces to U.

    // Numeric casts

    // A cast e as U is also valid in any of the following cases
    //  o e has type T and T and U are any numeric types; numeric-cast
    //  o e is a C-like enum (with no data attached to the variants), and U is an integer
    //    type; enum-cast
    //  o e has type bool or char and U is an integer type; prim-int-cast
    //  o e has type u8 and U is char; u8-char-cast

    // Numeric cast examples
    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;

    println!("true cast as u8: {}", one);
    println!("64 cast as char: {}", at_sign);
    println!("-56i8 cast as u8: {}", two_hundred);

    // See https://doc.rust-lang.org/book/first-edition/casting-between-types.html
    // for information regarding the semantics of type casting between numeric
    // types.

    // It is safe to cast raw pointers to and from integers, and to cast between
    // pointers to different types subject to some constraints. It is only
    // unsafe to dereference the pointer

    let a = 300 as *const char; // `a` is a pointer to location 300.
    let b = a as u32;

    println!("pointer a cast to u32 is {}", b);

    // e as U is a valid pointer cast in any of the following cases
    //   o e has type *T, U has type *U_0, and either U_0: Sized or
    //     unsize_kind(T) == unsize_kind(U_0); a ptr-ptr-cast
    //   o e has type *T and U is a numeric type, while T: Sized; ptr-addr-cast
    //   o e is an integer and U is *U_0, while U_0: Sized; addr-ptr-cast
    //   o e has type &[T; n] and U is *const T; array-ptr-cast
    //   o e is a function pointer type and U has type *T, while T: Sized; fptr-ptr-cast
    //   o e is a function pointer type and U is an integer; fptr-addr-cast

    // transmute

    // as only allows safe casting, and will for example reject an attempt to
    // cast four bytes into a u32

    let c = [0u8, 0u8, 0u8, 0u8];

    // The following errors with error: non-scalar cast: `[u8; 4]` as `u32`
    // let d = c as u32; // Four u8s makes a u32.

    // This is a non scalar cast because we have multiple values here, the four
    // values in the array.

    // These kinds of casts are very dangerous, because they make assumptions
    // about the way that multiple underlying structures are implemented. For
    // this, we need something more dangerous.

    // The transmute function is very simple, but very scary. It tells Rust to
    // treat a value of one type as though it were another type. It does this
    // regardless of the typechecking system, and completely trusts you.

    // In our previous example, we know that an array of four u8s represents a
    // u32 properly, and so we want to do the cast. Using transmute instead of
    // as, Rust lets us.

    // Since this is an unsafe operation, we need an unsafe block.
    unsafe {
        let d: u32 = mem::transmute(c);
        println!("array of four bytes transmuted into u32 is: {}", d);
    }

    // While transmute does very little checking it does check that the types
    // are the same size. An attempt to transmute the array of 4 bytes into an
    // i64 will fail since there are not enough bytes to create the full i64.

    println!();
}
