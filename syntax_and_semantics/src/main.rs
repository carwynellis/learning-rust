// Some examples contain unused declarations so allow these to pass silently.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;

extern crate syntax_and_semantics;

// Examples from Rust Book - Syntax and Semantics.
// See https://doc.rust-lang.org/book/first-edition/syntax-and-semantics.html
fn main() {
    syntax_and_semantics::examples::variable_bindings::variable_bindings();
    syntax_and_semantics::examples::functions::functions();
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
}

fn primitive_types() {
    println!("Primitive Types\n");

    // Rust has primitive boolean types
    let x = true;
    let y: bool = false;

    println!("x is {}, y is {}", x, y);

    // Char represents a single unicode character
    // Note - char is thus 4 bytes
    let some_char: char = 'c';

    println!("some_char is {}", some_char);

    // Rust has a range of signed and unsigned numeric types.
    // Where no explicit type is provided rust defaults as follows
    let some_int = 42;    // defaults to i32
    let some_float = 1.0; // defaults to f64

    println!("some_int is {}, some_float is {}", some_int, some_float);

    // Arrays
    let nums = [ 1, 2, 3 ];

    println!("nums contains {} elements", nums.len());

    // Rust also provides shorthand for array initialization
    let nums2 = [ 0 ; 20 ];

    println!("nums2 contains {} elements", nums2.len());

    // Array elements are accessed using zero-based subscripts
    let names = [ "Foo", "Bar", "Baz" ];

    println!("third name is {}", names[2]);

    // In rust a slice offers a 'view' into a data structure without
    // dupliating the contents of that structure.
    let a = [ 0, 1, 2, 3, 4 ];

    let slice_all = &a[..]; // refers to all elements of a
    let slice_mid = &a[1..4]; // refers to elements 1, 2, 3

    println!("slice_all has {} elements", slice_all.len());
    println!("slice_mid has {} elements", slice_mid.len());

    // Rust also offers tuples
    let some_tuple = (1, "hello");

    // Elements of a tuple can be accessed using a destructuring let.
    let (t1, t2) = some_tuple;
    println!("some_tuple contains ( {}, {} )", t1, t2);

    // Tuple fields can also be accessed with indexing syntax.
    // Note - like arrays, the indexes are zero based
    println!("second value of some_tuple is {}", some_tuple.1);

    println!();
}

fn if_examples() {
    println!("If\n");

    // In Rust, if operates as you might expect.
    let x = 5;

    if x == 5 {
        println!("x is 5!");
    }
    else if x ==6 {
        println!("x is 6!");
    }
    else {
        println!("x is something else!");
    }

    // However, like in some other languages, if is also and expression which
    // makes the following possible.
    let y = if x == 5 { 10 } else { 0 };

    println!("y is {}", y);

    println!();
}

fn loops() {
    println!("Loops\n");

    // The infinite loop is the simplest form of loop in rust.
    // loop {
    //     println!("Looping!");
    // }

    // Rust also has while loops.
    let mut x = 5;

    while x > 0 {
        println!("while loop: x is {}", x);
        x -= 1;
    }

    println!();

    // For loops are also present, which use a neater syntax than the standard
    // c style loop
    // Note - 0..10 evaluates to 0-9 inclusive, i.e. the upper bound is
    // exclusive.
    for x in 0..10 {
        println!("for loop: x is {}", x);
    }

    println!();

    // Ranges can also be enumerated, if the position within a range is needed.
    for (index, value) in (5..10).enumerate() {
        println!("for with enumeration: value {} - index {}", value, index);

    }

    println!();

    // As can iterators...
    let lines = "First\nSecond\nThird\n".lines();

    for (line_number, line) in lines.enumerate() {
        println!("line {}: {}", line_number, line);
    }

    println!();

    // Ending iteration early.
    // As an alternative to a while loop a loop { } construct can be terminated
    // early by either using break or return.
    let mut x = 0;
    let limit = 5;

    loop {
        println!("for loop: x {}", x);
        x += 1;
        if x > limit {
            println!("x has reached limit {}, breaking loop", limit);
            break;
        }
    }

    println!();

    // Rust also provides a continue keyword that skips to the next iteration.
    // For example the following will only print out even numbers.
    for x in 1..11 {
        if x % 2 == 1 { continue; }
        println!("for loop with continue skipping odd values: {}", x);
    }

    println!();

    // Labelling of nested loops is also supported. The labels can be passed to
    // break and continue. The following will only print the x and y values
    // when both are equal.
    'outer: for x in 1..11 {
        'inner: for y in 1..11 {
            if x % 2 == 1 { continue; }
            if y % 2 == 1 { continue; }
            println!("nested for loops with labels: x {}, y {}", x, y);
        }
    }

    println!();
}

