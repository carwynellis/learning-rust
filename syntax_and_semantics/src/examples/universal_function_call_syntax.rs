pub fn universal_function_call_syntax() {
    println!("Universal Function Call Syntax\n");

    // Sometimes instances of functions can have the same name, for example
    // functions defined in traits as follows.
    trait Foo {
            fn f(&self);
    }

    trait Bar {
            fn f(&self);
    }

    struct Baz;

    impl Foo for Baz {
            fn f(&self) { println!("Baz’s impl of Foo"); }
    }

    impl Bar for Baz {
            fn f(&self) { println!("Baz’s impl of Bar"); }
    }

    let b = Baz;

    // Baz has access to two implementations of f, so if we try to call b.f()
    // the compiler would error out with multiple applicable methods in scope.

    // b.f(); // triggers a compiler error

    // In this instance we can work around this by using universal function
    // call syntax to disambiguate the implementations as follows.

    Foo::f(&b);
    Bar::f(&b);

    println!();

    // The invocations above are using a short hand form. A more verbose angle
    // bracket syntax is also available, and is needed in certain situations.
    //
    // <Type as Trait>::method(args);
    //
    // The type provides a type hint for the compiler.

    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);

    println!();
}
