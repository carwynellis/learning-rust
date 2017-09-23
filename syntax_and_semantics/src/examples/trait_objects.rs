pub fn trait_objects() {
    println!("Trait Objects\n");

    // Given a simple trait and some implementations...
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("String: {}", *self)
        }
    }

    // ...we can use trait bounds to give us polymorphism using static
    // despatch.
    {
      fn do_something<T: Foo>(x: T) -> String {
          x.method()
      }

      let x = 5u8;
      let y = "Hello".to_string();

      println!("do_something(x) returned {}", do_something(x));
      println!("do_something(y) returned {}", do_something(y));
    }

    println!();

    // Static dispatch is fast since the compiler can inline function calls.
    // However this comes at the cost of duplication of code at call sites.

    // Rust supports dynamic dispatch through a feature called 'trait objects'.
    // A trait object can be obtained from a pointer to a concrete type that
    // implements the type through casting or coercing as shown below.
    // x is cast as Foo
    {
      fn do_something(x: &Foo) -> String {
          x.method()
      }

      let x = 5u8;
      println!("result from casting x as Foo {}", do_something(&x as &Foo));
    }
    // y is coerced into a Foo
    {
      fn do_something(x: &Foo) -> String {
          x.method()
      }

      let y = "Hello".to_string();
      println!("result from casting x as Foo {}", do_something(&y));
    }

    // A function that takes a &Foo is not specialised to each of the types
    // that implements Foo meaning less code is generated. However this comes
    // at the cost of requiring slower virtual function calls.

    // Note - not every trait can be used to make trait objects.
    // For example, Vectors cannot be used to make a trait object since they
    // implement Clone which is not 'object-safe'.

    // A trait if object-safe if the following are true
    //  o the trait does not require that Self: Sized
    //  o all of its methods are object safe
    //
    // A method is object safe if the following are true
    //  o must not have any type parameters
    //  o must not use Self

    println!();
}