fn vectors() {
    println!("Vectors\n");

    // A vector is a dynamically sized array.
    let mut v = vec![1, 2, 3, 4, 5];

    // Note that vec! also supports initialization of a number of elements to
    // a given value, e.g vec![0, 2] is the same as vec![0, 0]

    // Vectors are stored as contiguous arrays on the heap.

    // Elements of a vector can be accessed in the same way as arrays.
    println!("The third element of v is {}", v[2]);

    println!();

    // Note that vector indexes must be stored using a usize.

    // Out of bounds access.
    // To avoid a thread panic, vectors provide get and getMut accessors that
    // return None if the value does not exist.
    match v.get(10) {
        Some(x) => println!("Value of the requestd element is {}" , x),
        None => println!("No value exists in v at the requested position")
    }

    println!();

    // There are three ways to iterate through a vector.

    // Immutable Reference.
    for i in &v {
        println!("Iterating over a reference to v: {}", i);
    }

    println!();

    // Mutable reference.
    for i in &mut v {
        println!("Iterating over a mutable reference to v: {}", i);
    }

    println!();

    // Taking ownership of the vector and its element.
    for i in v {
        println!("Iterating over v by taking ownership: {}", i);
    }

    println!();

    // Note that once you have taken owenership of the vector it cannot be
    // iterated over again. The reference based approaches allow the vector to
    // be iterated over multiple times.
}

fn ownership() {
    println!("Ownership\n");

    // Ownership can be passed to a function, which is the default behaviour in
    // rust. For example...
    let v = vec![1, 2, 3];

    take(v);

    // At this point we no-longer have ownership of v, and any attempt to
    // access v will result in an error.

    // The following code, if uncommented, will fail at compile time.
    // for i in v {
    //     println!("take iterating over v, at element {}", i);
    // }

    println!();

    // Primitive types implement the Copy trait which assigns a copy of the
    // value, and so ownership is not also moved with assignement.
    let i: i32 = 12;

    println!("doubled value is {}", double_i32(i));
    println!("i is still accessible after being passed, i = {}", i);

    println!();
}

fn take(v: Vec<i32>) {
    println!("take function has ownership of v");

    for i in v {
        println!("take iterating over v, at element {}", i);
    }
}

fn double_i32(i: i32) -> i32 {
    i * 2
}

fn references_and_borrowing() {
    println!("References and Borrowing\n");

    // Immutable references can be borrowed as follows.
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = sum_two_vecs(&v1, &v2);

    println!("Sum of v1 and v2 is {}", answer);

    // By default references are immutable.

    // Mutable references are also supported. For example...
    let mut x = 5;

    // Note that the following only works because we introduce an extra scope.
    {
        let y = &mut x;
        *y += 1;
    }

    println!("x is now {}", x);

    println!();
}

// Folds over a vec to compute the sum.
fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn sum_two_vecs(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    sum_vec(v1) + sum_vec(v2)
}

