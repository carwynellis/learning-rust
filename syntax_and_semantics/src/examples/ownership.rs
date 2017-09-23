pub fn ownership() {
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
