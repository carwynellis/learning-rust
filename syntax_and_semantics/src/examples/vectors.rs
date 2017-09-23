pub fn vectors() {
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