fn lifetimes() {
    println!("Lifetimes\n");

    // When passing references we can run into ambiguity about whether the
    // referenced resource will still exist at the point that we attempt to use
    // it.

    // Rust provides lifetime annotations which allow the lifetime of
    // references to be explicitly annotated. The compiler can then make use of
    // these annotations to determine whether the code is safe to run or not.

    // Lifetime annotations are usually required for more complex scenarios
    // since the compiler is able to check simpler cases without assistance.

    // Example using function that must declare lifetimes.
    let mut data = vec![1, 2, 3, 4, 5, 6, 7];

    let filtered;

    // We introduce a new scope to create ambiguity about the lifetime of one
    // of the parameters we will pass to a function.
    // This will not compile if the less_than function does not provide
    // lifetime annotations.
    // In this case filtered is in an outer scope relative to limit, which is
    // in an inner scope.
    // The function will need to declare two scopes and associate the result
    // with the scope where it will be used, in this case, the outer scope.
    {
        let limit = 5;
        filtered = less_than(&mut data, &limit);
    }

    println!(
        "Filtered numbers: {}",
        filtered
            .iter()
            .fold(
                String::new(),
                |acc, &elem| if acc.is_empty() { acc + &elem.to_string() } else { acc + ", " + &elem.to_string() }
            )
    );

    println!();
}

// This function must explicitly declare lifetimes for the incoming parameter
// references, and the result.
// In this case we declare that the lifetime of the result will share the outer
// scope of the incoming data parameter.
fn less_than<'outer, 'inner>(data: &'outer mut Vec<i32>, limit: &'inner i32) -> &'outer Vec<i32> {
    data.retain(|&d| d < *limit);
    return data;
}

fn mutability() {
    println!("Mutability\n");

    // Variables are immutable by default.
    // Mutability can be declared with the mut keyword.
    let mut x = 1;
    println!("x is {}", x);

    x = 2;
    println!("x is {} after assignment", x);

    // A reference to a value must also be declared mutable if that reference
    // is to be used to modify the value.
    {
        // Note that ref_x is immutable in that it cannot be changed to point to
        // another resource.
        let ref_x = &mut x;
        *ref_x = 12;
    }

    println!("x is now {} after modification via mutable reference", x);

    // Interior vs Exterior Mutability

    // Immutability refers to the exterior mutability of a resource. The 
    // resource itself can still have mutable interior state. For example.
    let arc = Arc::new(5);
    let arc_clone = arc.clone();
    println!("arc_clone is {}", arc_clone);
    // The Arc (reference counting ponter) instance referred to by x is still
    // able to update its internal state to allocate a reference to y, even
    // though the variable binding, x, is immutable.

    // In Rust, you can think of immutability as 'is this safe to have more
    // than one pointer to it?'.

    // The following example shows interior mutability, that is, obtaining a
    // mutable reference to a value held within another instance.
    // An immutable reference to a RefCell...
    let ref_cell = RefCell::new(42);
    {
        // We can borrow a mutable reference as follows...
        let mut ref_to_cell = ref_cell.borrow_mut();
        // which can be used to modify the value.
        *ref_to_cell = 24;
    }
    println!("ref_cell is now {}", ref_cell.into_inner());

    // Field Level Mutability

    // Mutability is either a property of a borrow or a binding.

    // This means that you cannot have a struct for example with a mixture of
    // immutable and mutable fields.

    // However a cell can be used to emulate field level mutability as follows.
    struct Point {
        a: i32,
        b: Cell<i32>,
    }

    let point = Point { a: 5, b: Cell::new(6) };

    point.b.set(7);

    println!("point after setting mutable field b: Point({}, {:?})", 
        point.a, point.b
    );

    println!();
}

