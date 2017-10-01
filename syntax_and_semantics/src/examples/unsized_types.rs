#![allow(dead_code)]

pub fn unsized_types() {
    println!("Unsized types\n");

    // Most types have a particular size in bytes that is known at compile
    // time, e.g. i32.
    //
    // However there are some types which are useful to express which do not
    // have a defined size. These are called 'unsized' or 'dynamically_sized'
    // types.
    //
    // One example is [T] which represents a certain number of T in a sequence.
    //
    // Rust understands a number of these types, which come with the following
    // restrictions.
    //
    //  o   We can only manipulate an instance of an unsized type via a pointer.
    //      An &[T] works fine, but a [T] does not.
    //  o   Variables and arguments cannot have dynamically sized types.
    //  o   Only the last field in a struct may have a dynamically sized type;
    //      the other fields must not. Enum variants must not have dynamically 
    //      sized types as data.
    //
    // So why bother?
    //
    // Well, because [T] can only be used behind a pointer, if we didn’t have 
    // language support for unsized types, it would be impossible to write this:
    //
    // impl Foo for str {
    //
    // or
    //
    // impl<T> Foo for [T] {
    //
    // Instead, you would have to write:
    //
    // impl Foo for &str {
    //
    // Meaning, this implementation would only work for references, and not 
    // other types of pointers. With the impl for str, all pointers, including 
    // (at some point, there are some bugs to fix first) user-defined custom 
    // smart pointers, can use this impl.

    // If you want to write a function that accepts a dynamically sized type
    // you can use the special bound syntax, ?Sized. For example.

    struct Foo<T: ?Sized> {
        f: T
    }

    // This ?Sized, read as “T may or may not be Sized”, which allows us to
    // match both sized and unsized types. All generic type parameters 
    // implicitly have the Sized bound, so the ?Sized can be used to opt-out of
    // the implicit bound.

    println!();
}
