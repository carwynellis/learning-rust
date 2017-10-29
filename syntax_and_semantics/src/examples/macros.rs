#![allow(unused_must_use)]
use std::fmt::Write;

pub fn macros() {
    println!("Macros\n");

    // The vec! macro can be used to initialise a vector with an arbitrary
    // number of elements. For example...
    let x = vec![1, 2, 3];

    println!("vec macro generated vector with {} elements", x.capacity());

    // We can implement this ourselves as follows...

    macro_rules! vecExample {
        // Match zero or more tokens separated by commas.
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let y = vecExample![1, 2, 3];

    println!("vecExample macro generated vector with {} elements\n", y.capacity());

    // Note that the implementation of vec! in libcollections differs to that
    // shown above for reasons of efficiency and reusability.

    // Matching

    // Breaking down the macro syntax we first define matchers which operate on
    // Rust syntax trees. These matchers have their own syntax and must match
    // exactly.

    // For example here's a simple macro that matches two key => value pairs.

    macro_rules! foo {
        (x => $e:expr) => (println!("mode X: {}", $e));
        (y => $e:expr) => (println!("mode Y: {}", $e));
    }

    // The following statements will output mode: X 2 and mode: Y 3 respectively.

    foo!(x => 2);
    foo!(y => 3);

    // Attempting to specify anything else, e.g. foo!(z => 4) will result in a
    // compilation error.

    // Expansion

    // The right hand side of a macro rule is ordinary Rust syntax for the most
    // part. But we can splice bits of syntax captured by the matcher. In the
    // vec example above, $( temp_vec.push($x); )* generates a push statement
    // each time the matcher matches, in this case, for each of the values
    // provided when the macro is called.

    // Repetition

    // The repetition operator follows two principle rules
    //
    // 1 $(...)* walks through one "layer" of repetitions, for all of the $names
    //   it contains, in lockstep
    // 2 each $name must be under at least as many $(...)*s as it was matched
    //   against. If it is under more, it’ll be duplicated, as appropriate.

    // The following macro illustrates the duplication of variables from outer
    // repetition levels.

    macro_rules! o_O {
        (
            $(
                $x:expr; [ $( $y:expr ),* ]
            );*
        ) => {
            &[ $($( $x + $y ),*),* ]
        }
    }

    let a: &[i32] = o_O!(10; [1, 2, 3]; 20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    // Hygiene

    // Some languages implement macros using simple text substitution which can
    // lead to unexpected issues.

    // The following macro produces the desired results in Rust without the need
    // for workarounds.

    macro_rules! five_times {
        ($x:expr) => (5 * $x);
    }

    assert_eq!(25, five_times!(2 + 3));

    // A simple text substitution could yield the expression 5 * 2 + 3 which
    // evaluates to 13 due to operator precedence.

    // See docs for further examples.

    // Recursive macros

    // The expansion of a macro may include further macro invocations.
    // This can be useful when processing tree structured input as shown in the
    // following example.
    macro_rules! write_html {
        ($w:expr, ) => (());

        ($w:expr, $e:tt) => (write!($w, "{}", $e));

        ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
            write!($w, "<{}>", stringify!($tag));
            write_html!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            write_html!($w, $($rest)*);
        }};
    }

    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are the best!"]]
        ]);

    println!("\nGenerated HTML: {}", out);

    println!();
}