fn structs() {
    println!("Structs\n");

    // As shown above, Structs provide a way to define more complex types.
    // For example we can combine a Point type defining a point in two
    // dimensional space as follows.
    // Note that by convention, struct names are camel cased, with the first
    // letter capitalised.

    struct Point {
        x: i32,
        y: i32
    }

    let some_point = Point { x: 12, y: 54 };

    println!("some_point defined as ({}, {})", some_point.x, some_point.y);

    // Use a mutable binding to a struct to allow mutability of the values
    // contained withing.

    let mut mut_point = Point { x: 1, y: 1 };

    mut_point.x = 100;

    println!(
        "mut_point after modification is ({}, {})",
        mut_point.x,
        mut_point.y
    );

    // Structs support an update syntax using .. which allows a struct to be
    // created using values from another.
    // In this case another_point takes the x value of some_point and uses the
    // y value we specify.

    let another_point = Point { y: 12, .. some_point };

    println!("another_point is ({}, {})", another_point.x, another_point.y);

    // Rust also offers Tuple Structs, a hybrid between tuples and structs.
    // These types have a name, but their fields do not.
    // For example...
    struct Color(i32, i32, i32);

    let red = Color(255, 0, 0);

    println!("red defined as ({}, {}, {})", red.0, red.1, red.2);

    // Tuple structs can be useful in the single element case, since we can
    // give a meaningful type to a single value. Incidentally this is referred
    // to as the 'newtype' pattern.

    struct Inches(i32);

    let length = Inches(10);

    // The value of the type can be extracted using a destructuring let as
    // follows.
    let Inches(length_value) = length;

    // It's always possible to use a struct instead of a tuple struct, and this
    // is typically clearer.

    println!("length is {} inches", length_value);

    // Note, that it's also possible to define a 'unit like' struct, that has
    // no members. This can be handy in specific situations, for example when
    // extending a trait that requires a Struct to be declared that is not
    // needed in the implementation.

    println!();
}

// TODO - unused enum entries generate compiler warnings in this example.
fn enums() {
    println!("Enums\n");

    // Enums in Rust are quite familiar. However one difference is that each
    // entry in the enum may contain additional optional values specific to
    // that entry.
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    // Note that the syntax for each entry is similar to that for structs.
    // Values may optionally be given names as in the Move entry above.

    // A given enum entry is accessed using :: as follows.

    let message = Message::Move { x: 1, y: 2 };

    // Note that the entry names are scoped to the enum type, so different
    // enums can share entry names.

    // Enum values cannot be destructured directly, but values can be extracted
    // using pattern matching.

    println!();
}

fn matching() {
    println!("Match\n");

    // Rust supports matching with exhaustiveness checking.

    // A value can be matched as follows.
    let x = 2;

    match x {
        1   => println!("one"),
        2   => println!("two"),
        3   => println!("three"),
        _   => println!("remaining numbers left as exercise for reader"),
    }

    // The last case is the default case which will be triggered if there were
    // no preceding matches. Without this the compiler will warn that the
    // match is not exhaustive, e.g. there are missing cases that should be
    // declared.

    // Match is also an expression so the result can be bound as follows.
    let number = match x {
        1   => "one",
        2   => "two",
        3   => "three",
        _   => "more typing required",
    };

    println!("{} matched {}", x, number);

    // We can also match on enums, as follows.

    // Redefine the enum from the previous example through the magic of copy
    // and paste.
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    // to_owned creates an owned String from a given string slice.
    // TODO - look into why this is necessary...
    let message = Message::Write("Hallo Welt!".to_owned());

    // Match on the message in a fairly useless way.
    match message {
        Message::Quit                   => println!("Byeee!"),
        Message::ChangeColor(r, g, b)   => println!("Changing color to ({}, {}, {})", r, g, b),
        Message::Move { x, y: y_alias } => println!("Got move x: {} y: {}", x, y_alias),
        Message::Write(message)         => println!("Got message '{}'", message),
    }

    println!();
}

