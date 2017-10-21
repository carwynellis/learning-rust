use std::ops::Deref;
use std::rc::Rc;

pub fn deref_coercions() {
    println!("Deref coercions\n");

    // The standard library provides a special trait Deref, which is normally
    // used to overload the dereference operator *. For example...

    struct DerefExample<T> {
        value: T,
    }

    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };

    println!("Dereferenced value: {}", *x);

    // This can be useful for writing custom pointer types.
    // However there is a language feature related to derer, 'deref coercions'.
    // If a type U implements Deref<Target=T> values of &U will automatically
    // coerce to &T. For example with Strings...

    fn foo(s: &str) {
        // Borrow a string for a second.
        println!("Borrowed string: {}", s);
    }

    // String implements Deref<Target=str>.
    let owned = "Hello".to_string();

    // Therefore, this works.
    foo(&owned);

    // Using an ampersand in front of a value takes a reference to it. So owned
    // is a String, &owned is an &String, and since impl Deref<Target=str> for 
    // String, &String will deref to &str, which foo() takes.

    // This is the only place that Rust will carry out an automatic conversion.
    // It can add a lot of flexibility. For example RC implements
    // Deref<Target=T> so a wrapped String will deref to str as follows...

    let another_string = "Counted string".to_string();
    let counted = Rc::new(another_string);
    foo(&counted);

    // Deref will also kick in for method calls.

    // Consider the following example.

    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo!"); }
    }

    let f = &&Foo;

    // Even though f is a &&Foo and foo takes a &self this works, because the 
    // following are equivalent.
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&f).foo();
    // The compiler will derefence the references which means that Deref will
    // be used.

    println!();
}
