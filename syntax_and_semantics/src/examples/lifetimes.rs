pub fn lifetimes() {
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