fn patterns() {
    println!("Patterns\n");

    // Patterns are quite common in Rust and are used in variable bindings,
    // match expressions and in other places.

    // Be aware of scope when pattern matching.
    // In the following example the value x within the match block shadows that
    // of the outer scope.

    let x = 1;
    let c = 'c';

    match c {
        x => println!("In match scope. x: {}, c: {}", x, c),
    }

    println!("Out of match scope. x: {}, c: {}", x, c);

    // Multiple patterns may be matched with the | operator.
    match x {
        1 | 2   => println!("x is one or two"),
        _       => println!("x is neither one nor two"),
    }

    // Destructuring

    // Compound data types such as structs, tuples and enums, can all be
    // destructured with patterns.

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("origin: ({}, {})", x, y),
    }

    // : can be used to provide field aliases...
    match origin {
        Point { x: alias_x, y: alias_y } =>
            println!("origin: ({}, {})", alias_x, alias_y),
    }

    // The pattern need only specify the members of interest...
    match origin {
        Point { x, .. } => println!("matched only x - got {}", x),
    }
    // ...and this applies to any member, not just the first.
    match origin {
        Point { y, .. } => println!("matched only y - got {}", y),
    }

    // _ can be used in any pattern to disregard the type and value. For
    // example...

    let result: Result<i32, &'static str> = Ok(12);

    match result {
        Ok(value)   => println!("Got successful result: {}", value),
        // In this pattern we discard the error value.
        Err(_)      => println!("An error occurred"),
    }

    // _ can be used to ignore values in more complex structures.
    let coord = (0, 1, 2);

    match coord {
        (_, _, z) => println!("z component of coord is: {}", z),
    }

    // Note that _ does not create a binding to the value being ignored.

    // You can also obtain a reference to the value being matched.
    let foo = 1;

    match foo {
        ref x => println!("Got a reference to {}", x),
    }

    // These references can also be mutabble.
    let mut bar = 2;

    match bar {
        ref mut x => *x += 1,
    }

    println!("bar is now {}", bar);

    // A range of values can be matched with ...
    let baz = 10;

    match baz {
        1 ... 10    => println!("{} is in the range 1 - 10 inclusive", baz),
        _           => println!("{} is outside of the range 1 - 10", baz),
    }

    // Ranges can also be used with chars.
    let some_char = '€';

    match some_char {
        'a' ... 'z' => println!("{} is a lowercase letter", some_char),
        'A' ... 'Z' => println!("{} is an uppercase letter", some_char),
        _           => println!("{} is not a letter in any case", some_char),
    }

    // You can also bind names to values with the @ operator.

    let another_char = 'c';

    match another_char {
        lower @ 'a' ... 'z' => println!("{} is a lowercase letter", lower),
        other @ _           => println!("{} is not a letter in any case", other),
    }

    // Which is useful when extracting a value from a nested structure.

    struct Person {
        name: Option<String>,
    }

    let person_result: Result<Person, &'static str> =
        Ok(Person { name: Some("Foo Bar".to_string()) });

    match person_result {
        Ok(Person { name: Some(a @ _) }) => println!("Got a person with name '{}'", a),
        _ => println!("Unexpected result"),
    }

    // Note that when using @ with | the name needs to be bound to each pattern.
    let z = 1;

    match z {
        n @ 1 ... 5 | n @ 101 ... 105 => println!("{} in ranges 1-5 or 101-105", n),
        n @ _                         => println!("{} not in any range", n),
    }

    // Matches can also be qualified with guards as follows.
    let int_result: Result<i32, &'static str> = Ok(1);

    match int_result {
        Ok(n) if n < 10 => println!("Got Ok with value {}, less than 10", n),
        Ok(n)           => println!("Got Ok with value {}, greater than 10", n),
        _               => println!("Got something else"),
    }

    println!();
}

fn method_syntax() {
    println!("Method Syntax\n");

    // Rust provides method style syntax via the impl keyword.
    // Consider the following example.

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // Methods take a first parameter which provides access to the instance
        // of the struct.
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };

    println!("Circle with radius: {} has area: {}", c.radius, c.area());

    // Note that multiple impl blocks may be provided as shown below.
    // Method calls can be chained provided that the method returns the struct
    // type. For example...

    impl Circle {
        // Grow method takes an increment that returns a new circle instance.
        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }
    }

    let increment = 2.0;

    println!(
        "Circle with radius: {} has area: {}",
        c.radius + increment,
        c.grow(increment).area(),
    );

    // Note that you can also define associated functions, that is functions
    // that do not take a self parameter. This is commonly used in rust to
    // implement constructors. For example...

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    // Note that associated functions are called using the :: syntax.
    let c2 = Circle::new(0.0, 0.0, 1.0);

    println!("Circle built with new function has area: {}", c2.area());

    // This pattern can also be extended to implement builders as shown below.

    // First we start with a struct that defines the properties to be used to
    // create a new circle.
    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    // We can then implement a builder using this struct that will ultimately
    // create a Circle instance.
    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder {
                // Define some default values.
                x: 0.0,
                y: 0.0,
                radius: 1.0,
            }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }

        fn finalize(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }

    // The builder can then be used to create a new Circle as follows.
    let c3 = CircleBuilder::new()
        .x(2.3)
        .y(4.5)
        .radius(50.0)
        .finalize();

    println!("Circle from builder has x: {}, y: {} and area: {}",
        c3.x,
        c3.y,
        c3.area(),
    );

    println!();
}

fn strings() {
    println!("Strings\n");

    // Strings are encoded as a stream of UTF-8 bytes and are not NUL
    // terminated so may include the NUL byte.

    // Rust has two main types of string, &str and String.

    // &str is also referred to as a string slice. A string slice is immutable.

    let greeting = "Hello there!"; // greeting: &'static str

    // Greeting is a binding to a string slice that is statically allocated.
    // Any function accepting a string slice, will also accept a string 
    // literal.

    // String literals can span multiple lines with or without preservation of
    // white space.
    let literal_with_whitespace = "This is a
multiline
  string
    with preservation of whitespace,
        including newlines.
";

    println!(
        "literal string with whitespace preserved: '{}'",
        literal_with_whitespace
    );

    // To ignore white space and newlines, delimit lines with a trailing \
    let literal_without_whitespace = "Foo\
        Bar\
        Baz";

    println!(
        "literal string without whitespace: '{}'",
        literal_without_whitespace
    );

    println!();

    // Note that you cannot normally access an str directly because it is an
    // unsized type which requires additional runtime information to be usable.
    // See unsized types.

    // Rust also has Strings which are heap allocated. Strings are growable.

    // Strings can be created by calling the to_string method on str.

    let mut some_string = "Hello".to_string();

    println!("some_string: {}", some_string);

    // some_string can be grown by appending another string to it.
    some_string.push_str(" World!");

    println!("some_string after push_str: {}", some_string);
    println!();

    // Strings can be passed to functions expecting a str by prefixing with &.

    // Note, coercing a String into an str is cheap, but creating a String from
    // an str requires memory to be allocated. So, only do this when absolutely
    // necessary.

    // Because strings are UTF-8 they do not support indexing.
    // E.g let s = "Hello"; let t = s[0]; // this will fail
    // Unicode doesn't strictly define a 'letter' so it's not easy to support
    // indexing of individual UTF-8 characters.

    // Strings can however be examined as individual bytes.
    let another_string = "Some characters";

    println!("String '{}' contains the bytes....", another_string);

    for b in another_string.as_bytes() {
        print!("{}, ", b);
    }

    println!("\n");

    // Strings can be sliced, for example...

    let first_five = &another_string[0..5];

    println!("First five characters of another_string are '{}'\n", first_five);

    // However this will fail at runtime if character boundaries are not
    // preserved.

    // An str can be concatenated to a String using the + operator.

    let hello = "Hello ".to_string();
    let world = "World!";

    // Note that these values are 'moved' after this call.
    let hello_world = hello + world;

    println!("Concatenation result is: {}", hello_world);

    // Two Strings can be concatenated, but the second must be prefixed with &.

    println!();
}

fn generics() {
    println!("Generics\n");

    // The Rust standard library makes use of generics, which provide
    // parametric polymorphism. For example the Option makes use of generics.
    let x: Option<i32> = Some(5);

    // Functions can be declared generic with as many types as required.
    let y = simple_generic_function(12);
    let z = multiple_typed_generic_function(1, "Foo");

    println!("simple_generic_function returned {}", y);
    println!("multiple_typed_generic_function returned {}", z);

    // Generic types can also be declared for structs.
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 12, y: 42 };
    let float_point = Point { x: 1.2, y: 4.2 };

    // It then follows that struct impls also support generic types.
    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    // Note that explicit type annotations will be required where the compiler
    // is unable to resolve ambiguous types.

    println!();
}

fn simple_generic_function<T>(x: T) -> T {
    return x;
}

fn multiple_typed_generic_function<A, B>(a: A, b: B) -> A {
    return a;
}

fn traits() {
    println!("Traits\n");

    // A trait tells the Rust compiler about functionality a type must provide.
    // Traits are similar to method syntax, except that now a trait is used to
    // define the method signature.
    struct Circle {
        radius: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // The area method is now available for Circle instances.

    let c = Circle { radius: 12.0 };

    println!("Circle with radius: {} has area: {}", c.radius, c.area());

    // Note - Self may be used in a type annotation to refer to an instance of
    // the type implementing this trait passed as a parameter. Self, &Self or
    // &mut Self may be used depending on the level of ownership required.

    // For example implementing an IsLarger trait using Self references.
    trait IsLarger {
        fn is_larger(&self, &Self) -> bool;
    }

    impl IsLarger for Circle {
        fn is_larger(&self, other: &Self) -> bool {
            self.area() > other.area()
        }
    }

    let d = Circle { radius: 24.0 };

    println!(
        "Circle with radius: {} is larger than circle with radius: {} - {}",
        d.radius,
        c.radius,
        d.is_larger(&c)
    );

    // Generic functions can declare trait bounds on the types they accept.
    // This allows a generic function to accept only values that implement a
    // given trait.

    // We can use this with the has area trait to ensure that a generic function
    // that prints the area of a given shape, will only accept arguemtns that
    // provide a HasArea impl.

    // So using the HasArea trait above we can also define a square as follows.
    struct Square {
       side_length: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side_length * self.side_length
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("shape has area: {}", shape.area());
    }

    // The print_area will accept any instance that provides a HasArea impl.
    let circle = Circle { radius: 100.0 };
    let square = Square { side_length: 100.0 };

    println!("\nCircle print_area");
    print_area(circle);
    println!("\nSquare print_area");
    print_area(square);

    // Trait bounds can also be declared on impl declarations for generic structs.

    // Multiple trait bounds may be declared using the + operator e.g.
    // fn foo<T: bar + baz>(something: T)

    // where can be used instead to make things a little easier to read.

    // The example above becomes
    //  fn foo<T>(something: T) where T: bar + baz

    // This is particularly useful when there are several types with bounds in
    // play.

    // Traits can be implemented for any type, such as f32.

    // Traits will only apply if they are in scope.

    // Default methods can also be defined in the trait declaration.

    // If an implemented trait depends on another trait then the implementor
    // must provide an implementation for both. For example...
    trait Foo {
        fn foo(&self);
    }

    trait FooBar : Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }

    let baz = Baz {};

    println!("\nTrait inheritence example");
    baz.foo();
    baz.foobar();

    // Rust can also derive implementations for a limited set of traits
    // provided in the standard library.
    // TODO - look into this and provide a decent example.

    println!();
}

fn drop() {
    println!("Drop\n");

    // Rust provides a trait, Drop, that will run some code when a value goes
    // out of scope.

    // This is useful for handling and cleanup, freeing of resources etc.. when
    // a value is no-longer in use.

    struct Firework {
        strength: i32,
    }

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }

    fn create_fireworks() {
        println!("Declaring firework strength: 1");
        let firecracker = Firework { strength: 1 };
        println!("Declaring firework strength: 100");
        let tnt = Firework { strength: 100 };
        // Both instances fall out of scope here, triggering the drop code.
    }

    // Note that drop invocations occur in the reverse order of declaration.
    // Calling create_fireworks will output 100 and then 1, which is the
    // reverse of the order of declaration.
    create_fireworks();

    // This makes sense since later declarations may depend on earlier
    // declarations. So, the cleanup order should the reverse of the
    // declaration order.

    println!();
}

fn if_let_while_let() {
    println!("if let and while let\n");

    // if let permits pattern matching within the condition of an if statement.
    // This can make certain statements more concise.

    // For example, consider working with an optional value.
    let something = Some(1);

    // Typically we only need to do something when there is some value.

    // We can match on the value...
    match something {
        Some(x) => println!("Got value {} when matching on the option", x),
        None    => {}
    }

    // ...or perhaps test if the value is present before doing something.
    if something.is_some() {
        let x = something.unwrap();
        println!("Got value {} when testing if the option has a value", x);
    }

    // However both approaches involve a degree of boiler plate.
    // Using if let we can express the same thing as follows...
    if let Some(x) = something {
        println!("Got value {} from option when using if let", x);
    }

    // If the pattern matches any appropriate values are bound to the specified
    // identifiers which can then be used within the subsequent block.
    // Note that you can also use else to take an alternative action should the
    // pattern not match.
    if let None = something {
        println!("the option does not contain a value");
    }
    else {
        println!("the option contains a value");
    }

    println!();

    // Along the same lines, while let can be used to loop until a pattern does
    // not match anymore.
    let mut v = vec![1, 3, 5, 7, 11];

    while let Some(x) = v.pop() {
        println!("while let - v.pop returned Some({})", x);
    }

    // In both cases, you need only express the happy path if that's all you
    // care about.

    println!();
}

fn trait_objects() {
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

fn closures() {
    println!("Closures\n");

    // A simple closure looks like this
    let plus_one = |x: i32| x + 1;

    // Arguments to the closure are delimited by pipes.
    // We can also create multi-line closures, for example in the following
    // rather contrived example.
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    }; // Note - you really do need the semi-colon

    println!("{} plus_one is {}", 2, plus_one(2));
    println!("{} plus_two is {}", 2, plus_two(2));

    // Note that closures do not require type annotations for arguments or the
    // return type which differs from function definitions.

    // The environment for a closure can include bindings from its enclosing
    // scope, in addition to parameters and local bindings.
    // For example...
    let num = 5;
    let plus_num = |x| x + num;

    println!("{} plus_num {} is {}", 2, num, plus_num(2));

    // The borrow checker will ensure that mutable references to bindings used
    // by a closure cannot be taken in later declarations.

    // A move keyword is also available that allows a closure to take ownership
    // of a copy of a binding.

    // For example, the following will mutate the mutable binding as you would
    // expect.
    let mut y = 5;

    {
        // The closure here takes a mutable reference to y so is able to modify
        // the value declared outside the scope of this block.
        let mut add_y = |x| y += x;
        add_y(5);
    }

    println!("y after call to add_y(5) is {}", y);

    // However, using the move keyword we get the following.
    let mut z = 5;
    {
        let mut add_z = move |x| z += x;
        add_z(5);
    }

    println!("z after call to move add_z(5) is {}", z);

    // add_z modifies a copy of z so the declaration in the outer scope is left
    // unmodified.

    // Another way to think about move closures is that they have their own
    // stack frame. Without a move, the closure may be tied to the stack frame
    // that created it. This means that generally you cannot return a non-move
    // closure from a function.

    // Further information about the underlying implementation and how to
    // return a closure is available at
    // https://doc.rust-lang.org/book/first-edition/closures.html

    println!();
}

fn universal_function_call_syntax() {
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
